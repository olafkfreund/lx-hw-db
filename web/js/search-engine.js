/**
 * Linux Hardware Compatibility Database - Client-Side Search Engine
 * 
 * A comprehensive search system that works with pre-built JSON indices
 * to provide fast, client-side hardware compatibility searching.
 */

class HardwareSearchEngine {
    constructor(options = {}) {
        this.baseUrl = options.baseUrl || './';
        this.indices = {};
        this.loadedIndices = new Set();
        this.cache = new Map();
        this.isLoading = false;
        
        // Search configuration
        this.config = {
            maxResults: options.maxResults || 50,
            fuzzyMatchThreshold: options.fuzzyMatchThreshold || 0.6,
            cacheTimeout: options.cacheTimeout || 300000, // 5 minutes
            enableFuzzySearch: options.enableFuzzySearch !== false,
            enableTypeahead: options.enableTypeahead !== false,
        };
        
        // Initialize event system
        this.events = new EventTarget();
        
        // Pre-load critical indices
        this.preloadCriticalIndices();
    }
    
    /**
     * Pre-load the most commonly used indices
     */
    async preloadCriticalIndices() {
        try {
            await Promise.all([
                this.loadIndex('vendors'),
                this.loadIndex('search-terms'),
                this.loadIndex('statistics')
            ]);
        } catch (error) {
            console.warn('Failed to preload some indices:', error);
        }
    }
    
    /**
     * Load a specific index from the API
     */
    async loadIndex(indexName) {
        if (this.loadedIndices.has(indexName)) {
            return this.indices[indexName];
        }
        
        const cacheKey = `index_${indexName}`;
        const cached = this.getFromCache(cacheKey);
        if (cached) {
            this.indices[indexName] = cached;
            this.loadedIndices.add(indexName);
            return cached;
        }
        
        try {
            const response = await fetch(`${this.baseUrl}indices/${indexName}.json`);
            if (!response.ok) {
                throw new Error(`Failed to load ${indexName} index: ${response.status}`);
            }
            
            const data = await response.json();
            this.indices[indexName] = data;
            this.loadedIndices.add(indexName);
            this.setCache(cacheKey, data);
            
            this.emit('indexLoaded', { indexName, data });
            return data;
        } catch (error) {
            console.error(`Error loading ${indexName} index:`, error);
            throw error;
        }
    }
    
    /**
     * Main search function with multiple search strategies
     */
    async search(query, options = {}) {
        if (!query || query.trim().length < 1) {
            return { results: [], metadata: { query, total: 0 } };
        }
        
        const searchOptions = { ...this.config, ...options };
        const normalizedQuery = this.normalizeSearchQuery(query);
        
        this.emit('searchStarted', { query, options: searchOptions });
        
        try {
            // Load required indices
            await this.ensureIndicesLoaded(['search-terms', 'vendors', 'components']);
            
            // Perform multi-strategy search
            const results = await this.performMultiStrategySearch(normalizedQuery, searchOptions);
            
            // Sort and limit results
            const sortedResults = this.sortSearchResults(results, normalizedQuery);
            const limitedResults = sortedResults.slice(0, searchOptions.maxResults);
            
            const searchResult = {
                results: limitedResults,
                metadata: {
                    query,
                    normalizedQuery,
                    total: sortedResults.length,
                    strategies: results.strategies,
                    searchTime: Date.now() - results.startTime
                }
            };
            
            this.emit('searchCompleted', searchResult);
            return searchResult;
            
        } catch (error) {
            this.emit('searchError', { query, error });
            throw error;
        }
    }
    
    /**
     * Perform search using multiple strategies
     */
    async performMultiStrategySearch(query, options) {
        const startTime = Date.now();
        const strategies = [];
        let allResults = [];
        
        // Strategy 1: Exact term matching
        const exactResults = await this.searchExactTerms(query);
        if (exactResults.length > 0) {
            strategies.push('exact');
            allResults = allResults.concat(exactResults.map(r => ({ ...r, strategy: 'exact', score: r.score + 0.3 })));
        }
        
        // Strategy 2: Vendor search
        const vendorResults = await this.searchVendors(query);
        if (vendorResults.length > 0) {
            strategies.push('vendor');
            allResults = allResults.concat(vendorResults.map(r => ({ ...r, strategy: 'vendor', score: r.score + 0.2 })));
        }
        
        // Strategy 3: Component search
        const componentResults = await this.searchComponents(query);
        if (componentResults.length > 0) {
            strategies.push('component');
            allResults = allResults.concat(componentResults.map(r => ({ ...r, strategy: 'component', score: r.score + 0.1 })));
        }
        
        // Strategy 4: Fuzzy search (if enabled)
        if (options.enableFuzzySearch && query.length >= 3) {
            const fuzzyResults = await this.searchFuzzy(query);
            if (fuzzyResults.length > 0) {
                strategies.push('fuzzy');
                allResults = allResults.concat(fuzzyResults.map(r => ({ ...r, strategy: 'fuzzy' })));
            }
        }
        
        // Strategy 5: Kernel version search
        if (this.isKernelVersionQuery(query)) {
            const kernelResults = await this.searchKernelVersions(query);
            if (kernelResults.length > 0) {
                strategies.push('kernel');
                allResults = allResults.concat(kernelResults.map(r => ({ ...r, strategy: 'kernel', score: r.score + 0.25 })));
            }
        }
        
        return {
            results: this.deduplicateResults(allResults),
            strategies,
            startTime
        };
    }
    
    /**
     * Search for exact term matches
     */
    async searchExactTerms(query) {
        const searchTerms = this.indices['search-terms'];
        if (!searchTerms) return [];
        
        const results = [];
        const queryLower = query.toLowerCase();
        
        Object.entries(searchTerms).forEach(([term, data]) => {
            if (term.toLowerCase().includes(queryLower)) {
                const score = this.calculateExactMatchScore(term, query);
                results.push({
                    type: 'hardware',
                    id: data.hardware_id,
                    title: data.display_name || term,
                    description: data.description || 'Hardware component',
                    compatibility: data.avg_compatibility || 0,
                    reportCount: data.report_count || 0,
                    score,
                    matchedTerm: term
                });
            }
        });
        
        return results;
    }
    
    /**
     * Search vendors
     */
    async searchVendors(query) {
        const vendors = this.indices['vendors'];
        if (!vendors) return [];
        
        const results = [];
        const queryLower = query.toLowerCase();
        
        Object.entries(vendors).forEach(([vendor, data]) => {
            const vendorLower = vendor.toLowerCase();
            if (vendorLower.includes(queryLower) || this.fuzzyMatch(vendorLower, queryLower) > this.config.fuzzyMatchThreshold) {
                const score = this.calculateVendorMatchScore(vendor, query, data);
                results.push({
                    type: 'vendor',
                    id: vendor,
                    title: vendor,
                    description: `${data.total_reports} reports, ${data.unique_models} models`,
                    compatibility: data.avg_compatibility || 0,
                    reportCount: data.total_reports || 0,
                    models: data.top_models || [],
                    score
                });
            }
        });
        
        return results;
    }
    
    /**
     * Search components
     */
    async searchComponents(query) {
        await this.loadIndex('components');
        const components = this.indices['components'];
        if (!components) return [];
        
        const results = [];
        const queryLower = query.toLowerCase();
        
        Object.entries(components).forEach(([componentType, data]) => {
            if (componentType.toLowerCase().includes(queryLower)) {
                const score = this.calculateComponentMatchScore(componentType, query, data);
                results.push({
                    type: 'component',
                    id: componentType,
                    title: this.formatComponentType(componentType),
                    description: `${data.total_reports} reports, ${data.vendors.length} vendors`,
                    compatibility: data.avg_compatibility || 0,
                    reportCount: data.total_reports || 0,
                    vendors: data.top_vendors || [],
                    score
                });
            }
        });
        
        return results;
    }
    
    /**
     * Fuzzy search implementation
     */
    async searchFuzzy(query) {
        const results = [];
        const queryLower = query.toLowerCase();
        const threshold = this.config.fuzzyMatchThreshold;
        
        // Search through all loaded indices
        for (const [indexName, indexData] of Object.entries(this.indices)) {
            if (typeof indexData === 'object') {
                Object.entries(indexData).forEach(([key, data]) => {
                    const similarity = this.fuzzyMatch(key.toLowerCase(), queryLower);
                    if (similarity > threshold) {
                        results.push({
                            type: indexName,
                            id: key,
                            title: key,
                            description: this.generateDescription(indexName, data),
                            compatibility: data.avg_compatibility || 0,
                            reportCount: data.total_reports || data.report_count || 0,
                            score: similarity * 0.8
                        });
                    }
                });
            }
        }
        
        return results;
    }
    
    /**
     * Search kernel versions
     */
    async searchKernelVersions(query) {
        await this.loadIndex('kernels');
        const kernels = this.indices['kernels'];
        if (!kernels) return [];
        
        const results = [];
        const kernelPattern = this.extractKernelVersion(query);
        
        if (kernelPattern) {
            Object.entries(kernels).forEach(([kernel, data]) => {
                if (this.matchesKernelPattern(kernel, kernelPattern)) {
                    const score = this.calculateKernelMatchScore(kernel, kernelPattern);
                    results.push({
                        type: 'kernel',
                        id: kernel,
                        title: `Linux ${kernel}`,
                        description: `${data.total_reports} reports, ${(data.success_rate || 0).toFixed(1)}% success rate`,
                        compatibility: data.success_rate || 0,
                        reportCount: data.total_reports || 0,
                        score
                    });
                }
            });
        }
        
        return results;
    }
    
    /**
     * Get hardware details by ID
     */
    async getHardwareDetails(hardwareId, type = 'hardware') {
        const cacheKey = `details_${type}_${hardwareId}`;
        const cached = this.getFromCache(cacheKey);
        if (cached) return cached;
        
        try {
            const response = await fetch(`${this.baseUrl}api/${type}/${encodeURIComponent(hardwareId)}.json`);
            if (!response.ok) {
                throw new Error(`Hardware details not found: ${response.status}`);
            }
            
            const data = await response.json();
            this.setCache(cacheKey, data);
            return data;
        } catch (error) {
            console.error(`Error loading hardware details for ${hardwareId}:`, error);
            return null;
        }
    }
    
    /**
     * Get search suggestions for typeahead
     */
    async getSuggestions(query, limit = 10) {
        if (!this.config.enableTypeahead || query.length < 2) {
            return [];
        }
        
        await this.ensureIndicesLoaded(['search-terms', 'vendors']);
        
        const suggestions = [];
        const queryLower = query.toLowerCase();
        
        // Get vendor suggestions
        Object.keys(this.indices['vendors'] || {}).forEach(vendor => {
            if (vendor.toLowerCase().startsWith(queryLower)) {
                suggestions.push({
                    text: vendor,
                    type: 'vendor',
                    priority: 3
                });
            }
        });
        
        // Get search term suggestions
        Object.keys(this.indices['search-terms'] || {}).forEach(term => {
            if (term.toLowerCase().startsWith(queryLower)) {
                suggestions.push({
                    text: term,
                    type: 'hardware',
                    priority: 2
                });
            }
        });
        
        // Sort by priority and relevance
        suggestions.sort((a, b) => {
            if (a.priority !== b.priority) return b.priority - a.priority;
            return a.text.length - b.text.length; // Shorter matches first
        });
        
        return suggestions.slice(0, limit);
    }
    
    /**
     * Get database statistics
     */
    async getStatistics() {
        await this.loadIndex('statistics');
        return this.indices['statistics'] || {};
    }
    
    // Utility methods
    
    normalizeSearchQuery(query) {
        return query.trim().toLowerCase()
            .replace(/[^\w\s.-]/g, ' ')
            .replace(/\s+/g, ' ');
    }
    
    calculateExactMatchScore(term, query) {
        const termLower = term.toLowerCase();
        const queryLower = query.toLowerCase();
        
        if (termLower === queryLower) return 1.0;
        if (termLower.startsWith(queryLower)) return 0.9;
        if (termLower.includes(queryLower)) return 0.7;
        return 0.5;
    }
    
    calculateVendorMatchScore(vendor, query, data) {
        const baseScore = this.calculateExactMatchScore(vendor, query);
        const popularityBoost = Math.log10(Math.max(1, data.total_reports || 0)) * 0.1;
        const compatibilityBoost = (data.avg_compatibility || 0) * 0.1;
        
        return Math.min(1.0, baseScore + popularityBoost + compatibilityBoost);
    }
    
    calculateComponentMatchScore(component, query, data) {
        const baseScore = this.calculateExactMatchScore(component, query);
        const reportBoost = Math.log10(Math.max(1, data.total_reports || 0)) * 0.05;
        
        return Math.min(1.0, baseScore + reportBoost);
    }
    
    calculateKernelMatchScore(kernel, pattern) {
        // Exact version match gets highest score
        if (kernel === pattern) return 1.0;
        
        // Same major.minor version gets high score
        const kernelParts = kernel.split('.');
        const patternParts = pattern.split('.');
        
        if (kernelParts.length >= 2 && patternParts.length >= 2) {
            if (kernelParts[0] === patternParts[0] && kernelParts[1] === patternParts[1]) {
                return 0.8;
            }
        }
        
        return 0.3;
    }
    
    fuzzyMatch(str1, str2) {
        // Simple Levenshtein distance-based fuzzy matching
        const matrix = [];
        const len1 = str1.length;
        const len2 = str2.length;
        
        if (len1 === 0) return len2 === 0 ? 1 : 0;
        if (len2 === 0) return 0;
        
        for (let i = 0; i <= len2; i++) matrix[i] = [i];
        for (let j = 0; j <= len1; j++) matrix[0][j] = j;
        
        for (let i = 1; i <= len2; i++) {
            for (let j = 1; j <= len1; j++) {
                if (str2.charAt(i - 1) === str1.charAt(j - 1)) {
                    matrix[i][j] = matrix[i - 1][j - 1];
                } else {
                    matrix[i][j] = Math.min(
                        matrix[i - 1][j - 1] + 1,
                        matrix[i][j - 1] + 1,
                        matrix[i - 1][j] + 1
                    );
                }
            }
        }
        
        const distance = matrix[len2][len1];
        const maxLength = Math.max(len1, len2);
        return (maxLength - distance) / maxLength;
    }
    
    isKernelVersionQuery(query) {
        return /^\d+\.\d+(\.\d+)?/.test(query.trim());
    }
    
    extractKernelVersion(query) {
        const match = query.match(/(\d+\.\d+(?:\.\d+)?)/);
        return match ? match[1] : null;
    }
    
    matchesKernelPattern(kernel, pattern) {
        return kernel.includes(pattern);
    }
    
    formatComponentType(type) {
        return type.split(/[-_]/)
            .map(word => word.charAt(0).toUpperCase() + word.slice(1))
            .join(' ');
    }
    
    generateDescription(indexType, data) {
        const reports = data.total_reports || data.report_count || 0;
        const compatibility = data.avg_compatibility || 0;
        
        return `${reports} reports, ${compatibility.toFixed(1)} compatibility score`;
    }
    
    sortSearchResults(results, query) {
        return results.sort((a, b) => {
            // Primary sort by score
            if (a.score !== b.score) return b.score - a.score;
            
            // Secondary sort by report count (popularity)
            if (a.reportCount !== b.reportCount) return b.reportCount - a.reportCount;
            
            // Tertiary sort by compatibility
            if (a.compatibility !== b.compatibility) return b.compatibility - a.compatibility;
            
            // Finally, alphabetical
            return a.title.localeCompare(b.title);
        });
    }
    
    deduplicateResults(results) {
        const seen = new Set();
        return results.filter(result => {
            const key = `${result.type}:${result.id}`;
            if (seen.has(key)) return false;
            seen.add(key);
            return true;
        });
    }
    
    async ensureIndicesLoaded(indexNames) {
        const loadPromises = indexNames
            .filter(name => !this.loadedIndices.has(name))
            .map(name => this.loadIndex(name));
            
        await Promise.all(loadPromises);
    }
    
    // Cache management
    
    getFromCache(key) {
        const cached = this.cache.get(key);
        if (cached && Date.now() - cached.timestamp < this.config.cacheTimeout) {
            return cached.data;
        }
        this.cache.delete(key);
        return null;
    }
    
    setCache(key, data) {
        this.cache.set(key, {
            data,
            timestamp: Date.now()
        });
    }
    
    clearCache() {
        this.cache.clear();
    }
    
    // Event system
    
    emit(eventType, data) {
        this.events.dispatchEvent(new CustomEvent(eventType, { detail: data }));
    }
    
    on(eventType, callback) {
        this.events.addEventListener(eventType, callback);
    }
    
    off(eventType, callback) {
        this.events.removeEventListener(eventType, callback);
    }
}

// Export for both browser and Node.js
if (typeof module !== 'undefined' && module.exports) {
    module.exports = HardwareSearchEngine;
} else if (typeof window !== 'undefined') {
    window.HardwareSearchEngine = HardwareSearchEngine;
}