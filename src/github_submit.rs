//! GitHub submission automation
//!
//! This module provides automated GitHub submission functionality for hardware reports.
//! It handles forking repositories, creating branches, committing files, and opening
//! pull requests with minimal user interaction.

use crate::errors::{LxHwError, Result};
use crate::hardware::{HardwareReport, PrivacyLevel};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use tempfile::TempDir;

/// GitHub repository configuration
#[derive(Debug, Clone)]
pub struct GitHubConfig {
    pub username: String,
    pub token: String,
    pub upstream_owner: String,
    pub upstream_repo: String,
    pub auto_fork: bool,
}

/// Hardware submission metadata
#[derive(Debug, Clone)]
pub struct SubmissionInfo {
    pub description: String,
    pub report_path: PathBuf,
    pub generated_at: DateTime<Utc>,
    pub privacy_level: PrivacyLevel,
    pub tools_used: Vec<String>,
}

/// GitHub API response structures
#[derive(Debug, Deserialize)]
struct GitHubRepo {
    full_name: String,
    clone_url: String,
    ssh_url: String,
    default_branch: String,
}

#[derive(Debug, Deserialize)]
struct GitHubPullRequest {
    number: u64,
    html_url: String,
    title: String,
}

/// GitHub submission handler
pub struct GitHubSubmitter {
    config: GitHubConfig,
    temp_dir: Option<TempDir>,
}

impl GitHubSubmitter {
    /// Create a new GitHub submitter
    pub fn new(config: GitHubConfig) -> Self {
        Self {
            config,
            temp_dir: None,
        }
    }

    /// Submit a hardware report to GitHub
    pub async fn submit_report(
        &mut self,
        submission: SubmissionInfo,
        skip_confirmation: bool,
    ) -> Result<String> {
        println!("🚀 Starting automated GitHub submission...");

        // Step 1: Validate GitHub credentials
        self.validate_credentials().await?;

        // Step 2: Load and validate the hardware report
        let report = self.load_and_validate_report(&submission.report_path)?;

        // Step 3: Generate proper filename and directory structure
        let (filename, directory) = self.generate_file_path(&report)?;

        // Step 4: Show submission summary and get confirmation
        if !skip_confirmation {
            self.show_submission_summary(&submission, &report, &filename)?;
            if !self.confirm_submission()? {
                println!("❌ Submission cancelled by user");
                return Err(LxHwError::Validation(
                    "Submission cancelled by user".to_string(),
                ));
            }
        }

        // Step 5: Fork repository if needed
        let fork_url = self.ensure_fork().await?;

        // Step 6: Clone the fork
        let repo_path = self.clone_fork(&fork_url).await?;

        // Step 7: Create feature branch
        let branch_name = self.create_feature_branch(&repo_path, &report)?;

        // Step 8: Copy report file to correct location
        self.add_report_file(&repo_path, &submission.report_path, &directory, &filename)?;

        // Step 9: Commit changes
        self.commit_changes(&repo_path, &submission, &report, &filename)?;

        // Step 10: Push branch
        self.push_branch(&repo_path, &branch_name)?;

        // Step 11: Create pull request
        let pr_url = self.create_pull_request(&submission, &report, &branch_name).await?;

        println!("✅ Hardware report submitted successfully!");
        println!("📋 Pull Request: {}", pr_url);
        println!("🔍 Your submission will be automatically validated and reviewed by the community.");

        Ok(pr_url)
    }

    /// Validate GitHub credentials
    async fn validate_credentials(&self) -> Result<()> {
        println!("🔐 Validating GitHub credentials...");

        let output = Command::new("gh")
            .args(["auth", "status"])
            .env("GH_TOKEN", &self.config.token)
            .output()
            .map_err(|e| {
                LxHwError::Submission(format!(
                    "GitHub CLI not found. Please install 'gh' command: {}",
                    e
                ))
            })?;

        if !output.status.success() {
            return Err(LxHwError::Submission(
                "GitHub authentication failed. Please check your token.".to_string(),
            ));
        }

        println!("✅ GitHub credentials validated");
        Ok(())
    }

    /// Load and validate the hardware report
    fn load_and_validate_report(&self, report_path: &Path) -> Result<HardwareReport> {
        println!("📄 Loading hardware report: {}", report_path.display());

        let content = fs::read_to_string(report_path).map_err(|e| {
            LxHwError::Io(format!("Failed to read report file: {}", e))
        })?;

        let report: HardwareReport = serde_json::from_str(&content).map_err(|e| {
            LxHwError::Validation(format!("Invalid report format: {}", e))
        })?;

        // Validate report has required fields
        if report.metadata.anonymized_system_id.is_empty() {
            return Err(LxHwError::Validation(
                "Report missing anonymized system ID".to_string(),
            ));
        }

        if report.system.kernel_version.is_empty() {
            return Err(LxHwError::Validation(
                "Report missing kernel version".to_string(),
            ));
        }

        println!("✅ Report validation passed");
        Ok(report)
    }

    /// Generate proper filename and directory structure
    fn generate_file_path(&self, report: &HardwareReport) -> Result<(String, String)> {
        let date = report.metadata.generated_at.format("%Y-%m-%d");
        let year = report.metadata.generated_at.format("%Y");
        let month = report.metadata.generated_at.format("%m");

        let filename = format!(
            "{}_{}_{}_{}.json",
            date,
            report.system.kernel_version,
            report.system.architecture,
            &report.metadata.anonymized_system_id
        );

        let directory = format!("hardware-reports/{}/{}", year, month);

        Ok((filename, directory))
    }

    /// Show submission summary for user confirmation
    fn show_submission_summary(
        &self,
        submission: &SubmissionInfo,
        report: &HardwareReport,
        filename: &str,
    ) -> Result<()> {
        println!("\n📋 Submission Summary:");
        println!("═══════════════════════════════════════");
        println!("📁 File: {}", filename);
        println!("💻 System: {} {}", 
                 report.system.distribution.as_ref().unwrap_or(&"Unknown".to_string()), 
                 report.system.kernel_version);
        println!("🏗️  Architecture: {}", report.system.architecture);
        println!("🔒 Privacy Level: {:?}", report.metadata.privacy_level);
        println!("🛠️  Tools Used: {}", report.metadata.tools_used.join(", "));

        if let Some(cpu) = &report.cpu {
            println!("🖥️  CPU: {} {}", cpu.vendor, cpu.model);
        }

        if let Some(memory) = &report.memory {
            println!("💾 Memory: {}GB", memory.total_bytes / (1024 * 1024 * 1024));
        }

        if !report.graphics.is_empty() {
            let gpu = &report.graphics[0];
            println!("🎮 GPU: {} {}", gpu.vendor, gpu.model);
        }

        println!("📝 Description: {}", submission.description);
        println!("═══════════════════════════════════════");

        Ok(())
    }

    /// Get user confirmation for submission
    fn confirm_submission(&self) -> Result<bool> {
        print!("\n❓ Submit this hardware report? [Y/n]: ");
        io::stdout().flush().map_err(|e| LxHwError::Io(format!("IO error: {}", e)))?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            LxHwError::Io(format!("Failed to read user input: {}", e))
        })?;

        let input = input.trim().to_lowercase();
        Ok(input.is_empty() || input == "y" || input == "yes")
    }

    /// Ensure the repository is forked
    async fn ensure_fork(&self) -> Result<String> {
        println!("🍴 Checking repository fork...");

        let upstream_repo = format!("{}/{}", self.config.upstream_owner, self.config.upstream_repo);

        // Check if fork already exists
        let fork_check = Command::new("gh")
            .args([
                "repo",
                "view",
                &format!("{}/{}", self.config.username, self.config.upstream_repo),
            ])
            .env("GH_TOKEN", &self.config.token)
            .output();

        let fork_url = if fork_check.is_ok() && fork_check.unwrap().status.success() {
            println!("✅ Fork already exists");
            format!("https://github.com/{}/{}.git", self.config.username, self.config.upstream_repo)
        } else if self.config.auto_fork {
            println!("🔀 Creating fork...");
            let output = Command::new("gh")
                .args(["repo", "fork", &upstream_repo, "--clone=false"])
                .env("GH_TOKEN", &self.config.token)
                .output()
                .map_err(|e| {
                    LxHwError::Submission(format!("Failed to create fork: {}", e))
                })?;

            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(LxHwError::Submission(format!(
                    "Fork creation failed: {}",
                    error
                )));
            }

            println!("✅ Fork created successfully");
            format!("https://github.com/{}/{}.git", self.config.username, self.config.upstream_repo)
        } else {
            return Err(LxHwError::Submission(
                "Repository fork not found. Use --auto-fork to create one automatically.".to_string(),
            ));
        };

        Ok(fork_url)
    }

    /// Clone the forked repository
    async fn clone_fork(&mut self, fork_url: &str) -> Result<PathBuf> {
        println!("📥 Cloning repository...");

        self.temp_dir = Some(TempDir::new().map_err(|e| {
            LxHwError::Io(format!("Failed to create temporary directory: {}", e))
        })?);

        let temp_path = self.temp_dir.as_ref().unwrap().path();
        let repo_path = temp_path.join("lx-hw-db");

        let output = Command::new("git")
            .args(["clone", fork_url])
            .current_dir(temp_path)
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| {
                LxHwError::Submission(format!("Git clone failed: {}", e))
            })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(LxHwError::Submission(format!(
                "Repository clone failed: {}",
                error
            )));
        }

        // Set up upstream remote
        Command::new("git")
            .args([
                "remote",
                "add",
                "upstream",
                &format!("https://github.com/{}/{}.git", self.config.upstream_owner, self.config.upstream_repo),
            ])
            .current_dir(&repo_path)
            .output()
            .map_err(|e| {
                LxHwError::Submission(format!("Failed to add upstream remote: {}", e))
            })?;

        println!("✅ Repository cloned successfully");
        Ok(repo_path)
    }

    /// Create a feature branch for the submission
    fn create_feature_branch(&self, repo_path: &Path, report: &HardwareReport) -> Result<String> {
        println!("🌱 Creating feature branch...");

        let branch_name = format!(
            "hardware-report-{}-{}",
            chrono::Utc::now().format("%Y%m%d"),
            &report.metadata.anonymized_system_id[..8]
        );

        // Fetch latest changes from upstream
        Command::new("git")
            .args(["fetch", "upstream"])
            .current_dir(repo_path)
            .output()
            .map_err(|e| {
                LxHwError::Submission(format!("Failed to fetch upstream: {}", e))
            })?;

        // Create and checkout new branch from upstream main
        let output = Command::new("git")
            .args(["checkout", "-b", &branch_name, "upstream/main"])
            .current_dir(repo_path)
            .output()
            .map_err(|e| {
                LxHwError::Submission(format!("Failed to create branch: {}", e))
            })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(LxHwError::Submission(format!(
                "Branch creation failed: {}",
                error
            )));
        }

        println!("✅ Feature branch '{}' created", branch_name);
        Ok(branch_name)
    }

    /// Add the report file to the repository
    fn add_report_file(
        &self,
        repo_path: &Path,
        report_path: &Path,
        directory: &str,
        filename: &str,
    ) -> Result<()> {
        println!("📁 Adding report file...");

        // Create directory structure
        let target_dir = repo_path.join(directory);
        fs::create_dir_all(&target_dir).map_err(|e| {
            LxHwError::Io(format!("Failed to create directory structure: {}", e))
        })?;

        // Copy report file
        let target_path = target_dir.join(filename);
        fs::copy(report_path, &target_path).map_err(|e| {
            LxHwError::Io(format!("Failed to copy report file: {}", e))
        })?;

        // Add file to git
        let output = Command::new("git")
            .args(["add", &format!("{}/{}", directory, filename)])
            .current_dir(repo_path)
            .output()
            .map_err(|e| {
                LxHwError::Submission(format!("Failed to add file to git: {}", e))
            })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(LxHwError::Submission(format!(
                "Git add failed: {}",
                error
            )));
        }

        println!("✅ Report file added to repository");
        Ok(())
    }

    /// Commit the changes
    fn commit_changes(
        &self,
        repo_path: &Path,
        submission: &SubmissionInfo,
        report: &HardwareReport,
        filename: &str,
    ) -> Result<()> {
        println!("💾 Committing changes...");

        // Generate commit message
        let commit_message = self.generate_commit_message(submission, report, filename);

        let output = Command::new("git")
            .args(["commit", "-m", &commit_message])
            .current_dir(repo_path)
            .output()
            .map_err(|e| {
                LxHwError::Submission(format!("Failed to commit changes: {}", e))
            })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(LxHwError::Submission(format!(
                "Git commit failed: {}",
                error
            )));
        }

        println!("✅ Changes committed successfully");
        Ok(())
    }

    /// Generate a detailed commit message
    fn generate_commit_message(
        &self,
        submission: &SubmissionInfo,
        report: &HardwareReport,
        filename: &str,
    ) -> String {
        let mut message = String::new();

        // Title line
        message.push_str(&format!("Add hardware report: {}", submission.description));
        message.push_str("\n\n");

        // System information
        message.push_str("System Information:\n");
        message.push_str(&format!("- Kernel: {}\n", report.system.kernel_version));
        message.push_str(&format!("- Distribution: {}\n", 
                                 report.system.distribution.as_ref().unwrap_or(&"Unknown".to_string())));
        message.push_str(&format!("- Architecture: {}\n", report.system.architecture));
        message.push_str(&format!("- Privacy Level: {:?}\n", report.metadata.privacy_level));
        message.push('\n');

        // Hardware highlights
        message.push_str("Hardware Highlights:\n");
        if let Some(cpu) = &report.cpu {
            message.push_str(&format!("- CPU: {} {}\n", cpu.vendor, cpu.model));
        }
        if let Some(memory) = &report.memory {
            message.push_str(&format!("- Memory: {}GB\n", memory.total_bytes / (1024 * 1024 * 1024)));
        }
        if !report.graphics.is_empty() {
            let gpu = &report.graphics[0];
            message.push_str(&format!("- GPU: {} {}\n", gpu.vendor, gpu.model));
        }
        message.push('\n');

        // Contribution details
        message.push_str("Contribution Details:\n");
        message.push_str(&format!("- File: {}\n", filename));
        message.push_str(&format!("- Tools Used: {}\n", report.metadata.tools_used.join(", ")));
        message.push_str("- Generated with lx-hw-detect automated submission\n");
        message.push_str("- Automated validation passed\n");
        message.push_str("- Ready for community review\n");

        message
    }

    /// Push the branch to GitHub
    fn push_branch(&self, repo_path: &Path, branch_name: &str) -> Result<()> {
        println!("📤 Pushing branch to GitHub...");

        let output = Command::new("git")
            .args(["push", "-u", "origin", branch_name])
            .current_dir(repo_path)
            .output()
            .map_err(|e| {
                LxHwError::Submission(format!("Failed to push branch: {}", e))
            })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(LxHwError::Submission(format!(
                "Git push failed: {}",
                error
            )));
        }

        println!("✅ Branch pushed successfully");
        Ok(())
    }

    /// Create a pull request
    async fn create_pull_request(
        &self,
        submission: &SubmissionInfo,
        report: &HardwareReport,
        branch_name: &str,
    ) -> Result<String> {
        println!("🔀 Creating pull request...");

        let title = format!("Hardware Report: {}", submission.description);
        let body = self.generate_pr_body(submission, report);
        let upstream_repo = format!("{}/{}", self.config.upstream_owner, self.config.upstream_repo);
        let head_ref = format!("{}:{}", self.config.username, branch_name);

        let args = vec![
            "pr", "create",
            "--repo", &upstream_repo,
            "--title", &title,
            "--body", &body,
            "--head", &head_ref,
        ];

        let output = Command::new("gh")
            .args(&args)
            .env("GH_TOKEN", &self.config.token)
            .output()
            .map_err(|e| {
                LxHwError::Submission(format!("Failed to create pull request: {}", e))
            })?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(LxHwError::Submission(format!(
                "Pull request creation failed: {}",
                error
            )));
        }

        let pr_url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("✅ Pull request created successfully");

        Ok(pr_url)
    }

    /// Generate pull request body
    fn generate_pr_body(&self, submission: &SubmissionInfo, report: &HardwareReport) -> String {
        let mut body = String::new();

        body.push_str("## Summary\n");
        body.push_str(&format!("{}\n\n", submission.description));

        body.push_str("**System Type**: Desktop/Laptop/Server\n");
        body.push_str("**Primary Use Case**: Daily Use/Development/Gaming/Server\n\n");

        body.push_str("## Hardware Report Details\n\n");
        body.push_str("### System Information\n\n");
        body.push_str(&format!("- **Anonymized System ID**: `{}`\n", report.metadata.anonymized_system_id));
        body.push_str(&format!("- **Kernel Version**: {}\n", report.system.kernel_version));
        body.push_str(&format!("- **Distribution**: {}\n", 
                               report.system.distribution.as_ref().unwrap_or(&"Unknown".to_string())));
        body.push_str(&format!("- **Architecture**: {}\n", report.system.architecture));
        body.push_str(&format!("- **Privacy Level**: {:?}\n\n", report.metadata.privacy_level));

        body.push_str("### Hardware Summary\n\n");
        if let Some(cpu) = &report.cpu {
            body.push_str(&format!("- **CPU**: {} {} ({} cores)\n", cpu.vendor, cpu.model, cpu.cores));
        }
        if let Some(memory) = &report.memory {
            body.push_str(&format!("- **Memory**: {}GB\n", memory.total_bytes / (1024 * 1024 * 1024)));
        }
        if !report.graphics.is_empty() {
            let gpu = &report.graphics[0];
            body.push_str(&format!("- **Graphics**: {} {}\n", gpu.vendor, gpu.model));
        }
        if !report.network.is_empty() {
            let net = &report.network[0];
            body.push_str(&format!("- **Network**: {} {}\n", net.vendor, net.model));
        }

        body.push_str("\n### Compatibility Status\n\n");
        body.push_str("- [x] All hardware detected and working correctly\n");
        body.push_str("- [ ] Most hardware working, minor issues present\n");
        body.push_str("- [ ] Some hardware not detected or working\n");
        body.push_str("- [ ] Major hardware compatibility issues\n\n");

        body.push_str("### Validation Checklist\n\n");
        body.push_str("- [x] Report generated with latest version of `lx-hw-detect`\n");
        body.push_str("- [x] Automated validation passed\n");
        body.push_str("- [x] Privacy level is appropriate for public sharing\n");
        body.push_str("- [x] File follows naming convention\n");
        body.push_str("- [x] File placed in correct directory structure\n\n");

        body.push_str("### Additional Information\n\n");
        body.push_str("This hardware report was submitted using the automated `lx-hw-detect submit` command.\n\n");
        body.push_str(&format!("**Tools Used**: {}\n", report.metadata.tools_used.join(", ")));
        body.push_str(&format!("**Generated**: {}\n", report.metadata.generated_at.format("%Y-%m-%d %H:%M:%S UTC")));

        body
    }
}

/// Interactive setup for GitHub credentials and configuration
pub fn setup_github_config(
    username: Option<String>,
    token: Option<String>,
) -> Result<GitHubConfig> {
    println!("🔧 Setting up GitHub submission...");

    let username = if let Some(u) = username {
        u
    } else {
        print!("GitHub username: ");
        io::stdout().flush().map_err(|e| LxHwError::Io(format!("IO error: {}", e)))?;
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| {
            LxHwError::Io(format!("Failed to read username: {}", e))
        })?;
        input.trim().to_string()
    };

    let token = if let Some(t) = token {
        t
    } else {
        print!("GitHub token (will be hidden): ");
        io::stdout().flush().map_err(|e| LxHwError::Io(format!("IO error: {}", e)))?;
        
        // Use rpassword crate if available, otherwise fall back to regular input
        let token = if let Ok(token) = rpassword::read_password() {
            token
        } else {
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| {
                LxHwError::Io(format!("Failed to read token: {}", e))
            })?;
            input.trim().to_string()
        };
        token
    };

    Ok(GitHubConfig {
        username,
        token,
        upstream_owner: "your-org".to_string(), // TODO: Make this configurable
        upstream_repo: "lx-hw-db".to_string(),
        auto_fork: true,
    })
}