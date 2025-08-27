
/**
 * Hardware Search Engine using FlexSearch
 * Provides fast, flexible searching across hardware database
 */

class HardwareSearchEngine {
    constructor() {
        this.index = null;
        this.documents = [];
        this.isInitialized = false;
        this.categories = {
            cpu: 'CPU',
            gpu: 'Graphics',
            memory: 'Memory', 
            storage: 'Storage',
            network: 'Network',
            audio: 'Audio',
            system: 'System'
        };
    }

    /**
     * Initialize the search index with hardware data
     */
    async initialize() {
        try {
            console.log('Initializing hardware search engine...');
            
            // Create FlexSearch index with optimal configuration
            this.index = new FlexSearch.Document({
                document: {
                    id: 'id',
                    index: [
                        // System fields
                        'system.distribution',
                        'system.kernel_version',
                        'system.architecture',
                        
                        // CPU fields  
                        'cpu.vendor',
                        'cpu.model',
                        
                        // Memory fields
                        'memory.type',
                        
                        // Graphics fields
                        'graphics:vendor',
                        'graphics:model',
                        'graphics:driver',
                        
                        // Network fields
                        'network:vendor', 
                        'network:model',
                        'network:driver',
                        
                        // Storage fields
                        'storage:vendor',
                        'storage:model',
                        'storage:interface',
                        
                        // Audio fields
                        'audio:vendor',
                        'audio:model',
                        'audio:driver',
                        
                        // Compatibility
                        'compatibility.overall_status',
                        'compatibility.notes'
                    ]
                },
                tokenize: 'forward',
                resolution: 9,
                depth: 4,
                bidirectional: true
            });

            // Load multiple data sources for enhanced search
            await Promise.all([
                this.loadHardwareData(),
                this.loadSearchTerms(),
                this.loadVendorIndex(),
                this.loadComponentIndex()
            ]);

            console.log(`Loaded ${this.documents.length} hardware reports`);

            // Index all documents
            for (const doc of this.documents) {
                await this.indexDocument(doc);
            }

            this.isInitialized = true;
            console.log('Hardware search engine initialized successfully');
            
            // Trigger ready event
            document.dispatchEvent(new CustomEvent('searchEngineReady', {
                detail: { engine: this }
            }));
            
        } catch (error) {
            console.error('Failed to initialize search engine:', error);
            throw error;
        }
    }

    /**
     * Load main hardware data
     */
    async loadHardwareData() {
        try {
            const response = await fetch('data/hardware-index.json');
            if (!response.ok) {
                throw new Error(`Failed to load hardware data: ${response.status}`);
            }
            this.documents = await response.json();
        } catch (error) {
            console.warn('Could not load hardware-index.json, falling back to hardware-database.json');
            const response = await fetch('data/hardware-database.json');
            if (!response.ok) {
                throw new Error(`Failed to load fallback hardware data: ${response.status}`);
            }
            this.documents = await response.json();
        }
    }

    /**
     * Load search terms index for enhanced search
     */
    async loadSearchTerms() {
        try {
            const response = await fetch('indices/search-terms.json');
            if (response.ok) {
                this.searchTerms = await response.json();
                console.log(`Loaded ${Object.keys(this.searchTerms).length} search terms`);
            }
        } catch (error) {
            console.warn('Could not load search terms index:', error);
            this.searchTerms = {};
        }
    }

    /**
     * Load vendor index for vendor-specific search
     */
    async loadVendorIndex() {
        try {
            const response = await fetch('indices/by-vendor.json');
            if (response.ok) {
                this.vendorIndex = await response.json();
                console.log(`Loaded ${Object.keys(this.vendorIndex).length} vendors in index`);
            }
        } catch (error) {
            console.warn('Could not load vendor index:', error);
            this.vendorIndex = {};
        }
    }

    /**
     * Load component index for component-specific search
     */
    async loadComponentIndex() {
        try {
            const response = await fetch('indices/by-component.json');
            if (response.ok) {
                this.componentIndex = await response.json();
                console.log(`Loaded ${Object.keys(this.componentIndex).length} component types in index`);
            }
        } catch (error) {
            console.warn('Could not load component index:', error);
            this.componentIndex = {};
        }
    }

    /**
     * Index a single hardware document
     */
    async indexDocument(doc) {
        // Flatten arrays for indexing
        const flattened = {
            id: doc.id,
            system: doc.system,
            cpu: doc.cpu,
            memory: doc.memory,
            compatibility: doc.compatibility,
            
            // Flatten arrays
            graphics: doc.graphics?.map(g => ({ 
                vendor: g.vendor, 
                model: g.model, 
                driver: g.driver 
            })) || [],
            
            network: doc.network?.map(n => ({ 
                vendor: n.vendor, 
                model: n.model, 
                driver: n.driver 
            })) || [],
            
            storage: doc.storage?.map(s => ({ 
                vendor: s.vendor, 
                model: s.model, 
                interface: s.interface 
            })) || [],
            
            audio: doc.audio?.map(a => ({ 
                vendor: a.vendor, 
                model: a.model, 
                driver: a.driver 
            })) || []
        };

        this.index.add(flattened);
    }

    /**
     * Search hardware database with enhanced indexing
     * @param {string} query - Search query
     * @param {Object} filters - Search filters
     * @returns {Array} Search results
     */
    async search(query, filters = {}) {
        if (!this.isInitialized) {
            console.warn('Search engine not initialized');
            return [];
        }

        if (!query || query.trim().length < 2) {
            return this.documents.slice(0, 20); // Return recent documents
        }

        try {
            const queryLower = query.trim().toLowerCase();
            
            // First, try to find exact matches in search terms index
            const termMatches = this.searchInTermsIndex(queryLower);
            
            // Then perform FlexSearch for fuzzy matching
            const flexSearchResults = this.index.search(query.trim(), {
                limit: 50,
                enrich: true
            });

            // Combine and deduplicate results
            const allResultIds = new Set();
            const allResults = [];

            // Add term matches (higher priority)
            termMatches.forEach(docId => {
                allResultIds.add(docId);
                allResults.push({ id: docId, source: 'terms' });
            });

            // Add FlexSearch results
            for (const fieldResult of flexSearchResults) {
                for (const result of fieldResult.result) {
                    if (!allResultIds.has(result.id)) {
                        allResultIds.add(result.id);
                        allResults.push({ id: result.id, source: 'flexsearch' });
                    }
                }
            }

            // Get full document data for results
            const enrichedResults = allResults
                .map(result => {
                    const doc = this.documents.find(d => d.id === result.id);
                    if (!doc) return null;
                    return {
                        ...doc,
                        _searchSource: result.source,
                        _searchScore: this.calculateRelevanceScore(doc, query)
                    };
                })
                .filter(Boolean);

            // Apply filters
            let filteredResults = enrichedResults;
            
            if (filters.category) {
                filteredResults = this.filterByCategory(filteredResults, filters.category);
            }
            
            if (filters.vendor) {
                filteredResults = this.filterByVendor(filteredResults, filters.vendor);
            }
            
            if (filters.compatibility) {
                filteredResults = this.filterByCompatibility(filteredResults, filters.compatibility);
            }
            
            if (filters.architecture) {
                filteredResults = filteredResults.filter(doc => 
                    doc.system.architecture === filters.architecture
                );
            }

            // Sort by relevance score (term matches get priority boost)
            filteredResults.sort((a, b) => {
                const scoreA = a._searchScore + (a._searchSource === 'terms' ? 10 : 0);
                const scoreB = b._searchScore + (b._searchSource === 'terms' ? 10 : 0);
                return scoreB - scoreA;
            });

            console.log(`Search for "${query}" returned ${filteredResults.length} results`);
            return filteredResults;

        } catch (error) {
            console.error('Search error:', error);
            return [];
        }
    }

    /**
     * Search in the terms index for exact matches
     */
    searchInTermsIndex(query) {
        if (!this.searchTerms) return [];
        
        const matches = [];
        const queryTerms = query.split(/\s+/);
        
        // Check for exact term matches
        queryTerms.forEach(term => {
            if (this.searchTerms[term]) {
                matches.push(...this.searchTerms[term]);
            }
        });
        
        // Check for partial matches in term keys
        Object.keys(this.searchTerms).forEach(term => {
            if (term.toLowerCase().includes(query) || query.includes(term.toLowerCase())) {
                matches.push(...this.searchTerms[term]);
            }
        });
        
        // Return unique document IDs
        return [...new Set(matches)];
    }

    /**
     * Calculate relevance score for search results
     */
    calculateRelevanceScore(doc, query) {
        let score = 0;
        const queryLower = query.toLowerCase();
        const queryTerms = queryLower.split(/\s+/);

        // CPU matches (high weight)
        if (doc.cpu) {
            if (doc.cpu.vendor.toLowerCase().includes(queryLower)) score += 10;
            if (doc.cpu.model.toLowerCase().includes(queryLower)) score += 8;
        }

        // GPU matches (high weight)
        doc.graphics?.forEach(gpu => {
            if (gpu.vendor.toLowerCase().includes(queryLower)) score += 8;
            if (gpu.model.toLowerCase().includes(queryLower)) score += 6;
        });

        // System matches (medium weight)
        if (doc.system.distribution?.toLowerCase().includes(queryLower)) score += 5;
        if (doc.system.kernel_version.includes(queryLower)) score += 4;

        // Other hardware matches (medium weight)
        doc.network?.forEach(net => {
            if (net.vendor.toLowerCase().includes(queryLower)) score += 4;
            if (net.model.toLowerCase().includes(queryLower)) score += 3;
        });

        // Term matching bonus
        queryTerms.forEach(term => {
            if (this.documentContainsTerm(doc, term)) {
                score += 2;
            }
        });

        return score;
    }

    /**
     * Check if document contains search term
     */
    documentContainsTerm(doc, term) {
        const searchText = JSON.stringify(doc).toLowerCase();
        return searchText.includes(term);
    }

    /**
     * Filter results by hardware category
     */
    filterByCategory(results, category) {
        switch (category) {
            case 'cpu':
                return results.filter(doc => doc.cpu);
            case 'gpu':
                return results.filter(doc => doc.graphics && doc.graphics.length > 0);
            case 'memory':
                return results.filter(doc => doc.memory);
            case 'storage':
                return results.filter(doc => doc.storage && doc.storage.length > 0);
            case 'network':
                return results.filter(doc => doc.network && doc.network.length > 0);
            case 'audio':
                return results.filter(doc => doc.audio && doc.audio.length > 0);
            default:
                return results;
        }
    }

    /**
     * Filter results by vendor
     */
    filterByVendor(results, vendor) {
        return results.filter(doc => {
            return [
                doc.cpu?.vendor,
                ...doc.graphics?.map(g => g.vendor) || [],
                ...doc.network?.map(n => n.vendor) || [],
                ...doc.storage?.map(s => s.vendor) || [],
                ...doc.audio?.map(a => a.vendor) || []
            ].some(v => v?.toLowerCase().includes(vendor.toLowerCase()));
        });
    }

    /**
     * Filter results by compatibility status
     */
    filterByCompatibility(results, compatibility) {
        return results.filter(doc => 
            doc.compatibility?.overall_status === compatibility
        );
    }

    /**
     * Get search suggestions based on partial input
     */
    getSuggestions(partialQuery, limit = 5) {
        if (!partialQuery || partialQuery.length < 2) return [];

        const suggestions = [];
        const queryLower = partialQuery.toLowerCase();

        // Add suggestions from search terms index (fast lookup)
        if (this.searchTerms) {
            Object.keys(this.searchTerms).forEach(term => {
                if (term.toLowerCase().includes(queryLower)) {
                    suggestions.push({
                        text: term,
                        type: 'term',
                        count: this.searchTerms[term].length
                    });
                }
            });
        }

        // Add vendor suggestions from vendor index
        if (this.vendorIndex) {
            Object.keys(this.vendorIndex).forEach(vendorId => {
                const vendor = this.vendorIndex[vendorId];
                if (vendorId.toLowerCase().includes(queryLower)) {
                    suggestions.push({
                        text: vendorId,
                        type: 'vendor',
                        count: vendor.total_reports,
                        details: `${vendor.total_reports} reports`
                    });
                }
            });
        }

        // Add component type suggestions
        if (this.componentIndex) {
            Object.keys(this.componentIndex).forEach(componentType => {
                const component = this.componentIndex[componentType];
                if (componentType.toLowerCase().includes(queryLower)) {
                    suggestions.push({
                        text: componentType,
                        type: 'component',
                        count: component.total_reports,
                        details: `${component.total_reports} reports`
                    });
                }
            });
        }

        // Add model suggestions from documents (fallback)
        this.documents.forEach(doc => {
            [
                doc.cpu?.model,
                ...doc.graphics?.map(g => g.model) || []
            ].forEach(model => {
                if (model && model.toLowerCase().includes(queryLower)) {
                    suggestions.push({
                        text: model,
                        type: 'model',
                        count: 1
                    });
                }
            });
        });

        // Sort by relevance and count
        suggestions.sort((a, b) => {
            // Exact matches first
            const aExact = a.text.toLowerCase() === queryLower ? 1 : 0;
            const bExact = b.text.toLowerCase() === queryLower ? 1 : 0;
            if (aExact !== bExact) return bExact - aExact;

            // Then by count
            return (b.count || 0) - (a.count || 0);
        });

        // Remove duplicates and limit
        const unique = [];
        const seen = new Set();
        for (const suggestion of suggestions) {
            if (!seen.has(suggestion.text) && unique.length < limit) {
                seen.add(suggestion.text);
                unique.push(suggestion);
            }
        }

        return unique;
    }

    /**
     * Get hardware statistics for dashboard
     */
    getStatistics() {
        const stats = {
            totalReports: this.documents.length,
            distributions: {},
            architectures: {},
            vendors: { cpu: {}, gpu: {} },
            compatibility: {}
        };

        this.documents.forEach(doc => {
            // Count distributions
            const distro = doc.system.distribution;
            stats.distributions[distro] = (stats.distributions[distro] || 0) + 1;

            // Count architectures  
            const arch = doc.system.architecture;
            stats.architectures[arch] = (stats.architectures[arch] || 0) + 1;

            // Count CPU vendors
            if (doc.cpu?.vendor) {
                stats.vendors.cpu[doc.cpu.vendor] = (stats.vendors.cpu[doc.cpu.vendor] || 0) + 1;
            }

            // Count GPU vendors
            doc.graphics?.forEach(gpu => {
                if (gpu.vendor) {
                    stats.vendors.gpu[gpu.vendor] = (stats.vendors.gpu[gpu.vendor] || 0) + 1;
                }
            });

            // Count compatibility status
            const status = doc.compatibility?.overall_status;
            if (status) {
                stats.compatibility[status] = (stats.compatibility[status] || 0) + 1;
            }
        });

        return stats;
    }

    /**
     * Get unique values for filter options using indices for better performance
     */
    getFilterOptions() {
        // Use indices when available for better performance
        if (this.vendorIndex && this.componentIndex) {
            const vendors = Object.keys(this.vendorIndex).sort();
            const componentTypes = Object.keys(this.componentIndex).sort();
            
            // Fallback to documents for other options
            const architectures = new Set();
            const distributions = new Set();
            const compatibility = new Set();
            
            this.documents.forEach(doc => {
                architectures.add(doc.system.architecture);
                distributions.add(doc.system.distribution);
                if (doc.compatibility?.overall_status) {
                    compatibility.add(doc.compatibility.overall_status);
                }
            });

            return {
                vendors,
                componentTypes,
                architectures: Array.from(architectures).sort(),
                distributions: Array.from(distributions).sort(),
                compatibility: Array.from(compatibility).sort()
            };
        }

        // Fallback to document scanning
        const options = {
            vendors: new Set(),
            componentTypes: new Set(),
            architectures: new Set(), 
            distributions: new Set(),
            compatibility: new Set()
        };

        this.documents.forEach(doc => {
            // Collect vendors
            [
                doc.cpu?.vendor,
                ...doc.graphics?.map(g => g.vendor) || [],
                ...doc.network?.map(n => n.vendor) || [],
                ...doc.storage?.map(s => s.vendor) || []
            ].forEach(vendor => {
                if (vendor) options.vendors.add(vendor);
            });

            // Add component types
            if (doc.cpu) options.componentTypes.add('CPU');
            if (doc.graphics?.length) options.componentTypes.add('Graphics');
            if (doc.memory) options.componentTypes.add('Memory');
            if (doc.storage?.length) options.componentTypes.add('Storage');
            if (doc.network?.length) options.componentTypes.add('Network');
            if (doc.audio?.length) options.componentTypes.add('Audio');

            options.architectures.add(doc.system.architecture);
            options.distributions.add(doc.system.distribution);
            
            if (doc.compatibility?.overall_status) {
                options.compatibility.add(doc.compatibility.overall_status);
            }
        });

        return {
            vendors: Array.from(options.vendors).sort(),
            componentTypes: Array.from(options.componentTypes).sort(),
            architectures: Array.from(options.architectures).sort(),
            distributions: Array.from(options.distributions).sort(),
            compatibility: Array.from(options.compatibility).sort()
        };
    }

    /**
     * Get vendor details from vendor index
     */
    getVendorDetails(vendorId) {
        return this.vendorIndex?.[vendorId] || null;
    }

    /**
     * Get component details from component index
     */
    getComponentDetails(componentType) {
        return this.componentIndex?.[componentType] || null;
    }
}

// Global instance
window.hardwareSearch = new HardwareSearchEngine();
