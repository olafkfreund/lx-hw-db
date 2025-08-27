
// Search UI Components
class SearchUI {
    constructor() {
        this.searchEngine = new SearchEngine();
        this.currentResults = [];
        this.comparisonItems = [];
        this.initializeUI();
        this.setupEventListeners();
        this.setupAdvancedFilters();
    }

    initializeUI() {
        const searchForm = document.querySelector('#search-form');
        if (searchForm) {
            searchForm.addEventListener('submit', (e) => {
                e.preventDefault();
                this.performSearch();
            });
        }

        // Real-time search on input
        const searchInput = document.querySelector('#search-input');
        if (searchInput) {
            searchInput.addEventListener('input', this.debounce(() => {
                const query = searchInput.value.trim();
                if (query.length >= 2) {
                    this.performSearch();
                } else if (query.length === 0) {
                    this.clearResults();
                }
            }, 300));
        }
    }

    setupEventListeners() {
        // Category filter
        const categoryFilter = document.querySelector('#category-filter');
        if (categoryFilter) {
            categoryFilter.addEventListener('change', () => {
                this.performSearch();
            });
        }

        // Manufacturer filter
        const manufacturerFilter = document.querySelector('#manufacturer-filter');
        if (manufacturerFilter) {
            manufacturerFilter.addEventListener('change', () => {
                this.performSearch();
            });
        }

        // Compatibility filter
        const compatibilityFilter = document.querySelector('#compatibility-filter');
        if (compatibilityFilter) {
            compatibilityFilter.addEventListener('change', () => {
                this.performSearch();
            });
        }

        // Clear search button
        const clearButton = document.querySelector('#clear-search');
        if (clearButton) {
            clearButton.addEventListener('click', () => {
                this.clearSearch();
            });
        }

        // Advanced filters toggle
        const advancedToggle = document.querySelector('#advanced-filters-toggle');
        if (advancedToggle) {
            advancedToggle.addEventListener('click', () => {
                this.toggleAdvancedFilters();
            });
        }

        // Advanced specification filters
        const specificationFilters = ['#interface-filter', '#memory-filter', '#driver-filter'];
        specificationFilters.forEach(selector => {
            const filter = document.querySelector(selector);
            if (filter) {
                filter.addEventListener('change', () => {
                    this.performSearch();
                });
            }
        });

        // Comparison controls
        const clearComparison = document.querySelector('#clear-comparison');
        if (clearComparison) {
            clearComparison.addEventListener('click', () => {
                this.clearComparison();
            });
        }

        const compareSelected = document.querySelector('#compare-selected');
        if (compareSelected) {
            compareSelected.addEventListener('click', () => {
                this.showComparisonModal();
            });
        }
    }

    performSearch() {
        const query = document.querySelector('#search-input')?.value.trim();
        const category = document.querySelector('#category-filter')?.value || 'all';
        const manufacturer = document.querySelector('#manufacturer-filter')?.value || 'all';
        const compatibility = document.querySelector('#compatibility-filter')?.value || 'all';
        
        // Advanced specification filters
        const interface_filter = document.querySelector('#interface-filter')?.value || '';
        const memory_filter = document.querySelector('#memory-filter')?.value || '';
        const driver_filter = document.querySelector('#driver-filter')?.value || '';

        if (!query || query.length < 1) {
            this.clearResults();
            return;
        }

        // Show loading state
        this.showLoading();

        // Wait for search engine to be ready
        if (!this.searchEngine.isReady()) {
            console.log('Search engine not ready, waiting...');
            setTimeout(() => this.performSearch(), 500);
            return;
        }

        try {
            const options = {
                category: category !== 'all' ? category : null,
                limit: 50
            };

            let results = this.searchEngine.search(query, options);

            // Apply manufacturer filter
            if (manufacturer && manufacturer !== 'all') {
                results = results.filter(hardware => 
                    hardware.manufacturer.toLowerCase() === manufacturer.toLowerCase()
                );
            }

            // Apply compatibility filter
            if (compatibility && compatibility !== 'all') {
                results = results.filter(hardware => 
                    hardware.compatibility_status === compatibility
                );
            }

            // Apply advanced specification filters
            if (interface_filter) {
                results = results.filter(hardware => {
                    return hardware.specifications && 
                           hardware.specifications.interface && 
                           hardware.specifications.interface.toLowerCase().includes(interface_filter.toLowerCase());
                });
            }

            if (memory_filter) {
                results = results.filter(hardware => {
                    const memoryValue = this.extractMemoryValue(hardware);
                    const requiredMemory = this.extractMemoryFromFilter(memory_filter);
                    return memoryValue >= requiredMemory;
                });
            }

            if (driver_filter) {
                results = results.filter(hardware => {
                    return this.matchesDriverFilter(hardware, driver_filter);
                });
            }

            this.currentResults = results;
            this.displayResults(results, query);
        } catch (error) {
            console.error('Search error:', error);
            this.showError('Search failed. Please try again.');
        }
    }

    displayResults(results, query) {
        const resultsContainer = document.querySelector('#search-results');
        const resultsCount = document.querySelector('#results-count');
        
        if (!resultsContainer) {
            console.error('Search results container not found');
            return;
        }

        this.hideLoading();

        // Update results count
        if (resultsCount) {
            resultsCount.textContent = `${results.length} result${results.length !== 1 ? 's' : ''} found`;
            if (query) {
                resultsCount.textContent += ` for "${query}"`;
            }
        }

        if (results.length === 0) {
            resultsContainer.innerHTML = `
                <div class="no-results">
                    <div class="no-results-icon">🔍</div>
                    <h3>No hardware found</h3>
                    <p>Try adjusting your search terms or filters.</p>
                </div>
            `;
            return;
        }

        // Generate results HTML
        const resultsHTML = results.map(hardware => this.createHardwareCard(hardware)).join('');
        resultsContainer.innerHTML = resultsHTML;

        // Add event listeners for "View Details" buttons
        resultsContainer.querySelectorAll('.view-details-btn').forEach(button => {
            button.addEventListener('click', (e) => {
                e.preventDefault();
                const hardwareId = button.dataset.hardwareId;
                if (hardwareId && window.hardwareDetails) {
                    window.hardwareDetails.showDetails(hardwareId);
                }
            });
        });

        // Add keyboard navigation for hardware cards
        resultsContainer.querySelectorAll('.hardware-card').forEach(card => {
            card.addEventListener('keydown', (e) => {
                if (e.key === 'Enter' || e.key === ' ') {
                    e.preventDefault();
                    const viewDetailsBtn = card.querySelector('.view-details-btn');
                    if (viewDetailsBtn) {
                        viewDetailsBtn.click();
                    }
                }
            });
        });
    }

    createHardwareCard(hardware) {
        const compatibilityBadge = this.getCompatibilityBadge(hardware.compatibility_status);
        const categoryIcon = this.getCategoryIcon(hardware.category);
        
        return `
            <article class="card card--hardware card--interactive" data-hardware-id="${hardware.id}" role="article" tabindex="0">
                <div class="card__header">
                    <div class="hardware-header">
                        <div class="hardware-icon" aria-hidden="true">${categoryIcon}</div>
                        <div class="hardware-info">
                            <h3 class="card__title hardware-name">${hardware.name}</h3>
                            <p class="card__subtitle hardware-meta">${hardware.manufacturer} • ${hardware.category}</p>
                        </div>
                        <div class="compatibility-badge ${hardware.compatibility_status}" 
                             role="status" 
                             aria-label="Compatibility status: ${hardware.compatibility_status}">
                            ${compatibilityBadge}
                        </div>
                    </div>
                </div>
                
                <div class="card__body hardware-details">
                    ${this.getHardwareSpecs(hardware)}
                </div>
                
                <div class="card__footer hardware-footer">
                    <div class="hardware-stats" role="group" aria-label="Hardware statistics">
                        <span class="stat" aria-label="Community rating ${hardware.community_rating || 'Not available'}">
                            ⭐ ${hardware.community_rating || 'N/A'}
                        </span>
                        <span class="stat" aria-label="${hardware.reports_count || 0} compatibility reports">
                            📊 ${hardware.reports_count || 0} reports
                        </span>
                        <span class="stat" aria-label="Last tested ${hardware.last_tested || 'Unknown'}">
                            📅 ${hardware.last_tested || 'Unknown'}
                        </span>
                    </div>
                    <button class="btn btn-primary view-details-btn" 
                            data-hardware-id="${hardware.id}"
                            aria-label="View detailed information for ${hardware.name}">
                        View Details
                    </button>
                    <button class="btn btn-secondary add-to-comparison" 
                            data-hardware-id="${hardware.id}"
                            aria-label="Add ${hardware.name} to comparison">
                        + Compare
                    </button>
                </div>
            </article>
        `;
    }

    getCompatibilityBadge(status) {
        const badges = {
            'full': '✅ Full',
            'partial': '⚠️ Partial',
            'limited': '⛔ Limited',
            'none': '❌ None'
        };
        return badges[status] || '❓ Unknown';
    }

    getCategoryIcon(category) {
        const icons = {
            'gpu': '🎮',
            'cpu': '🔧',
            'motherboard': '🖥️',
            'network': '🌐',
            'storage': '💾',
            'audio': '🔊',
            'usb': '🔌',
            'bluetooth': '📶'
        };
        return icons[category] || '🔧';
    }

    getHardwareSpecs(hardware) {
        if (!hardware.specifications) return '';
        
        const specs = [];
        const specMap = {
            'memory': 'Memory',
            'cuda_cores': 'CUDA Cores',
            'cores': 'Cores',
            'threads': 'Threads',
            'base_clock': 'Base Clock',
            'boost_clock': 'Boost Clock',
            'interface': 'Interface'
        };

        Object.entries(hardware.specifications).forEach(([key, value]) => {
            if (specMap[key] && value) {
                specs.push(`<span class="spec"><strong>${specMap[key]}:</strong> ${value}</span>`);
            }
        });

        return specs.length > 0 ? `<div class="specs">${specs.join('')}</div>` : '';
    }

    showLoading() {
        const resultsContainer = document.querySelector('#search-results');
        if (resultsContainer) {
            resultsContainer.innerHTML = `
                <div class="loading-state">
                    <div class="loading-spinner"></div>
                    <p>Searching hardware database...</p>
                </div>
            `;
        }
    }

    hideLoading() {
        // Loading state is replaced by displayResults
    }

    showError(message) {
        const resultsContainer = document.querySelector('#search-results');
        if (resultsContainer) {
            resultsContainer.innerHTML = `
                <div class="error-state">
                    <div class="error-icon">⚠️</div>
                    <h3>Search Error</h3>
                    <p>${message}</p>
                </div>
            `;
        }
    }

    clearResults() {
        const resultsContainer = document.querySelector('#search-results');
        const resultsCount = document.querySelector('#results-count');
        
        if (resultsContainer) {
            resultsContainer.innerHTML = '';
        }
        
        if (resultsCount) {
            resultsCount.textContent = '';
        }
        
        this.currentResults = [];
    }

    clearSearch() {
        const searchInput = document.querySelector('#search-input');
        const categoryFilter = document.querySelector('#category-filter');
        const manufacturerFilter = document.querySelector('#manufacturer-filter');
        const compatibilityFilter = document.querySelector('#compatibility-filter');
        
        if (searchInput) searchInput.value = '';
        if (categoryFilter) categoryFilter.value = 'all';
        if (manufacturerFilter) manufacturerFilter.value = 'all';
        if (compatibilityFilter) compatibilityFilter.value = 'all';
        
        this.clearResults();
    }

    // Utility function for debouncing input
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

    // Public method to get current results for other components
    getCurrentResults() {
        return this.currentResults;
    }

    // Public method to populate filter dropdowns
    populateFilters() {
        if (!this.searchEngine.isReady()) {
            setTimeout(() => this.populateFilters(), 500);
            return;
        }

        // Populate category filter
        const categoryFilter = document.querySelector('#category-filter');
        if (categoryFilter) {
            const categories = this.searchEngine.getCategories();
            const currentValue = categoryFilter.value;
            
            categoryFilter.innerHTML = '<option value="all">All Categories</option>';
            categories.forEach(category => {
                const option = document.createElement('option');
                option.value = category;
                option.textContent = category.charAt(0).toUpperCase() + category.slice(1);
                if (category === currentValue) option.selected = true;
                categoryFilter.appendChild(option);
            });
        }

        // Populate manufacturer filter
        const manufacturerFilter = document.querySelector('#manufacturer-filter');
        if (manufacturerFilter) {
            const manufacturers = this.searchEngine.getManufacturers();
            const currentValue = manufacturerFilter.value;
            
            manufacturerFilter.innerHTML = '<option value="all">All Manufacturers</option>';
            manufacturers.forEach(manufacturer => {
                const option = document.createElement('option');
                option.value = manufacturer;
                option.textContent = manufacturer;
                if (manufacturer === currentValue) option.selected = true;
                manufacturerFilter.appendChild(option);
            });
        }
    }

    // Advanced filters functionality
    setupAdvancedFilters() {
        // Add comparison checkboxes to hardware cards when they're displayed
        document.addEventListener('click', (event) => {
            if (event.target.classList.contains('add-to-comparison')) {
                const hardwareId = event.target.dataset.hardwareId;
                this.addToComparison(hardwareId);
            }
        });
    }

    toggleAdvancedFilters() {
        const advancedFilters = document.querySelector('#advanced-filters');
        const toggleButton = document.querySelector('#advanced-filters-toggle');
        
        if (advancedFilters.style.display === 'none' || advancedFilters.style.display === '') {
            advancedFilters.style.display = 'block';
            toggleButton.classList.add('active');
            toggleButton.textContent = '🔧 Hide Advanced';
        } else {
            advancedFilters.style.display = 'none';
            toggleButton.classList.remove('active');
            toggleButton.textContent = '🔧 Advanced';
        }
    }

    // Helper methods for advanced filtering
    extractMemoryValue(hardware) {
        if (hardware.specifications && hardware.specifications.memory) {
            const memory = hardware.specifications.memory.toString();
            const match = memory.match(/(\d+)\s*GB/i);
            return match ? parseInt(match[1]) : 0;
        }
        return 0;
    }

    extractMemoryFromFilter(filter) {
        const match = filter.match(/(\d+)GB/i);
        return match ? parseInt(match[1]) : 0;
    }

    matchesDriverFilter(hardware, filter) {
        // Check various places for driver information
        const driverSources = [
            hardware.specifications?.driver,
            hardware.tested_distributions?.nixos?.driver,
            hardware.tested_distributions?.debian?.driver,
            hardware.tested_distributions?.arch?.driver,
            hardware.tested_distributions?.fedora?.driver,
        ];

        for (const driver of driverSources) {
            if (driver) {
                switch (filter) {
                    case 'built-in':
                        return driver.toLowerCase().includes('built') || driver.toLowerCase().includes('kernel');
                    case 'proprietary':
                        return driver.toLowerCase().includes('proprietary') || driver.toLowerCase().includes('nvidia');
                    case 'open-source':
                        return driver.toLowerCase().includes('open') || driver.toLowerCase().includes('libre');
                    default:
                        return true;
                }
            }
        }
        return false;
    }

    // Hardware comparison functionality
    addToComparison(hardwareId) {
        const hardware = this.currentResults.find(h => h.id === hardwareId);
        if (!hardware) return;

        // Check if already in comparison
        if (this.comparisonItems.find(item => item.id === hardwareId)) {
            return;
        }

        // Limit to 4 items for comparison
        if (this.comparisonItems.length >= 4) {
            alert('Maximum 4 items can be compared at once. Remove some items first.');
            return;
        }

        this.comparisonItems.push(hardware);
        this.updateComparisonUI();
    }

    removeFromComparison(hardwareId) {
        this.comparisonItems = this.comparisonItems.filter(item => item.id !== hardwareId);
        this.updateComparisonUI();
    }

    clearComparison() {
        this.comparisonItems = [];
        this.updateComparisonUI();
    }

    updateComparisonUI() {
        const comparisonItems = document.querySelector('#comparison-items');
        const compareButton = document.querySelector('#compare-selected');
        
        if (!comparisonItems || !compareButton) return;

        if (this.comparisonItems.length === 0) {
            comparisonItems.innerHTML = '<p style="text-align: center; color: var(--fg3); margin: 0;">Add hardware to comparison by clicking the "+" button on hardware cards</p>';
            compareButton.disabled = true;
        } else {
            comparisonItems.innerHTML = this.comparisonItems.map(hardware => `
                <div class="comparison-item">
                    <span>${hardware.name}</span>
                    <button class="comparison-item-remove" onclick="window.searchUI.removeFromComparison('${hardware.id}')">×</button>
                </div>
            `).join('');
            compareButton.disabled = false;
        }
    }

    showComparisonModal() {
        if (this.comparisonItems.length < 2) {
            alert('Please select at least 2 items to compare.');
            return;
        }

        // Create and show comparison modal
        this.createComparisonModal();
    }

    createComparisonModal() {
        // Create modal overlay
        const modalOverlay = document.createElement('div');
        modalOverlay.className = 'modal-overlay comparison-modal-overlay';
        modalOverlay.innerHTML = `
            <div class="modal comparison-modal">
                <div class="modal-header">
                    <h2>Hardware Comparison</h2>
                    <button class="modal-close" onclick="this.closest('.modal-overlay').remove()">×</button>
                </div>
                <div class="modal-content">
                    ${this.generateComparisonTable()}
                </div>
            </div>
        `;

        document.body.appendChild(modalOverlay);

        // Add click outside to close
        modalOverlay.addEventListener('click', (e) => {
            if (e.target === modalOverlay) {
                modalOverlay.remove();
            }
        });
    }

    generateComparisonTable() {
        const headers = ['Property', ...this.comparisonItems.map(item => item.name)];
        const properties = [
            { key: 'manufacturer', label: 'Manufacturer' },
            { key: 'category', label: 'Category' },
            { key: 'compatibility_status', label: 'Compatibility' },
            { key: 'pci_id', label: 'PCI ID' },
            { key: 'specifications.memory', label: 'Memory' },
            { key: 'specifications.interface', label: 'Interface' },
            { key: 'specifications.detection_method', label: 'Detection Method' },
            { key: 'last_tested', label: 'Last Tested' }
        ];

        const tableHTML = `
            <table class="comparison-table">
                <thead>
                    <tr>
                        ${headers.map(header => `<th>${header}</th>`).join('')}
                    </tr>
                </thead>
                <tbody>
                    ${properties.map(prop => `
                        <tr>
                            <td class="comparison-property">${prop.label}</td>
                            ${this.comparisonItems.map(item => {
                                const value = this.getNestedProperty(item, prop.key);
                                return `<td>${value || 'N/A'}</td>`;
                            }).join('')}
                        </tr>
                    `).join('')}
                </tbody>
            </table>
        `;

        return tableHTML;
    }

    getNestedProperty(obj, path) {
        return path.split('.').reduce((current, key) => {
            return current && current[key] !== undefined ? current[key] : null;
        }, obj);
    }
}

window.SearchUI = SearchUI;

// Initialize search UI when DOM is loaded
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        window.searchUI = new SearchUI();
        // Populate filters after a short delay to ensure search engine is ready
        setTimeout(() => {
            if (window.searchUI) {
                window.searchUI.populateFilters();
            }
        }, 1000);
    });
} else {
    window.searchUI = new SearchUI();
    setTimeout(() => {
        if (window.searchUI) {
            window.searchUI.populateFilters();
        }
    }, 1000);
}
