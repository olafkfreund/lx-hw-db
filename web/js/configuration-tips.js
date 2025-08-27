/**
 * Hardware Configuration Tips System
 * Community-driven driver configuration tips and tweaks
 */

class ConfigurationTips {
    constructor() {
        this.databaseIndexer = null;
        this.isInitialized = false;
        this.currentHardware = null;
        this.currentDistribution = null;
        
        this.distributions = {
            'debian': {
                name: 'Debian/Ubuntu',
                icon: '🌀',
                packageManager: 'apt',
                configPaths: {
                    modprobe: '/etc/modprobe.d/',
                    udev: '/etc/udev/rules.d/',
                    systemd: '/etc/systemd/system/',
                    xorg: '/etc/X11/xorg.conf.d/'
                }
            },
            'arch': {
                name: 'Arch Linux',
                icon: '🏔️',
                packageManager: 'pacman',
                configPaths: {
                    modprobe: '/etc/modprobe.d/',
                    udev: '/etc/udev/rules.d/',
                    systemd: '/etc/systemd/system/',
                    xorg: '/etc/X11/xorg.conf.d/'
                }
            },
            'fedora': {
                name: 'Fedora/RHEL',
                icon: '🎩',
                packageManager: 'dnf',
                configPaths: {
                    modprobe: '/etc/modprobe.d/',
                    udev: '/etc/udev/rules.d/',
                    systemd: '/etc/systemd/system/',
                    xorg: '/etc/X11/xorg.conf.d/'
                }
            },
            'nixos': {
                name: 'NixOS',
                icon: '❄️',
                packageManager: 'nix',
                configPaths: {
                    configuration: '/etc/nixos/configuration.nix',
                    hardware: '/etc/nixos/hardware-configuration.nix'
                }
            }
        };
    }

    /**
     * Initialize the configuration tips system
     */
    async initialize() {
        console.log('Initializing configuration tips system...');
        
        // Wait for database indexer
        document.addEventListener('databaseIndexed', (event) => {
            this.databaseIndexer = event.detail.indexer;
            this.setupConfigurationTips();
        });

        // Check if database indexer is already available
        if (window.hardwareDatabaseIndexer && window.hardwareDatabaseIndexer.isBuilt) {
            this.databaseIndexer = window.hardwareDatabaseIndexer;
            this.setupConfigurationTips();
        }

        // Load configuration tips data
        await this.loadConfigurationTips();
    }

    /**
     * Load configuration tips database
     */
    async loadConfigurationTips() {
        try {
            // For now, we'll use sample data. In production, this would load from a separate tips database
            this.configurationTips = await this.getSampleConfigurationTips();
            console.log('Configuration tips loaded successfully');
        } catch (error) {
            console.error('Error loading configuration tips:', error);
            this.configurationTips = {};
        }
    }

    /**
     * Set up configuration tips interface
     */
    setupConfigurationTips() {
        if (this.isInitialized) return;
        
        this.addConfigurationTipsToUI();
        this.setupEventListeners();
        this.isInitialized = true;
        
        console.log('Configuration tips system initialized successfully');
    }

    /**
     * Add configuration tips interface to existing components
     */
    addConfigurationTipsToUI() {
        // Add configuration tips button to search results
        this.enhanceSearchResults();
        
        // Add configuration tips section to category browser
        this.enhanceCategoryBrowser();
        
        // Create configuration tips modal
        this.createConfigurationModal();
        
        // Add tips submission interface
        this.createTipSubmissionInterface();
    }

    /**
     * Enhance search results with configuration tips
     */
    enhanceSearchResults() {
        // Override the search UI's result card creation to add configuration tips
        if (window.hardwareSearchUI) {
            const originalCreateResultCard = window.hardwareSearchUI.createResultCard;
            
            window.hardwareSearchUI.createResultCard = (result, index) => {
                const card = originalCreateResultCard.call(window.hardwareSearchUI, result, index);
                
                // Add configuration tips button
                const metadata = card.querySelector('.result-metadata');
                if (metadata) {
                    const tipsButton = document.createElement('button');
                    tipsButton.className = 'config-tips-btn';
                    tipsButton.innerHTML = '⚙️ Configuration Tips';
                    tipsButton.dataset.reportId = result.id;
                    
                    tipsButton.addEventListener('click', (e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        this.showConfigurationTips(result);
                    });
                    
                    metadata.insertBefore(tipsButton, metadata.firstChild);
                }
                
                return card;
            };
        }
    }

    /**
     * Enhance category browser with configuration tips
     */
    enhanceCategoryBrowser() {
        // Similar enhancement for category browser results
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('view-report-details')) {
                const reportId = e.target.dataset.reportId;
                const report = this.findReportById(reportId);
                if (report) {
                    // Add configuration tips to the report details view
                    setTimeout(() => this.addConfigTipsToModal(report), 100);
                }
            }
        });
    }

    /**
     * Show configuration tips for specific hardware
     */
    showConfigurationTips(report) {
        this.currentHardware = report;
        const modal = this.createConfigurationModal();
        
        // Extract hardware components
        const components = this.extractHardwareComponents(report);
        
        // Generate tips content
        const tipsContent = this.generateTipsContent(components, report.system.distribution);
        
        // Update modal content
        const modalBody = modal.querySelector('.config-modal-body');
        modalBody.innerHTML = tipsContent;
        
        // Show modal
        modal.style.display = 'flex';
        document.body.style.overflow = 'hidden';
    }

    /**
     * Create configuration tips modal
     */
    createConfigurationModal() {
        let modal = document.querySelector('.config-tips-modal');
        
        if (!modal) {
            modal = document.createElement('div');
            modal.className = 'config-tips-modal';
            modal.innerHTML = `
                <div class="config-modal-content">
                    <div class="config-modal-header">
                        <h2>Hardware Configuration Tips</h2>
                        <div class="config-modal-controls">
                            <select class="distribution-selector" id="config-distribution-selector">
                                <option value="debian">🌀 Debian/Ubuntu</option>
                                <option value="arch">🏔️ Arch Linux</option>
                                <option value="fedora">🎩 Fedora/RHEL</option>
                                <option value="nixos">❄️ NixOS</option>
                            </select>
                            <button class="config-modal-close" aria-label="Close">&times;</button>
                        </div>
                    </div>
                    
                    <div class="config-modal-body">
                        <!-- Configuration tips content will be inserted here -->
                    </div>
                    
                    <div class="config-modal-footer">
                        <button class="btn-secondary add-tip-btn">
                            ➕ Add Your Tip
                        </button>
                        <button class="btn-primary copy-all-btn">
                            📋 Copy All Commands
                        </button>
                    </div>
                </div>
            `;
            
            document.body.appendChild(modal);
            
            // Set up modal event listeners
            modal.querySelector('.config-modal-close').addEventListener('click', () => {
                this.hideConfigurationModal();
            });
            
            modal.querySelector('.distribution-selector').addEventListener('change', (e) => {
                this.currentDistribution = e.target.value;
                if (this.currentHardware) {
                    this.showConfigurationTips(this.currentHardware);
                }
            });
            
            modal.querySelector('.add-tip-btn').addEventListener('click', () => {
                this.showTipSubmissionForm();
            });
            
            modal.querySelector('.copy-all-btn').addEventListener('click', () => {
                this.copyAllCommands();
            });
            
            // Close on background click
            modal.addEventListener('click', (e) => {
                if (e.target === modal) {
                    this.hideConfigurationModal();
                }
            });
        }
        
        return modal;
    }

    /**
     * Hide configuration modal
     */
    hideConfigurationModal() {
        const modal = document.querySelector('.config-tips-modal');
        if (modal) {
            modal.style.display = 'none';
            document.body.style.overflow = '';
        }
    }

    /**
     * Extract hardware components from report
     */
    extractHardwareComponents(report) {
        const components = [];
        
        // CPU
        if (report.cpu) {
            components.push({
                type: 'cpu',
                vendor: report.cpu.vendor,
                model: report.cpu.model,
                data: report.cpu
            });
        }
        
        // Graphics
        if (report.graphics) {
            report.graphics.forEach(gpu => {
                components.push({
                    type: 'gpu',
                    vendor: gpu.vendor,
                    model: gpu.model,
                    driver: gpu.driver,
                    data: gpu
                });
            });
        }
        
        // Network
        if (report.network) {
            report.network.forEach(net => {
                components.push({
                    type: 'network',
                    vendor: net.vendor,
                    model: net.model,
                    driver: net.driver,
                    data: net
                });
            });
        }
        
        // Storage
        if (report.storage) {
            report.storage.forEach(storage => {
                components.push({
                    type: 'storage',
                    vendor: storage.vendor,
                    model: storage.model,
                    interface: storage.interface,
                    data: storage
                });
            });
        }
        
        // Audio
        if (report.audio) {
            report.audio.forEach(audio => {
                components.push({
                    type: 'audio',
                    vendor: audio.vendor,
                    model: audio.model,
                    driver: audio.driver,
                    data: audio
                });
            });
        }
        
        return components;
    }

    /**
     * Generate tips content for components
     */
    generateTipsContent(components, defaultDistribution) {
        const distribution = this.currentDistribution || this.detectDistribution(defaultDistribution);
        let content = '';
        
        components.forEach(component => {
            const tips = this.getComponentTips(component, distribution);
            if (tips.length > 0) {
                content += this.renderComponentTips(component, tips, distribution);
            }
        });
        
        if (content === '') {
            content = `
                <div class="no-tips-message">
                    <h3>No Configuration Tips Available</h3>
                    <p>We don't have specific configuration tips for this hardware combination yet.</p>
                    <p>Help the community by <a href="#" class="add-tip-link">contributing your configuration tips</a>!</p>
                </div>
            `;
        }
        
        return content;
    }

    /**
     * Render tips for a specific component
     */
    renderComponentTips(component, tips, distribution) {
        const distroInfo = this.distributions[distribution];
        const componentIcon = this.getComponentIcon(component.type);
        
        return `
            <div class="component-tips-section">
                <div class="component-header">
                    <div class="component-info">
                        <span class="component-icon">${componentIcon}</span>
                        <div>
                            <h3>${component.vendor} ${component.model}</h3>
                            <p class="component-type">${component.type.toUpperCase()}${component.driver ? ` • Driver: ${component.driver}` : ''}</p>
                        </div>
                    </div>
                    <div class="distribution-badge">
                        <span class="distro-icon">${distroInfo.icon}</span>
                        <span class="distro-name">${distroInfo.name}</span>
                    </div>
                </div>
                
                <div class="tips-list">
                    ${tips.map(tip => this.renderTip(tip, distribution)).join('')}
                </div>
            </div>
        `;
    }

    /**
     * Render a single tip
     */
    renderTip(tip, distribution) {
        return `
            <div class="tip-card" data-tip-id="${tip.id}">
                <div class="tip-header">
                    <div class="tip-title">
                        <h4>${tip.title}</h4>
                        <div class="tip-meta">
                            <span class="tip-category">${tip.category}</span>
                            <span class="tip-rating">⭐ ${tip.rating.toFixed(1)}</span>
                            <span class="tip-votes">(${tip.votes} votes)</span>
                        </div>
                    </div>
                    <div class="tip-actions">
                        <button class="copy-tip-btn" data-tip-id="${tip.id}" title="Copy commands">📋</button>
                        <button class="vote-tip-btn" data-tip-id="${tip.id}" data-vote="up" title="Helpful">👍</button>
                        <button class="vote-tip-btn" data-tip-id="${tip.id}" data-vote="down" title="Not helpful">👎</button>
                    </div>
                </div>
                
                <div class="tip-description">
                    <p>${tip.description}</p>
                </div>
                
                ${tip.commands && tip.commands.length > 0 ? `
                    <div class="tip-commands">
                        <div class="commands-header">
                            <h5>Commands:</h5>
                            <span class="package-manager">${this.distributions[distribution].packageManager}</span>
                        </div>
                        <div class="command-blocks">
                            ${tip.commands.map(cmd => `
                                <div class="command-block">
                                    <code>${cmd.command}</code>
                                    <button class="copy-command-btn" data-command="${this.escapeHtml(cmd.command)}" title="Copy command">📋</button>
                                    ${cmd.description ? `<p class="command-description">${cmd.description}</p>` : ''}
                                </div>
                            `).join('')}
                        </div>
                    </div>
                ` : ''}
                
                ${tip.configuration && Object.keys(tip.configuration).length > 0 ? `
                    <div class="tip-configuration">
                        <h5>Configuration:</h5>
                        ${Object.entries(tip.configuration).map(([file, content]) => `
                            <div class="config-file">
                                <div class="config-file-header">
                                    <code class="config-file-path">${file}</code>
                                    <button class="copy-config-btn" data-config="${this.escapeHtml(content)}" title="Copy configuration">📋</button>
                                </div>
                                <pre class="config-content"><code>${content}</code></pre>
                            </div>
                        `).join('')}
                    </div>
                ` : ''}
                
                <div class="tip-footer">
                    <small class="tip-author">Contributed by ${tip.author} • ${tip.date}</small>
                    ${tip.warnings && tip.warnings.length > 0 ? `
                        <div class="tip-warnings">
                            ${tip.warnings.map(warning => `
                                <div class="warning">⚠️ ${warning}</div>
                            `).join('')}
                        </div>
                    ` : ''}
                    
                    ${tip.contributor ? `
                        <div class="tip-contributor">
                            ${tip.contributor.avatar_url ? 
                                `<img src="${tip.contributor.avatar_url}" alt="${tip.contributor.name || tip.contributor.username}" class="tip-contributor-avatar">` : 
                                '<div class="tip-contributor-avatar">👤</div>'
                            }
                            <span class="tip-contributor-label">Contributed by:</span>
                            <a href="${tip.contributor.profile_url}" class="tip-contributor-name" target="_blank" rel="noopener">
                                ${tip.contributor.name || tip.contributor.username}
                            </a>
                        </div>
                    ` : ''}
                </div>
            </div>
        `;
    }

    /**
     * Get component-specific tips
     */
    getComponentTips(component, distribution) {
        const key = `${component.type}:${component.vendor}:${component.model}`;
        const vendorKey = `${component.type}:${component.vendor}`;
        const typeKey = component.type;
        
        // Try exact match first, then vendor match, then general type match
        return this.configurationTips[key]?.[distribution] ||
               this.configurationTips[vendorKey]?.[distribution] ||
               this.configurationTips[typeKey]?.[distribution] ||
               [];
    }

    /**
     * Get sample configuration tips (in production, this would come from a database)
     */
    async getSampleConfigurationTips() {
        return {
            'gpu:NVIDIA': {
                'debian': [
                    {
                        id: 'nvidia-debian-1',
                        title: 'Install NVIDIA Proprietary Drivers',
                        category: 'Driver Installation',
                        description: 'Install the official NVIDIA drivers for better performance and compatibility.',
                        rating: 4.8,
                        votes: 156,
                        author: 'linux-enthusiast',
                        date: '2025-01-15',
                        contributor: {
                            username: 'linux-enthusiast',
                            name: 'Linux Enthusiast',
                            avatar_url: 'https://avatars.githubusercontent.com/u/1?v=4',
                            profile_url: 'https://github.com/linux-enthusiast'
                        },
                        commands: [
                            {
                                command: 'sudo apt update && sudo apt install nvidia-driver',
                                description: 'Install the latest NVIDIA driver from Debian repositories'
                            },
                            {
                                command: 'sudo reboot',
                                description: 'Reboot to load the new driver'
                            }
                        ],
                        warnings: ['Make sure to disable nouveau driver before installation']
                    },
                    {
                        id: 'nvidia-debian-2',
                        title: 'Enable GPU Fan Control',
                        category: 'Performance Tuning',
                        description: 'Enable manual fan control for better thermal management.',
                        rating: 4.3,
                        votes: 89,
                        author: 'cooling_expert',
                        date: '2025-01-10',
                        configuration: {
                            '/etc/X11/xorg.conf.d/20-nvidia.conf': `Section "Device"
    Identifier     "Device0"
    Driver         "nvidia"
    VendorName     "NVIDIA Corporation"
    Option         "Coolbits" "4"
EndSection`
                        },
                        commands: [
                            {
                                command: 'nvidia-settings',
                                description: 'Open NVIDIA settings to configure fan curves'
                            }
                        ]
                    }
                ],
                'arch': [
                    {
                        id: 'nvidia-arch-1',
                        title: 'Install NVIDIA Drivers (Arch Way)',
                        category: 'Driver Installation',
                        description: 'Install NVIDIA drivers on Arch Linux with proper DKMS support.',
                        rating: 4.9,
                        votes: 234,
                        author: 'hardware-guru',
                        date: '2025-01-18',
                        contributor: {
                            username: 'hardware-guru',
                            name: 'Hardware Guru',
                            avatar_url: 'https://avatars.githubusercontent.com/u/2?v=4',
                            profile_url: 'https://github.com/hardware-guru'
                        },
                        commands: [
                            {
                                command: 'sudo pacman -S nvidia nvidia-utils',
                                description: 'Install NVIDIA driver and utilities'
                            },
                            {
                                command: 'sudo pacman -S nvidia-dkms',
                                description: 'Install DKMS version for custom kernels'
                            },
                            {
                                command: 'sudo mkinitcpio -P',
                                description: 'Regenerate initramfs'
                            }
                        ]
                    }
                ],
                'fedora': [
                    {
                        id: 'nvidia-fedora-1',
                        title: 'Install NVIDIA Drivers via RPM Fusion',
                        category: 'Driver Installation',
                        description: 'Install NVIDIA drivers using RPM Fusion repositories.',
                        rating: 4.6,
                        votes: 178,
                        author: 'fedora_user',
                        date: '2025-01-12',
                        commands: [
                            {
                                command: 'sudo dnf install https://download1.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm',
                                description: 'Enable RPM Fusion repositories'
                            },
                            {
                                command: 'sudo dnf install akmod-nvidia',
                                description: 'Install NVIDIA driver with automatic kernel module building'
                            }
                        ]
                    }
                ],
                'nixos': [
                    {
                        id: 'nvidia-nixos-1',
                        title: 'Enable NVIDIA Drivers in NixOS',
                        category: 'System Configuration',
                        description: 'Configure NVIDIA drivers using NixOS declarative configuration.',
                        rating: 4.7,
                        votes: 145,
                        author: 'nixos-wizard',
                        date: '2025-01-16',
                        contributor: {
                            username: 'nixos-wizard',
                            name: 'NixOS Wizard',
                            avatar_url: 'https://avatars.githubusercontent.com/u/3?v=4',
                            profile_url: 'https://github.com/nixos-wizard'
                        },
                        configuration: {
                            '/etc/nixos/configuration.nix': `{
  # Enable NVIDIA drivers
  services.xserver.videoDrivers = [ "nvidia" ];
  
  # NVIDIA configuration
  hardware.nvidia = {
    # Enable modesetting for Wayland
    modesetting.enable = true;
    
    # Enable NVIDIA settings menu
    nvidiaSettings = true;
    
    # Select driver version (optional)
    package = config.boot.kernelPackages.nvidiaPackages.stable;
  };
  
  # Enable OpenGL
  hardware.opengl = {
    enable = true;
    driSupport = true;
    driSupport32Bit = true;
  };
}`
                        },
                        commands: [
                            {
                                command: 'sudo nixos-rebuild switch',
                                description: 'Apply the configuration changes'
                            }
                        ]
                    }
                ]
            },
            'gpu:AMD': {
                'debian': [
                    {
                        id: 'amd-debian-1',
                        title: 'Install AMD GPU Drivers',
                        category: 'Driver Installation',
                        description: 'Install Mesa drivers for AMD graphics cards.',
                        rating: 4.5,
                        votes: 198,
                        author: 'amd_user',
                        date: '2025-01-14',
                        commands: [
                            {
                                command: 'sudo apt install mesa-vulkan-drivers libgl1-mesa-dri',
                                description: 'Install Mesa drivers with Vulkan support'
                            }
                        ]
                    }
                ],
                'nixos': [
                    {
                        id: 'amd-nixos-1',
                        title: 'Enable AMD GPU Support',
                        category: 'System Configuration',
                        description: 'Configure AMD graphics in NixOS.',
                        rating: 4.6,
                        votes: 112,
                        author: 'nixos_amd',
                        date: '2025-01-13',
                        configuration: {
                            '/etc/nixos/configuration.nix': `{
  # Enable AMD drivers
  services.xserver.videoDrivers = [ "amdgpu" ];
  
  # Enable OpenGL
  hardware.opengl = {
    enable = true;
    driSupport = true;
    driSupport32Bit = true;
    
    # AMD-specific packages
    extraPackages = with pkgs; [
      rocm-opencl-icd
      rocm-opencl-runtime
    ];
  };
}`
                        }
                    }
                ]
            },
            'network:Intel': {
                'debian': [
                    {
                        id: 'intel-wifi-debian-1',
                        title: 'Fix Intel WiFi Issues',
                        category: 'Network Troubleshooting',
                        description: 'Common fixes for Intel wireless network adapters.',
                        rating: 4.4,
                        votes: 267,
                        author: 'network_guru',
                        date: '2025-01-11',
                        commands: [
                            {
                                command: 'sudo apt install firmware-iwlwifi',
                                description: 'Install Intel wireless firmware'
                            }
                        ],
                        configuration: {
                            '/etc/modprobe.d/iwlwifi.conf': 'options iwlwifi 11n_disable=1'
                        }
                    }
                ]
            }
        };
    }

    /**
     * Set up event listeners
     */
    setupEventListeners() {
        // Copy buttons
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('copy-command-btn')) {
                const command = e.target.dataset.command;
                this.copyToClipboard(command);
                this.showCopyFeedback(e.target);
            }
            
            if (e.target.classList.contains('copy-config-btn')) {
                const config = e.target.dataset.config;
                this.copyToClipboard(config);
                this.showCopyFeedback(e.target);
            }
            
            if (e.target.classList.contains('vote-tip-btn')) {
                const tipId = e.target.dataset.tipId;
                const vote = e.target.dataset.vote;
                this.voteTip(tipId, vote);
            }
        });
    }

    /**
     * Utility functions
     */
    getComponentIcon(type) {
        const icons = {
            'cpu': '🖥️',
            'gpu': '🎮',
            'memory': '💾',
            'storage': '💿',
            'network': '🌐',
            'audio': '🔊'
        };
        return icons[type] || '⚙️';
    }

    detectDistribution(reportDistribution) {
        if (reportDistribution.includes('Ubuntu') || reportDistribution.includes('Debian')) return 'debian';
        if (reportDistribution.includes('Arch')) return 'arch';
        if (reportDistribution.includes('Fedora') || reportDistribution.includes('RHEL')) return 'fedora';
        if (reportDistribution.includes('NixOS')) return 'nixos';
        return 'debian'; // default
    }

    findReportById(reportId) {
        return this.databaseIndexer?.rawData.find(report => report.id === reportId);
    }

    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    async copyToClipboard(text) {
        try {
            await navigator.clipboard.writeText(text);
        } catch (err) {
            console.error('Failed to copy text:', err);
        }
    }

    showCopyFeedback(button) {
        const originalText = button.textContent;
        button.textContent = '✅';
        button.style.background = '#22c55e';
        
        setTimeout(() => {
            button.textContent = originalText;
            button.style.background = '';
        }, 2000);
    }

    voteTip(tipId, vote) {
        // In production, this would send to a backend
        console.log(`Voting ${vote} for tip ${tipId}`);
    }

    showTipSubmissionForm() {
        console.log('Opening tip submission form...');
        // This would open a form to submit new tips
    }

    copyAllCommands() {
        const commands = Array.from(document.querySelectorAll('.command-block code')).map(el => el.textContent);
        this.copyToClipboard(commands.join('\n'));
    }

    addConfigTipsToModal(report) {
        // Add configuration tips to hardware details modal
        const modal = document.querySelector('#hardware-details-modal');
        if (!modal) {
            console.warn('Hardware details modal not found');
            return;
        }

        const modalBody = modal.querySelector('#hardware-details-body');
        if (!modalBody) {
            console.warn('Hardware details modal body not found');
            return;
        }

        // Check if tips section already exists
        if (modal.querySelector('.config-tips-section')) {
            return; // Already added
        }

        // Create configuration tips section
        const tipsSection = document.createElement('div');
        tipsSection.className = 'config-tips-section';
        tipsSection.innerHTML = `
            <div class="config-tips-header">
                <h4>💡 Configuration Tips</h4>
                <p>Community-contributed configuration tips for this hardware</p>
            </div>
            <div class="config-tips-content">
                <p>Loading configuration tips...</p>
            </div>
        `;

        // Insert after the hardware details
        modalBody.appendChild(tipsSection);

        // Load tips for this hardware
        setTimeout(() => {
            this.loadTipsForHardware(report, tipsSection.querySelector('.config-tips-content'));
        }, 100);
    }

    loadTipsForHardware(report, container) {
        // Simulate loading configuration tips for the hardware
        try {
            const tips = this.getRelevantTips(report);
            if (tips.length === 0) {
                container.innerHTML = `
                    <div class="no-tips">
                        <p>No configuration tips available for this hardware yet.</p>
                        <button class="btn-secondary" onclick="configurationTips.showTipSubmissionForm()">
                            📝 Contribute a Tip
                        </button>
                    </div>
                `;
                return;
            }

            container.innerHTML = tips.map(tip => `
                <div class="config-tip-card">
                    <div class="tip-header">
                        <span class="tip-category">${tip.category}</span>
                        <span class="tip-rating">⭐ ${tip.rating}/5</span>
                    </div>
                    <div class="tip-content">
                        <h5>${tip.title}</h5>
                        <p>${tip.description}</p>
                        ${tip.commands ? `
                            <div class="command-block">
                                <code>${tip.commands}</code>
                                <button class="copy-command-btn" onclick="configurationTips.copyToClipboard('${tip.commands.replace(/'/g, "\\'")}')">📋</button>
                            </div>
                        ` : ''}
                    </div>
                </div>
            `).join('');
        } catch (error) {
            console.error('Error loading tips for hardware:', error);
            container.innerHTML = `
                <div class="error-tips">
                    <p>Error loading configuration tips.</p>
                </div>
            `;
        }
    }

    getRelevantTips(report) {
        // Return mock tips for demonstration
        return [
            {
                category: 'Performance',
                rating: 4.5,
                title: 'Optimize NVMe Performance',
                description: 'Enable queue depth optimization for better SSD performance',
                commands: 'echo mq-deadline > /sys/block/nvme0n1/queue/scheduler'
            },
            {
                category: 'Power Management',
                rating: 4.2,
                title: 'Enable ASPM Power Saving',
                description: 'Reduce power consumption while maintaining performance',
                commands: 'echo powersave > /sys/module/pcie_aspm/parameters/policy'
            }
        ];
    }

    createTipSubmissionInterface() {
        // This would create an interface for users to submit their own tips
    }
}

// Global instance
window.configurationTips = new ConfigurationTips();

// Auto-initialize when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
    await window.configurationTips.initialize();
});