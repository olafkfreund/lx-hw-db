/**
 * Configuration Profile Builder
 * Creates personalized hardware configuration profiles based on detected hardware
 * and community-contributed tips
 */
class ConfigurationProfileBuilder {
    constructor() {
        this.detectedHardware = [];
        this.relevantTips = [];
        this.currentProfile = null;
        this.builderModal = null;
        this.detectionStatus = 'idle'; // idle, detecting, complete, error
        
        // Hardware detection patterns for web-based detection
        this.browserHardwareDetection = {
            gpu: this.detectGPUInfo.bind(this),
            screen: this.detectScreenInfo.bind(this),
            platform: this.detectPlatformInfo.bind(this),
            memory: this.detectMemoryInfo.bind(this),
            network: this.detectNetworkInfo.bind(this)
        };
        
        this.init();
    }
    
    init() {
        this.createProfileBuilderInterface();
        this.bindEvents();
        this.loadSampleHardwareProfiles();
    }
    
    createProfileBuilderInterface() {
        // Add profile builder button to main interface
        const searchContainer = document.getElementById('search-container');
        if (searchContainer) {
            const profileBuilderButton = document.createElement('button');
            profileBuilderButton.id = 'profile-builder-btn';
            profileBuilderButton.className = 'profile-builder-trigger';
            profileBuilderButton.innerHTML = `
                <svg class="profile-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                    <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"></path>
                    <circle cx="12" cy="7" r="4"></circle>
                </svg>
                Build My Configuration Profile
            `;
            
            searchContainer.appendChild(profileBuilderButton);
        }
    }
    
    bindEvents() {
        document.addEventListener('click', (e) => {
            if (e.target.id === 'profile-builder-btn' || e.target.closest('#profile-builder-btn')) {
                this.openProfileBuilder();
            }
            
            if (e.target.classList.contains('detect-hardware-btn')) {
                this.startHardwareDetection();
            }
            
            if (e.target.classList.contains('manual-hardware-btn')) {
                this.openManualHardwareEntry();
            }
            
            if (e.target.classList.contains('generate-profile-btn')) {
                this.generateConfigurationProfile();
            }
            
            if (e.target.classList.contains('export-profile-btn')) {
                this.exportProfile();
            }
            
            if (e.target.classList.contains('save-profile-btn')) {
                this.saveProfile();
            }
        });
        
        // Listen for custom events from other modules
        document.addEventListener('hardware-detected', (e) => {
            this.handleDetectedHardware(e.detail);
        });
        
        document.addEventListener('tips-loaded', (e) => {
            this.updateRelevantTips();
        });
    }
    
    openProfileBuilder() {
        this.builderModal = document.createElement('div');
        this.builderModal.className = 'modal-overlay profile-builder-modal';
        this.builderModal.innerHTML = `
            <div class="modal-content profile-builder-content">
                <div class="modal-header">
                    <h2>🔧 Configuration Profile Builder</h2>
                    <button class="modal-close" aria-label="Close">&times;</button>
                </div>
                
                <div class="profile-builder-body">
                    <!-- Hardware Detection Section -->
                    <div class="profile-section hardware-detection-section">
                        <h3>💻 Hardware Detection</h3>
                        <p>Let's identify your hardware to find the best configuration tips.</p>
                        
                        <div class="detection-options">
                            <button class="detect-hardware-btn primary-button">
                                🔍 Auto-Detect Hardware
                            </button>
                            <button class="manual-hardware-btn secondary-button">
                                ✏️ Manual Entry
                            </button>
                        </div>
                        
                        <div class="detection-status">
                            <div class="status-indicator ${this.detectionStatus}">
                                <span class="status-text">Ready to detect</span>
                                <div class="detection-progress">
                                    <div class="progress-bar"></div>
                                </div>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Detected Hardware Section -->
                    <div class="profile-section detected-hardware-section">
                        <h3>🖥️ Detected Hardware</h3>
                        <div class="hardware-list">
                            ${this.renderHardwareList()}
                        </div>
                    </div>
                    
                    <!-- Distribution Selection -->
                    <div class="profile-section distribution-section">
                        <h3>🐧 Linux Distribution</h3>
                        <select id="profile-distribution" class="distribution-select">
                            <option value="">Select your distribution...</option>
                            <option value="debian">Debian / Ubuntu</option>
                            <option value="arch">Arch Linux</option>
                            <option value="fedora">Fedora / RHEL / CentOS</option>
                            <option value="nixos">NixOS</option>
                            <option value="opensuse">openSUSE</option>
                            <option value="gentoo">Gentoo</option>
                            <option value="alpine">Alpine Linux</option>
                        </select>
                    </div>
                    
                    <!-- Use Case Selection -->
                    <div class="profile-section usecase-section">
                        <h3>🎯 Primary Use Case</h3>
                        <div class="usecase-grid">
                            <label class="usecase-option">
                                <input type="radio" name="usecase" value="desktop">
                                <div class="usecase-card">
                                    <span class="usecase-icon">🖥️</span>
                                    <span class="usecase-label">Desktop</span>
                                </div>
                            </label>
                            <label class="usecase-option">
                                <input type="radio" name="usecase" value="gaming">
                                <div class="usecase-card">
                                    <span class="usecase-icon">🎮</span>
                                    <span class="usecase-label">Gaming</span>
                                </div>
                            </label>
                            <label class="usecase-option">
                                <input type="radio" name="usecase" value="server">
                                <div class="usecase-card">
                                    <span class="usecase-icon">🖧</span>
                                    <span class="usecase-label">Server</span>
                                </div>
                            </label>
                            <label class="usecase-option">
                                <input type="radio" name="usecase" value="development">
                                <div class="usecase-card">
                                    <span class="usecase-icon">💻</span>
                                    <span class="usecase-label">Development</span>
                                </div>
                            </label>
                            <label class="usecase-option">
                                <input type="radio" name="usecase" value="media">
                                <div class="usecase-card">
                                    <span class="usecase-icon">🎬</span>
                                    <span class="usecase-label">Media Center</span>
                                </div>
                            </label>
                            <label class="usecase-option">
                                <input type="radio" name="usecase" value="workstation">
                                <div class="usecase-card">
                                    <span class="usecase-icon">🏢</span>
                                    <span class="usecase-label">Workstation</span>
                                </div>
                            </label>
                        </div>
                    </div>
                    
                    <!-- Profile Generation -->
                    <div class="profile-section generation-section">
                        <div class="generation-controls">
                            <button class="generate-profile-btn primary-button" disabled>
                                ⚙️ Generate Configuration Profile
                            </button>
                        </div>
                    </div>
                    
                    <!-- Generated Profile Display -->
                    <div class="profile-section profile-results-section" style="display: none;">
                        <h3>📋 Your Configuration Profile</h3>
                        <div class="profile-results">
                            <!-- Profile content will be populated here -->
                        </div>
                        
                        <div class="profile-actions">
                            <button class="export-profile-btn secondary-button">
                                📤 Export Profile
                            </button>
                            <button class="save-profile-btn secondary-button">
                                💾 Save Profile
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        `;
        
        document.body.appendChild(this.builderModal);
        
        // Bind modal close events
        this.builderModal.querySelector('.modal-close').addEventListener('click', () => {
            this.closeProfileBuilder();
        });
        
        this.builderModal.addEventListener('click', (e) => {
            if (e.target === this.builderModal) {
                this.closeProfileBuilder();
            }
        });
        
        // Enable generate button when requirements are met
        this.updateGenerateButtonState();
        
        // Listen for form changes to update button state
        this.builderModal.addEventListener('change', () => {
            this.updateGenerateButtonState();
        });
    }
    
    closeProfileBuilder() {
        if (this.builderModal) {
            this.builderModal.remove();
            this.builderModal = null;
        }
    }
    
    startHardwareDetection() {
        this.detectionStatus = 'detecting';
        this.updateDetectionStatus('Detecting hardware...', 0);
        
        // Start detection process
        this.performBrowserHardwareDetection();
    }
    
    async performBrowserHardwareDetection() {
        const detectionSteps = [
            { name: 'GPU Information', method: 'gpu' },
            { name: 'Display Information', method: 'screen' },
            { name: 'Platform Information', method: 'platform' },
            { name: 'Memory Information', method: 'memory' },
            { name: 'Network Information', method: 'network' }
        ];
        
        let completedSteps = 0;
        this.detectedHardware = [];
        
        for (const step of detectionSteps) {
            try {
                this.updateDetectionStatus(`Detecting ${step.name}...`, 
                    (completedSteps / detectionSteps.length) * 100);
                
                const hardware = await this.browserHardwareDetection[step.method]();
                if (hardware) {
                    this.detectedHardware.push(hardware);
                }
                
                completedSteps++;
                await this.sleep(500); // Small delay for UX
            } catch (error) {
                console.warn(`Failed to detect ${step.name}:`, error);
                completedSteps++;
            }
        }
        
        this.detectionStatus = 'complete';
        this.updateDetectionStatus('Hardware detection complete!', 100);
        this.updateHardwareDisplay();
        this.updateGenerateButtonState();
    }
    
    async detectGPUInfo() {
        try {
            const canvas = document.createElement('canvas');
            const gl = canvas.getContext('webgl') || canvas.getContext('experimental-webgl');
            
            if (!gl) {
                throw new Error('WebGL not supported');
            }
            
            const debugInfo = gl.getExtension('WEBGL_debug_renderer_info');
            const vendor = debugInfo ? gl.getParameter(debugInfo.UNMASKED_VENDOR_WEBGL) : 'Unknown';
            const renderer = debugInfo ? gl.getParameter(debugInfo.UNMASKED_RENDERER_WEBGL) : 'Unknown';
            
            return {
                category: 'gpu',
                name: `${vendor} ${renderer}`.trim(),
                vendor: vendor,
                model: renderer,
                detectionMethod: 'WebGL',
                capabilities: {
                    webgl_version: gl.getParameter(gl.VERSION),
                    shading_language_version: gl.getParameter(gl.SHADING_LANGUAGE_VERSION),
                    max_texture_size: gl.getParameter(gl.MAX_TEXTURE_SIZE)
                }
            };
        } catch (error) {
            return {
                category: 'gpu',
                name: 'GPU Detection Failed',
                vendor: 'Unknown',
                model: 'Unknown',
                detectionMethod: 'Browser Limitation',
                error: error.message
            };
        }
    }
    
    async detectScreenInfo() {
        try {
            return {
                category: 'display',
                name: `Display ${screen.width}x${screen.height}`,
                resolution: {
                    width: screen.width,
                    height: screen.height
                },
                colorDepth: screen.colorDepth,
                pixelRatio: window.devicePixelRatio || 1,
                detectionMethod: 'Screen API'
            };
        } catch (error) {
            return null;
        }
    }
    
    async detectPlatformInfo() {
        try {
            return {
                category: 'system',
                name: `${navigator.platform} System`,
                platform: navigator.platform,
                userAgent: navigator.userAgent,
                languages: navigator.languages,
                cookieEnabled: navigator.cookieEnabled,
                detectionMethod: 'Navigator API'
            };
        } catch (error) {
            return null;
        }
    }
    
    async detectMemoryInfo() {
        try {
            // This requires HTTPS and specific flags, so it's often not available
            if ('memory' in performance) {
                return {
                    category: 'memory',
                    name: 'System Memory Info',
                    totalJSHeap: performance.memory.totalJSHeapSize,
                    usedJSHeap: performance.memory.usedJSHeapSize,
                    jsHeapLimit: performance.memory.jsHeapSizeLimit,
                    detectionMethod: 'Performance Memory API'
                };
            }
            return null;
        } catch (error) {
            return null;
        }
    }
    
    async detectNetworkInfo() {
        try {
            if ('connection' in navigator) {
                const connection = navigator.connection;
                return {
                    category: 'network',
                    name: `Network Connection (${connection.effectiveType})`,
                    effectiveType: connection.effectiveType,
                    downlink: connection.downlink,
                    rtt: connection.rtt,
                    saveData: connection.saveData,
                    detectionMethod: 'Network Information API'
                };
            }
            return null;
        } catch (error) {
            return null;
        }
    }
    
    openManualHardwareEntry() {
        const manualEntryModal = document.createElement('div');
        manualEntryModal.className = 'modal-overlay manual-hardware-modal';
        manualEntryModal.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h3>✏️ Manual Hardware Entry</h3>
                    <button class="modal-close" aria-label="Close">&times;</button>
                </div>
                
                <form class="manual-hardware-form">
                    <div class="form-group">
                        <label for="hw-category">Hardware Category:</label>
                        <select id="hw-category" name="category" required>
                            <option value="">Select category...</option>
                            <option value="cpu">CPU</option>
                            <option value="gpu">GPU</option>
                            <option value="motherboard">Motherboard</option>
                            <option value="memory">Memory</option>
                            <option value="storage">Storage</option>
                            <option value="network">Network</option>
                            <option value="audio">Audio</option>
                            <option value="usb">USB Controller</option>
                            <option value="bluetooth">Bluetooth</option>
                            <option value="wifi">WiFi</option>
                        </select>
                    </div>
                    
                    <div class="form-group">
                        <label for="hw-vendor">Vendor:</label>
                        <input type="text" id="hw-vendor" name="vendor" 
                               placeholder="e.g., Intel, AMD, NVIDIA" required>
                    </div>
                    
                    <div class="form-group">
                        <label for="hw-model">Model:</label>
                        <input type="text" id="hw-model" name="model" 
                               placeholder="e.g., RTX 3080, Ryzen 5 3600" required>
                    </div>
                    
                    <div class="form-group">
                        <label for="hw-notes">Additional Notes:</label>
                        <textarea id="hw-notes" name="notes" rows="3"
                                  placeholder="Any additional details..."></textarea>
                    </div>
                    
                    <div class="form-actions">
                        <button type="submit" class="primary-button">Add Hardware</button>
                        <button type="button" class="secondary-button close-manual">Cancel</button>
                    </div>
                </form>
            </div>
        `;
        
        document.body.appendChild(manualEntryModal);
        
        const closeModal = () => {
            manualEntryModal.remove();
        };
        
        manualEntryModal.querySelector('.modal-close').addEventListener('click', closeModal);
        manualEntryModal.querySelector('.close-manual').addEventListener('click', closeModal);
        
        manualEntryModal.querySelector('.manual-hardware-form').addEventListener('submit', (e) => {
            e.preventDefault();
            const formData = new FormData(e.target);
            
            this.detectedHardware.push({
                category: formData.get('category'),
                name: `${formData.get('vendor')} ${formData.get('model')}`,
                vendor: formData.get('vendor'),
                model: formData.get('model'),
                notes: formData.get('notes'),
                detectionMethod: 'Manual Entry'
            });
            
            this.updateHardwareDisplay();
            this.updateGenerateButtonState();
            closeModal();
        });
        
        manualEntryModal.addEventListener('click', (e) => {
            if (e.target === manualEntryModal) {
                closeModal();
            }
        });
    }
    
    updateDetectionStatus(text, progress) {
        const statusText = this.builderModal?.querySelector('.status-text');
        const progressBar = this.builderModal?.querySelector('.progress-bar');
        
        if (statusText) statusText.textContent = text;
        if (progressBar) progressBar.style.width = `${progress}%`;
    }
    
    updateHardwareDisplay() {
        const hardwareList = this.builderModal?.querySelector('.hardware-list');
        if (!hardwareList) return;
        
        hardwareList.innerHTML = this.renderHardwareList();
    }
    
    renderHardwareList() {
        if (this.detectedHardware.length === 0) {
            return `
                <div class="no-hardware-message">
                    <p>No hardware detected yet. Use the detection buttons above to identify your hardware.</p>
                </div>
            `;
        }
        
        return this.detectedHardware.map(hardware => `
            <div class="hardware-item">
                <div class="hardware-info">
                    <div class="hardware-category">${hardware.category.toUpperCase()}</div>
                    <div class="hardware-name">${hardware.name}</div>
                    <div class="hardware-details">
                        Detected via: ${hardware.detectionMethod}
                    </div>
                </div>
                <button class="remove-hardware-btn" data-hardware-id="${hardware.name}">
                    ❌
                </button>
            </div>
        `).join('');
    }
    
    updateGenerateButtonState() {
        const generateBtn = this.builderModal?.querySelector('.generate-profile-btn');
        if (!generateBtn) return;
        
        const hasHardware = this.detectedHardware.length > 0;
        const hasDistribution = this.builderModal?.querySelector('#profile-distribution')?.value;
        const hasUseCase = this.builderModal?.querySelector('input[name="usecase"]:checked')?.value;
        
        const canGenerate = hasHardware && hasDistribution && hasUseCase;
        
        generateBtn.disabled = !canGenerate;
        generateBtn.textContent = canGenerate ? 
            '⚙️ Generate Configuration Profile' : 
            '⚙️ Complete requirements above';
    }
    
    async generateConfigurationProfile() {
        const distribution = this.builderModal.querySelector('#profile-distribution').value;
        const useCase = this.builderModal.querySelector('input[name="usecase"]:checked').value;
        
        // Show loading state
        const resultsSection = this.builderModal.querySelector('.profile-results-section');
        const profileResults = this.builderModal.querySelector('.profile-results');
        
        resultsSection.style.display = 'block';
        profileResults.innerHTML = `
            <div class="profile-loading">
                <div class="loading-spinner"></div>
                <p>Analyzing your hardware and finding relevant configuration tips...</p>
            </div>
        `;
        
        // Simulate analysis delay
        await this.sleep(2000);
        
        // Find relevant tips for detected hardware
        await this.findRelevantTips(distribution, useCase);
        
        // Generate the profile
        this.currentProfile = this.buildConfigurationProfile(distribution, useCase);
        
        // Display the results
        profileResults.innerHTML = this.renderConfigurationProfile();
    }
    
    async findRelevantTips(distribution, useCase) {
        // In a real implementation, this would query the tips database
        // For now, we'll simulate finding relevant tips
        
        const allTips = window.configurationTips?.getAllTips() || this.getSampleTips();
        this.relevantTips = [];
        
        // Match tips based on detected hardware
        for (const hardware of this.detectedHardware) {
            const hardwareTips = allTips.filter(tip => 
                tip.component.toLowerCase() === hardware.category ||
                tip.title.toLowerCase().includes(hardware.vendor.toLowerCase()) ||
                tip.title.toLowerCase().includes(hardware.model.toLowerCase()) ||
                tip.distributions.includes(distribution)
            );
            
            this.relevantTips.push(...hardwareTips);
        }
        
        // Remove duplicates and sort by rating
        this.relevantTips = Array.from(new Set(this.relevantTips.map(tip => tip.id)))
            .map(id => this.relevantTips.find(tip => tip.id === id))
            .sort((a, b) => b.rating - a.rating)
            .slice(0, 10); // Limit to top 10 tips
    }
    
    buildConfigurationProfile(distribution, useCase) {
        return {
            id: `profile_${Date.now()}`,
            name: `${distribution.charAt(0).toUpperCase() + distribution.slice(1)} ${useCase.charAt(0).toUpperCase() + useCase.slice(1)} Profile`,
            created: new Date().toISOString(),
            hardware: this.detectedHardware,
            distribution: distribution,
            useCase: useCase,
            tips: this.relevantTips,
            recommendations: this.generateRecommendations(distribution, useCase),
            summary: this.generateProfileSummary()
        };
    }
    
    generateRecommendations(distribution, useCase) {
        const recommendations = {
            priority_tips: [],
            package_recommendations: [],
            kernel_parameters: [],
            configuration_tweaks: []
        };
        
        // Analyze detected hardware for recommendations
        for (const hardware of this.detectedHardware) {
            switch (hardware.category) {
                case 'gpu':
                    if (hardware.vendor.toLowerCase().includes('nvidia')) {
                        recommendations.package_recommendations.push({
                            type: 'driver',
                            package: this.getPackageName('nvidia-driver', distribution),
                            description: 'NVIDIA proprietary drivers for optimal performance'
                        });
                    } else if (hardware.vendor.toLowerCase().includes('amd')) {
                        recommendations.package_recommendations.push({
                            type: 'driver',
                            package: this.getPackageName('mesa-vulkan', distribution),
                            description: 'Open source AMD GPU drivers with Vulkan support'
                        });
                    }
                    break;
                    
                case 'system':
                    if (useCase === 'gaming') {
                        recommendations.kernel_parameters.push({
                            parameter: 'mitigations=off',
                            description: 'Disable CPU mitigations for better gaming performance (reduces security)'
                        });
                    }
                    break;
            }
        }
        
        // Use case specific recommendations
        switch (useCase) {
            case 'gaming':
                recommendations.configuration_tweaks.push({
                    description: 'Enable GameMode for automatic game optimizations',
                    command: this.getPackageName('gamemode', distribution, 'install')
                });
                break;
                
            case 'development':
                recommendations.configuration_tweaks.push({
                    description: 'Increase inotify limits for development tools',
                    file: '/etc/sysctl.conf',
                    setting: 'fs.inotify.max_user_watches=524288'
                });
                break;
        }
        
        return recommendations;
    }
    
    getPackageName(genericName, distribution, action = 'name') {
        const packageMappings = {
            'nvidia-driver': {
                debian: 'nvidia-driver',
                arch: 'nvidia',
                fedora: 'akmod-nvidia',
                nixos: 'nvidia_x11',
                opensuse: 'nvidia-driver-G06',
                gentoo: 'nvidia-drivers',
                alpine: 'nvidia-driver'
            },
            'mesa-vulkan': {
                debian: 'mesa-vulkan-drivers',
                arch: 'vulkan-radeon',
                fedora: 'mesa-vulkan-drivers',
                nixos: 'mesa_drivers',
                opensuse: 'Mesa-vulkan-drivers',
                gentoo: 'media-libs/mesa',
                alpine: 'mesa-vulkan-ati'
            },
            'gamemode': {
                debian: 'gamemode',
                arch: 'gamemode',
                fedora: 'gamemode',
                nixos: 'gamemode',
                opensuse: 'gamemode',
                gentoo: 'games-util/gamemode',
                alpine: 'gamemode'
            }
        };
        
        const packageName = packageMappings[genericName]?.[distribution] || genericName;
        
        if (action === 'install') {
            const installCommands = {
                debian: `sudo apt install ${packageName}`,
                arch: `sudo pacman -S ${packageName}`,
                fedora: `sudo dnf install ${packageName}`,
                nixos: `# Add to configuration.nix: environment.systemPackages = [ pkgs.${packageName} ];`,
                opensuse: `sudo zypper install ${packageName}`,
                gentoo: `sudo emerge ${packageName}`,
                alpine: `sudo apk add ${packageName}`
            };
            return installCommands[distribution] || `# Install ${packageName}`;
        }
        
        return packageName;
    }
    
    generateProfileSummary() {
        const hardwareCount = this.detectedHardware.length;
        const tipCount = this.relevantTips.length;
        const topRated = this.relevantTips.length > 0 ? Math.max(...this.relevantTips.map(t => t.rating)) : 0;
        
        return {
            hardware_detected: hardwareCount,
            relevant_tips: tipCount,
            top_tip_rating: topRated,
            confidence_score: Math.min(100, (hardwareCount * 10) + (tipCount * 5))
        };
    }
    
    renderConfigurationProfile() {
        if (!this.currentProfile) return '';
        
        return `
            <div class="configuration-profile">
                <div class="profile-header">
                    <h4>${this.currentProfile.name}</h4>
                    <div class="profile-meta">
                        <span class="profile-confidence">
                            Confidence: ${this.currentProfile.summary.confidence_score}%
                        </span>
                        <span class="profile-date">
                            Created: ${new Date(this.currentProfile.created).toLocaleDateString()}
                        </span>
                    </div>
                </div>
                
                <div class="profile-sections">
                    <!-- Hardware Summary -->
                    <div class="profile-section">
                        <h5>💻 Hardware Summary</h5>
                        <div class="hardware-summary">
                            ${this.currentProfile.hardware.map(hw => `
                                <div class="hw-summary-item">
                                    <span class="hw-category">${hw.category.toUpperCase()}</span>
                                    <span class="hw-name">${hw.name}</span>
                                </div>
                            `).join('')}
                        </div>
                    </div>
                    
                    <!-- Recommended Packages -->
                    ${this.currentProfile.recommendations.package_recommendations.length > 0 ? `
                        <div class="profile-section">
                            <h5>📦 Recommended Packages</h5>
                            <div class="package-recommendations">
                                ${this.currentProfile.recommendations.package_recommendations.map(pkg => `
                                    <div class="package-item">
                                        <div class="package-name">${pkg.package}</div>
                                        <div class="package-description">${pkg.description}</div>
                                    </div>
                                `).join('')}
                            </div>
                        </div>
                    ` : ''}
                    
                    <!-- Configuration Tips -->
                    ${this.currentProfile.tips.length > 0 ? `
                        <div class="profile-section">
                            <h5>💡 Relevant Configuration Tips</h5>
                            <div class="profile-tips">
                                ${this.currentProfile.tips.slice(0, 5).map(tip => `
                                    <div class="profile-tip">
                                        <div class="tip-header">
                                            <span class="tip-title">${tip.title}</span>
                                            <span class="tip-rating">⭐ ${tip.rating.toFixed(1)}</span>
                                        </div>
                                        <div class="tip-description">${tip.description}</div>
                                        <div class="tip-component">${tip.component}</div>
                                    </div>
                                `).join('')}
                            </div>
                        </div>
                    ` : ''}
                    
                    <!-- Kernel Parameters -->
                    ${this.currentProfile.recommendations.kernel_parameters.length > 0 ? `
                        <div class="profile-section">
                            <h5>⚙️ Kernel Parameters</h5>
                            <div class="kernel-parameters">
                                ${this.currentProfile.recommendations.kernel_parameters.map(param => `
                                    <div class="kernel-param">
                                        <code>${param.parameter}</code>
                                        <span class="param-description">${param.description}</span>
                                    </div>
                                `).join('')}
                            </div>
                        </div>
                    ` : ''}
                    
                    <!-- Configuration Tweaks -->
                    ${this.currentProfile.recommendations.configuration_tweaks.length > 0 ? `
                        <div class="profile-section">
                            <h5>🔧 Configuration Tweaks</h5>
                            <div class="config-tweaks">
                                ${this.currentProfile.recommendations.configuration_tweaks.map(tweak => `
                                    <div class="config-tweak">
                                        <div class="tweak-description">${tweak.description}</div>
                                        ${tweak.command ? `<code class="tweak-command">${tweak.command}</code>` : ''}
                                        ${tweak.file && tweak.setting ? `
                                            <div class="tweak-file">
                                                <span class="file-path">${tweak.file}</span>
                                                <code class="file-setting">${tweak.setting}</code>
                                            </div>
                                        ` : ''}
                                    </div>
                                `).join('')}
                            </div>
                        </div>
                    ` : ''}
                </div>
            </div>
        `;
    }
    
    exportProfile() {
        if (!this.currentProfile) return;
        
        const exportModal = document.createElement('div');
        exportModal.className = 'modal-overlay export-profile-modal';
        exportModal.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h3>📤 Export Configuration Profile</h3>
                    <button class="modal-close" aria-label="Close">&times;</button>
                </div>
                
                <div class="export-options">
                    <div class="export-format-grid">
                        <button class="export-btn" data-format="json">
                            📄 JSON
                        </button>
                        <button class="export-btn" data-format="yaml">
                            📝 YAML
                        </button>
                        <button class="export-btn" data-format="markdown">
                            📖 Markdown
                        </button>
                        <button class="export-btn" data-format="script">
                            🔧 Install Script
                        </button>
                    </div>
                </div>
                
                <div class="export-preview">
                    <textarea readonly class="export-content" placeholder="Select an export format above..."></textarea>
                </div>
                
                <div class="export-actions">
                    <button class="download-btn secondary-button" disabled>
                        ⬇️ Download
                    </button>
                    <button class="copy-btn secondary-button" disabled>
                        📋 Copy to Clipboard
                    </button>
                </div>
            </div>
        `;
        
        document.body.appendChild(exportModal);
        
        const closeModal = () => exportModal.remove();
        exportModal.querySelector('.modal-close').addEventListener('click', closeModal);
        
        // Handle export format selection
        exportModal.addEventListener('click', (e) => {
            if (e.target.classList.contains('export-btn')) {
                const format = e.target.dataset.format;
                const content = this.generateExportContent(format);
                const contentArea = exportModal.querySelector('.export-content');
                const downloadBtn = exportModal.querySelector('.download-btn');
                const copyBtn = exportModal.querySelector('.copy-btn');
                
                contentArea.value = content;
                downloadBtn.disabled = false;
                copyBtn.disabled = false;
                
                // Update active state
                exportModal.querySelectorAll('.export-btn').forEach(btn => 
                    btn.classList.remove('active'));
                e.target.classList.add('active');
                
                // Handle download
                downloadBtn.onclick = () => this.downloadProfile(content, format);
                
                // Handle copy
                copyBtn.onclick = () => {
                    contentArea.select();
                    document.execCommand('copy');
                    copyBtn.textContent = '✅ Copied!';
                    setTimeout(() => {
                        copyBtn.textContent = '📋 Copy to Clipboard';
                    }, 2000);
                };
            }
            
            if (e.target === exportModal) {
                closeModal();
            }
        });
    }
    
    generateExportContent(format) {
        switch (format) {
            case 'json':
                return JSON.stringify(this.currentProfile, null, 2);
            
            case 'yaml':
                return this.toYAML(this.currentProfile);
            
            case 'markdown':
                return this.toMarkdown(this.currentProfile);
            
            case 'script':
                return this.toInstallScript(this.currentProfile);
            
            default:
                return '';
        }
    }
    
    toYAML(profile) {
        return `# Configuration Profile: ${profile.name}
# Created: ${profile.created}

name: ${profile.name}
distribution: ${profile.distribution}
use_case: ${profile.useCase}
confidence_score: ${profile.summary.confidence_score}

detected_hardware:
${profile.hardware.map(hw => `  - category: ${hw.category}
    name: "${hw.name}"
    vendor: ${hw.vendor}
    model: ${hw.model}`).join('\n')}

recommended_packages:
${profile.recommendations.package_recommendations.map(pkg => `  - name: ${pkg.package}
    description: "${pkg.description}"`).join('\n')}

configuration_tips:
${profile.tips.slice(0, 5).map(tip => `  - title: "${tip.title}"
    rating: ${tip.rating}
    component: ${tip.component}
    description: "${tip.description}"`).join('\n')}`;
    }
    
    toMarkdown(profile) {
        return `# ${profile.name}

**Created:** ${new Date(profile.created).toLocaleDateString()}  
**Distribution:** ${profile.distribution}  
**Use Case:** ${profile.useCase}  
**Confidence Score:** ${profile.summary.confidence_score}%

## Detected Hardware

${profile.hardware.map(hw => `- **${hw.category.toUpperCase()}:** ${hw.name}`).join('\n')}

${profile.recommendations.package_recommendations.length > 0 ? `
## Recommended Packages

${profile.recommendations.package_recommendations.map(pkg => `- **${pkg.package}:** ${pkg.description}`).join('\n')}
` : ''}

${profile.tips.length > 0 ? `
## Configuration Tips

${profile.tips.slice(0, 5).map(tip => `
### ${tip.title} (⭐ ${tip.rating.toFixed(1)})
**Component:** ${tip.component}  
${tip.description}
`).join('\n')}
` : ''}`;
    }
    
    toInstallScript(profile) {
        const packages = profile.recommendations.package_recommendations;
        const tweaks = profile.recommendations.configuration_tweaks;
        
        return `#!/bin/bash
# Configuration Profile Installation Script: ${profile.name}
# Generated: ${new Date().toISOString()}
# Distribution: ${profile.distribution}

set -e

echo "Installing configuration profile: ${profile.name}"
echo "Distribution: ${profile.distribution}"
echo ""

${packages.length > 0 ? `
# Install recommended packages
echo "Installing recommended packages..."
${packages.map(pkg => {
    const cmd = this.getPackageName(pkg.package.split(' ')[0], profile.distribution, 'install');
    return `echo "Installing ${pkg.package}..."
${cmd}`;
}).join('\n')}
echo ""
` : ''}

${tweaks.length > 0 ? `
# Apply configuration tweaks
echo "Applying configuration tweaks..."
${tweaks.map(tweak => {
    if (tweak.command) {
        return `echo "${tweak.description}"
${tweak.command}`;
    }
    return `echo "${tweak.description}"
# Manual configuration required for: ${tweak.file || 'system settings'}`;
}).join('\n')}
echo ""
` : ''}

echo "Configuration profile installation complete!"
echo "Please reboot your system to ensure all changes take effect."`;
    }
    
    downloadProfile(content, format) {
        const extensions = {
            json: '.json',
            yaml: '.yml',
            markdown: '.md',
            script: '.sh'
        };
        
        const filename = `${this.currentProfile.name.toLowerCase().replace(/\s+/g, '_')}_profile${extensions[format]}`;
        const blob = new Blob([content], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        
        const a = document.createElement('a');
        a.href = url;
        a.download = filename;
        a.click();
        
        URL.revokeObjectURL(url);
    }
    
    saveProfile() {
        if (!this.currentProfile) return;
        
        // Save to localStorage
        const savedProfiles = JSON.parse(localStorage.getItem('lx-hw-db-profiles') || '[]');
        savedProfiles.push(this.currentProfile);
        localStorage.setItem('lx-hw-db-profiles', JSON.stringify(savedProfiles));
        
        // Show success message
        const saveBtn = this.builderModal.querySelector('.save-profile-btn');
        const originalText = saveBtn.textContent;
        saveBtn.textContent = '✅ Profile Saved!';
        saveBtn.disabled = true;
        
        setTimeout(() => {
            saveBtn.textContent = originalText;
            saveBtn.disabled = false;
        }, 3000);
        
        // Dispatch event for other components
        document.dispatchEvent(new CustomEvent('profile-saved', {
            detail: { profile: this.currentProfile }
        }));
    }
    
    loadSampleHardwareProfiles() {
        // Load some sample profiles for demonstration
        const sampleProfiles = [
            {
                id: 'sample_gaming_rig',
                name: 'Gaming Rig Profile',
                distribution: 'arch',
                useCase: 'gaming',
                hardware: [
                    { category: 'gpu', name: 'NVIDIA GeForce RTX 3080', vendor: 'NVIDIA', model: 'RTX 3080' },
                    { category: 'cpu', name: 'AMD Ryzen 7 5800X', vendor: 'AMD', model: 'Ryzen 7 5800X' }
                ]
            }
        ];
        
        // Store sample profiles if none exist
        if (!localStorage.getItem('lx-hw-db-profiles')) {
            localStorage.setItem('lx-hw-db-profiles', JSON.stringify(sampleProfiles));
        }
    }
    
    getSampleTips() {
        return [
            {
                id: 'nvidia_gaming_tip',
                title: 'NVIDIA Gaming Optimization',
                description: 'Enable NVIDIA GPU scaling and disable composition for better gaming performance',
                component: 'gpu',
                distributions: ['arch', 'debian', 'fedora'],
                rating: 4.8
            },
            {
                id: 'amd_performance_tip',
                title: 'AMD GPU Performance Tweaks',
                description: 'Configure mesa drivers and enable AMD GPU performance states',
                component: 'gpu',
                distributions: ['arch', 'fedora'],
                rating: 4.5
            }
        ];
    }
    
    sleep(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        window.configurationProfileBuilder = new ConfigurationProfileBuilder();
    });
} else {
    window.configurationProfileBuilder = new ConfigurationProfileBuilder();
}