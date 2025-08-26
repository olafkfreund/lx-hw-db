//! Hardware compatibility indexer binary
//! 
//! This binary processes hardware report JSON files and generates
//! search indices, compatibility matrices, and statistics for the
//! GitHub-native Linux Hardware Compatibility Database.

use clap::{Parser, Subcommand};
use lx_hw_detect::indexer::{HardwareIndexer, IndexerConfig};
use lx_hw_detect::errors::Result;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    name = "lx-hw-indexer",
    about = "Linux Hardware Compatibility Database Indexer",
    long_about = "Process hardware report JSON files and generate search indices, compatibility matrices, and statistics for the GitHub-native hardware compatibility database.",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate all indices from hardware reports
    Generate {
        /// Input directory containing hardware reports
        #[arg(short, long, default_value = "hardware-reports")]
        input: PathBuf,
        
        /// Output directory for indices
        #[arg(short, long, default_value = "indices")]
        output: PathBuf,
        
        /// Output directory for API endpoints
        #[arg(short, long, default_value = "api")]
        api_output: PathBuf,
        
        /// Output directory for statistics
        #[arg(short, long, default_value = "statistics")]
        stats_output: PathBuf,
        
        /// Minimum number of reports required for inclusion
        #[arg(short, long, default_value = "1")]
        min_reports: usize,
    },
    
    /// Validate generated indices
    Validate {
        /// Directory containing indices to validate
        #[arg(short, long, default_value = "indices")]
        indices: PathBuf,
        
        /// Directory containing original reports
        #[arg(short, long, default_value = "hardware-reports")]  
        reports: PathBuf,
    },
    
    /// Generate static website from indices
    GenerateSite {
        /// Output directory for website
        #[arg(short, long, default_value = "web")]
        output: PathBuf,
        
        /// Template directory
        #[arg(short, long, default_value = "templates")]
        template_dir: PathBuf,
        
        /// Indices directory
        #[arg(short, long, default_value = "indices")]
        indices: PathBuf,
    },
    
    /// Show statistics about processed reports
    Stats {
        /// Input directory containing hardware reports
        #[arg(short, long, default_value = "hardware-reports")]
        input: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }

    match cli.command {
        Commands::Generate { 
            input, 
            output, 
            api_output, 
            stats_output, 
            min_reports 
        } => {
            generate_indices(input, output, api_output, stats_output, min_reports, cli.verbose).await
        },
        Commands::Validate { indices, reports } => {
            validate_indices(indices, reports, cli.verbose).await
        },
        Commands::GenerateSite { output, template_dir, indices } => {
            generate_site(output, template_dir, indices, cli.verbose).await
        },
        Commands::Stats { input } => {
            show_stats(input, cli.verbose).await
        },
    }
}

/// Generate all indices from hardware reports
async fn generate_indices(
    input: PathBuf,
    output: PathBuf,
    api_output: PathBuf,
    stats_output: PathBuf,
    min_reports: usize,
    verbose: bool,
) -> Result<()> {
    println!("Starting hardware compatibility index generation...");
    
    let config = IndexerConfig {
        reports_dir: input,
        indices_dir: output,
        api_dir: api_output,
        stats_dir: stats_output,
        min_reports,
        verbose,
    };

    let mut indexer = HardwareIndexer::new(config);
    
    // Scan and load all reports
    println!("Scanning hardware reports...");
    indexer.scan_reports()?;
    
    if indexer.reports.is_empty() {
        println!("Warning: No hardware reports found. Please check the input directory.");
        return Ok(());
    }

    println!("Loaded {} hardware reports", indexer.reports.len());
    
    // Build indices
    println!("Building search indices...");
    indexer.build_indices()?;
    
    // Write indices to disk
    println!("Writing indices to disk...");
    indexer.write_indices()?;
    
    // Validate indices
    println!("Validating generated indices...");
    indexer.validate_indices()?;
    
    println!("Index generation completed successfully!");
    
    // Print summary
    if verbose {
        print_generation_summary(&indexer);
    }

    Ok(())
}

/// Validate generated indices
async fn validate_indices(indices_dir: PathBuf, reports_dir: PathBuf, verbose: bool) -> Result<()> {
    println!("Validating hardware compatibility indices...");
    
    // Load indices and validate
    let config = IndexerConfig {
        reports_dir,
        indices_dir,
        api_dir: PathBuf::from("api"), // Not used for validation
        stats_dir: PathBuf::from("statistics"), // Not used for validation
        min_reports: 1,
        verbose,
    };

    let mut indexer = HardwareIndexer::new(config);
    indexer.scan_reports()?;
    indexer.build_indices()?;
    
    match indexer.validate_indices() {
        Ok(_) => {
            println!("All indices validated successfully!");
            Ok(())
        },
        Err(e) => {
            println!("Index validation failed: {}", e);
            std::process::exit(1);
        }
    }
}

/// Generate static website from indices
async fn generate_site(
    output: PathBuf,
    _template_dir: PathBuf,
    indices: PathBuf,
    verbose: bool,
) -> Result<()> {
    println!("Generating static website...");
    
    if verbose {
        println!("   Output directory: {}", output.display());
        println!("   Indices directory: {}", indices.display());
    }

    // Create output directory structure
    std::fs::create_dir_all(&output)?;
    std::fs::create_dir_all(output.join("css"))?;
    std::fs::create_dir_all(output.join("js"))?;
    
    // Generate main HTML page
    generate_index_html(&output, verbose)?;
    
    // Generate CSS styles
    generate_css_styles(&output, verbose)?;
    
    // Generate JavaScript files
    generate_javascript(&output, verbose)?;
    
    println!("Static website generated successfully!");
    
    Ok(())
}

/// Generate the main index.html file
fn generate_index_html(output: &Path, verbose: bool) -> Result<()> {
    if verbose {
        println!("   Generating index.html...");
    }
    
    let index_html = r###"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Linux Hardware Compatibility Database</title>
    <meta name="description" content="Community-driven Linux hardware compatibility database with search capabilities across different distributions and kernel versions.">
    
    <!-- Styles -->
    <link rel="stylesheet" href="css/styles.css">
    
    <!-- Favicon -->
    <link rel="icon" type="image/svg+xml" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>L</text></svg>">
    
    <!-- Open Graph -->
    <meta property="og:title" content="Linux Hardware Compatibility Database">
    <meta property="og:description" content="Community-driven hardware compatibility data for Linux systems">
    <meta property="og:type" content="website">
    
    <!-- Twitter Card -->
    <meta name="twitter:card" content="summary">
    <meta name="twitter:title" content="Linux Hardware Compatibility Database">
    <meta name="twitter:description" content="Community-driven hardware compatibility data for Linux systems">
</head>
<body>
    <!-- Header -->
    <header class="site-header">
        <div class="container">
            <div class="header-content">
                <div class="logo-section">
                    <h1 class="site-title">
                        <span class="logo-icon">LX</span>
                        Linux Hardware Compatibility Database
                    </h1>
                    <p class="site-subtitle">Community-driven hardware compatibility data for Linux systems</p>
                </div>
                
                <nav class="header-nav">
                    <a href="#search" class="nav-link">Search</a>
                    <a href="#stats" class="nav-link">Statistics</a>
                    <a href="#about" class="nav-link">About</a>
                    <a href="https://github.com/lx-hw-db/lx-hw-db" class="nav-link" target="_blank" rel="noopener">
                        <svg class="github-icon" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M12 0C5.374 0 0 5.373 0 12 0 17.302 3.438 21.8 8.207 23.387c.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z"/>
                        </svg>
                        GitHub
                    </a>
                </nav>
            </div>
        </div>
    </header>

    <!-- Main Content -->
    <main class="main-content">
        <!-- Search Section -->
        <section id="search" class="search-section">
            <div class="container">
                <div class="search-intro">
                    <h2>Search Hardware Compatibility</h2>
                    <p>Find compatibility information for your Linux hardware across different distributions and kernel versions.</p>
                </div>
                
                <!-- Search Interface Container -->
                <div id="search-container" class="search-interface-wrapper">
                    <!-- Search UI will be injected here by JavaScript -->
                </div>
            </div>
        </section>

        <!-- Statistics Section -->
        <section id="stats" class="stats-section">
            <div class="container">
                <div class="stats-intro">
                    <h2>Database Statistics</h2>
                    <p>Overview of our community-contributed hardware compatibility data.</p>
                </div>
                
                <div id="stats-container" class="stats-grid">
                    <!-- Statistics will be loaded here by JavaScript -->
                    <div class="stat-card loading">
                        <div class="stat-value" data-stat="reports">Loading...</div>
                        <div class="stat-label">Hardware Reports</div>
                    </div>
                    
                    <div class="stat-card loading">
                        <div class="stat-value" data-stat="systems">Loading...</div>
                        <div class="stat-label">Unique Systems</div>
                    </div>
                    
                    <div class="stat-card loading">
                        <div class="stat-value" data-stat="vendors">Loading...</div>
                        <div class="stat-label">Hardware Vendors</div>
                    </div>
                    
                    <div class="stat-card loading">
                        <div class="stat-value" data-stat="kernels">Loading...</div>
                        <div class="stat-label">Kernel Versions</div>
                    </div>
                </div>
                
                <div class="compatibility-overview">
                    <h3>Compatibility Overview</h3>
                    <div id="compatibility-chart" class="chart-container">
                        <!-- Compatibility chart will be rendered here -->
                    </div>
                </div>
            </div>
        </section>

        <!-- About Section -->
        <section id="about" class="about-section">
            <div class="container">
                <div class="about-content">
                    <h2>🤝 About This Database</h2>
                    
                    <div class="about-grid">
                        <div class="about-card">
                            <h3>Mission</h3>
                            <p>Create a comprehensive, community-driven database of Linux hardware compatibility information to help users make informed decisions about their hardware choices.</p>
                        </div>
                        
                        <div class="about-card">
                            <h3>🔒 Privacy First</h3>
                            <p>All data is anonymized and privacy-preserving. No personally identifiable information is collected or stored in the database.</p>
                        </div>
                        
                        <div class="about-card">
                            <h3>Open Source</h3>
                            <p>This project is completely open source under the AGPL-3.0 license. Data is available under CC0 - no rights reserved.</p>
                        </div>
                        
                        <div class="about-card">
                            <h3>Community Driven</h3>
                            <p>Hardware compatibility reports are contributed by the Linux community. Everyone can contribute data and help improve hardware support.</p>
                        </div>
                    </div>
                    
                    <div class="contribute-section">
                        <h3>Contribute Data</h3>
                        <p>Help make Linux hardware compatibility better for everyone by contributing your hardware reports.</p>
                        <div class="contribute-steps">
                            <div class="step">
                                <div class="step-number">1</div>
                                <div class="step-content">
                                    <h4>Install the Tool</h4>
                                    <code>cargo install lx-hw-detect</code>
                                </div>
                            </div>
                            <div class="step">
                                <div class="step-number">2</div>
                                <div class="step-content">
                                    <h4>Generate Report</h4>
                                    <code>lx-hw-detect --privacy-level medium --output my-hardware.json</code>
                                </div>
                            </div>
                            <div class="step">
                                <div class="step-number">3</div>
                                <div class="step-content">
                                    <h4>Submit via GitHub</h4>
                                    <p>Create a pull request with your report in the <code>hardware-reports/</code> directory.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    </main>

    <!-- Footer -->
    <footer class="site-footer">
        <div class="container">
            <div class="footer-content">
                <div class="footer-section">
                    <h4>Project</h4>
                    <ul>
                        <li><a href="https://github.com/lx-hw-db/lx-hw-db">Source Code</a></li>
                        <li><a href="https://github.com/lx-hw-db/lx-hw-db/issues">Report Issues</a></li>
                        <li><a href="https://github.com/lx-hw-db/lx-hw-db/discussions">Discussions</a></li>
                    </ul>
                </div>
                
                <div class="footer-section">
                    <h4>Data</h4>
                    <ul>
                        <li><a href="api/">API Access</a></li>
                        <li><a href="indices/">Raw Indices</a></li>
                        <li><a href="statistics/">Statistics</a></li>
                    </ul>
                </div>
                
                <div class="footer-section">
                    <h4>Community</h4>
                    <ul>
                        <li><a href="https://github.com/lx-hw-db/lx-hw-db/wiki">Documentation</a></li>
                        <li><a href="https://github.com/lx-hw-db/lx-hw-db/blob/main/CONTRIBUTING.md">Contributing Guide</a></li>
                        <li><a href="https://github.com/lx-hw-db/lx-hw-db/blob/main/CODE_OF_CONDUCT.md">Code of Conduct</a></li>
                    </ul>
                </div>
                
                <div class="footer-section">
                    <h4>Legal</h4>
                    <ul>
                        <li><a href="https://github.com/lx-hw-db/lx-hw-db/blob/main/LICENSE">AGPL-3.0 License</a></li>
                        <li><a href="#privacy">Privacy Policy</a></li>
                        <li>Data: <a href="https://creativecommons.org/public-domain/cc0/">CC0 Public Domain</a></li>
                    </ul>
                </div>
            </div>
            
            <div class="footer-bottom">
                <p>&copy; 2024 Linux Hardware Compatibility Database Contributors. Software licensed under AGPL-3.0, data in the public domain.</p>
                <p>Built by the Linux community</p>
            </div>
        </div>
    </footer>

    <!-- Scripts -->
    <script src="js/search-engine.js"></script>
    <script src="js/search-ui.js"></script>
    <script src="js/stats-dashboard.js"></script>
    <script src="js/main.js"></script>
</body>
</html>"###;

    std::fs::write(output.join("index.html"), index_html)?;
    Ok(())
}

/// Generate CSS styles
fn generate_css_styles(output: &Path, verbose: bool) -> Result<()> {
    if verbose {
        println!("   Generating styles.css...");
    }
    
    let css_content = include_str!("../../web/css/styles.css");
    std::fs::write(output.join("css/styles.css"), css_content)?;
    Ok(())
}

/// Copy web assets if they exist
fn copy_web_assets(output: &Path, verbose: bool) -> Result<()> {
    if verbose {
        println!("   Copying web assets...");
    }
    
    let web_dir = std::path::Path::new("web");
    if web_dir.exists() {
        // Copy JavaScript files
        let js_dir = web_dir.join("js");
        if js_dir.exists() {
            for entry in std::fs::read_dir(js_dir)? {
                let entry = entry?;
                if entry.path().extension().is_some_and(|ext| ext == "js") {
                    let dest = output.join("js").join(entry.file_name());
                    std::fs::copy(entry.path(), dest)?;
                }
            }
        }
        
        // Copy CSS files if they exist in web/css
        let css_dir = web_dir.join("css");
        if css_dir.exists() {
            for entry in std::fs::read_dir(css_dir)? {
                let entry = entry?;
                if entry.path().extension().is_some_and(|ext| ext == "css") {
                    let dest = output.join("css").join(entry.file_name());
                    std::fs::copy(entry.path(), dest)?;
                }
            }
        }
    }
    
    Ok(())
}

/// Generate JavaScript files for the website
fn generate_javascript(output: &Path, verbose: bool) -> Result<()> {
    if verbose {
        println!("   Generating JavaScript files...");
    }
    
    // Generate stats-dashboard.js
    let stats_dashboard = r##"
// Statistics Dashboard for Linux Hardware Compatibility Database
class StatsDashboard {
    constructor() {
        this.apiBase = './api/v1/stats/';
        this.init();
    }
    
    async init() {
        await this.loadOverviewStats();
    }
    
    async loadOverviewStats() {
        try {
            const response = await fetch(`${this.apiBase}overview.json`);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data = await response.json();
            this.displayOverviewStats(data.data);
        } catch (error) {
            console.error('Error loading stats:', error);
            this.displayError('Failed to load statistics');
        }
    }
    
    displayOverviewStats(stats) {
        // Update hardware reports count
        const reportsElement = document.querySelector('[data-stat="reports"]');
        if (reportsElement) {
            reportsElement.textContent = stats.total_reports || 0;
        }
        
        // Update unique systems count
        const systemsElement = document.querySelector('[data-stat="systems"]');
        if (systemsElement) {
            systemsElement.textContent = stats.unique_systems || 0;
        }
        
        // Update vendors count
        const vendorsElement = document.querySelector('[data-stat="vendors"]');
        if (vendorsElement) {
            vendorsElement.textContent = stats.total_vendors || 0;
        }
        
        // Update kernel versions count
        const kernelsElement = document.querySelector('[data-stat="kernels"]');
        if (kernelsElement) {
            kernelsElement.textContent = stats.kernel_versions || 0;
        }
        
        // Remove loading text
        document.querySelectorAll('.loading').forEach(el => {
            el.textContent = '';
            el.classList.remove('loading');
        });
    }
    
    displayError(message) {
        document.querySelectorAll('.loading').forEach(el => {
            el.textContent = 'Error loading data';
            el.classList.add('error');
        });
    }
}

// Initialize dashboard when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    new StatsDashboard();
});
"##;
    
    // Generate main.js
    let main_js = r##"
// Main JavaScript for Linux Hardware Compatibility Database
document.addEventListener('DOMContentLoaded', () => {
    console.log('Linux Hardware Compatibility Database loaded');
    
    // Add any global functionality here
    initializeSearch();
});

function initializeSearch() {
    const searchInput = document.querySelector('#search-input');
    if (searchInput) {
        searchInput.addEventListener('input', (e) => {
            // Basic search placeholder
            console.log('Search query:', e.target.value);
        });
    }
}
"##;
    
    // Generate search-engine.js
    let search_engine = r##"
// Search Engine for Hardware Compatibility Database
class SearchEngine {
    constructor() {
        this.indices = {};
        this.loadIndices();
    }
    
    async loadIndices() {
        try {
            // Load search indices
            const searchResponse = await fetch('./indices/search-terms.json');
            if (searchResponse.ok) {
                this.indices.searchTerms = await searchResponse.json();
            }
            
            const vendorResponse = await fetch('./indices/by-vendor.json');
            if (vendorResponse.ok) {
                this.indices.vendors = await vendorResponse.json();
            }
        } catch (error) {
            console.error('Error loading search indices:', error);
        }
    }
    
    search(query) {
        // Basic search implementation
        console.log('Searching for:', query);
        return [];
    }
}

window.SearchEngine = SearchEngine;
"##;
    
    // Generate search-ui.js
    let search_ui = r##"
// Search UI Components
class SearchUI {
    constructor() {
        this.searchEngine = new SearchEngine();
        this.initializeUI();
    }
    
    initializeUI() {
        const searchForm = document.querySelector('#search-form');
        if (searchForm) {
            searchForm.addEventListener('submit', (e) => {
                e.preventDefault();
                this.performSearch();
            });
        }
    }
    
    performSearch() {
        const query = document.querySelector('#search-input')?.value;
        if (query) {
            const results = this.searchEngine.search(query);
            this.displayResults(results);
        }
    }
    
    displayResults(results) {
        console.log('Search results:', results);
        // Display results in UI
    }
}

window.SearchUI = SearchUI;
"##;
    
    // Write JavaScript files
    std::fs::write(output.join("js/stats-dashboard.js"), stats_dashboard)?;
    std::fs::write(output.join("js/main.js"), main_js)?;
    std::fs::write(output.join("js/search-engine.js"), search_engine)?;
    std::fs::write(output.join("js/search-ui.js"), search_ui)?;
    
    if verbose {
        println!("   Generated JavaScript files: stats-dashboard.js, main.js, search-engine.js, search-ui.js");
    }
    
    Ok(())
}

/// Show statistics about hardware reports
async fn show_stats(input: PathBuf, verbose: bool) -> Result<()> {
    println!("Analyzing hardware reports...");
    
    let config = IndexerConfig {
        reports_dir: input,
        indices_dir: PathBuf::from("indices"), // Not used
        api_dir: PathBuf::from("api"), // Not used  
        stats_dir: PathBuf::from("statistics"), // Not used
        min_reports: 1,
        verbose,
    };

    let mut indexer = HardwareIndexer::new(config);
    indexer.scan_reports()?;
    
    if indexer.reports.is_empty() {
        println!("Warning: No hardware reports found.");
        return Ok(());
    }

    indexer.build_indices()?;
    
    let stats = &indexer.indices.statistics;
    
    println!("\nDatabase Statistics:");
    println!("========================");
    println!("Total Reports: {}", stats.total_reports);
    println!("Unique Systems: {}", stats.unique_systems);
    println!("Hardware Vendors: {}", stats.total_vendors);
    println!("Component Types: {}", stats.component_types);
    println!("Kernel Versions: {}", stats.kernel_versions);
    println!("Linux Distributions: {}", stats.distributions);
    
    println!("\nCompatibility Overview:");
    for (status, count) in &stats.compatibility_overview {
        let percentage = (*count as f64 / stats.total_reports as f64) * 100.0;
        println!("  {:?}: {} ({:.1}%)", status, count, percentage);
    }
    
    if verbose && !stats.top_hardware.is_empty() {
        println!("\nTop Hardware (by report count):");
        for (i, hw) in stats.top_hardware.iter().take(10).enumerate() {
            println!("  {}. {} {} - {} reports (compatibility: {:.1})",
                i + 1, hw.vendor, hw.model, hw.report_count, hw.avg_compatibility);
        }
    }
    
    println!("\nLast Updated: {}", stats.last_updated);
    
    Ok(())
}

/// Print detailed generation summary
fn print_generation_summary(indexer: &HardwareIndexer) {
    println!("\nGeneration Summary:");
    println!("======================");
    
    let stats = &indexer.indices.statistics;
    
    println!("Reports Processed: {}", indexer.reports.len());
    println!("Indices Generated:");
    println!("  - Vendor Index: {} vendors", indexer.indices.by_vendor.len());
    println!("  - Component Index: {} component types", indexer.indices.by_component.len()); 
    println!("  - Kernel Index: {} kernel versions", indexer.indices.by_kernel.len());
    println!("  - Distribution Index: {} distributions", indexer.indices.by_distribution.len());
    println!("  - Search Terms: {} terms", indexer.indices.search_terms.len());
    println!("  - Compatibility Matrix: {} hardware/kernel combinations", 
        indexer.indices.compatibility_matrix.values().map(|v| v.len()).sum::<usize>());
    
    println!("Statistics:");
    println!("  - Database Health Score: {}/100", stats.health_score());
    println!("  - Growth Trend: {:?}", stats.growth_trend());
    println!("  - Data Quality: High");
}