/**
 * Hardware Database Indexing System
 * Creates searchable indices from hardware compatibility data
 */

class HardwareDatabaseIndexer {
    constructor() {
        this.rawData = [];
        this.indices = {
            byCategory: {},
            byVendor: {},
            byCompatibility: {},
            byArchitecture: {},
            byDistribution: {},
            byKernel: {}
        };
        this.statistics = null;
        this.isBuilt = false;
    }

    /**
     * Load hardware data from various sources
     */
    async loadData(sources = []) {
        console.log('Loading hardware data from sources...');
        
        const defaultSources = [
            'data/hardware-index.json'
        ];

        const allSources = sources.length > 0 ? sources : defaultSources;
        const loadPromises = allSources.map(source => this.loadFromSource(source));
        
        try {
            const results = await Promise.all(loadPromises);
            this.rawData = results.flat();
            console.log(`Loaded ${this.rawData.length} hardware reports from ${allSources.length} sources`);
            return this.rawData;
        } catch (error) {
            console.error('Error loading hardware data:', error);
            throw error;
        }
    }

    /**
     * Load data from a single source
     */
    async loadFromSource(source) {
        try {
            const response = await fetch(source);
            if (!response.ok) {
                throw new Error(`Failed to load ${source}: ${response.status}`);
            }
            
            const data = await response.json();
            
            // Validate data format
            if (!Array.isArray(data)) {
                throw new Error(`Invalid data format in ${source}: expected array`);
            }

            // Validate each report
            const validatedData = data.filter(report => this.validateReport(report));
            console.log(`Loaded ${validatedData.length}/${data.length} valid reports from ${source}`);
            
            return validatedData;
        } catch (error) {
            console.error(`Error loading from ${source}:`, error);
            return [];
        }
    }

    /**
     * Validate a hardware report structure
     */
    validateReport(report) {
        const required = ['id', 'system', 'metadata'];
        const systemRequired = ['kernel_version', 'distribution', 'architecture'];
        
        // Check required top-level fields
        for (const field of required) {
            if (!report[field]) {
                console.warn(`Invalid report: missing ${field}`, report.id);
                return false;
            }
        }

        // Check required system fields
        for (const field of systemRequired) {
            if (!report.system[field]) {
                console.warn(`Invalid report: missing system.${field}`, report.id);
                return false;
            }
        }

        return true;
    }

    /**
     * Build searchable indices from loaded data
     */
    buildIndices() {
        console.log('Building hardware database indices...');
        
        if (this.rawData.length === 0) {
            console.warn('No data loaded for indexing');
            return;
        }

        // Reset indices
        this.indices = {
            byCategory: {},
            byVendor: {},
            byCompatibility: {},
            byArchitecture: {},
            byDistribution: {},
            byKernel: {}
        };

        // Process each hardware report
        this.rawData.forEach(report => {
            this.indexReport(report);
        });

        // Build statistics
        this.buildStatistics();
        
        this.isBuilt = true;
        console.log('Hardware database indices built successfully');
        
        // Dispatch event for other components
        document.dispatchEvent(new CustomEvent('databaseIndexed', {
            detail: {
                indexer: this,
                reportCount: this.rawData.length,
                indices: this.indices
            }
        }));
    }

    /**
     * Index a single hardware report
     */
    indexReport(report) {
        // Index by architecture
        this.addToIndex('byArchitecture', report.system.architecture, report);

        // Index by distribution
        this.addToIndex('byDistribution', report.system.distribution, report);

        // Index by kernel version
        this.addToIndex('byKernel', report.system.kernel_version, report);

        // Index by compatibility status
        if (report.compatibility?.overall_status) {
            this.addToIndex('byCompatibility', report.compatibility.overall_status, report);
        }

        // Index hardware categories
        this.indexHardwareCategories(report);

        // Index vendors
        this.indexVendors(report);
    }

    /**
     * Index hardware by categories
     */
    indexHardwareCategories(report) {
        const categories = ['cpu', 'memory', 'graphics', 'network', 'storage', 'audio'];
        
        categories.forEach(category => {
            if (report[category]) {
                // Handle arrays (graphics, network, storage, audio)
                if (Array.isArray(report[category])) {
                    if (report[category].length > 0) {
                        this.addToIndex('byCategory', category, report);
                    }
                } else {
                    // Handle objects (cpu, memory)
                    this.addToIndex('byCategory', category, report);
                }
            }
        });
    }

    /**
     * Index by hardware vendors
     */
    indexVendors(report) {
        const vendors = new Set();

        // CPU vendor
        if (report.cpu?.vendor) {
            vendors.add(report.cpu.vendor);
        }

        // Graphics vendors
        if (report.graphics) {
            report.graphics.forEach(gpu => {
                if (gpu.vendor) vendors.add(gpu.vendor);
            });
        }

        // Network vendors
        if (report.network) {
            report.network.forEach(net => {
                if (net.vendor) vendors.add(net.vendor);
            });
        }

        // Storage vendors
        if (report.storage) {
            report.storage.forEach(storage => {
                if (storage.vendor) vendors.add(storage.vendor);
            });
        }

        // Audio vendors
        if (report.audio) {
            report.audio.forEach(audio => {
                if (audio.vendor) vendors.add(audio.vendor);
            });
        }

        // Add to vendor index
        vendors.forEach(vendor => {
            this.addToIndex('byVendor', vendor, report);
        });
    }

    /**
     * Add report to a specific index
     */
    addToIndex(indexName, key, report) {
        if (!this.indices[indexName][key]) {
            this.indices[indexName][key] = [];
        }
        
        // Avoid duplicates
        if (!this.indices[indexName][key].find(r => r.id === report.id)) {
            this.indices[indexName][key].push(report);
        }
    }

    /**
     * Build comprehensive statistics
     */
    buildStatistics() {
        const stats = {
            totalReports: this.rawData.length,
            distributions: {},
            architectures: {},
            kernelVersions: {},
            vendors: {
                cpu: {},
                gpu: {},
                network: {},
                storage: {},
                audio: {}
            },
            compatibility: {},
            categories: {},
            generatedAt: new Date().toISOString()
        };

        // Count distributions
        Object.entries(this.indices.byDistribution).forEach(([distro, reports]) => {
            stats.distributions[distro] = reports.length;
        });

        // Count architectures
        Object.entries(this.indices.byArchitecture).forEach(([arch, reports]) => {
            stats.architectures[arch] = reports.length;
        });

        // Count kernel versions
        Object.entries(this.indices.byKernel).forEach(([kernel, reports]) => {
            stats.kernelVersions[kernel] = reports.length;
        });

        // Count compatibility status
        Object.entries(this.indices.byCompatibility).forEach(([status, reports]) => {
            stats.compatibility[status] = reports.length;
        });

        // Count categories
        Object.entries(this.indices.byCategory).forEach(([category, reports]) => {
            stats.categories[category] = reports.length;
        });

        // Count vendors by category
        this.rawData.forEach(report => {
            // CPU vendors
            if (report.cpu?.vendor) {
                stats.vendors.cpu[report.cpu.vendor] = 
                    (stats.vendors.cpu[report.cpu.vendor] || 0) + 1;
            }

            // GPU vendors
            report.graphics?.forEach(gpu => {
                if (gpu.vendor) {
                    stats.vendors.gpu[gpu.vendor] = 
                        (stats.vendors.gpu[gpu.vendor] || 0) + 1;
                }
            });

            // Network vendors
            report.network?.forEach(net => {
                if (net.vendor) {
                    stats.vendors.network[net.vendor] = 
                        (stats.vendors.network[net.vendor] || 0) + 1;
                }
            });

            // Storage vendors
            report.storage?.forEach(storage => {
                if (storage.vendor) {
                    stats.vendors.storage[storage.vendor] = 
                        (stats.vendors.storage[storage.vendor] || 0) + 1;
                }
            });

            // Audio vendors
            report.audio?.forEach(audio => {
                if (audio.vendor) {
                    stats.vendors.audio[audio.vendor] = 
                        (stats.vendors.audio[audio.vendor] || 0) + 1;
                }
            });
        });

        this.statistics = stats;
        console.log('Hardware statistics compiled:', stats);
    }

    /**
     * Query reports by multiple criteria
     */
    query(criteria = {}) {
        if (!this.isBuilt) {
            console.warn('Database indices not built yet');
            return [];
        }

        let results = this.rawData;

        // Filter by architecture
        if (criteria.architecture) {
            const archReports = this.indices.byArchitecture[criteria.architecture] || [];
            results = results.filter(report => 
                archReports.some(r => r.id === report.id)
            );
        }

        // Filter by distribution
        if (criteria.distribution) {
            const distroReports = this.indices.byDistribution[criteria.distribution] || [];
            results = results.filter(report => 
                distroReports.some(r => r.id === report.id)
            );
        }

        // Filter by vendor
        if (criteria.vendor) {
            const vendorReports = this.indices.byVendor[criteria.vendor] || [];
            results = results.filter(report => 
                vendorReports.some(r => r.id === report.id)
            );
        }

        // Filter by compatibility
        if (criteria.compatibility) {
            const compatReports = this.indices.byCompatibility[criteria.compatibility] || [];
            results = results.filter(report => 
                compatReports.some(r => r.id === report.id)
            );
        }

        // Filter by category
        if (criteria.category) {
            const categoryReports = this.indices.byCategory[criteria.category] || [];
            results = results.filter(report => 
                categoryReports.some(r => r.id === report.id)
            );
        }

        return results;
    }

    /**
     * Get reports similar to a given report
     */
    findSimilar(targetReport, limit = 5) {
        if (!this.isBuilt) return [];

        const scores = this.rawData
            .filter(report => report.id !== targetReport.id)
            .map(report => ({
                report: report,
                score: this.calculateSimilarityScore(targetReport, report)
            }))
            .sort((a, b) => b.score - a.score)
            .slice(0, limit);

        return scores.map(s => s.report);
    }

    /**
     * Calculate similarity score between two reports
     */
    calculateSimilarityScore(report1, report2) {
        let score = 0;

        // Architecture match (high weight)
        if (report1.system.architecture === report2.system.architecture) {
            score += 10;
        }

        // Distribution similarity (medium weight)
        if (report1.system.distribution === report2.system.distribution) {
            score += 5;
        } else if (this.isDistributionSimilar(
            report1.system.distribution, 
            report2.system.distribution
        )) {
            score += 2;
        }

        // CPU vendor match (high weight)
        if (report1.cpu?.vendor === report2.cpu?.vendor) {
            score += 8;
        }

        // GPU vendor matches (medium weight)
        const gpu1Vendors = report1.graphics?.map(g => g.vendor) || [];
        const gpu2Vendors = report2.graphics?.map(g => g.vendor) || [];
        const commonGpuVendors = gpu1Vendors.filter(v => gpu2Vendors.includes(v));
        score += commonGpuVendors.length * 3;

        // Compatibility status match (low weight)
        if (report1.compatibility?.overall_status === report2.compatibility?.overall_status) {
            score += 2;
        }

        return score;
    }

    /**
     * Check if two distributions are similar (same family)
     */
    isDistributionSimilar(dist1, dist2) {
        const families = [
            ['Ubuntu', 'Debian'],
            ['Fedora', 'Red Hat', 'CentOS'],
            ['Arch Linux', 'Manjaro'],
            ['openSUSE', 'SUSE']
        ];

        return families.some(family => 
            family.some(d => dist1.includes(d)) && 
            family.some(d => dist2.includes(d))
        );
    }

    /**
     * Export indices to JSON for static generation
     */
    exportIndices() {
        return {
            indices: this.indices,
            statistics: this.statistics,
            metadata: {
                totalReports: this.rawData.length,
                generatedAt: new Date().toISOString(),
                version: '1.0.0'
            }
        };
    }

    /**
     * Get available filter options
     */
    getFilterOptions() {
        if (!this.isBuilt) return {};

        return {
            architectures: Object.keys(this.indices.byArchitecture).sort(),
            distributions: Object.keys(this.indices.byDistribution).sort(),
            vendors: Object.keys(this.indices.byVendor).sort(),
            compatibility: Object.keys(this.indices.byCompatibility).sort(),
            categories: Object.keys(this.indices.byCategory).sort()
        };
    }
}

// Global instance
window.hardwareDatabaseIndexer = new HardwareDatabaseIndexer();

// Auto-initialize when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
    try {
        console.log('Initializing hardware database indexer...');
        await window.hardwareDatabaseIndexer.loadData();
        window.hardwareDatabaseIndexer.buildIndices();
    } catch (error) {
        console.error('Failed to initialize database indexer:', error);
    }
});