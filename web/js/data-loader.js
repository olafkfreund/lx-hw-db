/**
 * Data Loader
 * Handles loading hardware database and configuration tips data
 */
class DataLoader {
    constructor() {
        this.hardwareData = null;
        this.configurationTips = null;
        this.loadingPromises = [];
        this.baseUrl = this.getBaseUrl();
        
        this.init();
    }
    
    getBaseUrl() {
        // Use relative paths for all environments to avoid CORS issues
        // This works for both local development and GitHub Pages
        return './';
    }
    
    init() {
        // Start loading data immediately
        this.loadingPromises.push(this.loadHardwareDatabase());
        this.loadingPromises.push(this.loadConfigurationTips());
        
        // Wait for all data to load, then initialize the app
        Promise.all(this.loadingPromises).then(() => {
            this.onDataLoaded();
        }).catch(error => {
            console.error('Failed to load application data:', error);
            this.onDataLoadError(error);
        });
    }
    
    async loadHardwareDatabase() {
        try {
            console.log('Loading hardware database...');
            const response = await fetch(`${this.baseUrl}data/hardware-database.json`);
            
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            
            this.hardwareData = await response.json();
            console.log(`Loaded ${this.hardwareData.hardware.length} hardware entries`);
            
            // Store in window for global access
            window.hardwareDatabase = this.hardwareData;
            
            // Dispatch event for other modules
            document.dispatchEvent(new CustomEvent('hardware-database-loaded', {
                detail: { data: this.hardwareData }
            }));
            
        } catch (error) {
            console.error('Failed to load hardware database:', error);
            
            // Create empty database structure if loading fails
            this.hardwareData = {
                hardware: [],
                statistics: {
                    total_hardware: 0,
                    total_reports: 0,
                    distributions_covered: 0,
                    categories: {},
                    compatibility_breakdown: {
                        full: 0,
                        partial: 0,
                        limited: 0,
                        none: 0
                    },
                    last_updated: new Date().toISOString()
                }
            };
            
            window.hardwareDatabase = this.hardwareData;
            throw error;
        }
    }
    
    async loadConfigurationTips() {
        try {
            console.log('Loading configuration tips...');
            const response = await fetch(`${this.baseUrl}data/configuration-tips.json`);
            
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            
            this.configurationTips = await response.json();
            console.log(`Loaded ${this.configurationTips.tips.length} configuration tips`);
            
            // Store in window for global access
            window.configurationTipsData = this.configurationTips;
            
            // Initialize the configuration tips module with the data
            if (window.configurationTips) {
                window.configurationTips.setTipsData(this.configurationTips.tips);
                window.configurationTips.setCategoriesData(this.configurationTips.categories);
            }
            
            // Dispatch event for other modules
            document.dispatchEvent(new CustomEvent('configuration-tips-loaded', {
                detail: { data: this.configurationTips }
            }));
            
        } catch (error) {
            console.error('Failed to load configuration tips:', error);
            
            // Create empty tips structure if loading fails
            this.configurationTips = {
                tips: [],
                categories: [],
                statistics: {
                    total_tips: 0,
                    verified_tips: 0,
                    average_rating: 0,
                    total_votes: 0,
                    distributions_covered: 0,
                    components_covered: [],
                    categories_covered: [],
                    last_updated: new Date().toISOString()
                }
            };
            
            window.configurationTipsData = this.configurationTips;
            throw error;
        }
    }
    
    onDataLoaded() {
        console.log('All application data loaded successfully');
        
        // Update page statistics
        this.updateStatisticsDisplay();
        
        // Initialize search engines with data
        this.initializeSearchEngines();
        
        // Update UI to show data is ready
        this.updateLoadingStates();
        
        // Dispatch global ready event
        document.dispatchEvent(new CustomEvent('app-data-ready', {
            detail: {
                hardware: this.hardwareData,
                tips: this.configurationTips
            }
        }));
    }
    
    onDataLoadError(error) {
        console.error('Application data loading failed:', error);
        
        // Show error message to user
        this.showErrorMessage(`
            <div class="data-load-error">
                <h3>⚠️ Data Loading Error</h3>
                <p>Failed to load hardware database. This could be because:</p>
                <ul>
                    <li>You're opening the HTML file directly (use a local server)</li>
                    <li>Network connection issues</li>
                    <li>Missing data files</li>
                </ul>
                <p><strong>To run locally:</strong></p>
                <code>python3 -m http.server 8000</code>
                <p>Then open <a href="http://localhost:8000">http://localhost:8000</a></p>
            </div>
        `);
    }
    
    updateStatisticsDisplay() {
        // Update the statistics cards
        const statsElements = {
            reports: document.querySelector('[data-stat="reports"]'),
            systems: document.querySelector('[data-stat="systems"]'),
            vendors: document.querySelector('[data-stat="vendors"]'),
            kernels: document.querySelector('[data-stat="kernels"]')
        };
        
        if (this.hardwareData && statsElements.reports) {
            const stats = this.hardwareData.statistics;
            const vendors = [...new Set(this.hardwareData.hardware.map(h => h.manufacturer))].length;
            const kernelVersions = this.extractKernelVersions().length;
            
            statsElements.reports.textContent = stats.total_reports.toLocaleString();
            statsElements.systems.textContent = stats.total_hardware.toLocaleString();
            statsElements.vendors.textContent = vendors.toLocaleString();
            statsElements.kernels.textContent = kernelVersions.toLocaleString();
            
            // Remove loading class
            document.querySelectorAll('.stat-card.loading').forEach(card => {
                card.classList.remove('loading');
            });
        }
        
        // Update tips statistics if available
        if (this.configurationTips) {
            const tipStats = this.configurationTips.statistics;
            console.log(`Tips loaded: ${tipStats.total_tips} tips, average rating: ${tipStats.average_rating}`);
        }
    }
    
    extractKernelVersions() {
        const kernelVersions = new Set();
        
        this.hardwareData.hardware.forEach(hardware => {
            Object.values(hardware.tested_distributions || {}).forEach(dist => {
                if (dist.kernel_version) {
                    kernelVersions.add(dist.kernel_version);
                }
            });
        });
        
        return Array.from(kernelVersions);
    }
    
    initializeSearchEngines() {
        // Initialize hardware search if available
        if (window.searchEngine && this.hardwareData) {
            window.searchEngine.indexHardware(this.hardwareData.hardware);
        }
        
        // Initialize tips search if available
        if (window.tipSearch && this.configurationTips) {
            window.tipSearch.indexTips(this.configurationTips.tips);
        }
    }
    
    updateLoadingStates() {
        // Hide loading indicators
        document.querySelectorAll('.loading-indicator').forEach(indicator => {
            indicator.style.display = 'none';
        });
        
        // Enable search functionality
        const searchInput = document.getElementById('search-input');
        if (searchInput) {
            searchInput.disabled = false;
            searchInput.placeholder = 'Search hardware by vendor, model, or distribution...';
        }
        
        // Enable other interactive elements
        document.querySelectorAll('.disabled-until-loaded').forEach(element => {
            element.disabled = false;
            element.classList.remove('disabled-until-loaded');
        });
    }
    
    showErrorMessage(html) {
        const searchResults = document.getElementById('search-results');
        if (searchResults) {
            searchResults.innerHTML = html;
        } else {
            // Fallback to console if no search results container
            console.error('Data loading error displayed in console (no UI container found)');
        }
    }
    
    // Public API for other modules
    getHardwareData() {
        return this.hardwareData;
    }
    
    getConfigurationTips() {
        return this.configurationTips;
    }
    
    isDataReady() {
        return this.hardwareData !== null && this.configurationTips !== null;
    }
    
    // Method to add new hardware data (for when the CLI tool runs)
    addHardwareEntry(hardwareEntry) {
        if (!this.hardwareData) {
            console.error('Hardware database not loaded yet');
            return false;
        }
        
        // Check for duplicates
        const existingEntry = this.hardwareData.hardware.find(h => 
            h.pci_id === hardwareEntry.pci_id || 
            h.usb_id === hardwareEntry.usb_id ||
            h.id === hardwareEntry.id
        );
        
        if (existingEntry) {
            console.log('Hardware entry already exists, updating...', hardwareEntry.id);
            Object.assign(existingEntry, hardwareEntry);
        } else {
            console.log('Adding new hardware entry:', hardwareEntry.id);
            this.hardwareData.hardware.push(hardwareEntry);
            this.hardwareData.statistics.total_hardware++;
        }
        
        // Re-index search engines
        if (window.searchEngine) {
            window.searchEngine.indexHardware(this.hardwareData.hardware);
        }
        
        // Dispatch update event
        document.dispatchEvent(new CustomEvent('hardware-database-updated', {
            detail: { entry: hardwareEntry, action: existingEntry ? 'updated' : 'added' }
        }));
        
        return true;
    }
    
    // Method to add new configuration tip (for community submissions)
    addConfigurationTip(tip) {
        if (!this.configurationTips) {
            console.error('Configuration tips not loaded yet');
            return false;
        }
        
        // Generate ID if not provided
        if (!tip.id) {
            tip.id = `tip_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
        }
        
        // Set defaults
        tip.created = tip.created || new Date().toISOString();
        tip.updated = tip.updated || tip.created;
        tip.rating = tip.rating || 0;
        tip.votes = tip.votes || 0;
        tip.verified = tip.verified || false;
        
        this.configurationTips.tips.push(tip);
        this.configurationTips.statistics.total_tips++;
        
        // Re-index search engines
        if (window.tipSearch) {
            window.tipSearch.indexTips(this.configurationTips.tips);
        }
        
        // Dispatch update event
        document.dispatchEvent(new CustomEvent('configuration-tip-added', {
            detail: { tip: tip }
        }));
        
        console.log('Added new configuration tip:', tip.id);
        return true;
    }
}

// Initialize data loader when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        window.dataLoader = new DataLoader();
    });
} else {
    window.dataLoader = new DataLoader();
}