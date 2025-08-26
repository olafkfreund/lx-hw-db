/**
 * Linux Hardware Compatibility Database - Search UI Components
 * 
 * UI components and interactions for the hardware compatibility search interface
 */

class HardwareSearchUI {
    constructor(searchEngine, options = {}) {
        this.searchEngine = searchEngine;
        this.options = {
            containerId: options.containerId || 'search-container',
            resultsPerPage: options.resultsPerPage || 20,
            enableHistory: options.enableHistory !== false,
            enableFilters: options.enableFilters !== false,
            enableExport: options.enableExport !== false,
            ...options
        };
        
        this.currentResults = [];
        this.currentPage = 1;
        this.activeFilters = new Map();
        this.searchHistory = this.loadSearchHistory();
        
        this.initializeUI();
        this.bindEvents();
    }
    
    /**
     * Initialize the search UI
     */
    initializeUI() {
        const container = document.getElementById(this.options.containerId);
        if (!container) {
            console.error(`Search container #${this.options.containerId} not found`);
            return;
        }
        
        container.innerHTML = this.generateSearchHTML();
        this.bindUIElements();
        this.initializeFilters();
        this.loadUrlParameters();
    }
    
    /**
     * Generate the main search interface HTML
     */
    generateSearchHTML() {
        return `
            <div class="hw-search-interface">
                <!-- Search Header -->
                <div class="search-header">
                    <div class="search-input-container">
                        <input type="text" 
                               id="hw-search-input" 
                               class="search-input"
                               placeholder="Search hardware, vendors, or compatibility info..."
                               autocomplete="off">
                        <button id="hw-search-btn" class="search-button" type="button">
                            <svg class="search-icon" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd"/>
                            </svg>
                        </button>
                        <button id="hw-clear-btn" class="clear-button hidden" type="button">
                            <svg viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"/>
                            </svg>
                        </button>
                        
                        <!-- Typeahead suggestions -->
                        <div id="hw-suggestions" class="suggestions-container hidden"></div>
                    </div>
                    
                    <!-- Quick filters -->
                    <div class="quick-filters">
                        <button class="filter-btn" data-filter="type:vendor">Vendors</button>
                        <button class="filter-btn" data-filter="type:component">Components</button>
                        <button class="filter-btn" data-filter="compatibility:excellent">Excellent</button>
                        <button class="filter-btn" data-filter="compatibility:good">Good</button>
                    </div>
                </div>
                
                <!-- Advanced Filters -->
                ${this.options.enableFilters ? this.generateFiltersHTML() : ''}
                
                <!-- Search Status -->
                <div id="hw-search-status" class="search-status hidden">
                    <div class="status-content">
                        <span class="status-text"></span>
                        <div class="loading-spinner hidden">
                            <svg class="animate-spin" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
                                <path class="opacity-75" fill="currentColor" d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
                            </svg>
                        </div>
                    </div>
                </div>
                
                <!-- Search Results -->
                <div id="hw-search-results" class="search-results hidden">
                    <div class="results-header">
                        <div class="results-info">
                            <span id="results-count">0</span> results
                            <span id="results-query" class="query-display"></span>
                        </div>
                        
                        <div class="results-controls">
                            <select id="sort-select" class="sort-select">
                                <option value="relevance">Most Relevant</option>
                                <option value="popularity">Most Popular</option>
                                <option value="compatibility">Best Compatibility</option>
                                <option value="alphabetical">Alphabetical</option>
                            </select>
                            
                            ${this.options.enableExport ? '<button id="export-btn" class="export-button">Export Results</button>' : ''}
                        </div>
                    </div>
                    
                    <div id="results-container" class="results-container"></div>
                    
                    <!-- Pagination -->
                    <div id="pagination" class="pagination hidden"></div>
                </div>
                
                <!-- Empty State -->
                <div id="hw-empty-state" class="empty-state">
                    <div class="empty-state-content">
                        <svg class="empty-icon" viewBox="0 0 48 48">
                            <path d="M24 4C13 4 4 13 4 24s9 20 20 20 20-9 20-20S35 4 24 4zm-4 30l-8-8 2.83-2.83L20 28.34l15.17-15.17L38 16 20 34z"/>
                        </svg>
                        <h3>Linux Hardware Compatibility Database</h3>
                        <p>Search for hardware compatibility information across different Linux distributions and kernel versions.</p>
                        <div class="example-searches">
                            <p><strong>Try searching for:</strong></p>
                            <button class="example-btn" data-query="nvidia">NVIDIA graphics</button>
                            <button class="example-btn" data-query="intel">Intel hardware</button>
                            <button class="example-btn" data-query="6.1">Kernel 6.1</button>
                            <button class="example-btn" data-query="wifi">WiFi adapters</button>
                        </div>
                    </div>
                </div>
            </div>
        `;
    }
    
    /**
     * Generate filters HTML
     */
    generateFiltersHTML() {
        return `
            <div class="advanced-filters hidden">
                <div class="filters-content">
                    <div class="filter-group">
                        <label>Hardware Type</label>
                        <select id="filter-component" class="filter-select" multiple>
                            <option value="">All Components</option>
                        </select>
                    </div>
                    
                    <div class="filter-group">
                        <label>Compatibility</label>
                        <select id="filter-compatibility" class="filter-select">
                            <option value="">Any Compatibility</option>
                            <option value="excellent">Excellent</option>
                            <option value="good">Good</option>
                            <option value="fair">Fair</option>
                            <option value="poor">Poor</option>
                        </select>
                    </div>
                    
                    <div class="filter-group">
                        <label>Kernel Version</label>
                        <select id="filter-kernel" class="filter-select">
                            <option value="">Any Kernel</option>
                        </select>
                    </div>
                    
                    <div class="filter-group">
                        <label>Distribution</label>
                        <select id="filter-distribution" class="filter-select">
                            <option value="">Any Distribution</option>
                        </select>
                    </div>
                    
                    <div class="filter-actions">
                        <button id="apply-filters-btn" class="apply-filters-btn">Apply Filters</button>
                        <button id="clear-filters-btn" class="clear-filters-btn">Clear All</button>
                    </div>
                </div>
            </div>
        `;
    }
    
    /**
     * Bind UI elements to properties
     */
    bindUIElements() {
        this.elements = {
            searchInput: document.getElementById('hw-search-input'),
            searchBtn: document.getElementById('hw-search-btn'),
            clearBtn: document.getElementById('hw-clear-btn'),
            suggestions: document.getElementById('hw-suggestions'),
            searchStatus: document.getElementById('hw-search-status'),
            searchResults: document.getElementById('hw-search-results'),
            resultsContainer: document.getElementById('results-container'),
            emptyState: document.getElementById('hw-empty-state'),
            resultsCount: document.getElementById('results-count'),
            resultsQuery: document.getElementById('results-query'),
            sortSelect: document.getElementById('sort-select'),
            pagination: document.getElementById('pagination')
        };
        
        // Filter elements
        if (this.options.enableFilters) {
            this.elements.filters = {
                component: document.getElementById('filter-component'),
                compatibility: document.getElementById('filter-compatibility'),
                kernel: document.getElementById('filter-kernel'),
                distribution: document.getElementById('filter-distribution')
            };
        }
        
        // Export button
        if (this.options.enableExport) {
            this.elements.exportBtn = document.getElementById('export-btn');
        }
    }
    
    /**
     * Bind event handlers
     */
    bindEvents() {
        // Search input events
        this.elements.searchInput.addEventListener('input', this.debounce(this.handleSearchInput.bind(this), 300));
        this.elements.searchInput.addEventListener('keydown', this.handleSearchKeydown.bind(this));
        this.elements.searchInput.addEventListener('focus', this.handleSearchFocus.bind(this));
        this.elements.searchInput.addEventListener('blur', this.handleSearchBlur.bind(this));
        
        // Search button
        this.elements.searchBtn.addEventListener('click', this.handleSearch.bind(this));
        
        // Clear button
        this.elements.clearBtn.addEventListener('click', this.handleClear.bind(this));
        
        // Sort control
        if (this.elements.sortSelect) {
            this.elements.sortSelect.addEventListener('change', this.handleSortChange.bind(this));
        }
        
        // Export button
        if (this.elements.exportBtn) {
            this.elements.exportBtn.addEventListener('click', this.handleExport.bind(this));
        }
        
        // Quick filters
        document.querySelectorAll('.filter-btn').forEach(btn => {
            btn.addEventListener('click', this.handleQuickFilter.bind(this));
        });
        
        // Example searches
        document.querySelectorAll('.example-btn').forEach(btn => {
            btn.addEventListener('click', this.handleExampleSearch.bind(this));
        });
        
        // Search engine events
        this.searchEngine.on('searchStarted', this.handleSearchStarted.bind(this));
        this.searchEngine.on('searchCompleted', this.handleSearchCompleted.bind(this));
        this.searchEngine.on('searchError', this.handleSearchError.bind(this));
        
        // Window events
        if (this.options.enableHistory) {
            window.addEventListener('popstate', this.handlePopState.bind(this));
        }
        
        // Keyboard shortcuts
        document.addEventListener('keydown', this.handleGlobalKeydown.bind(this));
    }
    
    /**
     * Handle search input
     */
    async handleSearchInput(event) {
        const query = event.target.value.trim();
        
        // Update clear button visibility
        this.elements.clearBtn.classList.toggle('hidden', query.length === 0);
        
        // Show suggestions for typeahead
        if (query.length >= 2) {
            this.showSuggestions(query);
        } else {
            this.hideSuggestions();
        }
    }
    
    /**
     * Handle search keydown
     */
    handleSearchKeydown(event) {
        if (event.key === 'Enter') {
            event.preventDefault();
            this.handleSearch();
        } else if (event.key === 'Escape') {
            this.hideSuggestions();
            this.elements.searchInput.blur();
        } else if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
            this.handleSuggestionNavigation(event);
        }
    }
    
    /**
     * Handle main search action
     */
    async handleSearch() {
        const query = this.elements.searchInput.value.trim();
        if (!query) return;
        
        this.hideSuggestions();
        await this.performSearch(query);
        this.updateHistory(query);
    }
    
    /**
     * Perform the actual search
     */
    async performSearch(query, options = {}) {
        try {
            const searchOptions = {
                ...options,
                ...this.getActiveFilters()
            };
            
            const results = await this.searchEngine.search(query, searchOptions);
            this.displayResults(results, query);
            
        } catch (error) {
            console.error('Search error:', error);
            this.showError('Search failed. Please try again.');
        }
    }
    
    /**
     * Display search results
     */
    displayResults(searchResult, query) {
        this.currentResults = searchResult.results;
        this.currentPage = 1;
        
        // Update UI state
        this.elements.emptyState.classList.add('hidden');
        this.elements.searchResults.classList.remove('hidden');
        
        // Update results info
        this.elements.resultsCount.textContent = searchResult.metadata.total;
        this.elements.resultsQuery.textContent = query ? `for "${query}"` : '';
        
        // Render results
        this.renderResultsPage();
        this.renderPagination();
    }
    
    /**
     * Render current page of results
     */
    renderResultsPage() {
        const startIdx = (this.currentPage - 1) * this.options.resultsPerPage;
        const endIdx = startIdx + this.options.resultsPerPage;
        const pageResults = this.currentResults.slice(startIdx, endIdx);
        
        this.elements.resultsContainer.innerHTML = pageResults
            .map(result => this.renderResult(result))
            .join('');
        
        // Bind result click handlers
        this.elements.resultsContainer.querySelectorAll('.result-item').forEach(item => {
            item.addEventListener('click', this.handleResultClick.bind(this));
        });
    }
    
    /**
     * Render a single search result
     */
    renderResult(result) {
        const compatibilityClass = this.getCompatibilityClass(result.compatibility);
        const compatibilityText = this.getCompatibilityText(result.compatibility);
        
        return `
            <div class="result-item" data-id="${result.id}" data-type="${result.type}">
                <div class="result-content">
                    <div class="result-header">
                        <h3 class="result-title">${this.escapeHtml(result.title)}</h3>
                        <div class="result-badges">
                            <span class="result-type-badge">${result.type}</span>
                            ${result.compatibility > 0 ? `<span class="compatibility-badge ${compatibilityClass}">${compatibilityText}</span>` : ''}
                        </div>
                    </div>
                    
                    <p class="result-description">${this.escapeHtml(result.description)}</p>
                    
                    <div class="result-meta">
                        ${result.reportCount > 0 ? `<span class="report-count">${result.reportCount} reports</span>` : ''}
                        ${result.strategy ? `<span class="match-type">Found via ${result.strategy} search</span>` : ''}
                        <span class="relevance-score">Relevance: ${(result.score * 100).toFixed(0)}%</span>
                    </div>
                </div>
                
                <div class="result-actions">
                    <button class="action-btn view-details" data-action="view">
                        <svg viewBox="0 0 20 20" fill="currentColor">
                            <path d="M10 12a2 2 0 100-4 2 2 0 000 4z"/>
                            <path fill-rule="evenodd" d="M.458 10C1.732 5.943 5.522 3 10 3s8.268 2.943 9.542 7c-1.274 4.057-5.064 7-9.542 7S1.732 14.057.458 10zM14 10a4 4 0 11-8 0 4 4 0 018 0z" clip-rule="evenodd"/>
                        </svg>
                        View Details
                    </button>
                </div>
            </div>
        `;
    }
    
    /**
     * Show search suggestions
     */
    async showSuggestions(query) {
        try {
            const suggestions = await this.searchEngine.getSuggestions(query, 8);
            
            if (suggestions.length > 0) {
                this.elements.suggestions.innerHTML = suggestions
                    .map(suggestion => `
                        <div class="suggestion-item" data-query="${this.escapeHtml(suggestion.text)}">
                            <span class="suggestion-text">${this.escapeHtml(suggestion.text)}</span>
                            <span class="suggestion-type">${suggestion.type}</span>
                        </div>
                    `)
                    .join('');
                
                this.elements.suggestions.classList.remove('hidden');
                
                // Bind suggestion click handlers
                this.elements.suggestions.querySelectorAll('.suggestion-item').forEach(item => {
                    item.addEventListener('click', this.handleSuggestionClick.bind(this));
                });
            } else {
                this.hideSuggestions();
            }
        } catch (error) {
            console.warn('Failed to load suggestions:', error);
        }
    }
    
    /**
     * Show loading state
     */
    showLoading(message = 'Searching hardware database...') {
        this.elements.searchStatus.classList.remove('hidden');
        this.elements.searchStatus.querySelector('.status-text').textContent = message;
        this.elements.searchStatus.querySelector('.loading-spinner').classList.remove('hidden');
    }
    
    /**
     * Hide loading state
     */
    hideLoading() {
        this.elements.searchStatus.classList.add('hidden');
        this.elements.searchStatus.querySelector('.loading-spinner').classList.add('hidden');
    }
    
    /**
     * Show error message
     */
    showError(message) {
        this.elements.searchStatus.classList.remove('hidden');
        this.elements.searchStatus.querySelector('.status-text').textContent = message;
        this.elements.searchStatus.querySelector('.loading-spinner').classList.add('hidden');
        
        setTimeout(() => {
            this.hideLoading();
        }, 5000);
    }
    
    // Event handlers
    
    handleSearchStarted(event) {
        this.showLoading();
    }
    
    handleSearchCompleted(event) {
        this.hideLoading();
    }
    
    handleSearchError(event) {
        this.hideLoading();
        this.showError('Search failed. Please try again.');
    }
    
    handleClear() {
        this.elements.searchInput.value = '';
        this.elements.clearBtn.classList.add('hidden');
        this.hideSuggestions();
        this.clearResults();
        this.elements.searchInput.focus();
    }
    
    handleSortChange() {
        if (this.currentResults.length > 0) {
            this.sortResults(this.elements.sortSelect.value);
            this.renderResultsPage();
        }
    }
    
    clearResults() {
        this.currentResults = [];
        this.elements.searchResults.classList.add('hidden');
        this.elements.emptyState.classList.remove('hidden');
    }
    
    // Utility methods
    
    debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }
    
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
    
    getCompatibilityClass(score) {
        if (score >= 90) return 'excellent';
        if (score >= 75) return 'good';
        if (score >= 50) return 'fair';
        return 'poor';
    }
    
    getCompatibilityText(score) {
        if (score >= 90) return 'Excellent';
        if (score >= 75) return 'Good';
        if (score >= 50) return 'Fair';
        if (score > 0) return 'Poor';
        return 'Unknown';
    }
    
    // Stub methods for additional functionality
    
    initializeFilters() {
        // Initialize advanced filters
        // This would populate filter dropdowns with data from indices
    }
    
    getActiveFilters() {
        // Return currently active filters
        return {};
    }
    
    loadUrlParameters() {
        // Load search parameters from URL
        const params = new URLSearchParams(window.location.search);
        const query = params.get('q');
        if (query) {
            this.elements.searchInput.value = query;
            this.performSearch(query);
        }
    }
    
    updateHistory(query) {
        if (!this.options.enableHistory) return;
        
        const url = new URL(window.location);
        url.searchParams.set('q', query);
        history.pushState({ query }, '', url);
    }
    
    loadSearchHistory() {
        try {
            return JSON.parse(localStorage.getItem('hw-search-history') || '[]');
        } catch {
            return [];
        }
    }
    
    renderPagination() {
        // Render pagination controls
        const totalPages = Math.ceil(this.currentResults.length / this.options.resultsPerPage);
        if (totalPages <= 1) {
            this.elements.pagination.classList.add('hidden');
            return;
        }
        
        // Implementation would create pagination buttons
        this.elements.pagination.classList.remove('hidden');
    }
    
    sortResults(sortType) {
        // Sort current results by the specified type
        switch (sortType) {
            case 'popularity':
                this.currentResults.sort((a, b) => b.reportCount - a.reportCount);
                break;
            case 'compatibility':
                this.currentResults.sort((a, b) => b.compatibility - a.compatibility);
                break;
            case 'alphabetical':
                this.currentResults.sort((a, b) => a.title.localeCompare(b.title));
                break;
            default:
                this.currentResults.sort((a, b) => b.score - a.score);
        }
    }
    
    // Additional stub methods would be implemented for:
    // - handleSuggestionClick
    // - handleSuggestionNavigation  
    // - handleResultClick
    // - handleQuickFilter
    // - handleExampleSearch
    // - handlePopState
    // - handleGlobalKeydown
    // - handleSearchFocus
    // - handleSearchBlur
    // - handleExport
    // - hideSuggestions
}

// Export for both browser and Node.js
if (typeof module !== 'undefined' && module.exports) {
    module.exports = HardwareSearchUI;
} else if (typeof window !== 'undefined') {
    window.HardwareSearchUI = HardwareSearchUI;
}