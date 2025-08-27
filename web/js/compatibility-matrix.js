/**
 * Compatibility Matrix Visualization
 * Creates visual matrices showing hardware compatibility across distributions and kernel versions
 */

class CompatibilityMatrix {
    constructor() {
        this.databaseIndexer = null;
        this.isInitialized = false;
        this.currentMatrix = null;
        this.matrixType = 'distribution'; // 'distribution', 'kernel', 'vendor'
        
        this.compatibilityColors = {
            excellent: '#b8bb26', // Green
            good: '#83a598',      // Blue  
            partial: '#fabd2f',   // Yellow
            poor: '#fb4934',      // Red
            unknown: '#665c54'    // Gray
        };
    }

    /**
     * Initialize the compatibility matrix
     */
    async initialize() {
        console.log('Initializing compatibility matrix...');
        
        // Check if already initialized
        if (this.isInitialized) {
            console.log('Compatibility matrix already initialized');
            return;
        }
        
        // Always create the UI container first, regardless of search engine status
        this.setupMatrixVisualization();
        
        // Then check if we can populate it with data
        this.populateMatrixWhenReady();
        
        // Mark as initialized
        this.isInitialized = true;
    }
    
    /**
     * Populate matrix when search engine becomes available
     */
    populateMatrixWhenReady() {
        // Check if search engine exists and is ready
        console.log('SearchEngine available:', !!window.searchEngine);
        if (window.searchEngine) {
            console.log('SearchEngine ready:', window.searchEngine.isReady());
        }
        
        if (window.searchEngine && window.searchEngine.isReady()) {
            console.log('SearchEngine ready, generating matrix with real data...');
            this.generateDefaultMatrix();
        } else {
            console.log('SearchEngine not ready, will generate sample matrix and retry...');
            // Generate a sample matrix to show the interface works
            this.generateSampleMatrix();
            
            // Wait for search engine to load real data
            const checkSearchEngine = () => {
                console.log('Checking search engine... Available:', !!window.searchEngine, 'Ready:', window.searchEngine?.isReady());
                if (window.searchEngine && window.searchEngine.isReady()) {
                    console.log('SearchEngine now ready, updating matrix with real data...');
                    this.generateDefaultMatrix();
                } else {
                    setTimeout(checkSearchEngine, 1000);
                }
            };
            setTimeout(checkSearchEngine, 1000);
        }
    }

    /**
     * Set up the matrix visualization interface
     */
    setupMatrixVisualization() {
        if (this.isInitialized) return;
        
        this.createMatrixContainer();
        this.setupEventListeners();
        this.isInitialized = true;
        
        console.log('Compatibility matrix initialized successfully');
    }

    /**
     * Initialize existing matrix HTML structure with JavaScript functionality
     */
    createMatrixContainer() {
        console.log('Initializing existing matrix container...');
        
        // The matrix HTML structure is already in place, just add interactivity
        const matrixContainer = document.querySelector('#matrix-container');
        if (!matrixContainer) {
            console.log('Matrix container not found, creating JavaScript-based version...');
            this.createFallbackMatrix();
            return;
        }

        console.log('Matrix container found in HTML, adding interactivity...');
        
        // Add matrix styles
        this.addMatrixStyles();
        
        // Add interactivity to existing matrix cells
        this.setupMatrixInteraction();
        
        console.log('Matrix container initialized with existing HTML structure');
    }
    
    /**
     * Create fallback matrix if HTML structure is not found
     */
    createFallbackMatrix() {
        console.log('Creating fallback JavaScript-generated matrix...');
        // Fallback logic could go here if needed
        // For now, we rely on the HTML structure being present
    }

    /**
     * Generate the default compatibility matrix
     */
    generateDefaultMatrix() {
        this.generateMatrix('distribution');
    }
    
    /**
     * Generate sample matrix for a specific type
     */
    generateSampleMatrixForType(type, categoryFilter = null) {
        switch (type) {
            case 'kernel':
                this.generateSampleKernelMatrix(categoryFilter);
                break;
            case 'vendor':
                this.generateSampleVendorMatrix(categoryFilter);
                break;
            case 'category':
                this.generateSampleCategoryMatrix(categoryFilter);
                break;
            case 'distribution':
            default:
                this.generateSampleMatrix(categoryFilter);
                break;
        }
    }

    /**
     * Generate sample kernel version matrix
     */
    generateSampleKernelMatrix(categoryFilter = null) {
        const sampleData = {
            type: 'kernel',
            rows: ['5.15', '6.1', '6.4', '6.6', '6.8'],
            columns: categoryFilter ? [categoryFilter] : ['GPU', 'CPU', 'Network', 'Audio', 'Storage'],
            data: {
                '5.15': {
                    'GPU': { total: 2, statuses: { good: 1, partial: 1 } },
                    'CPU': { total: 4, statuses: { excellent: 3, good: 1 } },
                    'Network': { total: 2, statuses: { good: 2 } },
                    'Audio': { total: 1, statuses: { partial: 1 } },
                    'Storage': { total: 2, statuses: { excellent: 2 } }
                },
                '6.1': {
                    'GPU': { total: 3, statuses: { excellent: 1, good: 2 } },
                    'CPU': { total: 4, statuses: { excellent: 4 } },
                    'Network': { total: 3, statuses: { excellent: 2, good: 1 } },
                    'Audio': { total: 1, statuses: { good: 1 } },
                    'Storage': { total: 2, statuses: { excellent: 2 } }
                },
                '6.4': {
                    'GPU': { total: 3, statuses: { excellent: 2, good: 1 } },
                    'CPU': { total: 4, statuses: { excellent: 4 } },
                    'Network': { total: 3, statuses: { excellent: 3 } },
                    'Audio': { total: 2, statuses: { excellent: 1, good: 1 } },
                    'Storage': { total: 2, statuses: { excellent: 2 } }
                },
                '6.6': {
                    'GPU': { total: 3, statuses: { excellent: 3 } },
                    'CPU': { total: 4, statuses: { excellent: 4 } },
                    'Network': { total: 3, statuses: { excellent: 3 } },
                    'Audio': { total: 2, statuses: { excellent: 2 } },
                    'Storage': { total: 2, statuses: { excellent: 2 } }
                },
                '6.8': {
                    'GPU': { total: 3, statuses: { excellent: 3 } },
                    'CPU': { total: 4, statuses: { excellent: 4 } },
                    'Network': { total: 3, statuses: { excellent: 3 } },
                    'Audio': { total: 2, statuses: { excellent: 2 } },
                    'Storage': { total: 2, statuses: { excellent: 2 } }
                }
            },
            title: 'Kernel Version vs Hardware Category Compatibility (Sample Data)'
        };
        
        // Filter data if category filter is applied
        if (categoryFilter) {
            const filteredData = {};
            Object.keys(sampleData.data).forEach(kernel => {
                filteredData[kernel] = {};
                if (sampleData.data[kernel][categoryFilter]) {
                    filteredData[kernel][categoryFilter] = sampleData.data[kernel][categoryFilter];
                }
            });
            sampleData.data = filteredData;
            sampleData.title = `Kernel Version vs ${categoryFilter} Compatibility (Sample Data)`;
        }
        
        this.currentMatrix = sampleData;
        this.renderMatrix(sampleData);
        this.updateMatrixStats(sampleData);
        console.log('Sample kernel matrix generated successfully');
    }

    /**
     * Generate sample vendor matrix
     */
    generateSampleVendorMatrix(categoryFilter = null) {
        const sampleData = {
            type: 'vendor',
            rows: ['Intel', 'AMD', 'NVIDIA', 'Realtek', 'Broadcom'],
            columns: categoryFilter ? [categoryFilter] : ['GPU', 'CPU', 'Network', 'Audio'],
            data: {
                'Intel': {
                    'GPU': { total: 2, statuses: { excellent: 1, good: 1 } },
                    'CPU': { total: 4, statuses: { excellent: 4 } },
                    'Network': { total: 2, statuses: { excellent: 2 } },
                    'Audio': { total: 1, statuses: { good: 1 } }
                },
                'AMD': {
                    'GPU': { total: 2, statuses: { excellent: 1, good: 1 } },
                    'CPU': { total: 3, statuses: { excellent: 3 } },
                    'Network': { total: 0, statuses: {} },
                    'Audio': { total: 1, statuses: { good: 1 } }
                },
                'NVIDIA': {
                    'GPU': { total: 3, statuses: { good: 2, partial: 1 } },
                    'CPU': { total: 0, statuses: {} },
                    'Network': { total: 0, statuses: {} },
                    'Audio': { total: 0, statuses: {} }
                },
                'Realtek': {
                    'GPU': { total: 0, statuses: {} },
                    'CPU': { total: 0, statuses: {} },
                    'Network': { total: 3, statuses: { excellent: 2, good: 1 } },
                    'Audio': { total: 2, statuses: { excellent: 1, good: 1 } }
                },
                'Broadcom': {
                    'GPU': { total: 0, statuses: {} },
                    'CPU': { total: 0, statuses: {} },
                    'Network': { total: 2, statuses: { good: 1, partial: 1 } },
                    'Audio': { total: 0, statuses: {} }
                }
            },
            title: 'Vendor vs Hardware Category Compatibility (Sample Data)'
        };
        
        // Filter data if category filter is applied
        if (categoryFilter) {
            const filteredData = {};
            Object.keys(sampleData.data).forEach(vendor => {
                filteredData[vendor] = {};
                if (sampleData.data[vendor][categoryFilter]) {
                    filteredData[vendor][categoryFilter] = sampleData.data[vendor][categoryFilter];
                }
            });
            sampleData.data = filteredData;
            sampleData.title = `Vendor vs ${categoryFilter} Compatibility (Sample Data)`;
        }
        
        this.currentMatrix = sampleData;
        this.renderMatrix(sampleData);
        this.updateMatrixStats(sampleData);
        console.log('Sample vendor matrix generated successfully');
    }

    /**
     * Generate sample category matrix
     */
    generateSampleCategoryMatrix(categoryFilter = null) {
        const sampleData = {
            type: 'category',
            rows: categoryFilter ? [categoryFilter] : ['GPU', 'CPU', 'Network', 'Audio', 'Storage'],
            columns: ['NixOS', 'Debian', 'Arch', 'Fedora', 'Ubuntu'],
            data: {
                'GPU': {
                    'NixOS': { total: 3, statuses: { excellent: 2, good: 1 } },
                    'Debian': { total: 2, statuses: { good: 1, partial: 1 } },
                    'Arch': { total: 3, statuses: { excellent: 1, good: 2 } },
                    'Fedora': { total: 2, statuses: { good: 2 } },
                    'Ubuntu': { total: 2, statuses: { good: 1, partial: 1 } }
                },
                'CPU': {
                    'NixOS': { total: 4, statuses: { excellent: 4 } },
                    'Debian': { total: 4, statuses: { excellent: 3, good: 1 } },
                    'Arch': { total: 4, statuses: { excellent: 4 } },
                    'Fedora': { total: 4, statuses: { excellent: 3, good: 1 } },
                    'Ubuntu': { total: 4, statuses: { excellent: 2, good: 2 } }
                },
                'Network': {
                    'NixOS': { total: 2, statuses: { good: 2 } },
                    'Debian': { total: 3, statuses: { excellent: 2, good: 1 } },
                    'Arch': { total: 2, statuses: { excellent: 2 } },
                    'Fedora': { total: 3, statuses: { good: 3 } },
                    'Ubuntu': { total: 3, statuses: { good: 3 } }
                },
                'Audio': {
                    'NixOS': { total: 1, statuses: { partial: 1 } },
                    'Debian': { total: 2, statuses: { good: 2 } },
                    'Arch': { total: 1, statuses: { good: 1 } },
                    'Fedora': { total: 2, statuses: { excellent: 1, good: 1 } },
                    'Ubuntu': { total: 2, statuses: { good: 2 } }
                },
                'Storage': {
                    'NixOS': { total: 2, statuses: { excellent: 2 } },
                    'Debian': { total: 2, statuses: { excellent: 2 } },
                    'Arch': { total: 2, statuses: { excellent: 2 } },
                    'Fedora': { total: 2, statuses: { excellent: 2 } },
                    'Ubuntu': { total: 2, statuses: { excellent: 1, good: 1 } }
                }
            },
            title: 'Hardware Category vs Distribution Compatibility (Sample Data)'
        };
        
        // Filter data if category filter is applied
        if (categoryFilter && sampleData.data[categoryFilter]) {
            sampleData.data = { [categoryFilter]: sampleData.data[categoryFilter] };
            sampleData.title = `${categoryFilter} vs Distribution Compatibility (Sample Data)`;
        }
        
        this.currentMatrix = sampleData;
        this.renderMatrix(sampleData);
        this.updateMatrixStats(sampleData);
        console.log('Sample category matrix generated successfully');
    }

    /**
     * Generate a sample matrix to show the interface works
     */
    generateSampleMatrix(categoryFilter = null) {
        const sampleData = {
            type: 'distribution',
            rows: ['NixOS', 'Debian', 'Arch', 'Fedora', 'Ubuntu'],
            columns: categoryFilter ? [categoryFilter] : ['GPU', 'CPU', 'Network', 'Audio', 'Storage'],
            data: {
                'NixOS': {
                    'GPU': { total: 3, statuses: { excellent: 2, good: 1 } },
                    'CPU': { total: 4, statuses: { excellent: 4 } },
                    'Network': { total: 2, statuses: { good: 2 } },
                    'Audio': { total: 1, statuses: { partial: 1 } },
                    'Storage': { total: 2, statuses: { excellent: 2 } }
                },
                'Debian': {
                    'GPU': { total: 2, statuses: { good: 1, partial: 1 } },
                    'CPU': { total: 4, statuses: { excellent: 3, good: 1 } },
                    'Network': { total: 3, statuses: { excellent: 2, good: 1 } },
                    'Audio': { total: 2, statuses: { good: 2 } },
                    'Storage': { total: 2, statuses: { excellent: 2 } }
                },
                'Arch': {
                    'GPU': { total: 3, statuses: { excellent: 1, good: 2 } },
                    'CPU': { total: 4, statuses: { excellent: 4 } },
                    'Network': { total: 2, statuses: { excellent: 2 } },
                    'Audio': { total: 1, statuses: { good: 1 } },
                    'Storage': { total: 2, statuses: { excellent: 2 } }
                },
                'Fedora': {
                    'GPU': { total: 2, statuses: { good: 2 } },
                    'CPU': { total: 4, statuses: { excellent: 3, good: 1 } },
                    'Network': { total: 3, statuses: { good: 3 } },
                    'Audio': { total: 2, statuses: { excellent: 1, good: 1 } },
                    'Storage': { total: 2, statuses: { excellent: 2 } }
                },
                'Ubuntu': {
                    'GPU': { total: 2, statuses: { good: 1, partial: 1 } },
                    'CPU': { total: 4, statuses: { excellent: 2, good: 2 } },
                    'Network': { total: 3, statuses: { good: 3 } },
                    'Audio': { total: 2, statuses: { good: 2 } },
                    'Storage': { total: 2, statuses: { excellent: 1, good: 1 } }
                }
            },
            title: 'Distribution vs Hardware Category Compatibility (Sample Data)'
        };
        
        // Filter data if category filter is applied
        if (categoryFilter) {
            const filteredData = {};
            Object.keys(sampleData.data).forEach(distro => {
                filteredData[distro] = {};
                if (sampleData.data[distro][categoryFilter]) {
                    filteredData[distro][categoryFilter] = sampleData.data[distro][categoryFilter];
                }
            });
            sampleData.data = filteredData;
            sampleData.title = `Distribution vs ${categoryFilter} Compatibility (Sample Data)`;
        }
        
        this.currentMatrix = sampleData;
        this.renderMatrix(sampleData);
        this.updateMatrixStats(sampleData);
        console.log('Sample matrix generated successfully');
    }

    /**
     * Generate compatibility matrix based on type
     */
    generateMatrix(type, categoryFilter = null) {
        if (!window.searchEngine) return;

        this.matrixType = type;
        const hardware = this.getFilteredHardware(categoryFilter);
        let matrixData;

        switch (type) {
            case 'distribution':
                matrixData = this.generateDistributionMatrix(hardware);
                break;
            case 'kernel':
                matrixData = this.generateKernelMatrix(hardware);
                break;
            case 'vendor':
                matrixData = this.generateVendorMatrix(hardware);
                break;
            case 'category':
                matrixData = this.generateCategoryMatrix(hardware);
                break;
            default:
                matrixData = this.generateDistributionMatrix(hardware);
        }

        this.currentMatrix = matrixData;
        this.renderMatrix(matrixData);
        this.updateMatrixStats(matrixData);
    }

    /**
     * Get filtered hardware based on category
     */
    getFilteredHardware(categoryFilter) {
        if (!window.searchEngine) return [];
        
        const allHardware = window.searchEngine.hardwareData;
        
        if (!categoryFilter) {
            return allHardware;
        }

        return allHardware.filter(hardware => hardware.category === categoryFilter);
    }

    /**
     * Generate distribution vs hardware compatibility matrix
     */
    generateDistributionMatrix(hardwareList) {
        const distributions = {};
        const categories = new Set();

        // Get unique distributions from tested_distributions
        const allDistributions = new Set(['nixos', 'debian', 'arch', 'fedora', 'ubuntu']);

        hardwareList.forEach(hardware => {
            const category = hardware.category;
            categories.add(category);

            // Check each distribution's compatibility
            allDistributions.forEach(distro => {
                if (!distributions[distro]) {
                    distributions[distro] = {};
                }
                
                if (!distributions[distro][category]) {
                    distributions[distro][category] = { total: 0, statuses: {} };
                }

                distributions[distro][category].total++;
                
                // Get compatibility status for this distribution
                let status = 'unknown';
                if (hardware.tested_distributions && 
                    hardware.tested_distributions[distro] && 
                    hardware.tested_distributions[distro].status) {
                    status = hardware.tested_distributions[distro].status;
                } else if (hardware.compatibility_status) {
                    // Fallback to general compatibility status
                    status = hardware.compatibility_status;
                }

                // Map our status values to matrix status values
                const mappedStatus = this.mapCompatibilityStatus(status);
                distributions[distro][category].statuses[mappedStatus] = 
                    (distributions[distro][category].statuses[mappedStatus] || 0) + 1;
            });
        });

        return {
            type: 'distribution',
            rows: Array.from(allDistributions).sort(),
            columns: Array.from(categories).sort(),
            data: distributions,
            title: 'Distribution vs Hardware Category Compatibility'
        };
    }

    /**
     * Generate kernel version compatibility matrix
     */
    generateKernelMatrix(hardwareList) {
        const kernels = {};
        const categories = new Set();
        
        // Sample kernel versions for demonstration
        const sampleKernels = ['5.15', '6.1', '6.4', '6.6', '6.8'];

        hardwareList.forEach(hardware => {
            const category = hardware.category;
            categories.add(category);
            
            // For each sample kernel version
            sampleKernels.forEach(kernel => {
                if (!kernels[kernel]) {
                    kernels[kernel] = {};
                }
                
                if (!kernels[kernel][category]) {
                    kernels[kernel][category] = { total: 0, statuses: {} };
                }
                
                kernels[kernel][category].total++;
                
                // Simulate kernel compatibility based on hardware compatibility
                let status = this.mapCompatibilityStatus(hardware.compatibility_status || 'unknown');
                
                // Older kernels might have slightly lower compatibility
                if (kernel === '5.15' && status === 'excellent') {
                    status = Math.random() > 0.7 ? 'excellent' : 'good';
                }
                
                kernels[kernel][category].statuses[status] = 
                    (kernels[kernel][category].statuses[status] || 0) + 1;
            });
        });

        return {
            type: 'kernel',
            rows: sampleKernels.sort((a, b) => this.compareKernelVersions(a, b)),
            columns: Array.from(categories).sort(),
            data: kernels,
            title: 'Kernel Version vs Hardware Category Compatibility'
        };
    }

    /**
     * Map compatibility status from database to matrix values
     */
    mapCompatibilityStatus(status) {
        const statusMapping = {
            'full': 'excellent',
            'partial': 'partial', 
            'limited': 'poor',
            'none': 'poor',
            'working': 'good',
            'excellent': 'excellent',
            'good': 'good',
            'poor': 'poor',
            'unknown': 'unknown'
        };
        return statusMapping[status] || 'unknown';
    }

    /**
     * Generate vendor compatibility matrix
     */
    generateVendorMatrix(hardwareList) {
        const vendors = {};
        const categories = new Set();

        hardwareList.forEach(hardware => {
            const vendor = hardware.manufacturer;
            const category = hardware.category;
            
            if (!vendor) return;
            
            categories.add(category);
            
            if (!vendors[vendor]) {
                vendors[vendor] = {};
            }
            
            if (!vendors[vendor][category]) {
                vendors[vendor][category] = { total: 0, statuses: {} };
            }
            
            vendors[vendor][category].total++;
            const status = this.mapCompatibilityStatus(hardware.compatibility_status || 'unknown');
            vendors[vendor][category].statuses[status] = 
                (vendors[vendor][category].statuses[status] || 0) + 1;
        });

        return {
            type: 'vendor',
            rows: Object.keys(vendors).sort(),
            columns: Array.from(categories).sort(),
            data: vendors,
            title: 'Vendor vs Hardware Category Compatibility'
        };
    }

    /**
     * Generate category compatibility matrix
     */
    generateCategoryMatrix(hardwareList) {
        const categories = {};
        const distributions = ['nixos', 'debian', 'arch', 'fedora', 'ubuntu'];

        hardwareList.forEach(hardware => {
            const category = hardware.category;
            
            if (!categories[category]) {
                categories[category] = {};
            }
            
            distributions.forEach(distro => {
                if (!categories[category][distro]) {
                    categories[category][distro] = { total: 0, statuses: {} };
                }
                
                categories[category][distro].total++;
                
                // Get compatibility status for this distribution
                let status = 'unknown';
                if (hardware.tested_distributions && 
                    hardware.tested_distributions[distro] && 
                    hardware.tested_distributions[distro].status) {
                    status = hardware.tested_distributions[distro].status;
                } else if (hardware.compatibility_status) {
                    status = hardware.compatibility_status;
                }

                const mappedStatus = this.mapCompatibilityStatus(status);
                categories[category][distro].statuses[mappedStatus] = 
                    (categories[category][distro].statuses[mappedStatus] || 0) + 1;
            });
        });

        return {
            type: 'category',
            rows: Object.keys(categories).sort(),
            columns: distributions.sort(),
            data: categories,
            title: 'Hardware Category vs Distribution Compatibility'
        };
    }


    /**
     * Render the compatibility matrix
     */
    renderMatrix(matrixData) {
        const container = document.querySelector('#matrix-container');
        if (!container) return;

        const { rows, columns, data, title } = matrixData;
        
        if (rows.length === 0 || columns.length === 0) {
            container.innerHTML = `
                <div class="matrix-empty">
                    <p>No data available for the selected matrix type.</p>
                </div>
            `;
            return;
        }

        container.innerHTML = `
            <div class="matrix-title">${title}</div>
            <div class="matrix-table-container">
                <table class="matrix-table">
                    <thead>
                        <tr>
                            <th class="matrix-corner"></th>
                            ${columns.map(col => `<th class="matrix-column-header" title="${col}">${this.truncateText(col, 15)}</th>`).join('')}
                        </tr>
                    </thead>
                    <tbody>
                        ${rows.map(row => this.renderMatrixRow(row, columns, data)).join('')}
                    </tbody>
                </table>
            </div>
        `;

        // Add cell hover handlers
        this.setupMatrixInteraction();
    }

    /**
     * Render a matrix row
     */
    renderMatrixRow(row, columns, data) {
        const rowData = data[row] || {};
        
        return `
            <tr>
                <th class="matrix-row-header" title="${row}">${this.truncateText(row, 20)}</th>
                ${columns.map(col => {
                    const cellData = rowData[col];
                    const { color, intensity, tooltip } = this.calculateCellAppearance(cellData);
                    
                    return `
                        <td class="matrix-cell" 
                            style="background-color: ${color}; opacity: ${intensity}"
                            data-row="${row}" 
                            data-column="${col}"
                            data-tooltip="${tooltip}"
                            title="${tooltip}">
                            <span class="matrix-cell-value">${cellData ? cellData.total : 0}</span>
                        </td>
                    `;
                }).join('')}
            </tr>
        `;
    }

    /**
     * Calculate cell appearance based on compatibility data
     */
    calculateCellAppearance(cellData) {
        if (!cellData || cellData.total === 0) {
            return {
                color: this.compatibilityColors.unknown,
                intensity: 0.2,
                tooltip: 'No data available'
            };
        }

        const { total, statuses } = cellData;
        
        // Calculate weighted compatibility score
        const weights = { excellent: 4, good: 3, partial: 2, poor: 1, unknown: 0 };
        let weightedSum = 0;
        let totalWeight = 0;
        
        Object.entries(statuses).forEach(([status, count]) => {
            const weight = weights[status] || 0;
            weightedSum += weight * count;
            totalWeight += count;
        });
        
        const averageScore = totalWeight > 0 ? weightedSum / totalWeight : 0;
        
        // Determine dominant status
        let dominantStatus = 'unknown';
        let maxCount = 0;
        Object.entries(statuses).forEach(([status, count]) => {
            if (count > maxCount) {
                maxCount = count;
                dominantStatus = status;
            }
        });

        // Calculate intensity based on sample size
        const maxIntensity = 1.0;
        const minIntensity = 0.3;
        const intensity = Math.max(minIntensity, Math.min(maxIntensity, total / 10));

        // Create tooltip
        const statusBreakdown = Object.entries(statuses)
            .filter(([, count]) => count > 0)
            .map(([status, count]) => `${status}: ${count}`)
            .join(', ');
        
        const tooltip = `Total: ${total} reports\\n${statusBreakdown}\\nAverage score: ${averageScore.toFixed(1)}/4`;

        return {
            color: this.compatibilityColors[dominantStatus] || this.compatibilityColors.unknown,
            intensity: intensity,
            tooltip: tooltip
        };
    }

    /**
     * Set up matrix interaction handlers
     */
    setupMatrixInteraction() {
        const cells = document.querySelectorAll('.matrix-cell');
        
        cells.forEach(cell => {
            cell.addEventListener('mouseenter', (e) => {
                const row = e.target.dataset.row;
                const column = e.target.dataset.column;
                const tooltip = e.target.dataset.tooltip;
                
                // Highlight row and column
                this.highlightMatrixRowColumn(row, column);
                
                // Show tooltip
                this.showMatrixTooltip(e.target, tooltip);
            });
            
            cell.addEventListener('mouseleave', () => {
                this.clearMatrixHighlight();
                this.hideMatrixTooltip();
            });
            
            cell.addEventListener('click', (e) => {
                const row = e.target.dataset.row;
                const column = e.target.dataset.column;
                this.showMatrixCellDetails(row, column);
            });
        });
    }

    /**
     * Highlight matrix row and column
     */
    highlightMatrixRowColumn(row, column) {
        // Remove existing highlights
        this.clearMatrixHighlight();
        
        // Add highlights
        document.querySelectorAll(`[data-row="${row}"]`).forEach(el => {
            el.classList.add('matrix-row-highlight');
        });
        
        document.querySelectorAll(`[data-column="${column}"]`).forEach(el => {
            el.classList.add('matrix-column-highlight');
        });
        
        // Highlight headers
        const rowHeaders = document.querySelectorAll('.matrix-row-header');
        const columnHeaders = document.querySelectorAll('.matrix-column-header');
        
        rowHeaders.forEach((header, index) => {
            if (header.textContent.includes(this.truncateText(row, 20))) {
                header.classList.add('matrix-header-highlight');
            }
        });
        
        columnHeaders.forEach((header, index) => {
            if (header.textContent.includes(this.truncateText(column, 15))) {
                header.classList.add('matrix-header-highlight');
            }
        });
    }

    /**
     * Clear matrix highlights
     */
    clearMatrixHighlight() {
        document.querySelectorAll('.matrix-row-highlight').forEach(el => {
            el.classList.remove('matrix-row-highlight');
        });
        
        document.querySelectorAll('.matrix-column-highlight').forEach(el => {
            el.classList.remove('matrix-column-highlight');
        });
        
        document.querySelectorAll('.matrix-header-highlight').forEach(el => {
            el.classList.remove('matrix-header-highlight');
        });
    }

    /**
     * Show matrix tooltip
     */
    showMatrixTooltip(target, tooltip) {
        let tooltipEl = document.querySelector('.matrix-tooltip');
        
        if (!tooltipEl) {
            tooltipEl = document.createElement('div');
            tooltipEl.className = 'matrix-tooltip';
            document.body.appendChild(tooltipEl);
        }
        
        tooltipEl.textContent = tooltip.replace(/\\n/g, '\n');
        tooltipEl.style.display = 'block';
        
        const rect = target.getBoundingClientRect();
        tooltipEl.style.left = (rect.left + rect.width / 2) + 'px';
        tooltipEl.style.top = (rect.top - tooltipEl.offsetHeight - 5) + 'px';
    }

    /**
     * Hide matrix tooltip
     */
    hideMatrixTooltip() {
        const tooltipEl = document.querySelector('.matrix-tooltip');
        if (tooltipEl) {
            tooltipEl.style.display = 'none';
        }
    }

    /**
     * Show detailed information for a matrix cell
     */
    showMatrixCellDetails(row, column) {
        if (!this.currentMatrix) return;
        
        const cellData = this.currentMatrix.data[row]?.[column];
        if (!cellData) return;
        
        console.log(`Matrix cell details: ${row} x ${column}`, cellData);
        // This could open a modal with detailed information
    }

    /**
     * Update matrix statistics
     */
    updateMatrixStats(matrixData) {
        const statsContainer = document.querySelector('#matrix-stats');
        if (!statsContainer) return;

        const { rows, columns, data } = matrixData;
        
        // Calculate statistics
        let totalCells = 0;
        let filledCells = 0;
        let totalReports = 0;
        const statusCounts = {};

        rows.forEach(row => {
            columns.forEach(column => {
                totalCells++;
                const cellData = data[row]?.[column];
                
                if (cellData && cellData.total > 0) {
                    filledCells++;
                    totalReports += cellData.total;
                    
                    Object.entries(cellData.statuses).forEach(([status, count]) => {
                        statusCounts[status] = (statusCounts[status] || 0) + count;
                    });
                }
            });
        });

        const coverage = totalCells > 0 ? (filledCells / totalCells * 100).toFixed(1) : 0;

        statsContainer.innerHTML = `
            <div class="matrix-stats-grid">
                <div class="matrix-stat">
                    <span class="stat-value">${rows.length}</span>
                    <span class="stat-label">Rows</span>
                </div>
                <div class="matrix-stat">
                    <span class="stat-value">${columns.length}</span>
                    <span class="stat-label">Columns</span>
                </div>
                <div class="matrix-stat">
                    <span class="stat-value">${coverage}%</span>
                    <span class="stat-label">Coverage</span>
                </div>
                <div class="matrix-stat">
                    <span class="stat-value">${totalReports}</span>
                    <span class="stat-label">Total Reports</span>
                </div>
            </div>
            
            <div class="matrix-status-breakdown">
                <h4>Compatibility Breakdown</h4>
                <div class="status-breakdown-grid">
                    ${Object.entries(statusCounts)
                        .sort(([,a], [,b]) => b - a)
                        .map(([status, count]) => `
                            <div class="status-breakdown-item">
                                <span class="status-color" style="background: ${this.compatibilityColors[status]}"></span>
                                <span class="status-name">${status}</span>
                                <span class="status-count">${count}</span>
                                <span class="status-percent">${totalReports > 0 ? ((count / totalReports) * 100).toFixed(1) : 0}%</span>
                            </div>
                        `).join('')}
                </div>
            </div>
        `;
    }

    /**
     * Set up event listeners
     */
    setupEventListeners() {
        console.log('Setting up matrix event listeners...');
        
        // Matrix type selector
        const typeSelect = document.querySelector('#matrix-type-select');
        if (typeSelect) {
            console.log('Found matrix type selector, adding change listener');
            typeSelect.addEventListener('change', (e) => {
                console.log('Matrix type changed to:', e.target.value);
                const categoryFilter = document.querySelector('#matrix-category-filter')?.value;
                
                // For HTML-based matrix, we need to update the content dynamically
                if (window.searchEngine && window.searchEngine.isReady()) {
                    this.generateMatrix(e.target.value, categoryFilter || null);
                } else {
                    // Generate appropriate sample matrix for the selected type
                    this.generateSampleMatrixForType(e.target.value);
                }
            });
        } else {
            console.log('Matrix type selector not found');
        }

        // Category filter
        const categoryFilter = document.querySelector('#matrix-category-filter');
        if (categoryFilter) {
            console.log('Found matrix category filter, adding change listener');
            categoryFilter.addEventListener('change', (e) => {
                console.log('Matrix category filter changed to:', e.target.value);
                const matrixType = document.querySelector('#matrix-type-select')?.value || 'distribution';
                
                if (window.searchEngine && window.searchEngine.isReady()) {
                    this.generateMatrix(matrixType, e.target.value || null);
                } else {
                    this.generateSampleMatrixForType(matrixType, e.target.value);
                }
            });
        } else {
            console.log('Matrix category filter not found');
        }

        // Fullscreen toggle
        const fullscreenBtn = document.querySelector('#matrix-fullscreen');
        if (fullscreenBtn) {
            console.log('Found fullscreen button, adding click listener');
            fullscreenBtn.addEventListener('click', () => {
                console.log('Fullscreen button clicked');
                this.toggleMatrixFullscreen();
            });
        } else {
            console.log('Fullscreen button not found');
        }
        
        console.log('Matrix event listeners setup complete');
    }

    /**
     * Toggle matrix fullscreen mode
     */
    toggleMatrixFullscreen() {
        // Try different selectors for the matrix container
        const matrixSection = document.querySelector('.compatibility-matrix-section') || 
                             document.querySelector('#compatibility-matrix') || 
                             document.querySelector('.matrix-content') || 
                             document.querySelector('#matrix-container')?.parentElement;
        
        if (matrixSection) {
            console.log('Toggling fullscreen mode for matrix section');
            matrixSection.classList.toggle('matrix-fullscreen-mode');
            
            // Update fullscreen button text/icon
            const fullscreenBtn = document.querySelector('#matrix-fullscreen');
            if (fullscreenBtn) {
                const isFullscreen = matrixSection.classList.contains('matrix-fullscreen-mode');
                fullscreenBtn.innerHTML = isFullscreen ? '⤴️' : '⤢';
                fullscreenBtn.title = isFullscreen ? 'Exit Fullscreen' : 'Enter Fullscreen';
            }
        } else {
            console.log('Could not find matrix section for fullscreen toggle');
        }
    }

    /**
     * Compare kernel versions for sorting
     */
    compareKernelVersions(a, b) {
        const parseVersion = (version) => {
            return version.split('.').map(num => parseInt(num) || 0);
        };
        
        const versionA = parseVersion(a);
        const versionB = parseVersion(b);
        
        for (let i = 0; i < Math.max(versionA.length, versionB.length); i++) {
            const numA = versionA[i] || 0;
            const numB = versionB[i] || 0;
            
            if (numA !== numB) {
                return numA - numB;
            }
        }
        
        return 0;
    }

    /**
     * Truncate text to specified length
     */
    truncateText(text, maxLength) {
        if (text.length <= maxLength) return text;
        return text.substring(0, maxLength - 3) + '...';
    }

    /**
     * Add matrix visualization styles
     */
    addMatrixStyles() {
        // Check if styles already exist
        if (document.querySelector('#compatibility-matrix-styles')) return;

        const styles = document.createElement('style');
        styles.id = 'compatibility-matrix-styles';
        styles.textContent = `
            /* Unified styling within the same container */
            .matrix-content {
                background: transparent;
                border: none;
                padding: 0;
                margin-top: var(--space-8);
                width: 100%;
            }
            
            .matrix-header {
                border-top: 1px solid var(--bg3);
                padding-top: var(--space-6);
                padding-bottom: var(--space-4);
                margin-bottom: var(--space-6);
            }
            
            .matrix-header h3 {
                margin: 0;
                font-size: 1.5rem;
                font-weight: 600;
                color: var(--fg0);
            }

            .compatibility-matrix-section.matrix-fullscreen-mode {
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                z-index: 1000;
                margin: 0;
                border-radius: 0;
                padding: 40px;
                background: var(--bg0, #282828);
                overflow: auto;
            }

            .matrix-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 20px;
                flex-wrap: wrap;
                gap: 15px;
            }

            .matrix-header h3 {
                margin: 0;
                color: var(--fg0, #fbf1c7);
                font-size: 1.5rem;
            }

            .matrix-controls {
                display: flex;
                align-items: center;
                gap: 12px;
            }

            .matrix-type-select,
            .matrix-category-filter {
                background: var(--bg0, #282828);
                color: var(--fg2, #d5c4a1);
                border: 1px solid var(--bg2, #504945);
                padding: 8px 12px;
                border-radius: 6px;
                font-size: 0.9rem;
            }

            .matrix-fullscreen {
                background: var(--bg2, #504945);
                border: none;
                color: var(--fg2, #d5c4a1);
                padding: 8px 12px;
                border-radius: 6px;
                cursor: pointer;
                font-size: 1.1rem;
                transition: background-color 0.3s;
            }

            .matrix-fullscreen:hover {
                background: var(--bg3, #665c54);
            }

            .matrix-legend {
                display: flex;
                justify-content: center;
                gap: 20px;
                margin-bottom: 20px;
                flex-wrap: wrap;
            }

            .legend-item {
                display: flex;
                align-items: center;
                gap: 6px;
                font-size: 0.85rem;
                color: var(--fg2, #d5c4a1);
            }

            .legend-color {
                width: 12px;
                height: 12px;
                border-radius: 2px;
                display: block;
            }

            .matrix-container {
                background: var(--bg0, #282828);
                border-radius: 8px;
                border: 1px solid var(--bg2, #504945);
                overflow: hidden;
            }

            .matrix-title {
                text-align: center;
                padding: 15px;
                background: var(--bg2, #504945);
                color: var(--fg0, #fbf1c7);
                font-weight: 600;
                border-bottom: 1px solid var(--bg3, #665c54);
            }

            .matrix-table-container {
                overflow: auto;
                max-height: 600px;
            }

            .matrix-fullscreen-mode .matrix-table-container {
                max-height: calc(100vh - 300px);
            }

            .matrix-table {
                width: 100%;
                border-collapse: separate;
                border-spacing: 0;
                font-size: 0.8rem;
            }

            .matrix-corner {
                background: var(--bg3, #665c54);
                width: 120px;
                min-width: 120px;
                position: sticky;
                left: 0;
                z-index: 3;
            }

            .matrix-column-header {
                background: var(--bg2, #504945);
                color: var(--fg1, #ebdbb2);
                padding: 10px 8px;
                text-align: center;
                font-size: 0.75rem;
                font-weight: 600;
                border-bottom: 1px solid var(--bg3, #665c54);
                min-width: 50px;
                position: sticky;
                top: 0;
                z-index: 2;
            }

            .matrix-row-header {
                background: var(--bg2, #504945);
                color: var(--fg1, #ebdbb2);
                padding: 8px 12px;
                text-align: left;
                font-size: 0.8rem;
                font-weight: 500;
                border-right: 1px solid var(--bg3, #665c54);
                width: 120px;
                min-width: 120px;
                position: sticky;
                left: 0;
                z-index: 1;
            }

            .matrix-cell {
                background: var(--bg0, #282828);
                padding: 8px;
                text-align: center;
                cursor: pointer;
                position: relative;
                min-width: 50px;
                transition: all 0.2s ease;
                border-right: 1px solid var(--bg2, #504945);
                border-bottom: 1px solid var(--bg2, #504945);
            }

            .matrix-cell:hover {
                transform: scale(1.1);
                z-index: 5;
                border: 2px solid var(--primary, #fabd2f);
            }

            .matrix-cell-value {
                font-size: 0.7rem;
                font-weight: 600;
                color: var(--bg0, #282828);
                text-shadow: 0 0 2px rgba(0,0,0,0.5);
            }

            .matrix-row-highlight {
                box-shadow: inset -3px 0 0 var(--primary, #fabd2f);
            }

            .matrix-column-highlight {
                box-shadow: inset 0 -3px 0 var(--primary, #fabd2f);
            }

            .matrix-header-highlight {
                background: var(--primary, #fabd2f) !important;
                color: var(--bg0, #282828) !important;
            }

            .matrix-tooltip {
                position: absolute;
                background: var(--bg0-hard, #1d2021);
                color: var(--fg0, #fbf1c7);
                padding: 8px 12px;
                border-radius: 6px;
                font-size: 0.8rem;
                border: 1px solid var(--bg3, #665c54);
                pointer-events: none;
                z-index: 1000;
                white-space: pre-line;
                max-width: 200px;
                box-shadow: 0 4px 12px rgba(0,0,0,0.3);
            }

            .matrix-loading {
                text-align: center;
                padding: 60px;
                color: var(--fg3, #bdae93);
            }

            .matrix-empty {
                text-align: center;
                padding: 40px;
                color: var(--fg3, #bdae93);
            }

            .matrix-stats {
                margin-top: 25px;
                background: var(--bg0, #282828);
                border-radius: 8px;
                padding: 20px;
                border: 1px solid var(--bg2, #504945);
            }

            .matrix-stats-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
                gap: 20px;
                margin-bottom: 20px;
            }

            .matrix-stat {
                text-align: center;
            }

            .matrix-stat .stat-value {
                display: block;
                font-size: 1.8rem;
                font-weight: 700;
                color: var(--primary, #fabd2f);
                font-family: monospace;
            }

            .matrix-stat .stat-label {
                font-size: 0.8rem;
                color: var(--fg3, #bdae93);
                margin-top: 4px;
            }

            .matrix-status-breakdown h4 {
                margin: 0 0 15px 0;
                color: var(--fg1, #ebdbb2);
                font-size: 1rem;
            }

            .status-breakdown-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
                gap: 12px;
            }

            .status-breakdown-item {
                display: flex;
                align-items: center;
                gap: 8px;
                padding: 8px 12px;
                background: var(--bg1, #3c3836);
                border-radius: 6px;
                font-size: 0.85rem;
            }

            .status-color {
                width: 10px;
                height: 10px;
                border-radius: 2px;
                flex-shrink: 0;
            }

            .status-name {
                flex: 1;
                color: var(--fg2, #d5c4a1);
                text-transform: capitalize;
            }

            .status-count {
                color: var(--fg1, #ebdbb2);
                font-weight: 600;
                font-family: monospace;
            }

            .status-percent {
                color: var(--fg3, #bdae93);
                font-size: 0.75rem;
                font-family: monospace;
            }

            /* Responsive design */
            @media (max-width: 768px) {
                .compatibility-matrix-section {
                    padding: 15px;
                }

                .matrix-header {
                    flex-direction: column;
                    align-items: stretch;
                }

                .matrix-controls {
                    justify-content: space-between;
                }

                .matrix-legend {
                    gap: 10px;
                }

                .legend-item {
                    font-size: 0.75rem;
                }

                .matrix-table-container {
                    max-height: 400px;
                }

                .matrix-stats-grid {
                    grid-template-columns: repeat(2, 1fr);
                }

                .status-breakdown-grid {
                    grid-template-columns: 1fr;
                }
            }
        `;
        
        document.head.appendChild(styles);
    }
}

// Global instance
window.compatibilityMatrix = new CompatibilityMatrix();

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    // Simple initialization - the matrix HTML is already in place
    setTimeout(() => {
        window.compatibilityMatrix.initialize();
    }, 1000); // 1 second delay to ensure other components are ready
});