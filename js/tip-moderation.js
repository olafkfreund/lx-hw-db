/**
 * Community Tip Validation and Moderation System
 * Handles quality control and community review of submitted tips
 */

class TipModerationSystem {
    constructor() {
        this.isInitialized = false;
        this.moderationQueue = [];
        this.validationRules = this.setupValidationRules();
        this.communityReviewers = new Set();
        this.moderationHistory = [];
    }

    /**
     * Initialize the moderation system
     */
    async initialize() {
        console.log('Initializing tip moderation system...');
        
        await this.loadModerationData();
        this.setupModerationInterface();
        this.isInitialized = true;
        
        console.log('Tip moderation system initialized successfully');
    }

    /**
     * Set up validation rules for submitted tips
     */
    setupValidationRules() {
        return {
            title: {
                minLength: 5,
                maxLength: 100,
                required: true,
                pattern: /^[a-zA-Z0-9\s\-_:()\/]+$/,
                forbiddenWords: ['spam', 'scam', 'hack', 'exploit']
            },
            description: {
                minLength: 20,
                maxLength: 500,
                required: true,
                forbiddenWords: ['buy now', 'click here', 'download crack']
            },
            commands: {
                maxCount: 10,
                maxLength: 500,
                forbiddenPatterns: [
                    /rm\s+-rf\s+\//, // Dangerous rm commands
                    /dd\s+if=.*of=\/dev/, // Dangerous dd commands
                    /wget.*\|\s*sh/, // Pipe to shell
                    /curl.*\|\s*bash/, // Pipe to bash
                    /chmod\s+777/, // Overly permissive permissions
                    /sudo\s+su/, // Unnecessary privilege escalation
                ]
            },
            configuration: {
                maxFiles: 5,
                maxFileSize: 2048,
                allowedPaths: [
                    '/etc/modprobe.d/',
                    '/etc/udev/rules.d/',
                    '/etc/X11/',
                    '/etc/systemd/',
                    '/etc/nixos/',
                    '~/.config/'
                ]
            }
        };
    }

    /**
     * Validate a submitted tip
     */
    async validateTip(tipData) {
        const validationResult = {
            isValid: true,
            errors: [],
            warnings: [],
            securityFlags: [],
            qualityScore: 0
        };

        // Title validation
        if (!this.validateTitle(tipData.title, validationResult)) {
            validationResult.isValid = false;
        }

        // Description validation
        if (!this.validateDescription(tipData.description, validationResult)) {
            validationResult.isValid = false;
        }

        // Commands validation (security critical)
        if (!this.validateCommands(tipData.distributions, validationResult)) {
            validationResult.isValid = false;
        }

        // Configuration files validation
        if (!this.validateConfiguration(tipData.distributions, validationResult)) {
            validationResult.isValid = false;
        }

        // Calculate quality score
        validationResult.qualityScore = this.calculateQualityScore(tipData, validationResult);

        return validationResult;
    }

    /**
     * Validate tip title
     */
    validateTitle(title, result) {
        const rules = this.validationRules.title;
        
        if (!title || title.length < rules.minLength) {
            result.errors.push(`Title must be at least ${rules.minLength} characters long`);
            return false;
        }

        if (title.length > rules.maxLength) {
            result.errors.push(`Title must not exceed ${rules.maxLength} characters`);
            return false;
        }

        if (!rules.pattern.test(title)) {
            result.errors.push('Title contains invalid characters');
            return false;
        }

        // Check for forbidden words
        const lowerTitle = title.toLowerCase();
        const forbiddenFound = rules.forbiddenWords.find(word => 
            lowerTitle.includes(word.toLowerCase())
        );
        
        if (forbiddenFound) {
            result.errors.push(`Title contains forbidden word: ${forbiddenFound}`);
            return false;
        }

        return true;
    }

    /**
     * Validate tip description
     */
    validateDescription(description, result) {
        const rules = this.validationRules.description;
        
        if (!description || description.length < rules.minLength) {
            result.errors.push(`Description must be at least ${rules.minLength} characters long`);
            return false;
        }

        if (description.length > rules.maxLength) {
            result.errors.push(`Description must not exceed ${rules.maxLength} characters`);
            return false;
        }

        // Check for spammy content
        const lowerDescription = description.toLowerCase();
        const forbiddenFound = rules.forbiddenWords.find(word => 
            lowerDescription.includes(word.toLowerCase())
        );
        
        if (forbiddenFound) {
            result.errors.push(`Description contains suspicious content: ${forbiddenFound}`);
            return false;
        }

        return true;
    }

    /**
     * Validate commands for security issues
     */
    validateCommands(distributions, result) {
        const rules = this.validationRules.commands;
        let isValid = true;

        Object.entries(distributions).forEach(([distro, distroData]) => {
            if (!distroData.commands) return;

            // Check command count
            if (distroData.commands.length > rules.maxCount) {
                result.warnings.push(`Too many commands for ${distro} (max ${rules.maxCount})`);
            }

            distroData.commands.forEach((cmdObj, index) => {
                const command = cmdObj.command;
                
                // Check command length
                if (command.length > rules.maxLength) {
                    result.warnings.push(`Command ${index + 1} for ${distro} is too long`);
                }

                // Security validation - check for dangerous patterns
                rules.forbiddenPatterns.forEach(pattern => {
                    if (pattern.test(command)) {
                        result.securityFlags.push(`Potentially dangerous command in ${distro}: ${command}`);
                        result.errors.push(`Security violation: Command contains dangerous pattern`);
                        isValid = false;
                    }
                });

                // Additional security checks
                if (this.containsSuspiciousCommands(command)) {
                    result.securityFlags.push(`Suspicious command detected in ${distro}: ${command}`);
                    result.warnings.push('Command flagged for manual review');
                }
            });
        });

        return isValid;
    }

    /**
     * Validate configuration files
     */
    validateConfiguration(distributions, result) {
        const rules = this.validationRules.configuration;
        let isValid = true;

        Object.entries(distributions).forEach(([distro, distroData]) => {
            if (!distroData.configurations) return;

            // Check file count
            if (distroData.configurations.length > rules.maxFiles) {
                result.warnings.push(`Too many config files for ${distro} (max ${rules.maxFiles})`);
            }

            distroData.configurations.forEach(configObj => {
                const filePath = configObj.file;
                const content = configObj.content;

                // Validate file path
                const isAllowedPath = rules.allowedPaths.some(allowedPath => 
                    filePath.startsWith(allowedPath)
                );

                if (!isAllowedPath) {
                    result.errors.push(`Configuration file path not allowed: ${filePath}`);
                    result.securityFlags.push(`Suspicious file path: ${filePath}`);
                    isValid = false;
                }

                // Check content size
                if (content.length > rules.maxFileSize) {
                    result.warnings.push(`Configuration file too large: ${filePath}`);
                }

                // Basic security check for config content
                if (this.containsSuspiciousConfig(content)) {
                    result.warnings.push(`Configuration file flagged for review: ${filePath}`);
                }
            });
        });

        return isValid;
    }

    /**
     * Check for suspicious commands
     */
    containsSuspiciousCommands(command) {
        const suspiciousPatterns = [
            /nc\s+.*-l.*-e/, // Netcat reverse shells
            /python.*-c.*exec/, // Python exec commands
            /base64.*decode/, // Base64 decode (could be obfuscated)
            /wget.*\$\(.*\)/, // Command substitution in wget
            /curl.*eval/, // Curl with eval
            /\$\(curl.*\)/, // Command substitution with curl
            /chmod\s+\+x.*tmp/, // Making temp files executable
        ];

        return suspiciousPatterns.some(pattern => pattern.test(command));
    }

    /**
     * Check for suspicious configuration content
     */
    containsSuspiciousConfig(content) {
        const suspiciousPatterns = [
            /password\s*=\s*[^#\n\r]+/, // Hardcoded passwords
            /key\s*=\s*[A-Za-z0-9+\/]{20,}/, // API keys or secrets
            /192\.168\.|10\.|172\./, // Private IP ranges (could be internal info)
            /\$\{.*\}/, // Variable substitution that could be malicious
        ];

        return suspiciousPatterns.some(pattern => pattern.test(content));
    }

    /**
     * Calculate quality score for the tip
     */
    calculateQualityScore(tipData, validationResult) {
        let score = 70; // Base score

        // Positive factors
        if (tipData.description.length > 100) score += 10;
        if (tipData.warnings && tipData.warnings.length > 0) score += 5;
        if (tipData.contributor) score += 10;
        
        // Count distribution coverage
        const distroCount = Object.keys(tipData.distributions).length;
        score += distroCount * 5;

        // Commands with descriptions
        let commandsWithDesc = 0;
        Object.values(tipData.distributions).forEach(distro => {
            if (distro.commands) {
                commandsWithDesc += distro.commands.filter(cmd => cmd.description).length;
            }
        });
        score += commandsWithDesc * 2;

        // Negative factors
        score -= validationResult.errors.length * 10;
        score -= validationResult.warnings.length * 3;
        score -= validationResult.securityFlags.length * 15;

        return Math.max(0, Math.min(100, score));
    }

    /**
     * Add tip to moderation queue
     */
    async queueTipForReview(tipData, validationResult) {
        const moderationItem = {
            id: 'mod_' + Date.now(),
            tip: tipData,
            validation: validationResult,
            status: 'pending',
            submittedAt: new Date().toISOString(),
            reviewedBy: null,
            reviewedAt: null,
            reviewNotes: null,
            communityVotes: {
                approve: 0,
                reject: 0,
                needsWork: 0
            }
        };

        // Auto-approve high-quality tips with no security issues
        if (validationResult.qualityScore >= 85 && 
            validationResult.securityFlags.length === 0 && 
            validationResult.errors.length === 0) {
            moderationItem.status = 'auto-approved';
            moderationItem.reviewedAt = new Date().toISOString();
            moderationItem.reviewNotes = 'Auto-approved based on high quality score and security validation';
        }
        
        // Auto-reject tips with security violations
        else if (validationResult.securityFlags.length > 0 || validationResult.errors.length > 2) {
            moderationItem.status = 'auto-rejected';
            moderationItem.reviewedAt = new Date().toISOString();
            moderationItem.reviewNotes = 'Auto-rejected due to security concerns or validation errors';
        }

        this.moderationQueue.push(moderationItem);
        this.saveModerationData();

        return moderationItem;
    }

    /**
     * Process community review vote
     */
    async processCommunityVote(moderationId, vote, reviewerId) {
        const item = this.moderationQueue.find(item => item.id === moderationId);
        if (!item) {
            throw new Error('Moderation item not found');
        }

        if (item.status !== 'pending') {
            throw new Error('Moderation item is not pending review');
        }

        // Add reviewer to community reviewers set
        this.communityReviewers.add(reviewerId);

        // Record vote
        if (['approve', 'reject', 'needsWork'].includes(vote)) {
            item.communityVotes[vote]++;
        }

        // Check if we have enough votes to make a decision
        const totalVotes = Object.values(item.communityVotes).reduce((a, b) => a + b, 0);
        
        if (totalVotes >= 3) { // Require at least 3 community votes
            const approveVotes = item.communityVotes.approve;
            const rejectVotes = item.communityVotes.reject;
            const needsWorkVotes = item.communityVotes.needsWork;

            if (approveVotes >= 2) {
                item.status = 'community-approved';
                item.reviewedAt = new Date().toISOString();
                item.reviewNotes = `Community approved (${approveVotes} approve, ${rejectVotes} reject, ${needsWorkVotes} needs work)`;
            } else if (rejectVotes >= 2) {
                item.status = 'community-rejected';
                item.reviewedAt = new Date().toISOString();
                item.reviewNotes = `Community rejected (${approveVotes} approve, ${rejectVotes} reject, ${needsWorkVotes} needs work)`;
            } else if (needsWorkVotes >= 2) {
                item.status = 'needs-work';
                item.reviewedAt = new Date().toISOString();
                item.reviewNotes = `Community marked as needs work (${approveVotes} approve, ${rejectVotes} reject, ${needsWorkVotes} needs work)`;
            }
        }

        this.saveModerationData();
        return item;
    }

    /**
     * Get moderation statistics
     */
    getModerationStats() {
        const stats = {
            totalSubmissions: this.moderationQueue.length,
            pending: this.moderationQueue.filter(item => item.status === 'pending').length,
            approved: this.moderationQueue.filter(item => 
                item.status === 'auto-approved' || item.status === 'community-approved'
            ).length,
            rejected: this.moderationQueue.filter(item => 
                item.status === 'auto-rejected' || item.status === 'community-rejected'
            ).length,
            needsWork: this.moderationQueue.filter(item => item.status === 'needs-work').length,
            averageQualityScore: this.calculateAverageQualityScore(),
            communityReviewers: this.communityReviewers.size
        };

        return stats;
    }

    /**
     * Calculate average quality score
     */
    calculateAverageQualityScore() {
        if (this.moderationQueue.length === 0) return 0;
        
        const totalScore = this.moderationQueue.reduce((sum, item) => 
            sum + (item.validation.qualityScore || 0), 0
        );
        
        return Math.round(totalScore / this.moderationQueue.length);
    }

    /**
     * Set up moderation interface
     */
    setupModerationInterface() {
        // Add moderation controls to admin interface
        this.createModerationDashboard();
        this.setupEventListeners();
    }

    /**
     * Create moderation dashboard
     */
    createModerationDashboard() {
        // This would create an admin interface for reviewing tips
        // For now, we'll just log the dashboard creation
        console.log('Moderation dashboard created');
        
        // In a real implementation, this would create HTML elements for:
        // - Moderation queue display
        // - Individual tip review interface
        // - Community voting interface
        // - Statistics dashboard
        // - Review history
    }

    /**
     * Set up event listeners for moderation actions
     */
    setupEventListeners() {
        document.addEventListener('tipSubmitted', async (event) => {
            const tipData = event.detail;
            
            // Validate the submitted tip
            const validationResult = await this.validateTip(tipData);
            
            // Queue for moderation
            const moderationItem = await this.queueTipForReview(tipData, validationResult);
            
            // Emit moderation event
            document.dispatchEvent(new CustomEvent('tipQueued', {
                detail: {
                    moderationItem,
                    validationResult
                }
            }));
        });
    }

    /**
     * Load moderation data (mock implementation)
     */
    async loadModerationData() {
        // In production, this would load from a database
        const savedData = localStorage.getItem('lx-hw-db-moderation');
        if (savedData) {
            try {
                const data = JSON.parse(savedData);
                this.moderationQueue = data.moderationQueue || [];
                this.communityReviewers = new Set(data.communityReviewers || []);
            } catch (error) {
                console.error('Error loading moderation data:', error);
            }
        }
    }

    /**
     * Save moderation data (mock implementation)
     */
    saveModerationData() {
        // In production, this would save to a database
        const data = {
            moderationQueue: this.moderationQueue,
            communityReviewers: Array.from(this.communityReviewers)
        };
        localStorage.setItem('lx-hw-db-moderation', JSON.stringify(data));
    }

    /**
     * Get pending moderation items
     */
    getPendingModerationItems() {
        return this.moderationQueue.filter(item => item.status === 'pending');
    }

    /**
     * Get approved tips ready for publication
     */
    getApprovedTips() {
        return this.moderationQueue.filter(item => 
            item.status === 'auto-approved' || item.status === 'community-approved'
        ).map(item => item.tip);
    }

    /**
     * Manual moderator approval/rejection
     */
    async moderatorReview(moderationId, decision, reviewerId, notes) {
        const item = this.moderationQueue.find(item => item.id === moderationId);
        if (!item) {
            throw new Error('Moderation item not found');
        }

        item.status = decision; // 'approved', 'rejected', 'needs-work'
        item.reviewedBy = reviewerId;
        item.reviewedAt = new Date().toISOString();
        item.reviewNotes = notes;

        this.saveModerationData();
        return item;
    }
}

// Global instance
window.tipModerationSystem = new TipModerationSystem();

// Auto-initialize when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
    await window.tipModerationSystem.initialize();
});

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = TipModerationSystem;
}