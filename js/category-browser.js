/**
 * Category Browser and Filtering System
 * Provides browsing and filtering of hardware by categories
 */

class CategoryBrowser {
    constructor() {
        this.searchEngine = null;
        this.databaseIndexer = null;
        this.currentCategory = null;
        this.currentFilter = {};
        this.isInitialized = false;
        
        this.categories = {
            'cpu': {
                name: 'Processors',
                icon: '🖥️',
                description: 'CPU and processor hardware',
                searchFields: ['cpu.vendor', 'cpu.model']
            },
            'gpu': {
                name: 'Graphics Cards',
                icon: '🎮',
                description: 'Graphics cards and display adapters',
                searchFields: ['graphics:vendor', 'graphics:model']
            },
            'memory': {
                name: 'Memory',
                icon: '💾',
                description: 'RAM and memory modules',
                searchFields: ['memory.type']
            },
            'storage': {
                name: 'Storage',
                icon: '💿',
                description: 'Hard drives, SSDs, and storage devices',
                searchFields: ['storage:vendor', 'storage:model', 'storage:interface']
            },
            'network': {
                name: 'Network',
                icon: '🌐',
                description: 'Network cards and wireless adapters',
                searchFields: ['network:vendor', 'network:model']
            },
            'audio': {
                name: 'Audio',
                icon: '🔊',
                description: 'Sound cards and audio devices',
                searchFields: ['audio:vendor', 'audio:model']
            }
        };
    }

    /**
     * Initialize the category browser
     */
    async initialize() {
        console.log('Initializing category browser...');
        
        // Wait for dependencies
        document.addEventListener('searchEngineReady', (event) => {
            this.searchEngine = event.detail.engine;
            this.setupCategoryBrowser();
        });

        document.addEventListener('databaseIndexed', (event) => {
            this.databaseIndexer = event.detail.indexer;
            this.setupCategoryBrowser();
        });

        // Check if dependencies are already available
        if (window.hardwareSearch && window.hardwareSearch.isInitialized) {
            this.searchEngine = window.hardwareSearch;
        }

        if (window.hardwareDatabaseIndexer && window.hardwareDatabaseIndexer.isBuilt) {
            this.databaseIndexer = window.hardwareDatabaseIndexer;
        }

        if (this.searchEngine && this.databaseIndexer) {
            this.setupCategoryBrowser();
        }
    }

    /**
     * Set up the category browser interface
     */
    setupCategoryBrowser() {
        if (this.isInitialized) return;
        
        this.createCategoryNavigation();
        this.setupEventListeners();
        this.isInitialized = true;
        
        console.log('Category browser initialized successfully');
    }

    /**
     * Create category navigation interface
     */
    createCategoryNavigation() {
        const searchContainer = document.querySelector('#search-container');
        if (!searchContainer) return;

        // Find or create category browser container
        let categoryContainer = document.querySelector('.category-browser');
        if (!categoryContainer) {
            categoryContainer = document.createElement('div');
            categoryContainer.className = 'category-browser';
            
            // Insert after search filters
            const searchFilters = searchContainer.querySelector('.search-filters');
            if (searchFilters) {
                searchFilters.insertAdjacentElement('afterend', categoryContainer);
            } else {
                searchContainer.appendChild(categoryContainer);
            }
        }

        categoryContainer.innerHTML = `
            <div class="category-browser-header">
                <h3>Browse by Category</h3>
                <button class="category-view-toggle" data-view="grid" aria-label="Toggle category view">
                    <span class="grid-icon">⊞</span>
                    <span class="list-icon">☰</span>
                </button>
            </div>
            
            <div class="category-grid" id="category-grid">
                ${this.renderCategoryCards()}
            </div>
            
            <div class="category-results" id="category-results">
                <!-- Category-specific results will appear here -->
            </div>
        `;

        // Add category browser styles
        this.addCategoryStyles();
    }

    /**
     * Render category cards
     */
    renderCategoryCards() {
        const stats = this.databaseIndexer ? this.databaseIndexer.statistics : null;
        
        return Object.entries(this.categories).map(([key, category]) => {
            const count = stats?.categories?.[key] || 0;
            const isActive = this.currentCategory === key;
            
            return `
                <div class="category-card ${isActive ? 'active' : ''}" 
                     data-category="${key}" 
                     tabindex="0"
                     role="button"
                     aria-pressed="${isActive}">
                    <div class="category-icon">${category.icon}</div>
                    <div class="category-info">
                        <h4 class="category-name">${category.name}</h4>
                        <p class="category-description">${category.description}</p>
                        <span class="category-count">${count} reports</span>
                    </div>
                    <div class="category-arrow">→</div>
                </div>
            `;
        }).join('');
    }

    /**
     * Set up event listeners
     */
    setupEventListeners() {
        // Category card clicks
        document.addEventListener('click', (e) => {
            const categoryCard = e.target.closest('.category-card');
            if (categoryCard) {
                const category = categoryCard.dataset.category;
                this.selectCategory(category);
            }

            // View toggle
            const viewToggle = e.target.closest('.category-view-toggle');
            if (viewToggle) {
                this.toggleView();
            }

            // Clear category filter
            const clearFilter = e.target.closest('.clear-category-filter');
            if (clearFilter) {
                this.clearCategoryFilter();
            }
        });

        // Keyboard navigation
        document.addEventListener('keydown', (e) => {
            const categoryCard = e.target.closest('.category-card');
            if (categoryCard && (e.key === 'Enter' || e.key === ' ')) {
                e.preventDefault();
                const category = categoryCard.dataset.category;
                this.selectCategory(category);
            }
        });
    }

    /**
     * Select a category and show filtered results
     */
    async selectCategory(categoryKey) {
        if (!this.databaseIndexer) return;

        this.currentCategory = categoryKey;
        const category = this.categories[categoryKey];
        
        // Update active state
        document.querySelectorAll('.category-card').forEach(card => {
            card.classList.toggle('active', card.dataset.category === categoryKey);
            card.setAttribute('aria-pressed', card.dataset.category === categoryKey);
        });

        // Query reports for this category
        const results = this.databaseIndexer.query({ category: categoryKey });
        
        // Show category results
        this.displayCategoryResults(category, results);
        
        // Scroll to results
        const resultsContainer = document.querySelector('#category-results');
        if (resultsContainer) {
            resultsContainer.scrollIntoView({ behavior: 'smooth' });
        }
    }

    /**
     * Display category-specific results
     */
    displayCategoryResults(category, results) {
        const resultsContainer = document.querySelector('#category-results');
        if (!resultsContainer) return;

        if (results.length === 0) {
            resultsContainer.innerHTML = `
                <div class="no-category-results">
                    <h4>No ${category.name} Found</h4>
                    <p>No hardware reports found for ${category.name.toLowerCase()}.</p>
                </div>
            `;
            return;
        }

        // Group results by vendor for this category
        const vendorGroups = this.groupResultsByVendor(results, category);
        
        resultsContainer.innerHTML = `
            <div class="category-results-header">
                <div class="category-results-title">
                    <span class="category-icon">${category.icon}</span>
                    <h3>${category.name}</h3>
                    <span class="results-count">${results.length} reports</span>
                </div>
                <div class="category-results-actions">
                    <button class="clear-category-filter">Clear Filter</button>
                    <select class="category-sort-select">
                        <option value="vendor">Sort by Vendor</option>
                        <option value="compatibility">Sort by Compatibility</option>
                        <option value="recent">Sort by Date</option>
                    </select>
                </div>
            </div>
            
            <div class="category-results-content">
                ${this.renderVendorGroups(vendorGroups)}
            </div>
        `;

        // Set up sort handler
        const sortSelect = resultsContainer.querySelector('.category-sort-select');
        sortSelect?.addEventListener('change', (e) => {
            this.sortCategoryResults(e.target.value);
        });
    }

    /**
     * Group results by vendor for the selected category
     */
    groupResultsByVendor(results, category) {
        const groups = {};
        
        results.forEach(report => {
            let vendors = [];
            
            // Extract vendors based on category
            switch (this.currentCategory) {
                case 'cpu':
                    if (report.cpu?.vendor) vendors.push(report.cpu.vendor);
                    break;
                case 'gpu':
                    if (report.graphics) {
                        vendors = report.graphics.map(g => g.vendor).filter(Boolean);
                    }
                    break;
                case 'network':
                    if (report.network) {
                        vendors = report.network.map(n => n.vendor).filter(Boolean);
                    }
                    break;
                case 'storage':
                    if (report.storage) {
                        vendors = report.storage.map(s => s.vendor).filter(Boolean);
                    }
                    break;
                case 'audio':
                    if (report.audio) {
                        vendors = report.audio.map(a => a.vendor).filter(Boolean);
                    }
                    break;
                case 'memory':
                    vendors = ['Memory']; // Memory doesn't always have vendor info
                    break;
            }

            vendors.forEach(vendor => {
                if (!groups[vendor]) {
                    groups[vendor] = [];
                }
                groups[vendor].push(report);
            });
        });

        return groups;
    }

    /**
     * Render vendor groups
     */
    renderVendorGroups(vendorGroups) {
        const sortedVendors = Object.keys(vendorGroups).sort();
        
        return sortedVendors.map(vendor => {
            const reports = vendorGroups[vendor];
            const compatibilityStats = this.getCompatibilityStats(reports);
            
            return `
                <div class="vendor-group">
                    <div class="vendor-group-header">
                        <h4 class="vendor-name">${vendor}</h4>
                        <div class="vendor-stats">
                            <span class="vendor-count">${reports.length} reports</span>
                            <div class="compatibility-badges">
                                ${Object.entries(compatibilityStats)
                                    .filter(([, count]) => count > 0)
                                    .map(([status, count]) => 
                                        `<span class="compat-badge compat-${status}">${this.getStatusIcon(status)} ${count}</span>`
                                    ).join('')}
                            </div>
                        </div>
                    </div>
                    
                    <div class="vendor-reports">
                        ${reports.slice(0, 5).map(report => this.renderCategoryReport(report)).join('')}
                        ${reports.length > 5 ? `<div class="show-more-reports" data-vendor="${vendor}">Show ${reports.length - 5} more reports...</div>` : ''}
                    </div>
                </div>
            `;
        }).join('');
    }

    /**
     * Render a single report for category view
     */
    renderCategoryReport(report) {
        const compatibility = report.compatibility?.overall_status || 'unknown';
        const statusIcon = this.getStatusIcon(compatibility);
        
        return `
            <div class="category-report-card" data-report-id="${report.id}">
                <div class="report-header">
                    <div class="system-info">
                        <strong>${report.system.distribution}</strong>
                        <span class="kernel-version">${report.system.kernel_version}</span>
                        <span class="architecture">${report.system.architecture}</span>
                    </div>
                    <div class="compatibility-status status-${compatibility}">
                        ${statusIcon} ${compatibility}
                    </div>
                </div>
                
                <div class="hardware-summary">
                    ${this.renderCategoryHardware(report)}
                </div>
                
                <div class="report-meta">
                    <span class="report-date">${new Date(report.metadata.generated_at).toLocaleDateString()}</span>
                    <button class="view-report-details" data-report-id="${report.id}">View Details</button>
                </div>
            </div>
        `;
    }

    /**
     * Render category-specific hardware information
     */
    renderCategoryHardware(report) {
        switch (this.currentCategory) {
            case 'cpu':
                return report.cpu ? `
                    <div class="hardware-item">
                        <strong>${report.cpu.vendor} ${report.cpu.model}</strong>
                        <small>${report.cpu.cores} cores, ${report.cpu.threads} threads</small>
                    </div>
                ` : '<div class="hardware-item">No CPU information</div>';
                
            case 'gpu':
                return report.graphics && report.graphics.length > 0 ? 
                    report.graphics.map(gpu => `
                        <div class="hardware-item">
                            <strong>${gpu.vendor} ${gpu.model}</strong>
                            <small>Driver: ${gpu.driver}</small>
                        </div>
                    `).join('') : '<div class="hardware-item">No GPU information</div>';
                    
            case 'memory':
                return report.memory ? `
                    <div class="hardware-item">
                        <strong>${report.memory.total_gb}GB ${report.memory.type}</strong>
                    </div>
                ` : '<div class="hardware-item">No memory information</div>';
                
            case 'storage':
                return report.storage && report.storage.length > 0 ?
                    report.storage.map(storage => `
                        <div class="hardware-item">
                            <strong>${storage.vendor} ${storage.model}</strong>
                            <small>${storage.size_gb}GB ${storage.interface}</small>
                        </div>
                    `).join('') : '<div class="hardware-item">No storage information</div>';
                    
            case 'network':
                return report.network && report.network.length > 0 ?
                    report.network.map(net => `
                        <div class="hardware-item">
                            <strong>${net.vendor} ${net.model}</strong>
                            <small>Driver: ${net.driver}</small>
                        </div>
                    `).join('') : '<div class="hardware-item">No network information</div>';
                    
            case 'audio':
                return report.audio && report.audio.length > 0 ?
                    report.audio.map(audio => `
                        <div class="hardware-item">
                            <strong>${audio.vendor} ${audio.model}</strong>
                            <small>Driver: ${audio.driver}</small>
                        </div>
                    `).join('') : '<div class="hardware-item">No audio information</div>';
                    
            default:
                return '<div class="hardware-item">Hardware information not available</div>';
        }
    }

    /**
     * Get compatibility statistics for reports
     */
    getCompatibilityStats(reports) {
        const stats = { excellent: 0, good: 0, partial: 0, poor: 0, unknown: 0 };
        
        reports.forEach(report => {
            const status = report.compatibility?.overall_status || 'unknown';
            stats[status] = (stats[status] || 0) + 1;
        });
        
        return stats;
    }

    /**
     * Get status icon for compatibility
     */
    getStatusIcon(status) {
        const icons = {
            excellent: '✅',
            good: '👍',
            partial: '⚠️',
            poor: '❌',
            unknown: '❓'
        };
        return icons[status] || icons.unknown;
    }

    /**
     * Sort category results
     */
    sortCategoryResults(sortBy) {
        // Implementation for sorting would go here
        console.log('Sorting by:', sortBy);
    }

    /**
     * Clear category filter
     */
    clearCategoryFilter() {
        this.currentCategory = null;
        
        // Clear active states
        document.querySelectorAll('.category-card').forEach(card => {
            card.classList.remove('active');
            card.setAttribute('aria-pressed', 'false');
        });
        
        // Clear results
        const resultsContainer = document.querySelector('#category-results');
        if (resultsContainer) {
            resultsContainer.innerHTML = '';
        }
    }

    /**
     * Toggle between grid and list view
     */
    toggleView() {
        const categoryGrid = document.querySelector('#category-grid');
        const toggleBtn = document.querySelector('.category-view-toggle');
        
        if (categoryGrid && toggleBtn) {
            const currentView = toggleBtn.dataset.view || 'grid';
            const newView = currentView === 'grid' ? 'list' : 'grid';
            
            categoryGrid.className = `category-${newView}`;
            toggleBtn.dataset.view = newView;
        }
    }

    /**
     * Add category browser styles
     */
    addCategoryStyles() {
        // Check if styles already exist
        if (document.querySelector('#category-browser-styles')) return;

        const styles = document.createElement('style');
        styles.id = 'category-browser-styles';
        styles.textContent = `
            .category-browser {
                margin: 30px 0;
                background: var(--bg1, #3c3836);
                border-radius: 12px;
                padding: 25px;
                border: 1px solid var(--bg2, #504945);
            }

            .category-browser-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 20px;
            }

            .category-browser-header h3 {
                margin: 0;
                color: var(--fg0, #fbf1c7);
                font-size: 1.3rem;
            }

            .category-view-toggle {
                background: var(--bg0, #282828);
                border: 1px solid var(--bg2, #504945);
                border-radius: 6px;
                padding: 8px 12px;
                color: var(--fg2, #d5c4a1);
                cursor: pointer;
                font-size: 1.2rem;
            }

            .category-view-toggle:hover {
                background: var(--bg2, #504945);
            }

            .category-grid {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
                gap: 20px;
                margin-bottom: 30px;
            }

            .category-list {
                display: flex;
                flex-direction: column;
                gap: 15px;
                margin-bottom: 30px;
            }

            .category-card {
                background: var(--bg0, #282828);
                border: 1px solid var(--bg2, #504945);
                border-radius: 8px;
                padding: 20px;
                cursor: pointer;
                transition: all 0.3s ease;
                display: flex;
                align-items: center;
                gap: 15px;
                position: relative;
                overflow: hidden;
            }

            .category-card:hover,
            .category-card.active {
                border-color: var(--primary, #fabd2f);
                transform: translateY(-2px);
                box-shadow: 0 5px 15px rgba(0,0,0,0.3);
            }

            .category-card.active::before {
                content: '';
                position: absolute;
                top: 0;
                left: 0;
                right: 0;
                height: 3px;
                background: var(--primary, #fabd2f);
            }

            .category-icon {
                font-size: 2rem;
                flex-shrink: 0;
            }

            .category-info {
                flex: 1;
            }

            .category-name {
                margin: 0 0 5px 0;
                color: var(--fg0, #fbf1c7);
                font-size: 1.1rem;
            }

            .category-description {
                margin: 0 0 8px 0;
                color: var(--fg3, #bdae93);
                font-size: 0.9rem;
                line-height: 1.4;
            }

            .category-count {
                color: var(--fg4, #a89984);
                font-size: 0.8rem;
                font-family: monospace;
            }

            .category-arrow {
                font-size: 1.2rem;
                color: var(--fg4, #a89984);
                transition: transform 0.3s ease;
            }

            .category-card:hover .category-arrow {
                transform: translateX(3px);
                color: var(--primary, #fabd2f);
            }

            .category-results {
                margin-top: 30px;
            }

            .category-results-header {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-bottom: 25px;
                flex-wrap: wrap;
                gap: 15px;
            }

            .category-results-title {
                display: flex;
                align-items: center;
                gap: 12px;
            }

            .category-results-title h3 {
                margin: 0;
                color: var(--fg0, #fbf1c7);
            }

            .results-count {
                background: var(--bg2, #504945);
                color: var(--fg2, #d5c4a1);
                padding: 4px 8px;
                border-radius: 4px;
                font-size: 0.8rem;
                font-family: monospace;
            }

            .category-results-actions {
                display: flex;
                align-items: center;
                gap: 12px;
            }

            .clear-category-filter {
                background: var(--bg2, #504945);
                color: var(--fg2, #d5c4a1);
                border: none;
                padding: 8px 16px;
                border-radius: 6px;
                cursor: pointer;
                font-size: 0.9rem;
                transition: background-color 0.3s;
            }

            .clear-category-filter:hover {
                background: var(--bg3, #665c54);
            }

            .category-sort-select {
                background: var(--bg0, #282828);
                color: var(--fg2, #d5c4a1);
                border: 1px solid var(--bg2, #504945);
                padding: 8px 12px;
                border-radius: 6px;
                font-size: 0.9rem;
            }

            .vendor-group {
                margin-bottom: 30px;
                background: var(--bg0, #282828);
                border-radius: 8px;
                border: 1px solid var(--bg2, #504945);
                overflow: hidden;
            }

            .vendor-group-header {
                background: var(--bg1, #3c3836);
                padding: 15px 20px;
                display: flex;
                justify-content: space-between;
                align-items: center;
            }

            .vendor-name {
                margin: 0;
                color: var(--fg0, #fbf1c7);
                font-size: 1.1rem;
            }

            .vendor-stats {
                display: flex;
                align-items: center;
                gap: 15px;
            }

            .vendor-count {
                color: var(--fg4, #a89984);
                font-size: 0.9rem;
                font-family: monospace;
            }

            .compatibility-badges {
                display: flex;
                gap: 6px;
            }

            .compat-badge {
                padding: 2px 6px;
                border-radius: 3px;
                font-size: 0.7rem;
                font-family: monospace;
            }

            .compat-badge.compat-excellent {
                background: var(--green-bright, #b8bb26);
                color: var(--bg0, #282828);
            }

            .compat-badge.compat-good {
                background: var(--blue-bright, #83a598);
                color: var(--bg0, #282828);
            }

            .compat-badge.compat-partial {
                background: var(--yellow-bright, #fabd2f);
                color: var(--bg0, #282828);
            }

            .compat-badge.compat-poor {
                background: var(--red-bright, #fb4934);
                color: var(--bg0, #282828);
            }

            .vendor-reports {
                padding: 20px;
                display: grid;
                gap: 15px;
            }

            .category-report-card {
                background: var(--bg1, #3c3836);
                border: 1px solid var(--bg2, #504945);
                border-radius: 6px;
                padding: 15px;
                transition: all 0.3s ease;
            }

            .category-report-card:hover {
                border-color: var(--primary, #fabd2f);
                transform: translateY(-1px);
            }

            .report-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-start;
                margin-bottom: 12px;
            }

            .system-info {
                display: flex;
                flex-direction: column;
                gap: 4px;
            }

            .system-info strong {
                color: var(--fg0, #fbf1c7);
            }

            .kernel-version,
            .architecture {
                color: var(--fg4, #a89984);
                font-size: 0.8rem;
                font-family: monospace;
            }

            .compatibility-status {
                padding: 4px 8px;
                border-radius: 4px;
                font-size: 0.8rem;
                font-weight: 600;
            }

            .status-excellent {
                background: var(--green-bright, #b8bb26);
                color: var(--bg0, #282828);
            }

            .status-good {
                background: var(--blue-bright, #83a598);
                color: var(--bg0, #282828);
            }

            .status-partial {
                background: var(--yellow-bright, #fabd2f);
                color: var(--bg0, #282828);
            }

            .status-poor {
                background: var(--red-bright, #fb4934);
                color: var(--bg0, #282828);
            }

            .hardware-summary {
                margin-bottom: 12px;
            }

            .hardware-item {
                padding: 8px 0;
                border-bottom: 1px solid var(--bg2, #504945);
            }

            .hardware-item:last-child {
                border-bottom: none;
            }

            .hardware-item strong {
                color: var(--fg1, #ebdbb2);
                display: block;
            }

            .hardware-item small {
                color: var(--fg4, #a89984);
                font-size: 0.8rem;
            }

            .report-meta {
                display: flex;
                justify-content: space-between;
                align-items: center;
                font-size: 0.8rem;
            }

            .report-date {
                color: var(--fg4, #a89984);
                font-family: monospace;
            }

            .view-report-details {
                background: none;
                border: 1px solid var(--primary, #fabd2f);
                color: var(--primary, #fabd2f);
                padding: 4px 8px;
                border-radius: 4px;
                cursor: pointer;
                font-size: 0.8rem;
                transition: all 0.3s;
            }

            .view-report-details:hover {
                background: var(--primary, #fabd2f);
                color: var(--bg0, #282828);
            }

            .show-more-reports {
                text-align: center;
                padding: 15px;
                color: var(--primary, #fabd2f);
                cursor: pointer;
                border-top: 1px solid var(--bg2, #504945);
                font-size: 0.9rem;
            }

            .show-more-reports:hover {
                background: var(--bg2, #504945);
            }

            .no-category-results {
                text-align: center;
                padding: 40px;
                color: var(--fg3, #bdae93);
            }

            .no-category-results h4 {
                margin: 0 0 10px 0;
                color: var(--fg2, #d5c4a1);
            }

            /* Responsive design */
            @media (max-width: 768px) {
                .category-grid {
                    grid-template-columns: 1fr;
                }

                .category-results-header {
                    flex-direction: column;
                    align-items: stretch;
                }

                .category-results-actions {
                    justify-content: space-between;
                }

                .report-header {
                    flex-direction: column;
                    gap: 8px;
                }

                .report-meta {
                    flex-direction: column;
                    gap: 8px;
                    align-items: stretch;
                }
            }
        `;
        
        document.head.appendChild(styles);
    }
}

// Global instance
window.categoryBrowser = new CategoryBrowser();

// Auto-initialize when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
    await window.categoryBrowser.initialize();
});