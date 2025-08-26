/**
 * Community Tip Submission System
 * Allows users to contribute hardware configuration tips
 */

class TipSubmissionSystem {
    constructor() {
        this.isInitialized = false;
        this.currentSubmission = null;
        this.submissionModal = null;
        
        // Tip categories and their descriptions
        this.categories = {
            'driver_installation': {
                name: 'Driver Installation',
                description: 'Installing and setting up hardware drivers',
                icon: '📦'
            },
            'performance_tuning': {
                name: 'Performance Tuning',
                description: 'Optimizing hardware performance and settings',
                icon: '⚡'
            },
            'troubleshooting': {
                name: 'Troubleshooting',
                description: 'Fixing common hardware issues',
                icon: '🔧'
            },
            'configuration': {
                name: 'Configuration',
                description: 'System and application configuration tweaks',
                icon: '⚙️'
            },
            'power_management': {
                name: 'Power Management',
                description: 'Battery life and power consumption optimization',
                icon: '🔋'
            },
            'security': {
                name: 'Security',
                description: 'Hardware-related security configurations',
                icon: '🔒'
            }
        };

        // Hardware component types
        this.componentTypes = {
            'cpu': { name: 'CPU/Processor', icon: '🖥️' },
            'gpu': { name: 'Graphics Card', icon: '🎮' },
            'memory': { name: 'Memory/RAM', icon: '💾' },
            'storage': { name: 'Storage Device', icon: '💿' },
            'network': { name: 'Network Adapter', icon: '🌐' },
            'audio': { name: 'Audio Device', icon: '🔊' },
            'input': { name: 'Input Device', icon: '⌨️' },
            'display': { name: 'Display/Monitor', icon: '🖥️' },
            'system': { name: 'System/BIOS', icon: '⚙️' }
        };

        // Supported distributions
        this.distributions = {
            'debian': { name: 'Debian/Ubuntu', icon: '🌀', packageManager: 'apt' },
            'arch': { name: 'Arch Linux', icon: '🏔️', packageManager: 'pacman' },
            'fedora': { name: 'Fedora/RHEL', icon: '🎩', packageManager: 'dnf' },
            'nixos': { name: 'NixOS', icon: '❄️', packageManager: 'nix' },
            'opensuse': { name: 'openSUSE', icon: '🦎', packageManager: 'zypper' },
            'gentoo': { name: 'Gentoo', icon: '🐧', packageManager: 'portage' },
            'alpine': { name: 'Alpine Linux', icon: '🏔️', packageManager: 'apk' }
        };
    }

    /**
     * Initialize the tip submission system
     */
    async initialize() {
        console.log('Initializing tip submission system...');
        
        this.createSubmissionModal();
        this.setupEventListeners();
        this.isInitialized = true;
        
        console.log('Tip submission system initialized successfully');
    }

    /**
     * Show the tip submission form
     */
    showTipSubmissionForm(prefilledData = {}) {
        if (!this.submissionModal) {
            this.createSubmissionModal();
        }

        // Reset form and populate with prefilled data
        this.resetSubmissionForm();
        this.populateForm(prefilledData);
        
        // Show modal
        this.submissionModal.style.display = 'flex';
        document.body.style.overflow = 'hidden';

        // Focus on first input
        const firstInput = this.submissionModal.querySelector('#tip-title');
        if (firstInput) {
            setTimeout(() => firstInput.focus(), 100);
        }
    }

    /**
     * Hide the submission modal
     */
    hideSubmissionModal() {
        if (this.submissionModal) {
            this.submissionModal.style.display = 'none';
            document.body.style.overflow = '';
            this.resetSubmissionForm();
        }
    }

    /**
     * Create the submission modal interface
     */
    createSubmissionModal() {
        this.submissionModal = document.createElement('div');
        this.submissionModal.className = 'tip-submission-modal';
        this.submissionModal.innerHTML = `
            <div class="submission-modal-content">
                <div class="submission-modal-header">
                    <h2>💡 Share Your Configuration Tip</h2>
                    <button class="submission-modal-close" aria-label="Close">&times;</button>
                </div>
                
                <form id="tip-submission-form" class="submission-form">
                    <div class="form-row">
                        <div class="form-group">
                            <label for="tip-title">Tip Title *</label>
                            <input type="text" id="tip-title" name="title" required
                                   placeholder="e.g., Fix NVIDIA Fan Control on Debian"
                                   maxlength="100">
                            <small>Concise title describing your tip</small>
                        </div>
                    </div>

                    <div class="form-row">
                        <div class="form-group">
                            <label for="tip-category">Category *</label>
                            <select id="tip-category" name="category" required>
                                <option value="">Select a category...</option>
                                ${Object.entries(this.categories).map(([key, cat]) => 
                                    `<option value="${key}">${cat.icon} ${cat.name}</option>`
                                ).join('')}
                            </select>
                        </div>
                        
                        <div class="form-group">
                            <label for="tip-component">Hardware Component *</label>
                            <select id="tip-component" name="component" required>
                                <option value="">Select component type...</option>
                                ${Object.entries(this.componentTypes).map(([key, comp]) => 
                                    `<option value="${key}">${comp.icon} ${comp.name}</option>`
                                ).join('')}
                            </select>
                        </div>
                    </div>

                    <div class="form-row">
                        <div class="form-group">
                            <label for="tip-vendor">Hardware Vendor</label>
                            <input type="text" id="tip-vendor" name="vendor" 
                                   placeholder="e.g., NVIDIA, Intel, AMD"
                                   maxlength="50">
                            <small>Optional: Specific hardware vendor</small>
                        </div>
                        
                        <div class="form-group">
                            <label for="tip-model">Hardware Model</label>
                            <input type="text" id="tip-model" name="model" 
                                   placeholder="e.g., GeForce RTX 4080, Core i7-13700K"
                                   maxlength="100">
                            <small>Optional: Specific hardware model</small>
                        </div>
                    </div>

                    <div class="form-row">
                        <div class="form-group full-width">
                            <label for="tip-description">Description *</label>
                            <textarea id="tip-description" name="description" required 
                                      rows="4" maxlength="500"
                                      placeholder="Describe what your tip accomplishes and why it's helpful..."></textarea>
                            <small class="char-counter">0/500 characters</small>
                        </div>
                    </div>

                    <div class="distribution-tabs">
                        <div class="tabs-header">
                            <label>Distribution-specific Instructions:</label>
                            <div class="tab-buttons">
                                ${Object.entries(this.distributions).map(([key, distro]) => 
                                    `<button type="button" class="tab-btn ${key === 'debian' ? 'active' : ''}" 
                                             data-distro="${key}">
                                        ${distro.icon} ${distro.name}
                                    </button>`
                                ).join('')}
                            </div>
                        </div>

                        <div class="tab-content">
                            ${Object.entries(this.distributions).map(([key, distro]) => 
                                this.createDistributionTab(key, distro)
                            ).join('')}
                        </div>
                    </div>

                    <div class="form-row">
                        <div class="form-group full-width">
                            <label for="tip-warnings">Warnings/Cautions</label>
                            <textarea id="tip-warnings" name="warnings" 
                                      rows="2" maxlength="300"
                                      placeholder="Any important warnings or cautions (optional)..."></textarea>
                            <small>Warn users about potential risks or prerequisites</small>
                        </div>
                    </div>

                    <!-- GitHub Authentication Section -->
                    <div id="github-login-section" class="form-row github-auth-section">
                        <div class="form-group full-width">
                            <div class="github-login-prompt">
                                <div class="github-prompt-content">
                                    <div class="github-icon">🐙</div>
                                    <div class="github-text">
                                        <h4>Get Credit for Your Contribution</h4>
                                        <p>Sign in with GitHub to have your profile credited for this tip</p>
                                    </div>
                                    <button type="button" id="github-login-btn" class="btn-github">
                                        <svg class="github-logo" viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M12 0C5.374 0 0 5.373 0 12 0 17.302 3.438 21.8 8.207 23.387c.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0112 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z"/>
                                        </svg>
                                        Sign in with GitHub
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- GitHub User Section (hidden by default) -->
                    <div id="github-user-section" class="form-row github-user-section" style="display: none;">
                        <div class="form-group full-width">
                            <div class="github-user-display">
                                <img id="user-avatar" class="user-avatar" src="" alt="User avatar">
                                <div class="user-info">
                                    <div class="user-name" id="user-name"></div>
                                    <div class="user-username" id="user-username"></div>
                                    <div class="user-stats" id="user-stats"></div>
                                </div>
                                <button type="button" id="github-logout-btn" class="btn-secondary btn-small">
                                    Sign Out
                                </button>
                            </div>
                        </div>
                    </div>

                    <!-- Attribution Controls -->
                    <div id="attribution-section" class="form-row attribution-section" style="display: none;">
                        <div class="form-group full-width">
                            <div class="checkbox-group attribution-toggle">
                                <input type="checkbox" id="attribution-toggle" name="attribution" checked>
                                <label for="attribution-toggle">
                                    <strong>Credit my GitHub profile for this tip</strong>
                                    <small>Your GitHub username and avatar will be displayed with this tip. Uncheck to contribute anonymously.</small>
                                </label>
                            </div>
                        </div>
                    </div>

                    <!-- Fallback Author Section -->
                    <div id="fallback-author-section" class="form-row">
                        <div class="form-group">
                            <label for="tip-author">Your Name</label>
                            <input type="text" id="tip-author" name="author" 
                                   placeholder="How you'd like to be credited"
                                   maxlength="50">
                            <small>Optional: How you'd like to be credited</small>
                        </div>
                        
                        <div class="form-group">
                            <label for="tip-email">Email (Private)</label>
                            <input type="email" id="tip-email" name="email" 
                                   placeholder="your@email.com">
                            <small>Optional: For moderation contact only</small>
                        </div>
                    </div>

                    <div class="form-row">
                        <div class="form-group full-width">
                            <div class="checkbox-group">
                                <input type="checkbox" id="tip-tested" name="tested" required>
                                <label for="tip-tested">
                                    I have personally tested this configuration and confirm it works *
                                </label>
                            </div>
                            
                            <div class="checkbox-group">
                                <input type="checkbox" id="tip-license" name="license" required>
                                <label for="tip-license">
                                    I agree to contribute this tip under CC0 (public domain) license *
                                </label>
                            </div>
                        </div>
                    </div>
                </form>
                
                <div class="submission-modal-footer">
                    <button type="button" class="btn-secondary cancel-submission">
                        Cancel
                    </button>
                    <button type="button" class="btn-secondary preview-tip">
                        👁️ Preview
                    </button>
                    <button type="submit" form="tip-submission-form" class="btn-primary submit-tip">
                        🚀 Submit Tip
                    </button>
                </div>
            </div>
        `;

        document.body.appendChild(this.submissionModal);
        this.setupSubmissionEventListeners();
    }

    /**
     * Create distribution-specific tab content
     */
    createDistributionTab(distroKey, distro) {
        return `
            <div class="tab-panel ${distroKey === 'debian' ? 'active' : ''}" data-distro="${distroKey}">
                <div class="distro-section">
                    <h4>${distro.icon} ${distro.name}</h4>
                    
                    <div class="form-group">
                        <label for="${distroKey}-commands">Terminal Commands</label>
                        <div class="command-input-container">
                            <div class="commands-list" id="${distroKey}-commands-list">
                                <div class="command-input-group">
                                    <input type="text" name="${distroKey}_commands[]" 
                                           placeholder="Enter command (e.g., sudo apt install package-name)"
                                           class="command-input">
                                    <input type="text" name="${distroKey}_descriptions[]" 
                                           placeholder="What this command does"
                                           class="command-description">
                                    <button type="button" class="remove-command-btn" title="Remove command">✕</button>
                                </div>
                            </div>
                            <button type="button" class="add-command-btn" data-distro="${distroKey}">
                                ➕ Add Another Command
                            </button>
                        </div>
                    </div>

                    <div class="form-group">
                        <label for="${distroKey}-config">Configuration Files</label>
                        <div class="config-input-container">
                            <div class="config-files-list" id="${distroKey}-config-list">
                                <div class="config-input-group">
                                    <input type="text" name="${distroKey}_config_files[]" 
                                           placeholder="File path (e.g., /etc/modprobe.d/options.conf)"
                                           class="config-file-path">
                                    <textarea name="${distroKey}_config_contents[]" 
                                              placeholder="File contents..."
                                              rows="3" class="config-file-content"></textarea>
                                    <button type="button" class="remove-config-btn" title="Remove configuration">✕</button>
                                </div>
                            </div>
                            <button type="button" class="add-config-btn" data-distro="${distroKey}">
                                📁 Add Configuration File
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * Set up event listeners for the submission form
     */
    setupSubmissionEventListeners() {
        // Modal close handlers
        this.submissionModal.querySelector('.submission-modal-close').addEventListener('click', () => {
            this.hideSubmissionModal();
        });

        this.submissionModal.querySelector('.cancel-submission').addEventListener('click', () => {
            this.hideSubmissionModal();
        });

        // Tab switching
        this.submissionModal.querySelectorAll('.tab-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                this.switchDistributionTab(e.target.dataset.distro);
            });
        });

        // Dynamic command/config addition
        this.submissionModal.addEventListener('click', (e) => {
            if (e.target.classList.contains('add-command-btn')) {
                this.addCommandInput(e.target.dataset.distro);
            }
            
            if (e.target.classList.contains('add-config-btn')) {
                this.addConfigInput(e.target.dataset.distro);
            }
            
            if (e.target.classList.contains('remove-command-btn')) {
                e.target.closest('.command-input-group').remove();
            }
            
            if (e.target.classList.contains('remove-config-btn')) {
                e.target.closest('.config-input-group').remove();
            }
        });

        // Character counter for description
        const descriptionTextarea = this.submissionModal.querySelector('#tip-description');
        const charCounter = this.submissionModal.querySelector('.char-counter');
        
        descriptionTextarea.addEventListener('input', () => {
            const length = descriptionTextarea.value.length;
            charCounter.textContent = `${length}/500 characters`;
            charCounter.style.color = length > 450 ? 'var(--warning)' : '';
        });

        // Form submission
        this.submissionModal.querySelector('#tip-submission-form').addEventListener('submit', (e) => {
            e.preventDefault();
            this.handleTipSubmission();
        });

        // Preview functionality
        this.submissionModal.querySelector('.preview-tip').addEventListener('click', () => {
            this.showTipPreview();
        });

        // Close on background click
        this.submissionModal.addEventListener('click', (e) => {
            if (e.target === this.submissionModal) {
                this.hideSubmissionModal();
            }
        });
    }

    /**
     * Set up global event listeners
     */
    setupEventListeners() {
        // Listen for "Add Your Tip" clicks from configuration tips modal
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('add-tip-btn') || e.target.classList.contains('add-tip-link')) {
                e.preventDefault();
                
                // Try to get context from current hardware if available
                let prefilledData = {};
                if (window.configurationTips && window.configurationTips.currentHardware) {
                    const hardware = window.configurationTips.currentHardware;
                    prefilledData = {
                        vendor: hardware.cpu?.vendor || hardware.graphics?.[0]?.vendor || '',
                        model: hardware.cpu?.model || hardware.graphics?.[0]?.model || ''
                    };
                }
                
                this.showTipSubmissionForm(prefilledData);
            }
        });
    }

    /**
     * Switch between distribution tabs
     */
    switchDistributionTab(distroKey) {
        // Update tab buttons
        this.submissionModal.querySelectorAll('.tab-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.distro === distroKey);
        });

        // Update tab panels
        this.submissionModal.querySelectorAll('.tab-panel').forEach(panel => {
            panel.classList.toggle('active', panel.dataset.distro === distroKey);
        });
    }

    /**
     * Add a new command input group
     */
    addCommandInput(distroKey) {
        const commandsList = this.submissionModal.querySelector(`#${distroKey}-commands-list`);
        const newGroup = document.createElement('div');
        newGroup.className = 'command-input-group';
        newGroup.innerHTML = `
            <input type="text" name="${distroKey}_commands[]" 
                   placeholder="Enter command"
                   class="command-input">
            <input type="text" name="${distroKey}_descriptions[]" 
                   placeholder="What this command does"
                   class="command-description">
            <button type="button" class="remove-command-btn" title="Remove command">✕</button>
        `;
        commandsList.appendChild(newGroup);
    }

    /**
     * Add a new config file input group
     */
    addConfigInput(distroKey) {
        const configList = this.submissionModal.querySelector(`#${distroKey}-config-list`);
        const newGroup = document.createElement('div');
        newGroup.className = 'config-input-group';
        newGroup.innerHTML = `
            <input type="text" name="${distroKey}_config_files[]" 
                   placeholder="File path (e.g., /etc/modprobe.d/options.conf)"
                   class="config-file-path">
            <textarea name="${distroKey}_config_contents[]" 
                      placeholder="File contents..."
                      rows="3" class="config-file-content"></textarea>
            <button type="button" class="remove-config-btn" title="Remove configuration">✕</button>
        `;
        configList.appendChild(newGroup);
    }

    /**
     * Reset the submission form
     */
    resetSubmissionForm() {
        const form = this.submissionModal.querySelector('#tip-submission-form');
        if (form) {
            form.reset();
            
            // Reset character counter
            const charCounter = this.submissionModal.querySelector('.char-counter');
            if (charCounter) {
                charCounter.textContent = '0/500 characters';
                charCounter.style.color = '';
            }

            // Reset to first tab
            this.switchDistributionTab('debian');

            // Clear dynamic inputs
            this.submissionModal.querySelectorAll('.commands-list, .config-files-list').forEach(container => {
                const firstGroup = container.querySelector('.command-input-group, .config-input-group');
                container.innerHTML = firstGroup ? firstGroup.outerHTML : '';
            });
        }
    }

    /**
     * Populate form with prefilled data
     */
    populateForm(data) {
        if (data.vendor) {
            const vendorInput = this.submissionModal.querySelector('#tip-vendor');
            if (vendorInput) vendorInput.value = data.vendor;
        }
        
        if (data.model) {
            const modelInput = this.submissionModal.querySelector('#tip-model');
            if (modelInput) modelInput.value = data.model;
        }
        
        if (data.component) {
            const componentSelect = this.submissionModal.querySelector('#tip-component');
            if (componentSelect) componentSelect.value = data.component;
        }
    }

    /**
     * Handle tip submission
     */
    async handleTipSubmission() {
        const formData = this.collectFormData();
        
        if (!this.validateSubmission(formData)) {
            return;
        }

        try {
            // Show loading state
            this.showSubmissionLoading();
            
            // Submit tip for validation and moderation
            const result = await this.submitTipData(formData);
            
            // Emit submission event for moderation system
            document.dispatchEvent(new CustomEvent('tipSubmitted', {
                detail: formData
            }));
            
            // Show success message
            this.showSubmissionSuccess(result);
            
            // Close modal after delay
            setTimeout(() => {
                this.hideSubmissionModal();
            }, 3000);
            
        } catch (error) {
            console.error('Tip submission error:', error);
            this.showSubmissionError(error.message);
        }
    }

    /**
     * Collect form data
     */
    collectFormData() {
        const form = this.submissionModal.querySelector('#tip-submission-form');
        const formData = new FormData(form);
        const data = {};

        // Basic fields
        data.title = formData.get('title');
        data.category = formData.get('category');
        data.component = formData.get('component');
        data.vendor = formData.get('vendor') || null;
        data.model = formData.get('model') || null;
        data.description = formData.get('description');
        data.warnings = formData.get('warnings') || null;
        data.tested = formData.get('tested') === 'on';
        data.license = formData.get('license') === 'on';

        // Attribution handling
        if (window.githubAuth && window.githubAuth.isAuthenticated) {
            const contributorInfo = window.githubAuth.getContributorInfo();
            if (contributorInfo) {
                data.contributor = contributorInfo;
                data.author = contributorInfo.name || contributorInfo.username;
            } else {
                // User is authenticated but chose anonymous contribution
                data.author = formData.get('author') || 'Anonymous';
                data.contributor = null;
            }
        } else {
            // No GitHub authentication, use fallback author info
            data.author = formData.get('author') || 'Anonymous';
            data.contributor = null;
        }
        
        data.email = formData.get('email') || null;

        // Distribution-specific data
        data.distributions = {};
        
        Object.keys(this.distributions).forEach(distroKey => {
            const commands = formData.getAll(`${distroKey}_commands[]`).filter(cmd => cmd.trim());
            const descriptions = formData.getAll(`${distroKey}_descriptions[]`);
            const configFiles = formData.getAll(`${distroKey}_config_files[]`).filter(file => file.trim());
            const configContents = formData.getAll(`${distroKey}_config_contents[]`);

            if (commands.length || configFiles.length) {
                data.distributions[distroKey] = {
                    commands: commands.map((cmd, i) => ({
                        command: cmd.trim(),
                        description: descriptions[i] ? descriptions[i].trim() : null
                    })),
                    configurations: configFiles.map((file, i) => ({
                        file: file.trim(),
                        content: configContents[i] ? configContents[i].trim() : ''
                    }))
                };
            }
        });

        return data;
    }

    /**
     * Validate submission data
     */
    validateSubmission(data) {
        const errors = [];

        if (!data.title || data.title.length < 5) {
            errors.push('Title must be at least 5 characters long');
        }

        if (!data.category) {
            errors.push('Please select a category');
        }

        if (!data.component) {
            errors.push('Please select a hardware component');
        }

        if (!data.description || data.description.length < 20) {
            errors.push('Description must be at least 20 characters long');
        }

        if (!data.tested) {
            errors.push('You must confirm that you have tested this configuration');
        }

        if (!data.license) {
            errors.push('You must agree to the CC0 license terms');
        }

        // Check if at least one distribution has content
        const hasDistributionContent = Object.values(data.distributions).some(distro => 
            distro.commands.length > 0 || distro.configurations.length > 0
        );
        
        if (!hasDistributionContent) {
            errors.push('Please provide commands or configuration for at least one distribution');
        }

        if (errors.length > 0) {
            this.showValidationErrors(errors);
            return false;
        }

        return true;
    }

    /**
     * Show validation errors
     */
    showValidationErrors(errors) {
        alert('Please fix the following errors:\n\n• ' + errors.join('\n• '));
    }

    /**
     * Submit tip data (mock implementation)
     */
    async submitTipData(data) {
        // Mock API delay
        await new Promise(resolve => setTimeout(resolve, 2000));
        
        // In production, this would be an actual API call
        console.log('Submitting tip data:', data);
        
        // Return mock result
        return {
            id: 'tip_' + Date.now(),
            status: 'submitted',
            message: 'Your tip has been submitted for review!'
        };
    }

    /**
     * Show submission loading state
     */
    showSubmissionLoading() {
        const submitBtn = this.submissionModal.querySelector('.submit-tip');
        submitBtn.disabled = true;
        submitBtn.innerHTML = '🔄 Submitting...';
    }

    /**
     * Show submission success
     */
    showSubmissionSuccess(result) {
        const submitBtn = this.submissionModal.querySelector('.submit-tip');
        submitBtn.innerHTML = '✅ Success!';
        submitBtn.style.background = 'var(--success)';
        
        // Show success message in modal
        const modalBody = this.submissionModal.querySelector('.submission-form');
        modalBody.innerHTML = `
            <div class="submission-success">
                <div class="success-icon">🎉</div>
                <h3>Thank You!</h3>
                <p>Your configuration tip has been submitted successfully!</p>
                <div class="success-details">
                    <p><strong>Tip ID:</strong> ${result.id}</p>
                    <p>Your tip will be reviewed by our community moderators and published within 24-48 hours.</p>
                    <p>You'll receive an email confirmation if you provided an email address.</p>
                </div>
            </div>
        `;
    }

    /**
     * Show submission error
     */
    showSubmissionError(message) {
        const submitBtn = this.submissionModal.querySelector('.submit-tip');
        submitBtn.disabled = false;
        submitBtn.innerHTML = '🚀 Submit Tip';
        
        alert('Submission failed: ' + message);
    }

    /**
     * Show tip preview
     */
    showTipPreview() {
        const data = this.collectFormData();
        console.log('Tip preview:', data);
        
        // This would show a preview of how the tip would look
        alert('Preview feature coming soon! For now, you can review your tip in the form fields above.');
    }
}

// Global instance
window.tipSubmissionSystem = new TipSubmissionSystem();

// Auto-initialize when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
    await window.tipSubmissionSystem.initialize();
});