
/**
 * Hardware Search User Interface
 * Handles search interactions, results display, and filtering
 */

class HardwareSearchUI {
    constructor() {
        this.searchEngine = null;
        this.currentQuery = '';
        this.currentFilters = {};
        this.currentResults = [];
        this.isSearching = false;
    }

    /**
     * Initialize the search UI
     */
    async initialize() {
        console.log('Initializing search UI...');
        
        // Wait for search engine to be ready
        document.addEventListener('searchEngineReady', (event) => {
            this.searchEngine = event.detail.engine;
            this.setupEventListeners();
            this.setupFilters();
            console.log('Search UI initialized successfully');
        });

        // Initialize search engine if not already done
        if (window.hardwareSearch) {
            if (!window.hardwareSearch.isInitialized) {
                await window.hardwareSearch.initialize();
            } else {
                this.searchEngine = window.hardwareSearch;
                this.setupEventListeners();
                this.setupFilters();
            }
        }
    }

    /**
     * Set up event listeners for search interactions
     */
    setupEventListeners() {
        // Main search input
        const searchInput = document.querySelector('#search-input');
        if (searchInput) {
            // Real-time search with debouncing
            let searchTimeout;
            searchInput.addEventListener('input', (e) => {
                clearTimeout(searchTimeout);
                searchTimeout = setTimeout(() => {
                    this.performSearch(e.target.value);
                }, 300);
            });

            // Handle enter key
            searchInput.addEventListener('keydown', (e) => {
                if (e.key === 'Enter') {
                    e.preventDefault();
                    this.performSearch(e.target.value);
                }
            });
        }

        // Search form submission
        const searchForm = document.querySelector('#search-form');
        if (searchForm) {
            searchForm.addEventListener('submit', (e) => {
                e.preventDefault();
                const query = document.querySelector('#search-input')?.value || '';
                this.performSearch(query);
            });
        }

        // Clear search button
        const clearButton = document.querySelector('#clear-search');
        if (clearButton) {
            clearButton.addEventListener('click', () => {
                this.clearSearch();
            });
        }
    }

    /**
     * Set up filter controls
     */
    async setupFilters() {
        if (!this.searchEngine) return;

        const filterOptions = this.searchEngine.getFilterOptions();
        
        // Create category filter
        this.createCategoryFilter();
        
        // Create vendor filter
        this.createVendorFilter(filterOptions.vendors);
        
        // Create architecture filter
        this.createArchitectureFilter(filterOptions.architectures);
        
        // Create compatibility filter
        this.createCompatibilityFilter(filterOptions.compatibility);

        // Set up filter change handlers
        document.querySelectorAll('.filter-select').forEach(select => {
            select.addEventListener('change', (e) => {
                this.updateFilter(e.target.dataset.filter, e.target.value);
            });
        });
    }

    /**
     * Create category filter dropdown
     */
    createCategoryFilter() {
        const categories = [
            { value: '', label: 'All Categories' },
            { value: 'cpu', label: '🖥️ CPU' },
            { value: 'gpu', label: '🎮 Graphics' },
            { value: 'memory', label: '💾 Memory' },
            { value: 'storage', label: '💿 Storage' },
            { value: 'network', label: '🌐 Network' },
            { value: 'audio', label: '🔊 Audio' }
        ];

        this.createFilterDropdown('category', 'Category', categories);
    }

    /**
     * Create vendor filter dropdown
     */
    createVendorFilter(vendors) {
        const options = [
            { value: '', label: 'All Vendors' },
            ...vendors.map(vendor => ({ value: vendor, label: vendor }))
        ];

        this.createFilterDropdown('vendor', 'Vendor', options);
    }

    /**
     * Create architecture filter dropdown
     */
    createArchitectureFilter(architectures) {
        const options = [
            { value: '', label: 'All Architectures' },
            ...architectures.map(arch => ({ value: arch, label: arch }))
        ];

        this.createFilterDropdown('architecture', 'Architecture', options);
    }

    /**
     * Create compatibility filter dropdown
     */
    createCompatibilityFilter(statuses) {
        const statusLabels = {
            excellent: '✅ Excellent',
            good: '👍 Good',
            partial: '⚠️ Partial',
            poor: '❌ Poor'
        };

        const options = [
            { value: '', label: 'All Status' },
            ...statuses.map(status => ({ 
                value: status, 
                label: statusLabels[status] || status 
            }))
        ];

        this.createFilterDropdown('compatibility', 'Compatibility', options);
    }

    /**
     * Create a filter dropdown element
     */
    createFilterDropdown(filterName, label, options) {
        const filtersContainer = document.querySelector('.search-filters');
        if (!filtersContainer) return;

        const filterDiv = document.createElement('div');
        filterDiv.className = 'filter-group';
        
        const filterLabel = document.createElement('label');
        filterLabel.textContent = label;
        filterLabel.className = 'filter-label';

        const select = document.createElement('select');
        select.className = 'filter-select';
        select.dataset.filter = filterName;

        options.forEach(option => {
            const optionElement = document.createElement('option');
            optionElement.value = option.value;
            optionElement.textContent = option.label;
            select.appendChild(optionElement);
        });

        filterDiv.appendChild(filterLabel);
        filterDiv.appendChild(select);
        filtersContainer.appendChild(filterDiv);
    }

    /**
     * Perform hardware search
     */
    async performSearch(query) {
        if (this.isSearching) return;
        
        this.isSearching = true;
        this.currentQuery = query;
        
        try {
            this.showSearchLoading();
            
            const results = await this.searchEngine.search(query, this.currentFilters);
            this.currentResults = results;
            
            this.displayResults(results);
            this.updateSearchStats(query, results.length);
            
        } catch (error) {
            console.error('Search error:', error);
            this.showSearchError(error.message);
        } finally {
            this.isSearching = false;
            this.hideSearchLoading();
        }
    }

    /**
     * Update filter and re-search
     */
    updateFilter(filterName, value) {
        if (value) {
            this.currentFilters[filterName] = value;
        } else {
            delete this.currentFilters[filterName];
        }

        // Re-search with current query and updated filters
        this.performSearch(this.currentQuery);
    }

    /**
     * Display search results
     */
    displayResults(results) {
        const resultsContainer = document.querySelector('#search-results');
        if (!resultsContainer) {
            console.warn('Search results container not found');
            return;
        }

        resultsContainer.innerHTML = '';

        if (results.length === 0) {
            this.showNoResults();
            return;
        }

        // Create results list
        const resultsList = document.createElement('div');
        resultsList.className = 'search-results-list';

        results.forEach((result, index) => {
            const resultCard = this.createResultCard(result, index);
            resultsList.appendChild(resultCard);
        });

        resultsContainer.appendChild(resultsList);
    }

    /**
     * Create a search result card
     */
    createResultCard(result, index) {
        const card = document.createElement('div');
        card.className = 'result-card';
        card.dataset.resultIndex = index;

        // Header with system info
        const header = document.createElement('div');
        header.className = 'result-header';
        header.innerHTML = `
            <div class="result-title">
                <h3>${result.system.distribution} ${result.system.kernel_version}</h3>
                <span class="result-architecture">${result.system.architecture}</span>
            </div>
            <div class="result-status status-${result.compatibility?.overall_status || 'unknown'}">
                ${this.getStatusBadge(result.compatibility?.overall_status)}
            </div>
        `;

        // Hardware summary
        const hardware = document.createElement('div');
        hardware.className = 'result-hardware';
        
        const hardwareItems = [];
        
        if (result.cpu) {
            hardwareItems.push(`
                <div class="hardware-item">
                    <span class="hardware-icon">🖥️</span>
                    <span class="hardware-info">
                        <strong>${result.cpu.vendor} ${result.cpu.model}</strong>
                        <small>${result.cpu.cores} cores, ${result.cpu.threads} threads</small>
                    </span>
                </div>
            `);
        }

        if (result.memory) {
            hardwareItems.push(`
                <div class="hardware-item">
                    <span class="hardware-icon">💾</span>
                    <span class="hardware-info">
                        <strong>${result.memory.total_gb}GB ${result.memory.type}</strong>
                    </span>
                </div>
            `);
        }

        if (result.graphics && result.graphics.length > 0) {
            const gpu = result.graphics[0];
            hardwareItems.push(`
                <div class="hardware-item">
                    <span class="hardware-icon">🎮</span>
                    <span class="hardware-info">
                        <strong>${gpu.vendor} ${gpu.model}</strong>
                        <small>${gpu.driver} driver</small>
                    </span>
                </div>
            `);
        }

        hardware.innerHTML = hardwareItems.join('');

        // Compatibility notes
        const notes = document.createElement('div');
        notes.className = 'result-notes';
        if (result.compatibility?.notes) {
            notes.innerHTML = `<p class="compatibility-notes">${result.compatibility.notes}</p>`;
        }

        // Issues if any
        if (result.compatibility?.issues && result.compatibility.issues.length > 0) {
            const issues = document.createElement('div');
            issues.className = 'result-issues';
            issues.innerHTML = `
                <h4>Known Issues:</h4>
                <ul>
                    ${result.compatibility.issues.map(issue => `<li>${issue}</li>`).join('')}
                </ul>
            `;
            notes.appendChild(issues);
        }

        // Metadata
        const metadata = document.createElement('div');
        metadata.className = 'result-metadata';
        const reportDate = new Date(result.metadata.generated_at).toLocaleDateString();
        metadata.innerHTML = `
            <small>
                Generated: ${reportDate} | 
                Privacy: ${result.metadata.privacy_level} |
                <a href="#" class="view-details" data-result-id="${result.id}">View Details</a>
            </small>
        `;

        card.appendChild(header);
        card.appendChild(hardware);
        card.appendChild(notes);
        card.appendChild(metadata);

        // Add click handler for details
        const detailsLink = card.querySelector('.view-details');
        detailsLink.addEventListener('click', (e) => {
            e.preventDefault();
            this.showResultDetails(result);
        });

        return card;
    }

    /**
     * Get status badge HTML
     */
    getStatusBadge(status) {
        const badges = {
            excellent: '✅ Excellent',
            good: '👍 Good',
            partial: '⚠️ Partial',
            poor: '❌ Poor'
        };
        return badges[status] || '❓ Unknown';
    }

    /**
     * Show detailed view of a result
     */
    showResultDetails(result) {
        // Create modal or expand result with full details
        console.log('Showing details for:', result.id);
        
        // This would open a modal or navigate to a detail page
        // For now, just log the full result
        const modal = document.createElement('div');
        modal.className = 'result-modal';
        modal.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h2>Hardware Report Details</h2>
                    <button class="modal-close">&times;</button>
                </div>
                <div class="modal-body">
                    <pre>${JSON.stringify(result, null, 2)}</pre>
                </div>
            </div>
        `;

        document.body.appendChild(modal);

        // Close modal handlers
        modal.querySelector('.modal-close').addEventListener('click', () => {
            document.body.removeChild(modal);
        });

        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                document.body.removeChild(modal);
            }
        });
    }

    /**
     * Show search loading state
     */
    showSearchLoading() {
        const resultsContainer = document.querySelector('#search-results');
        if (resultsContainer) {
            resultsContainer.innerHTML = `
                <div class="search-loading">
                    <div class="loading-spinner"></div>
                    <p>Searching hardware database...</p>
                </div>
            `;
        }
    }

    /**
     * Hide search loading state
     */
    hideSearchLoading() {
        // Loading state is replaced by results or no-results message
    }

    /**
     * Show search error
     */
    showSearchError(message) {
        const resultsContainer = document.querySelector('#search-results');
        if (resultsContainer) {
            resultsContainer.innerHTML = `
                <div class="search-error">
                    <h3>Search Error</h3>
                    <p>${message}</p>
                    <button onclick="location.reload()">Retry</button>
                </div>
            `;
        }
    }

    /**
     * Show no results message
     */
    showNoResults() {
        const resultsContainer = document.querySelector('#search-results');
        if (resultsContainer) {
            resultsContainer.innerHTML = `
                <div class="no-results">
                    <h3>No hardware reports found</h3>
                    <p>Try adjusting your search query or filters.</p>
                    <div class="search-suggestions">
                        <p>Suggestions:</p>
                        <ul>
                            <li>Try searching for vendor names like "AMD", "Intel", "NVIDIA"</li>
                            <li>Search for hardware models like "RTX 4080", "Ryzen 7"</li>
                            <li>Look for distributions like "Ubuntu", "Fedora", "NixOS"</li>
                        </ul>
                    </div>
                </div>
            `;
        }
    }

    /**
     * Update search statistics
     */
    updateSearchStats(query, resultCount) {
        const statsElement = document.querySelector('#search-stats');
        if (statsElement) {
            if (query && query.trim()) {
                statsElement.innerHTML = `
                    Found ${resultCount} hardware reports for "${query}"
                `;
            } else {
                statsElement.innerHTML = `
                    Showing ${resultCount} recent hardware reports
                `;
            }
        }
    }

    /**
     * Clear search and reset filters
     */
    clearSearch() {
        const searchInput = document.querySelector('#search-input');
        if (searchInput) {
            searchInput.value = '';
        }

        // Reset all filters
        document.querySelectorAll('.filter-select').forEach(select => {
            select.value = '';
        });

        this.currentQuery = '';
        this.currentFilters = {};
        
        // Show recent results
        this.performSearch('');
    }
}

// Global instance
window.hardwareSearchUI = new HardwareSearchUI();
