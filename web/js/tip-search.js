/**
 * Advanced Tip Search and Filtering System
 * Provides powerful search capabilities for configuration tips
 */

class TipSearchSystem {
    constructor() {
        this.isInitialized = false;
        this.searchIndex = null;
        this.allTips = [];
        this.filteredTips = [];
        this.searchFilters = {
            query: '',
            distributions: [],
            categories: [],
            components: [],
            contributors: [],
            minRating: 0,
            sortBy: 'relevance'
        };
        
        this.categories = {
            'driver_installation': 'Driver Installation',
            'performance_tuning': 'Performance Tuning',
            'troubleshooting': 'Troubleshooting',
            'configuration': 'Configuration',
            'power_management': 'Power Management',
            'security': 'Security'
        };

        this.components = {
            'cpu': 'CPU/Processor',
            'gpu': 'Graphics Card',
            'memory': 'Memory/RAM',
            'storage': 'Storage Device',
            'network': 'Network Adapter',
            'audio': 'Audio Device',
            'input': 'Input Device',
            'display': 'Display/Monitor',
            'system': 'System/BIOS'
        };

        this.distributions = {
            'debian': 'Debian/Ubuntu',
            'arch': 'Arch Linux',
            'fedora': 'Fedora/RHEL',
            'nixos': 'NixOS',
            'opensuse': 'openSUSE',
            'gentoo': 'Gentoo',
            'alpine': 'Alpine Linux'
        };
    }

    /**
     * Initialize the tip search system
     */
    async initialize() {
        console.log('Initializing tip search system...');
        
        // Initialize FlexSearch index
        this.initializeSearchIndex();
        
        // Load all tips from configuration tips system
        await this.loadAllTips();
        
        // Build search index
        this.buildSearchIndex();
        
        // Set up search interface
        this.setupSearchInterface();
        
        this.isInitialized = true;
        console.log('Tip search system initialized successfully');
    }

    /**
     * Initialize FlexSearch index for tips
     */
    initializeSearchIndex() {
        if (typeof FlexSearch === 'undefined') {
            console.warn('FlexSearch not loaded yet, retrying...');
            setTimeout(() => this.initializeSearchIndex(), 100);
            return;
        }
        
        this.searchIndex = new FlexSearch.Document({
            document: {
                id: 'id',
                index: [
                    'title',
                    'description',
                    'category',
                    'component',
                    'vendor',
                    'model',
                    'author',
                    'commands',
                    'configuration',
                    'warnings'
                ]
            },
            tokenize: 'forward',
            resolution: 9
        });
    }

    /**
     * Load all tips from the configuration tips system
     */
    async loadAllTips() {
        this.allTips = [];
        
        // Wait for configuration tips to be loaded
        if (window.configurationTips && window.configurationTips.configurationTips) {
            const tipsByComponent = window.configurationTips.configurationTips;
            
            // Flatten all tips from all components and distributions
            Object.entries(tipsByComponent).forEach(([componentKey, distributions]) => {
                Object.entries(distributions).forEach(([distroKey, tips]) => {
                    tips.forEach(tip => {
                        // Create a searchable version of the tip
                        const searchableTip = this.createSearchableTip(tip, componentKey, distroKey);
                        this.allTips.push(searchableTip);
                    });
                });
            });
        }
        
        this.filteredTips = [...this.allTips];
    }

    /**
     * Create a searchable version of a tip
     */
    createSearchableTip(tip, componentKey, distributionKey) {
        // Extract component and vendor from component key
        const [component, vendor] = componentKey.split(':');
        
        // Combine all searchable text
        const commandsText = tip.commands ? 
            tip.commands.map(cmd => `${cmd.command} ${cmd.description || ''}`).join(' ') : '';
        
        const configText = tip.configuration ? 
            Object.values(tip.configuration).join(' ') : '';
        
        const warningsText = tip.warnings ? 
            (Array.isArray(tip.warnings) ? tip.warnings.join(' ') : tip.warnings) : '';

        return {
            ...tip,
            component: component,
            vendor: vendor || '',
            distribution: distributionKey,
            commands: commandsText,
            configuration: configText,
            warnings: warningsText,
            searchText: `${tip.title} ${tip.description} ${commandsText} ${configText} ${warningsText}`.toLowerCase()
        };
    }

    /**
     * Build the search index
     */
    buildSearchIndex() {
        this.allTips.forEach(tip => {
            this.searchIndex.add(tip);
        });
    }

    /**
     * Set up search interface in configuration tips modal
     */
    setupSearchInterface() {
        // Enhance existing configuration tips modal with search
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('config-tips-btn') || 
                e.target.classList.contains('add-tip-btn')) {
                setTimeout(() => this.addSearchToModal(), 200);
            }
        });

        // Add search to any existing modal
        const existingModal = document.querySelector('.config-tips-modal');
        if (existingModal) {
            this.addSearchToModal();
        }
    }

    /**
     * Add search interface to configuration tips modal
     */
    addSearchToModal() {
        const modal = document.querySelector('.config-tips-modal');
        if (!modal) return;

        const modalHeader = modal.querySelector('.config-modal-header');
        if (!modalHeader || modalHeader.querySelector('.tip-search-container')) return;

        // Create search interface
        const searchContainer = document.createElement('div');
        searchContainer.className = 'tip-search-container';
        searchContainer.innerHTML = `
            <div class="tip-search-interface">
                <div class="search-input-row">
                    <div class="search-input-container">
                        <input type="text" id="tip-search-input" class="tip-search-input" 
                               placeholder="Search configuration tips..." autocomplete="off">
                        <button class="clear-search-btn" id="clear-tip-search" title="Clear search">✕</button>
                    </div>
                    <button class="search-filters-toggle" id="search-filters-toggle" title="Show filters">
                        🔍 Filters
                    </button>
                </div>
                
                <div class="search-filters-panel" id="search-filters-panel" style="display: none;">
                    <div class="filter-row">
                        <div class="filter-group">
                            <label>Categories:</label>
                            <div class="filter-checkboxes" id="category-filters">
                                ${Object.entries(this.categories).map(([key, name]) => `
                                    <label class="filter-checkbox">
                                        <input type="checkbox" value="${key}" name="category">
                                        <span>${name}</span>
                                    </label>
                                `).join('')}
                            </div>
                        </div>
                        
                        <div class="filter-group">
                            <label>Hardware:</label>
                            <div class="filter-checkboxes" id="component-filters">
                                ${Object.entries(this.components).map(([key, name]) => `
                                    <label class="filter-checkbox">
                                        <input type="checkbox" value="${key}" name="component">
                                        <span>${name}</span>
                                    </label>
                                `).join('')}
                            </div>
                        </div>
                    </div>
                    
                    <div class="filter-row">
                        <div class="filter-group">
                            <label>Distributions:</label>
                            <div class="filter-checkboxes" id="distribution-filters">
                                ${Object.entries(this.distributions).map(([key, name]) => `
                                    <label class="filter-checkbox">
                                        <input type="checkbox" value="${key}" name="distribution">
                                        <span>${name}</span>
                                    </label>
                                `).join('')}
                            </div>
                        </div>
                        
                        <div class="filter-group">
                            <label>Minimum Rating:</label>
                            <div class="rating-filter">
                                <input type="range" id="rating-filter" min="0" max="5" step="0.5" value="0">
                                <span id="rating-display">Any rating</span>
                            </div>
                        </div>
                    </div>
                    
                    <div class="filter-row">
                        <div class="filter-group">
                            <label>Sort by:</label>
                            <select id="sort-select">
                                <option value="relevance">Relevance</option>
                                <option value="rating">Highest Rating</option>
                                <option value="votes">Most Votes</option>
                                <option value="date">Newest</option>
                                <option value="title">Title A-Z</option>
                            </select>
                        </div>
                        
                        <div class="filter-actions">
                            <button class="btn-secondary clear-filters-btn" id="clear-all-filters">
                                Clear All
                            </button>
                            <button class="btn-primary apply-filters-btn" id="apply-filters">
                                Apply Filters
                            </button>
                        </div>
                    </div>
                </div>
                
                <div class="search-results-info" id="search-results-info">
                    <span id="results-count">Showing all tips</span>
                    <div class="search-suggestions" id="search-suggestions"></div>
                </div>
            </div>
        `;

        // Insert after the header controls
        modalHeader.appendChild(searchContainer);

        // Set up event listeners
        this.setupSearchEventListeners();
    }

    /**
     * Set up event listeners for search interface
     */
    setupSearchEventListeners() {
        const searchInput = document.getElementById('tip-search-input');
        const clearSearch = document.getElementById('clear-tip-search');
        const filtersToggle = document.getElementById('search-filters-toggle');
        const filtersPanel = document.getElementById('search-filters-panel');
        const applyFilters = document.getElementById('apply-filters');
        const clearAllFilters = document.getElementById('clear-all-filters');
        const ratingFilter = document.getElementById('rating-filter');
        const ratingDisplay = document.getElementById('rating-display');
        const sortSelect = document.getElementById('sort-select');

        // Search input with debounce
        let searchTimeout;
        if (searchInput) {
            searchInput.addEventListener('input', (e) => {
                clearTimeout(searchTimeout);
                searchTimeout = setTimeout(() => {
                    this.updateSearchQuery(e.target.value);
                    this.performSearch();
                }, 300);
            });

            searchInput.addEventListener('keydown', (e) => {
                if (e.key === 'Escape') {
                    this.clearSearch();
                }
            });
        }

        // Clear search
        if (clearSearch) {
            clearSearch.addEventListener('click', () => {
                this.clearSearch();
            });
        }

        // Filters toggle
        if (filtersToggle) {
            filtersToggle.addEventListener('click', () => {
                const isVisible = filtersPanel.style.display !== 'none';
                filtersPanel.style.display = isVisible ? 'none' : 'block';
                filtersToggle.textContent = isVisible ? '🔍 Filters' : '🔍 Hide Filters';
            });
        }

        // Rating filter
        if (ratingFilter && ratingDisplay) {
            ratingFilter.addEventListener('input', (e) => {
                const value = parseFloat(e.target.value);
                ratingDisplay.textContent = value === 0 ? 'Any rating' : `${value}+ stars`;
                this.searchFilters.minRating = value;
            });
        }

        // Sort select
        if (sortSelect) {
            sortSelect.addEventListener('change', (e) => {
                this.searchFilters.sortBy = e.target.value;
                this.performSearch();
            });
        }

        // Apply filters
        if (applyFilters) {
            applyFilters.addEventListener('click', () => {
                this.applyFilters();
            });
        }

        // Clear all filters
        if (clearAllFilters) {
            clearAllFilters.addEventListener('click', () => {
                this.clearAllFilters();
            });
        }

        // Category, component, and distribution checkboxes
        ['category', 'component', 'distribution'].forEach(filterType => {
            const checkboxes = document.querySelectorAll(`input[name="${filterType}"]`);
            checkboxes.forEach(checkbox => {
                checkbox.addEventListener('change', () => {
                    this.updateCheckboxFilters();
                });
            });
        });
    }

    /**
     * Update search query
     */
    updateSearchQuery(query) {
        this.searchFilters.query = query.trim();
    }

    /**
     * Update checkbox filters
     */
    updateCheckboxFilters() {
        this.searchFilters.categories = Array.from(document.querySelectorAll('input[name="category"]:checked'))
            .map(cb => cb.value);
        
        this.searchFilters.components = Array.from(document.querySelectorAll('input[name="component"]:checked'))
            .map(cb => cb.value);
        
        this.searchFilters.distributions = Array.from(document.querySelectorAll('input[name="distribution"]:checked'))
            .map(cb => cb.value);
    }

    /**
     * Apply all filters
     */
    applyFilters() {
        this.updateCheckboxFilters();
        this.performSearch();
        
        // Hide filters panel
        const filtersPanel = document.getElementById('search-filters-panel');
        const filtersToggle = document.getElementById('search-filters-toggle');
        if (filtersPanel && filtersToggle) {
            filtersPanel.style.display = 'none';
            filtersToggle.textContent = '🔍 Filters';
        }
    }

    /**
     * Clear all filters
     */
    clearAllFilters() {
        // Reset filters object
        this.searchFilters = {
            query: '',
            distributions: [],
            categories: [],
            components: [],
            contributors: [],
            minRating: 0,
            sortBy: 'relevance'
        };

        // Reset form elements
        const searchInput = document.getElementById('tip-search-input');
        if (searchInput) searchInput.value = '';

        const checkboxes = document.querySelectorAll('.search-filters-panel input[type="checkbox"]');
        checkboxes.forEach(cb => cb.checked = false);

        const ratingFilter = document.getElementById('rating-filter');
        const ratingDisplay = document.getElementById('rating-display');
        if (ratingFilter && ratingDisplay) {
            ratingFilter.value = '0';
            ratingDisplay.textContent = 'Any rating';
        }

        const sortSelect = document.getElementById('sort-select');
        if (sortSelect) sortSelect.value = 'relevance';

        // Perform search with cleared filters
        this.performSearch();
    }

    /**
     * Clear search only
     */
    clearSearch() {
        const searchInput = document.getElementById('tip-search-input');
        if (searchInput) {
            searchInput.value = '';
            this.searchFilters.query = '';
            this.performSearch();
        }
    }

    /**
     * Perform search with current filters
     */
    performSearch() {
        let results = this.allTips;

        // Text search
        if (this.searchFilters.query) {
            const searchResults = this.searchIndex.search(this.searchFilters.query, {
                limit: 1000,
                enrich: true
            });
            
            const resultIds = new Set();
            searchResults.forEach(result => {
                result.result.forEach(item => {
                    resultIds.add(item.id);
                });
            });

            results = results.filter(tip => resultIds.has(tip.id));
        }

        // Category filter
        if (this.searchFilters.categories.length > 0) {
            results = results.filter(tip => 
                this.searchFilters.categories.includes(tip.category)
            );
        }

        // Component filter
        if (this.searchFilters.components.length > 0) {
            results = results.filter(tip => 
                this.searchFilters.components.includes(tip.component)
            );
        }

        // Distribution filter
        if (this.searchFilters.distributions.length > 0) {
            results = results.filter(tip => 
                this.searchFilters.distributions.includes(tip.distribution)
            );
        }

        // Rating filter
        if (this.searchFilters.minRating > 0) {
            results = results.filter(tip => 
                (tip.rating || 0) >= this.searchFilters.minRating
            );
        }

        // Sort results
        results = this.sortResults(results);

        this.filteredTips = results;
        this.updateSearchResults();
        this.updateResultsInfo();
    }

    /**
     * Sort search results
     */
    sortResults(results) {
        switch (this.searchFilters.sortBy) {
            case 'rating':
                return results.sort((a, b) => (b.rating || 0) - (a.rating || 0));
            
            case 'votes':
                return results.sort((a, b) => (b.votes || 0) - (a.votes || 0));
            
            case 'date':
                return results.sort((a, b) => new Date(b.date || '2000-01-01') - new Date(a.date || '2000-01-01'));
            
            case 'title':
                return results.sort((a, b) => (a.title || '').localeCompare(b.title || ''));
            
            case 'relevance':
            default:
                return results; // FlexSearch already provides relevance ordering
        }
    }

    /**
     * Update search results display
     */
    updateSearchResults() {
        const modalBody = document.querySelector('.config-modal-body');
        if (!modalBody) return;

        if (this.filteredTips.length === 0) {
            modalBody.innerHTML = `
                <div class="no-search-results">
                    <div class="no-results-icon">🔍</div>
                    <h3>No tips found</h3>
                    <p>Try adjusting your search terms or filters.</p>
                    <button class="btn-secondary" onclick="window.tipSearchSystem.clearAllFilters()">
                        Clear All Filters
                    </button>
                </div>
            `;
            return;
        }

        // Group results by hardware component for better organization
        const groupedResults = this.groupResultsByComponent();
        
        let html = '';
        Object.entries(groupedResults).forEach(([component, tips]) => {
            html += `
                <div class="search-results-section">
                    <h3 class="component-header">
                        ${this.getComponentIcon(component)} ${this.getComponentName(component)}
                        <span class="tip-count">(${tips.length} ${tips.length === 1 ? 'tip' : 'tips'})</span>
                    </h3>
                    <div class="tips-grid">
                        ${tips.map(tip => this.renderSearchResultTip(tip)).join('')}
                    </div>
                </div>
            `;
        });

        modalBody.innerHTML = html;
        
        // Set up tip interaction handlers
        this.setupTipInteractionHandlers();
    }

    /**
     * Group search results by hardware component
     */
    groupResultsByComponent() {
        const grouped = {};
        
        this.filteredTips.forEach(tip => {
            const component = tip.component || 'other';
            if (!grouped[component]) {
                grouped[component] = [];
            }
            grouped[component].push(tip);
        });

        return grouped;
    }

    /**
     * Get component icon
     */
    getComponentIcon(component) {
        const icons = {
            'cpu': '🖥️',
            'gpu': '🎮',
            'memory': '💾',
            'storage': '💿',
            'network': '🌐',
            'audio': '🔊',
            'input': '⌨️',
            'display': '🖥️',
            'system': '⚙️'
        };
        return icons[component] || '🔧';
    }

    /**
     * Get component display name
     */
    getComponentName(component) {
        return this.components[component] || component.charAt(0).toUpperCase() + component.slice(1);
    }

    /**
     * Render a search result tip
     */
    renderSearchResultTip(tip) {
        return `
            <div class="search-result-tip" data-tip-id="${tip.id}">
                <div class="tip-header">
                    <h4 class="tip-title">${tip.title}</h4>
                    <div class="tip-badges">
                        <span class="distribution-badge">${this.distributions[tip.distribution] || tip.distribution}</span>
                        <span class="rating-badge">⭐ ${tip.rating?.toFixed(1) || 'N/A'}</span>
                    </div>
                </div>
                
                <p class="tip-description">${tip.description}</p>
                
                <div class="tip-meta">
                    <span class="tip-category">${this.categories[tip.category] || tip.category}</span>
                    ${tip.vendor ? `<span class="tip-vendor">${tip.vendor}</span>` : ''}
                    ${tip.contributor ? `
                        <span class="tip-contributor-mini">
                            By <a href="${tip.contributor.profile_url}" target="_blank">${tip.contributor.name || tip.contributor.username}</a>
                        </span>
                    ` : ''}
                </div>
                
                <div class="tip-actions">
                    <button class="btn-secondary view-tip-btn" data-tip-id="${tip.id}">
                        👁️ View Details
                    </button>
                    <button class="btn-secondary copy-tip-btn" data-tip-id="${tip.id}">
                        📋 Copy Commands
                    </button>
                </div>
            </div>
        `;
    }

    /**
     * Set up tip interaction handlers
     */
    setupTipInteractionHandlers() {
        // View tip details
        document.querySelectorAll('.view-tip-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const tipId = e.target.dataset.tipId;
                this.showTipDetails(tipId);
            });
        });

        // Copy tip commands
        document.querySelectorAll('.copy-tip-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const tipId = e.target.dataset.tipId;
                this.copyTipCommands(tipId);
            });
        });
    }

    /**
     * Show detailed view of a tip
     */
    showTipDetails(tipId) {
        const tip = this.filteredTips.find(t => t.id === tipId);
        if (!tip) return;

        // Use the existing configuration tips rendering
        if (window.configurationTips) {
            const renderedTip = window.configurationTips.renderTip(tip, tip.distribution);
            
            // Create a detail modal
            const detailModal = document.createElement('div');
            detailModal.className = 'tip-detail-modal';
            detailModal.innerHTML = `
                <div class="tip-detail-content">
                    <div class="tip-detail-header">
                        <h2>Configuration Tip Details</h2>
                        <button class="tip-detail-close" aria-label="Close">&times;</button>
                    </div>
                    <div class="tip-detail-body">
                        ${renderedTip}
                    </div>
                </div>
            `;

            document.body.appendChild(detailModal);
            detailModal.style.display = 'flex';

            // Close handler
            detailModal.querySelector('.tip-detail-close').addEventListener('click', () => {
                detailModal.remove();
            });

            detailModal.addEventListener('click', (e) => {
                if (e.target === detailModal) {
                    detailModal.remove();
                }
            });
        }
    }

    /**
     * Copy tip commands to clipboard
     */
    async copyTipCommands(tipId) {
        const tip = this.filteredTips.find(t => t.id === tipId);
        if (!tip || !tip.commands) return;

        const commands = tip.commands.map(cmd => cmd.command || cmd).join('\n');
        
        try {
            await navigator.clipboard.writeText(commands);
            this.showNotification('Commands copied to clipboard!', 'success');
        } catch (error) {
            console.error('Failed to copy commands:', error);
            this.showNotification('Failed to copy commands', 'error');
        }
    }

    /**
     * Update results info display
     */
    updateResultsInfo() {
        const resultsCount = document.getElementById('results-count');
        if (!resultsCount) return;

        const total = this.allTips.length;
        const filtered = this.filteredTips.length;
        
        if (this.hasActiveFilters()) {
            resultsCount.textContent = `Showing ${filtered} of ${total} tips`;
        } else {
            resultsCount.textContent = `Showing all ${total} tips`;
        }
    }

    /**
     * Check if any filters are active
     */
    hasActiveFilters() {
        return this.searchFilters.query ||
               this.searchFilters.categories.length > 0 ||
               this.searchFilters.components.length > 0 ||
               this.searchFilters.distributions.length > 0 ||
               this.searchFilters.minRating > 0 ||
               this.searchFilters.sortBy !== 'relevance';
    }

    /**
     * Show notification
     */
    showNotification(message, type = 'info') {
        if (window.githubAuth) {
            window.githubAuth.showNotification(message, type);
        } else {
            console.log(`${type.toUpperCase()}: ${message}`);
        }
    }

    /**
     * Get search suggestions based on current query
     */
    getSearchSuggestions(query) {
        if (!query || query.length < 2) return [];

        const suggestions = new Set();
        const lowerQuery = query.toLowerCase();

        // Add matching tip titles
        this.allTips.forEach(tip => {
            if (tip.title.toLowerCase().includes(lowerQuery)) {
                suggestions.add(tip.title);
            }
        });

        // Add matching vendors
        this.allTips.forEach(tip => {
            if (tip.vendor && tip.vendor.toLowerCase().includes(lowerQuery)) {
                suggestions.add(tip.vendor);
            }
        });

        return Array.from(suggestions).slice(0, 5);
    }
}

// Global instance
window.tipSearchSystem = new TipSearchSystem();

// Auto-initialize when configuration tips are ready
document.addEventListener('DOMContentLoaded', async () => {
    // Wait for configuration tips to be loaded
    if (window.configurationTips) {
        await window.tipSearchSystem.initialize();
    } else {
        // Wait for configuration tips system to initialize
        const checkConfigTips = setInterval(async () => {
            if (window.configurationTips && window.configurationTips.isInitialized) {
                clearInterval(checkConfigTips);
                await window.tipSearchSystem.initialize();
            }
        }, 500);
    }
});

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = TipSearchSystem;
}