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
        
        // Wait for database indexer
        document.addEventListener('databaseIndexed', (event) => {
            this.databaseIndexer = event.detail.indexer;
            this.setupMatrixVisualization();
        });

        // Check if database indexer is already available
        if (window.hardwareDatabaseIndexer && window.hardwareDatabaseIndexer.isBuilt) {
            this.databaseIndexer = window.hardwareDatabaseIndexer;
            this.setupMatrixVisualization();
        }
    }

    /**
     * Set up the matrix visualization interface
     */
    setupMatrixVisualization() {
        if (this.isInitialized) return;
        
        this.createMatrixContainer();
        this.generateDefaultMatrix();
        this.setupEventListeners();
        this.isInitialized = true;
        
        console.log('Compatibility matrix initialized successfully');
    }

    /**
     * Create matrix container in stats section
     */
    createMatrixContainer() {
        const statsContainer = document.querySelector('#stats-container');
        if (!statsContainer) return;

        // Find existing compatibility overview or create new section
        let matrixSection = document.querySelector('.compatibility-matrix-section');
        if (!matrixSection) {
            matrixSection = document.createElement('div');
            matrixSection.className = 'compatibility-matrix-section';
            
            // Insert after the compatibility overview section for proper vertical stacking
            const compatibilityOverview = statsContainer.querySelector('.compatibility-overview');
            if (compatibilityOverview) {
                compatibilityOverview.insertAdjacentElement('afterend', matrixSection);
            } else {
                // Fallback: insert after stats grid if no compatibility overview found
                const statsGrid = statsContainer.querySelector('.stats-grid');
                if (statsGrid) {
                    statsGrid.insertAdjacentElement('afterend', matrixSection);
                } else {
                    statsContainer.appendChild(matrixSection);
                }
            }
        }

        matrixSection.innerHTML = `
            <div class="matrix-header">
                <h3>Compatibility Matrix</h3>
                <div class="matrix-controls">
                    <select class="matrix-type-select" id="matrix-type-select">
                        <option value="distribution">By Distribution</option>
                        <option value="kernel">By Kernel Version</option>
                        <option value="vendor">By Vendor</option>
                        <option value="category">By Hardware Category</option>
                    </select>
                    
                    <select class="matrix-category-filter" id="matrix-category-filter">
                        <option value="">All Hardware</option>
                        <option value="cpu">CPU</option>
                        <option value="gpu">GPU</option>
                        <option value="memory">Memory</option>
                        <option value="storage">Storage</option>
                        <option value="network">Network</option>
                        <option value="audio">Audio</option>
                    </select>
                    
                    <button class="matrix-fullscreen" id="matrix-fullscreen" aria-label="Toggle fullscreen">
                        🔍
                    </button>
                </div>
            </div>
            
            <div class="matrix-legend">
                <div class="legend-item">
                    <span class="legend-color" style="background: ${this.compatibilityColors.excellent}"></span>
                    <span class="legend-label">Excellent</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color" style="background: ${this.compatibilityColors.good}"></span>
                    <span class="legend-label">Good</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color" style="background: ${this.compatibilityColors.partial}"></span>
                    <span class="legend-label">Partial</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color" style="background: ${this.compatibilityColors.poor}"></span>
                    <span class="legend-label">Poor</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color" style="background: ${this.compatibilityColors.unknown}"></span>
                    <span class="legend-label">Unknown</span>
                </div>
            </div>
            
            <div class="matrix-container" id="matrix-container">
                <div class="matrix-loading">
                    <div class="loading-spinner"></div>
                    <p>Generating compatibility matrix...</p>
                </div>
            </div>
            
            <div class="matrix-stats" id="matrix-stats">
                <!-- Matrix statistics will appear here -->
            </div>
        `;

        // Add matrix styles
        this.addMatrixStyles();
    }

    /**
     * Generate the default compatibility matrix
     */
    generateDefaultMatrix() {
        this.generateMatrix('distribution');
    }

    /**
     * Generate compatibility matrix based on type
     */
    generateMatrix(type, categoryFilter = null) {
        if (!this.databaseIndexer) return;

        this.matrixType = type;
        const reports = this.getFilteredReports(categoryFilter);
        let matrixData;

        switch (type) {
            case 'distribution':
                matrixData = this.generateDistributionMatrix(reports);
                break;
            case 'kernel':
                matrixData = this.generateKernelMatrix(reports);
                break;
            case 'vendor':
                matrixData = this.generateVendorMatrix(reports);
                break;
            case 'category':
                matrixData = this.generateCategoryMatrix(reports);
                break;
            default:
                matrixData = this.generateDistributionMatrix(reports);
        }

        this.currentMatrix = matrixData;
        this.renderMatrix(matrixData);
        this.updateMatrixStats(matrixData);
    }

    /**
     * Get filtered reports based on category
     */
    getFilteredReports(categoryFilter) {
        if (!categoryFilter) {
            return this.databaseIndexer.rawData;
        }

        return this.databaseIndexer.query({ category: categoryFilter });
    }

    /**
     * Generate distribution vs hardware compatibility matrix
     */
    generateDistributionMatrix(reports) {
        const distributions = {};
        const hardwareTypes = new Set();

        // Collect data
        reports.forEach(report => {
            const distro = report.system.distribution;
            if (!distributions[distro]) {
                distributions[distro] = {};
            }

            // Categorize hardware
            const hardware = this.categorizeReportHardware(report);
            hardware.forEach(({ category, name, compatibility }) => {
                hardwareTypes.add(category);
                
                if (!distributions[distro][category]) {
                    distributions[distro][category] = { total: 0, statuses: {} };
                }
                
                distributions[distro][category].total++;
                const status = compatibility || 'unknown';
                distributions[distro][category].statuses[status] = 
                    (distributions[distro][category].statuses[status] || 0) + 1;
            });
        });

        return {
            type: 'distribution',
            rows: Object.keys(distributions).sort(),
            columns: Array.from(hardwareTypes).sort(),
            data: distributions,
            title: 'Distribution vs Hardware Category Compatibility'
        };
    }

    /**
     * Generate kernel version compatibility matrix
     */
    generateKernelMatrix(reports) {
        const kernels = {};
        const distributions = new Set();

        reports.forEach(report => {
            const kernel = report.system.kernel_version;
            const distro = report.system.distribution;
            const status = report.compatibility?.overall_status || 'unknown';
            
            distributions.add(distro);
            
            if (!kernels[kernel]) {
                kernels[kernel] = {};
            }
            
            if (!kernels[kernel][distro]) {
                kernels[kernel][distro] = { total: 0, statuses: {} };
            }
            
            kernels[kernel][distro].total++;
            kernels[kernel][distro].statuses[status] = 
                (kernels[kernel][distro].statuses[status] || 0) + 1;
        });

        return {
            type: 'kernel',
            rows: Object.keys(kernels).sort((a, b) => this.compareKernelVersions(a, b)),
            columns: Array.from(distributions).sort(),
            data: kernels,
            title: 'Kernel Version vs Distribution Compatibility'
        };
    }

    /**
     * Generate vendor compatibility matrix
     */
    generateVendorMatrix(reports) {
        const vendors = {};
        const categories = new Set();

        reports.forEach(report => {
            const hardware = this.categorizeReportHardware(report);
            
            hardware.forEach(({ category, name, compatibility, vendor }) => {
                if (!vendor) return;
                
                categories.add(category);
                
                if (!vendors[vendor]) {
                    vendors[vendor] = {};
                }
                
                if (!vendors[vendor][category]) {
                    vendors[vendor][category] = { total: 0, statuses: {} };
                }
                
                vendors[vendor][category].total++;
                const status = compatibility || 'unknown';
                vendors[vendor][category].statuses[status] = 
                    (vendors[vendor][category].statuses[status] || 0) + 1;
            });
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
    generateCategoryMatrix(reports) {
        const categories = {};
        const distributions = new Set();

        reports.forEach(report => {
            const distro = report.system.distribution;
            distributions.add(distro);
            
            const hardware = this.categorizeReportHardware(report);
            
            hardware.forEach(({ category, compatibility }) => {
                if (!categories[category]) {
                    categories[category] = {};
                }
                
                if (!categories[category][distro]) {
                    categories[category][distro] = { total: 0, statuses: {} };
                }
                
                categories[category][distro].total++;
                const status = compatibility || 'unknown';
                categories[category][distro].statuses[status] = 
                    (categories[category][distro].statuses[status] || 0) + 1;
            });
        });

        return {
            type: 'category',
            rows: Object.keys(categories).sort(),
            columns: Array.from(distributions).sort(),
            data: categories,
            title: 'Hardware Category vs Distribution Compatibility'
        };
    }

    /**
     * Categorize hardware from a report
     */
    categorizeReportHardware(report) {
        const hardware = [];

        // CPU
        if (report.cpu) {
            hardware.push({
                category: 'cpu',
                name: `${report.cpu.vendor} ${report.cpu.model}`,
                vendor: report.cpu.vendor,
                compatibility: report.compatibility?.overall_status
            });
        }

        // Graphics
        if (report.graphics) {
            report.graphics.forEach(gpu => {
                hardware.push({
                    category: 'gpu',
                    name: `${gpu.vendor} ${gpu.model}`,
                    vendor: gpu.vendor,
                    compatibility: gpu.compatibility || report.compatibility?.overall_status
                });
            });
        }

        // Memory
        if (report.memory) {
            hardware.push({
                category: 'memory',
                name: `${report.memory.total_gb}GB ${report.memory.type}`,
                vendor: null,
                compatibility: report.compatibility?.overall_status
            });
        }

        // Storage
        if (report.storage) {
            report.storage.forEach(storage => {
                hardware.push({
                    category: 'storage',
                    name: `${storage.vendor} ${storage.model}`,
                    vendor: storage.vendor,
                    compatibility: storage.compatibility || report.compatibility?.overall_status
                });
            });
        }

        // Network
        if (report.network) {
            report.network.forEach(net => {
                hardware.push({
                    category: 'network',
                    name: `${net.vendor} ${net.model}`,
                    vendor: net.vendor,
                    compatibility: net.compatibility || report.compatibility?.overall_status
                });
            });
        }

        // Audio
        if (report.audio) {
            report.audio.forEach(audio => {
                hardware.push({
                    category: 'audio',
                    name: `${audio.vendor} ${audio.model}`,
                    vendor: audio.vendor,
                    compatibility: audio.compatibility || report.compatibility?.overall_status
                });
            });
        }

        return hardware;
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
        // Matrix type selector
        const typeSelect = document.querySelector('#matrix-type-select');
        if (typeSelect) {
            typeSelect.addEventListener('change', (e) => {
                const categoryFilter = document.querySelector('#matrix-category-filter')?.value;
                this.generateMatrix(e.target.value, categoryFilter || null);
            });
        }

        // Category filter
        const categoryFilter = document.querySelector('#matrix-category-filter');
        if (categoryFilter) {
            categoryFilter.addEventListener('change', (e) => {
                const matrixType = document.querySelector('#matrix-type-select')?.value || 'distribution';
                this.generateMatrix(matrixType, e.target.value || null);
            });
        }

        // Fullscreen toggle
        const fullscreenBtn = document.querySelector('#matrix-fullscreen');
        if (fullscreenBtn) {
            fullscreenBtn.addEventListener('click', () => {
                this.toggleMatrixFullscreen();
            });
        }
    }

    /**
     * Toggle matrix fullscreen mode
     */
    toggleMatrixFullscreen() {
        const matrixSection = document.querySelector('.compatibility-matrix-section');
        if (matrixSection) {
            matrixSection.classList.toggle('matrix-fullscreen-mode');
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
            .compatibility-matrix-section {
                background: var(--bg1, #3c3836);
                border-radius: 12px;
                padding: 25px;
                border: 1px solid var(--bg2, #504945);
                margin-top: 30px;
                transition: all 0.3s ease;
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

// Auto-initialize when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
    await window.compatibilityMatrix.initialize();
});