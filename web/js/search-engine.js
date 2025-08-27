
// Search Engine for Hardware Compatibility Database
class SearchEngine {
    constructor() {
        this.hardwareData = [];
        this.searchIndex = null;
        this.isInitialized = false;
        this.loadData();
    }

    async loadData() {
        try {
            // Load hardware database
            const response = await fetch('./data/hardware-database.json');
            if (response.ok) {
                const data = await response.json();
                this.hardwareData = data.hardware || [];
                await this.initializeSearch();
                this.isInitialized = true;
                console.log('Search engine initialized with', this.hardwareData.length, 'hardware items');
            } else {
                console.error('Failed to load hardware database');
            }
        } catch (error) {
            console.error('Error loading hardware data:', error);
        }
    }

    async initializeSearch() {
        // Wait for FlexSearch to be available
        if (typeof FlexSearch === 'undefined') {
            console.warn('FlexSearch not loaded yet, retrying...');
            setTimeout(() => this.initializeSearch(), 100);
            return;
        }

        // Initialize FlexSearch index
        this.searchIndex = new FlexSearch.Document({
            tokenize: 'forward',
            cache: 100,
            document: {
                id: 'id',
                index: [
                    'name',
                    'manufacturer',
                    'model',
                    'category',
                    'pci_id',
                    'specifications.memory',
                    'specifications.cuda_cores',
                    'specifications.cores',
                    'specifications.threads',
                    'specifications.detection_method',
                    'specifications.interface',
                    'specifications.type',
                    'tested_distributions.nixos.driver',
                    'tested_distributions.debian.driver',
                    'tested_distributions.arch.driver',
                    'tested_distributions.fedora.driver'
                ]
            }
        });

        // Add all hardware to search index
        for (const hardware of this.hardwareData) {
            this.searchIndex.add(hardware);
        }
        
        console.log('FlexSearch index created with', this.hardwareData.length, 'items');
    }

    search(query, options = {}) {
        if (!this.isInitialized || !this.searchIndex || !query.trim()) {
            console.log('Search not ready or empty query:', { initialized: this.isInitialized, hasIndex: !!this.searchIndex, query });
            return [];
        }

        try {
            // Perform search
            const searchResults = this.searchIndex.search(query.trim(), {
                limit: options.limit || 50,
                suggest: true
            });

            // Extract IDs from search results
            const resultIds = new Set();
            if (Array.isArray(searchResults)) {
                searchResults.forEach(result => {
                    if (result && result.result) {
                        result.result.forEach(id => resultIds.add(id));
                    }
                });
            }

            // Get full hardware objects
            const results = this.hardwareData.filter(hardware => 
                resultIds.has(hardware.id)
            );

            // Apply category filter if specified
            if (options.category && options.category !== 'all') {
                const filteredResults = results.filter(hardware => 
                    hardware.category === options.category
                );
                console.log('Search results after category filter:', filteredResults.length);
                return filteredResults;
            }

            console.log('Search results for "' + query + '":', results.length, 'items');
            return results;
        } catch (error) {
            console.error('Search error:', error);
            return [];
        }
    }

    getCategories() {
        const categories = new Set();
        this.hardwareData.forEach(hardware => {
            if (hardware.category) {
                categories.add(hardware.category);
            }
        });
        return Array.from(categories).sort();
    }

    getManufacturers() {
        const manufacturers = new Set();
        this.hardwareData.forEach(hardware => {
            if (hardware.manufacturer) {
                manufacturers.add(hardware.manufacturer);
            }
        });
        return Array.from(manufacturers).sort();
    }

    getHardwareByCategory(category) {
        if (!category || category === 'all') {
            return this.hardwareData;
        }
        return this.hardwareData.filter(hardware => hardware.category === category);
    }

    getCompatibilityStatuses() {
        const statuses = new Set();
        this.hardwareData.forEach(hardware => {
            if (hardware.compatibility_status) {
                statuses.add(hardware.compatibility_status);
            }
        });
        return Array.from(statuses).sort();
    }

    filterByCompatibility(hardwareList, status) {
        if (!status || status === 'all') {
            return hardwareList;
        }
        return hardwareList.filter(hardware => hardware.compatibility_status === status);
    }

    getHardwareStats() {
        const stats = {
            totalHardware: this.hardwareData.length,
            categories: {},
            manufacturers: {},
            compatibility: {}
        };

        this.hardwareData.forEach(hardware => {
            // Count by category
            if (hardware.category) {
                stats.categories[hardware.category] = (stats.categories[hardware.category] || 0) + 1;
            }
            
            // Count by manufacturer  
            if (hardware.manufacturer) {
                stats.manufacturers[hardware.manufacturer] = (stats.manufacturers[hardware.manufacturer] || 0) + 1;
            }
            
            // Count by compatibility status
            if (hardware.compatibility_status) {
                stats.compatibility[hardware.compatibility_status] = (stats.compatibility[hardware.compatibility_status] || 0) + 1;
            }
        });

        return stats;
    }

    isReady() {
        return this.isInitialized && this.searchIndex !== null;
    }
}

window.SearchEngine = SearchEngine;
