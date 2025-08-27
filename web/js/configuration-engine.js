/**
 * Configuration Engine Web Interface
 * 
 * This module provides the web interface for the Phase 5 configuration engine,
 * allowing users to generate optimized Linux configurations based on their hardware.
 */

class ConfigurationEngine {
    constructor() {
        this.currentHardware = null;
        this.currentMethod = 'auto-detect';
        this.detectedDevices = [];
        this.configuration = null;
        
        this.init();
    }

    init() {
        this.initMethodSwitcher();
        this.initAutoDetection();
        this.initFileUpload();
        this.initManualSelection();
        this.initConfigurationGeneration();
        this.initResultsTabs();
        
        // Load hardware options for manual selection
        this.loadHardwareOptions();
    }

    initMethodSwitcher() {
        const methodButtons = document.querySelectorAll('.method-btn');
        const configPanels = document.querySelectorAll('.config-panel');

        methodButtons.forEach(button => {
            button.addEventListener('click', () => {
                const method = button.id.replace('-btn', '').replace('-', '-');
                
                // Update active states
                methodButtons.forEach(btn => btn.classList.remove('active'));
                configPanels.forEach(panel => panel.classList.remove('active'));
                
                button.classList.add('active');
                document.getElementById(`${method}-panel`).classList.add('active');
                
                this.currentMethod = method.replace('-', '');
            });
        });
    }

    initAutoDetection() {
        const startButton = document.getElementById('start-detection');
        const statusDiv = document.getElementById('detection-status');
        const hardwareDiv = document.getElementById('detected-hardware');

        startButton.addEventListener('click', async () => {
            statusDiv.innerHTML = '<div class="status-message">🔍 Detecting hardware...</div>';
            hardwareDiv.innerHTML = '';
            
            try {
                const hardware = await this.detectHardware();
                this.currentHardware = hardware;
                this.displayDetectedHardware(hardware, hardwareDiv);
                statusDiv.innerHTML = '<div class="status-message success">✅ Hardware detection complete</div>';
                this.showConfigurationOptions();
            } catch (error) {
                console.error('Hardware detection failed:', error);
                statusDiv.innerHTML = '<div class="status-message error">❌ Hardware detection failed. Try uploading a hardware report instead.</div>';
            }
        });
    }

    async detectHardware() {
        // Simulate hardware detection using browser APIs
        const hardware = {
            metadata: {
                version: "1.0",
                generated_at: new Date().toISOString(),
                privacy_level: "Basic",
                tools_used: ["WebAPI"],
                anonymized_system_id: this.generateSystemId()
            },
            system: {
                anonymized_hostname: "web-detected",
                kernel_version: "Unknown",
                distribution: "Unknown",
                architecture: await this.detectArchitecture(),
                boot_time: null
            },
            cpu: await this.detectCPU(),
            memory: await this.detectMemory(),
            storage: [],
            graphics: await this.detectGraphics(),
            network: await this.detectNetwork(),
            usb: [],
            audio: await this.detectAudio(),
            kernel_support: null
        };

        return hardware;
    }

    async detectArchitecture() {
        return navigator.platform || 'Unknown';
    }

    async detectCPU() {
        // Limited CPU detection via navigator
        const concurrency = navigator.hardwareConcurrency || 1;
        return {
            model: "Web-detected CPU",
            vendor: "Unknown",
            cores: Math.max(1, Math.floor(concurrency / 2)),
            threads: concurrency,
            base_frequency: null,
            max_frequency: null,
            cache_l1: null,
            cache_l2: null,
            cache_l3: null,
            flags: []
        };
    }

    async detectMemory() {
        // Estimate memory from device info
        const memory = navigator.deviceMemory ? navigator.deviceMemory * 1024 * 1024 * 1024 : null;
        return memory ? {
            total_bytes: memory,
            available_bytes: Math.floor(memory * 0.7),
            dimms: [{
                size_bytes: memory,
                speed_mhz: null,
                memory_type: null,
                manufacturer: null
            }]
        } : null;
    }

    async detectGraphics() {
        // Basic graphics detection via WebGL
        const canvas = document.createElement('canvas');
        const gl = canvas.getContext('webgl') || canvas.getContext('experimental-webgl');
        
        if (!gl) return [];

        const debugInfo = gl.getExtension('WEBGL_debug_renderer_info');
        const renderer = debugInfo ? gl.getParameter(debugInfo.UNMASKED_RENDERER_WEBGL) : 'Unknown';
        const vendor = debugInfo ? gl.getParameter(debugInfo.UNMASKED_VENDOR_WEBGL) : 'Unknown';

        return [{
            vendor: vendor,
            model: renderer,
            driver: null,
            memory_bytes: null,
            pci_id: "web-detected"
        }];
    }

    async detectNetwork() {
        // Basic network detection
        const connection = navigator.connection || navigator.mozConnection || navigator.webkitConnection;
        const effectiveType = connection ? connection.effectiveType : 'unknown';
        
        return [{
            device_type: "ethernet",
            vendor: "Unknown",
            model: `Web-detected (${effectiveType})`,
            driver: null,
            anonymized_mac: this.generateAnonymizedMac()
        }];
    }

    async detectAudio() {
        // Basic audio detection via MediaDevices API
        try {
            if (!navigator.mediaDevices || !navigator.mediaDevices.enumerateDevices) {
                return [];
            }
            
            const devices = await navigator.mediaDevices.enumerateDevices();
            const audioDevices = devices.filter(device => 
                device.kind === 'audioinput' || device.kind === 'audiooutput'
            );
            
            return audioDevices.map((device, index) => ({
                vendor: "Unknown",
                model: device.label || `Web Audio Device ${index + 1}`,
                driver: null,
                device_type: device.kind === 'audioinput' ? 'capture' : 'playback'
            }));
        } catch (error) {
            console.warn('Audio detection failed:', error);
            return [];
        }
    }

    generateSystemId() {
        return 'web-' + Math.random().toString(36).substr(2, 16);
    }

    generateAnonymizedMac() {
        return Array.from({length: 6}, () => 
            Math.floor(Math.random() * 256).toString(16).padStart(2, '0')
        ).join(':');
    }

    displayDetectedHardware(hardware, container) {
        const components = [];
        
        if (hardware.cpu) {
            components.push({
                type: 'CPU',
                name: `${hardware.cpu.vendor} ${hardware.cpu.model}`,
                details: `${hardware.cpu.cores} cores, ${hardware.cpu.threads} threads`
            });
        }
        
        if (hardware.memory) {
            const gb = Math.round(hardware.memory.total_bytes / (1024 * 1024 * 1024));
            components.push({
                type: 'Memory',
                name: `${gb}GB RAM`,
                details: `${hardware.memory.dimms.length} DIMM(s)`
            });
        }
        
        hardware.graphics.forEach(gpu => {
            components.push({
                type: 'Graphics',
                name: `${gpu.vendor} ${gpu.model}`,
                details: 'WebGL detected'
            });
        });
        
        hardware.network.forEach(net => {
            components.push({
                type: 'Network',
                name: `${net.vendor} ${net.model}`,
                details: net.device_type
            });
        });
        
        hardware.audio.forEach(audio => {
            components.push({
                type: 'Audio',
                name: audio.model,
                details: audio.device_type
            });
        });

        container.innerHTML = components.map(comp => `
            <div class="hardware-item">
                <div class="hardware-type">${comp.type}</div>
                <div class="hardware-name">${comp.name}</div>
                <div class="hardware-details">${comp.details}</div>
            </div>
        `).join('');
    }

    initFileUpload() {
        const fileInput = document.getElementById('hardware-file-input');
        const uploadArea = document.getElementById('file-upload-area');
        const selectBtn = document.getElementById('select-file-btn');
        const statusDiv = document.getElementById('upload-status');
        const hardwareDiv = document.getElementById('uploaded-hardware');

        selectBtn.addEventListener('click', () => fileInput.click());
        
        uploadArea.addEventListener('dragover', (e) => {
            e.preventDefault();
            uploadArea.classList.add('dragover');
        });
        
        uploadArea.addEventListener('dragleave', () => {
            uploadArea.classList.remove('dragover');
        });
        
        uploadArea.addEventListener('drop', (e) => {
            e.preventDefault();
            uploadArea.classList.remove('dragover');
            const files = e.dataTransfer.files;
            if (files.length > 0) {
                this.handleFileUpload(files[0], statusDiv, hardwareDiv);
            }
        });

        fileInput.addEventListener('change', (e) => {
            if (e.target.files.length > 0) {
                this.handleFileUpload(e.target.files[0], statusDiv, hardwareDiv);
            }
        });
    }

    async handleFileUpload(file, statusDiv, hardwareDiv) {
        statusDiv.innerHTML = '<div class="status-message">📁 Processing hardware report...</div>';
        hardwareDiv.innerHTML = '';
        
        try {
            const text = await file.text();
            let hardware;
            
            if (file.name.endsWith('.json')) {
                hardware = JSON.parse(text);
            } else if (file.name.endsWith('.yaml') || file.name.endsWith('.yml')) {
                // For now, show error for YAML since we don't have a parser
                throw new Error('YAML parsing not yet supported. Please use JSON format.');
            } else {
                throw new Error('Unsupported file format. Please use JSON or YAML.');
            }
            
            this.currentHardware = hardware;
            this.displayUploadedHardware(hardware, hardwareDiv);
            statusDiv.innerHTML = '<div class="status-message success">✅ Hardware report loaded successfully</div>';
            this.showConfigurationOptions();
            
        } catch (error) {
            console.error('File upload failed:', error);
            statusDiv.innerHTML = `<div class="status-message error">❌ Failed to parse file: ${error.message}</div>`;
        }
    }

    displayUploadedHardware(hardware, container) {
        const stats = {
            cpu: hardware.cpu ? 1 : 0,
            memory: hardware.memory ? 1 : 0,
            graphics: hardware.graphics ? hardware.graphics.length : 0,
            network: hardware.network ? hardware.network.length : 0,
            audio: hardware.audio ? hardware.audio.length : 0,
            usb: hardware.usb ? hardware.usb.length : 0
        };

        const totalDevices = Object.values(stats).reduce((a, b) => a + b, 0);

        container.innerHTML = `
            <div class="upload-summary">
                <h4>Hardware Report Summary</h4>
                <div class="summary-grid">
                    <div class="summary-item">
                        <span class="summary-value">${totalDevices}</span>
                        <span class="summary-label">Total Components</span>
                    </div>
                    <div class="summary-item">
                        <span class="summary-value">${stats.cpu}</span>
                        <span class="summary-label">CPU</span>
                    </div>
                    <div class="summary-item">
                        <span class="summary-value">${stats.graphics}</span>
                        <span class="summary-label">Graphics</span>
                    </div>
                    <div class="summary-item">
                        <span class="summary-value">${stats.network}</span>
                        <span class="summary-label">Network</span>
                    </div>
                    <div class="summary-item">
                        <span class="summary-value">${stats.audio}</span>
                        <span class="summary-label">Audio</span>
                    </div>
                    <div class="summary-item">
                        <span class="summary-value">${stats.usb}</span>
                        <span class="summary-label">USB</span>
                    </div>
                </div>
                <div class="report-metadata">
                    <p><strong>Generated:</strong> ${hardware.metadata ? hardware.metadata.generated_at : 'Unknown'}</p>
                    <p><strong>Privacy Level:</strong> ${hardware.metadata ? hardware.metadata.privacy_level : 'Unknown'}</p>
                    <p><strong>Architecture:</strong> ${hardware.system ? hardware.system.architecture : 'Unknown'}</p>
                </div>
            </div>
        `;
    }

    async loadHardwareOptions() {
        // Load hardware options from the search indices
        try {
            const response = await fetch('api/search_terms.json');
            const searchTerms = await response.json();
            
            this.populateHardwareSelects(searchTerms);
        } catch (error) {
            console.error('Failed to load hardware options:', error);
        }
    }

    populateHardwareSelects(searchTerms) {
        const cpuSelect = document.getElementById('cpu-select');
        const gpuSelect = document.getElementById('gpu-select');
        const networkSelect = document.getElementById('network-select');
        const audioSelect = document.getElementById('audio-select');

        // Filter terms by category
        const cpuTerms = searchTerms.filter(term => 
            term.toLowerCase().includes('intel') || 
            term.toLowerCase().includes('amd') || 
            term.toLowerCase().includes('cpu') ||
            term.toLowerCase().includes('processor')
        );
        
        const gpuTerms = searchTerms.filter(term => 
            term.toLowerCase().includes('nvidia') || 
            term.toLowerCase().includes('radeon') || 
            term.toLowerCase().includes('gpu') ||
            term.toLowerCase().includes('graphics')
        );

        // Populate selects
        cpuTerms.forEach(term => {
            const option = document.createElement('option');
            option.value = term;
            option.textContent = term;
            cpuSelect.appendChild(option);
        });

        gpuTerms.forEach(term => {
            const option = document.createElement('option');
            option.value = term;
            option.textContent = term;
            gpuSelect.appendChild(option);
        });
    }

    initManualSelection() {
        const selects = document.querySelectorAll('.hardware-select');
        const manualHardware = document.getElementById('manual-hardware');

        selects.forEach(select => {
            select.addEventListener('change', () => {
                this.updateManualSelection(manualHardware);
            });
        });
    }

    updateManualSelection(container) {
        const cpu = document.getElementById('cpu-select').value;
        const gpu = document.getElementById('gpu-select').value;
        const network = document.getElementById('network-select').value;
        const audio = document.getElementById('audio-select').value;

        const selectedComponents = [];
        if (cpu) selectedComponents.push({ type: 'CPU', name: cpu });
        if (gpu) selectedComponents.push({ type: 'GPU', name: gpu });
        if (network) selectedComponents.push({ type: 'Network', name: network });
        if (audio) selectedComponents.push({ type: 'Audio', name: audio });

        if (selectedComponents.length > 0) {
            // Create mock hardware report from selections
            this.currentHardware = this.createManualHardwareReport(selectedComponents);
            
            container.innerHTML = `
                <div class="manual-summary">
                    <h4>Selected Hardware</h4>
                    ${selectedComponents.map(comp => `
                        <div class="selected-component">
                            <span class="component-type">${comp.type}:</span>
                            <span class="component-name">${comp.name}</span>
                        </div>
                    `).join('')}
                </div>
            `;
            
            this.showConfigurationOptions();
        } else {
            container.innerHTML = '';
            this.currentHardware = null;
        }
    }

    createManualHardwareReport(components) {
        const hardware = {
            metadata: {
                version: "1.0",
                generated_at: new Date().toISOString(),
                privacy_level: "Basic",
                tools_used: ["Manual"],
                anonymized_system_id: "manual-" + Math.random().toString(36).substr(2, 16)
            },
            system: {
                anonymized_hostname: "manual-selection",
                kernel_version: "Unknown",
                distribution: "Unknown", 
                architecture: "x86_64",
                boot_time: null
            },
            cpu: null,
            memory: null,
            storage: [],
            graphics: [],
            network: [],
            usb: [],
            audio: [],
            kernel_support: null
        };

        components.forEach(comp => {
            switch (comp.type) {
                case 'CPU':
                    hardware.cpu = {
                        model: comp.name,
                        vendor: comp.name.includes('Intel') ? 'Intel' : comp.name.includes('AMD') ? 'AMD' : 'Unknown',
                        cores: 4,
                        threads: 8,
                        base_frequency: null,
                        max_frequency: null,
                        cache_l1: null,
                        cache_l2: null,
                        cache_l3: null,
                        flags: []
                    };
                    break;
                case 'GPU':
                    hardware.graphics.push({
                        vendor: comp.name.includes('NVIDIA') ? 'NVIDIA' : comp.name.includes('AMD') ? 'AMD' : 'Unknown',
                        model: comp.name,
                        driver: null,
                        memory_bytes: null,
                        pci_id: "manual"
                    });
                    break;
                case 'Network':
                    hardware.network.push({
                        device_type: "ethernet",
                        vendor: "Unknown",
                        model: comp.name,
                        driver: null,
                        anonymized_mac: this.generateAnonymizedMac()
                    });
                    break;
                case 'Audio':
                    hardware.audio.push({
                        vendor: "Unknown",
                        model: comp.name,
                        driver: null,
                        device_type: "playback"
                    });
                    break;
            }
        });

        return hardware;
    }

    showConfigurationOptions() {
        document.getElementById('configuration-options-panel').style.display = 'block';
    }

    initConfigurationGeneration() {
        const generateBtn = document.getElementById('generate-configuration');
        
        generateBtn.addEventListener('click', async () => {
            if (!this.currentHardware) {
                alert('Please select or detect hardware first.');
                return;
            }
            
            generateBtn.disabled = true;
            generateBtn.textContent = 'Generating Configuration...';
            
            try {
                await this.generateConfiguration();
                this.showConfigurationResults();
            } catch (error) {
                console.error('Configuration generation failed:', error);
                alert('Configuration generation failed. Please try again.');
            } finally {
                generateBtn.disabled = false;
                generateBtn.textContent = 'Generate Configuration';
            }
        });
    }

    async generateConfiguration() {
        const distribution = document.getElementById('target-distribution').value;
        const performanceProfile = document.getElementById('performance-profile').value;
        const compatibilityLevel = document.getElementById('compatibility-level').value;

        // Simulate calling the Rust configuration engine API
        try {
            const response = await fetch('/api/configuration/generate', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    hardware: this.currentHardware,
                    target_distribution: distribution,
                    performance_profile: performanceProfile,
                    compatibility_level: compatibilityLevel
                })
            });
            
            if (response.ok) {
                this.configuration = await response.json();
            } else {
                // Fallback to mock configuration if API is not available
                console.warn('Configuration API not available, using mock data');
                this.configuration = this.generateMockConfiguration(distribution, performanceProfile);
            }
        } catch (error) {
            console.warn('Configuration API error, using mock data:', error);
            this.configuration = this.generateMockConfiguration(distribution, performanceProfile);
        }
    }

    generateMockConfiguration(distribution, performanceProfile) {
        const config = {
            system_id: this.currentHardware.metadata.anonymized_system_id,
            target_distribution: distribution,
            kernel_version: "6.1.0",
            hardware_profile: {
                cpu: this.currentHardware.cpu,
                gpu: this.currentHardware.graphics,
                network: this.currentHardware.network,
                storage: this.currentHardware.storage,
                audio: this.currentHardware.audio,
                usb_controllers: []
            },
            driver_recommendations: [],
            kernel_parameters: [],
            package_installations: [],
            dkms_modules: [],
            configuration_files: {},
            performance_optimizations: [],
            compatibility_score: 85.5
        };

        // Generate driver recommendations
        if (this.currentHardware.cpu) {
            const vendor = this.currentHardware.cpu.vendor.toLowerCase();
            if (vendor.includes('intel')) {
                config.driver_recommendations.push({
                    component_type: "CPU",
                    component_name: this.currentHardware.cpu.model,
                    recommended_driver: "intel_pstate",
                    driver_type: "built-in",
                    priority: "high",
                    compatibility_notes: "Native Intel CPU scaling driver"
                });
            } else if (vendor.includes('amd')) {
                config.driver_recommendations.push({
                    component_type: "CPU", 
                    component_name: this.currentHardware.cpu.model,
                    recommended_driver: "amd_pstate",
                    driver_type: "built-in",
                    priority: "high",
                    compatibility_notes: "Native AMD CPU scaling driver"
                });
            }
        }

        // Generate graphics drivers
        this.currentHardware.graphics.forEach(gpu => {
            const vendor = gpu.vendor.toLowerCase();
            if (vendor.includes('nvidia')) {
                config.driver_recommendations.push({
                    component_type: "GPU",
                    component_name: gpu.model,
                    recommended_driver: performanceProfile === 'performance' ? "nvidia" : "nouveau",
                    driver_type: performanceProfile === 'performance' ? "proprietary" : "open-source",
                    priority: "high",
                    compatibility_notes: performanceProfile === 'performance' ? 
                        "Proprietary driver for best performance" : 
                        "Open-source driver for better compatibility"
                });
            } else if (vendor.includes('amd')) {
                config.driver_recommendations.push({
                    component_type: "GPU",
                    component_name: gpu.model,
                    recommended_driver: "amdgpu",
                    driver_type: "open-source",
                    priority: "high",
                    compatibility_notes: "Open-source AMD graphics driver"
                });
            }
        });

        // Generate kernel parameters
        if (performanceProfile === 'performance') {
            config.kernel_parameters = [
                {
                    parameter: "mitigations",
                    value: "off",
                    description: "Disable CPU security mitigations for performance"
                },
                {
                    parameter: "preempt",
                    value: "none",
                    description: "Use non-preemptive kernel for better performance"
                }
            ];
        } else if (performanceProfile === 'power-save') {
            config.kernel_parameters = [
                {
                    parameter: "intel_pstate",
                    value: "powersave",
                    description: "Use power saving CPU scaling"
                }
            ];
        }

        // Generate package installations
        const packageManager = this.getPackageManager(distribution);
        config.package_installations = [
            {
                package_name: "linux-firmware",
                package_manager: packageManager,
                install_command: `${packageManager} install linux-firmware`,
                reason: "Hardware firmware support"
            }
        ];

        if (this.currentHardware.graphics.some(gpu => gpu.vendor.toLowerCase().includes('nvidia'))) {
            if (performanceProfile === 'performance') {
                config.package_installations.push({
                    package_name: "nvidia-driver",
                    package_manager: packageManager,
                    install_command: `${packageManager} install nvidia-driver`,
                    reason: "NVIDIA proprietary graphics driver"
                });
            }
        }

        return config;
    }

    getPackageManager(distribution) {
        const managers = {
            'debian': 'apt',
            'ubuntu': 'apt', 
            'fedora': 'dnf',
            'arch': 'pacman',
            'nixos': 'nix',
            'opensuse': 'zypper',
            'gentoo': 'emerge'
        };
        return managers[distribution] || 'apt';
    }

    showConfigurationResults() {
        document.getElementById('configuration-results').style.display = 'block';
        this.populateConfigurationResults();
        
        // Scroll to results
        document.getElementById('configuration-results').scrollIntoView({ behavior: 'smooth' });
    }

    populateConfigurationResults() {
        this.populateDriverRecommendations();
        this.populateKernelParameters(); 
        this.populatePackageCommands();
        this.populateConfigurationFiles();
        this.populateConfigurationSummary();
    }

    populateDriverRecommendations() {
        const container = document.getElementById('driver-recommendations');
        
        container.innerHTML = this.configuration.driver_recommendations.map(driver => `
            <div class="driver-recommendation">
                <div class="driver-header">
                    <div class="driver-component">${driver.component_type}: ${driver.component_name}</div>
                    <div class="driver-priority priority-${driver.priority}">${driver.priority.toUpperCase()}</div>
                </div>
                <div class="driver-details">
                    <div class="driver-name">
                        <strong>Recommended Driver:</strong> ${driver.recommended_driver}
                        <span class="driver-type ${driver.driver_type}">${driver.driver_type}</span>
                    </div>
                    <div class="driver-notes">${driver.compatibility_notes}</div>
                </div>
            </div>
        `).join('');
    }

    populateKernelParameters() {
        const container = document.getElementById('kernel-parameters');
        
        if (this.configuration.kernel_parameters.length === 0) {
            container.innerHTML = `<code># No additional kernel parameters recommended for this configuration</code>`;
            return;
        }

        const params = this.configuration.kernel_parameters.map(param => 
            param.value ? `${param.parameter}=${param.value}` : param.parameter
        ).join(' ');

        container.innerHTML = `<code># Add to GRUB_CMDLINE_LINUX in /etc/default/grub
# Then run: sudo update-grub (Debian/Ubuntu) or sudo grub-mkconfig -o /boot/grub/grub.cfg (Arch/Fedora)

${params}

# Parameter explanations:
${this.configuration.kernel_parameters.map(param => 
    `# ${param.parameter}: ${param.description}`
).join('\n')}</code>`;
    }

    populatePackageCommands() {
        const container = document.getElementById('package-commands');
        
        if (this.configuration.package_installations.length === 0) {
            container.innerHTML = '<div class="no-packages">No additional packages required</div>';
            return;
        }

        container.innerHTML = this.configuration.package_installations.map(pkg => `
            <div class="package-command">
                <div class="package-header">
                    <div class="package-name">${pkg.package_name}</div>
                    <div class="package-reason">${pkg.reason}</div>
                </div>
                <div class="command-block">
                    <code>${pkg.install_command}</code>
                    <button class="copy-command" onclick="navigator.clipboard.writeText('${pkg.install_command}')">Copy</button>
                </div>
            </div>
        `).join('');
    }

    populateConfigurationFiles() {
        const container = document.getElementById('config-files');
        
        if (Object.keys(this.configuration.configuration_files).length === 0) {
            container.innerHTML = '<div class="no-config-files">No additional configuration files required</div>';
            return;
        }

        container.innerHTML = Object.entries(this.configuration.configuration_files).map(([filename, content]) => `
            <div class="config-file">
                <div class="file-header">
                    <div class="filename">${filename}</div>
                    <button class="copy-file" onclick="navigator.clipboard.writeText(\`${content}\`)">Copy</button>
                </div>
                <div class="file-content">
                    <pre><code>${content}</code></pre>
                </div>
            </div>
        `).join('');
    }

    populateConfigurationSummary() {
        const container = document.getElementById('config-summary');
        
        container.innerHTML = `
            <div class="summary-overview">
                <h4>Configuration Overview</h4>
                <div class="overview-stats">
                    <div class="stat-item">
                        <div class="stat-value">${this.configuration.compatibility_score}%</div>
                        <div class="stat-label">Compatibility Score</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-value">${this.configuration.driver_recommendations.length}</div>
                        <div class="stat-label">Drivers</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-value">${this.configuration.kernel_parameters.length}</div>
                        <div class="stat-label">Kernel Parameters</div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-value">${this.configuration.package_installations.length}</div>
                        <div class="stat-label">Packages</div>
                    </div>
                </div>
            </div>
            
            <div class="summary-details">
                <h4>Configuration Details</h4>
                <div class="detail-grid">
                    <div class="detail-item">
                        <strong>Target Distribution:</strong> ${this.configuration.target_distribution}
                    </div>
                    <div class="detail-item">
                        <strong>Kernel Version:</strong> ${this.configuration.kernel_version}
                    </div>
                    <div class="detail-item">
                        <strong>System ID:</strong> ${this.configuration.system_id}
                    </div>
                    <div class="detail-item">
                        <strong>Generated:</strong> ${new Date().toLocaleString()}
                    </div>
                </div>
            </div>

            <div class="summary-actions">
                <h4>Next Steps</h4>
                <ol class="action-steps">
                    <li>Install recommended packages using the commands in the Packages tab</li>
                    <li>Add kernel parameters to your boot configuration if any are recommended</li>
                    <li>Apply any configuration files shown in the Configuration Files tab</li>
                    <li>Reboot your system to apply kernel parameter changes</li>
                    <li>Test your hardware functionality after applying the configuration</li>
                </ol>
            </div>
        `;
    }

    initResultsTabs() {
        const tabButtons = document.querySelectorAll('.config-tab');
        const tabContents = document.querySelectorAll('.config-tab-content');
        
        tabButtons.forEach(button => {
            button.addEventListener('click', () => {
                const targetTab = button.getAttribute('data-tab');
                
                // Update tab button states
                tabButtons.forEach(btn => btn.classList.remove('active'));
                button.classList.add('active');
                
                // Update tab content visibility
                tabContents.forEach(content => {
                    content.classList.remove('active');
                    if (content.id === `${targetTab}-tab`) {
                        content.classList.add('active');
                    }
                });
            });
        });

        // Initialize action buttons
        document.getElementById('download-config').addEventListener('click', () => {
            this.downloadConfiguration();
        });

        document.getElementById('copy-config').addEventListener('click', async () => {
            await this.copyConfigurationToClipboard();
        });

        document.getElementById('new-configuration').addEventListener('click', () => {
            this.resetConfiguration();
        });
    }

    downloadConfiguration() {
        if (!this.configuration) return;
        
        const configText = this.formatConfigurationForExport();
        const blob = new Blob([configText], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        
        const a = document.createElement('a');
        a.href = url;
        a.download = `linux-config-${this.configuration.target_distribution}-${Date.now()}.txt`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        
        URL.revokeObjectURL(url);
    }

    async copyConfigurationToClipboard() {
        if (!this.configuration) return;
        
        const configText = this.formatConfigurationForExport();
        
        try {
            await navigator.clipboard.writeText(configText);
            const btn = document.getElementById('copy-config');
            const originalText = btn.textContent;
            btn.textContent = 'Copied!';
            setTimeout(() => btn.textContent = originalText, 2000);
        } catch (error) {
            console.error('Failed to copy to clipboard:', error);
        }
    }

    formatConfigurationForExport() {
        let output = `# Linux Hardware Configuration
# Generated: ${new Date().toLocaleString()}
# Target Distribution: ${this.configuration.target_distribution}
# Compatibility Score: ${this.configuration.compatibility_score}%

`;

        // Driver recommendations
        if (this.configuration.driver_recommendations.length > 0) {
            output += `## Driver Recommendations\n\n`;
            this.configuration.driver_recommendations.forEach(driver => {
                output += `### ${driver.component_type}: ${driver.component_name}\n`;
                output += `- Driver: ${driver.recommended_driver} (${driver.driver_type})\n`;
                output += `- Priority: ${driver.priority}\n`;
                output += `- Notes: ${driver.compatibility_notes}\n\n`;
            });
        }

        // Kernel parameters
        if (this.configuration.kernel_parameters.length > 0) {
            output += `## Kernel Parameters\n\n`;
            output += `Add the following to GRUB_CMDLINE_LINUX in /etc/default/grub:\n\n`;
            const params = this.configuration.kernel_parameters.map(param => 
                param.value ? `${param.parameter}=${param.value}` : param.parameter
            ).join(' ');
            output += `${params}\n\n`;
            
            output += `Then run:\n`;
            output += `sudo update-grub  # Debian/Ubuntu\n`;
            output += `sudo grub-mkconfig -o /boot/grub/grub.cfg  # Arch/Fedora\n\n`;
        }

        // Package installations
        if (this.configuration.package_installations.length > 0) {
            output += `## Package Installation\n\n`;
            this.configuration.package_installations.forEach(pkg => {
                output += `# ${pkg.reason}\n`;
                output += `${pkg.install_command}\n\n`;
            });
        }

        return output;
    }

    resetConfiguration() {
        this.currentHardware = null;
        this.configuration = null;
        
        // Hide results and options
        document.getElementById('configuration-results').style.display = 'none';
        document.getElementById('configuration-options-panel').style.display = 'none';
        
        // Clear all status messages and displays
        document.getElementById('detection-status').innerHTML = '';
        document.getElementById('detected-hardware').innerHTML = '';
        document.getElementById('upload-status').innerHTML = '';
        document.getElementById('uploaded-hardware').innerHTML = '';
        document.getElementById('manual-hardware').innerHTML = '';
        
        // Reset form elements
        document.getElementById('hardware-file-input').value = '';
        document.querySelectorAll('.hardware-select').forEach(select => select.value = '');
        
        // Reset to auto-detect method
        document.querySelectorAll('.method-btn').forEach(btn => btn.classList.remove('active'));
        document.querySelectorAll('.config-panel').forEach(panel => panel.classList.remove('active'));
        
        document.getElementById('auto-detect-btn').classList.add('active');
        document.getElementById('auto-detect-panel').classList.add('active');
        
        this.currentMethod = 'auto-detect';
    }
}

// Initialize the Configuration Engine when the page loads
document.addEventListener('DOMContentLoaded', () => {
    window.configurationEngine = new ConfigurationEngine();
});