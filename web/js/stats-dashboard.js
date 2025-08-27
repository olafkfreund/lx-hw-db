
// Statistics Dashboard for Linux Hardware Compatibility Database
class StatsDashboard {
    constructor() {
        this.apiBase = './api/v1/stats/';
        this.retryCount = 0;
        this.maxRetries = 3;
        this.init();
    }

    async init() {
        // Wait for DOM to be ready
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', () => this.loadOverviewStats());
        } else {
            // Wait a bit for other scripts to initialize
            setTimeout(() => this.loadOverviewStats(), 100);
        }
    }

    async loadOverviewStats() {
        try {
            console.log('Loading overview stats...');
            
            const response = await fetch(`${this.apiBase}overview.json`);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            
            const data = await response.json();
            console.log('Stats data loaded:', data);
            
            // Check if data has the expected structure
            const statsData = data.data || data;
            this.displayOverviewStats(statsData);
            
        } catch (error) {
            console.error('Error loading stats:', error);
            
            if (this.retryCount < this.maxRetries) {
                this.retryCount++;
                console.log(`Retrying stats load (${this.retryCount}/${this.maxRetries})...`);
                setTimeout(() => this.loadOverviewStats(), 1000 * this.retryCount);
            } else {
                this.displayError('Failed to load statistics');
            }
        }
    }

    displayOverviewStats(stats) {
        console.log('Displaying stats:', stats);
        
        // Find stat elements more robustly
        const elements = {
            reports: document.querySelector('[data-stat="reports"]') || 
                    document.querySelector('.stat-value[data-stat="reports"]'),
            systems: document.querySelector('[data-stat="systems"]') || 
                    document.querySelector('.stat-value[data-stat="systems"]'),
            vendors: document.querySelector('[data-stat="vendors"]') || 
                    document.querySelector('.stat-value[data-stat="vendors"]'),
            kernels: document.querySelector('[data-stat="kernels"]') || 
                    document.querySelector('.stat-value[data-stat="kernels"]')
        };

        console.log('Found elements:', elements);

        // Update hardware reports count
        if (elements.reports) {
            elements.reports.textContent = stats.total_reports || 0;
            elements.reports.parentElement.classList.remove('loading');
        }

        // Update unique systems count
        if (elements.systems) {
            elements.systems.textContent = stats.unique_systems || 0;
            elements.systems.parentElement.classList.remove('loading');
        }

        // Update vendors count
        if (elements.vendors) {
            elements.vendors.textContent = stats.total_vendors || 0;
            elements.vendors.parentElement.classList.remove('loading');
        }

        // Update kernel versions count  
        if (elements.kernels) {
            elements.kernels.textContent = stats.kernel_versions || 0;
            elements.kernels.parentElement.classList.remove('loading');
        }

        // Remove loading class from any remaining loading elements
        document.querySelectorAll('.stat-card.loading').forEach(el => {
            el.classList.remove('loading');
        });
        
        // Update any loading text
        document.querySelectorAll('.stat-value').forEach(el => {
            if (el.textContent === 'Loading...') {
                el.textContent = '0';
            }
        });

        console.log('Stats display completed');
    }

    displayError(message) {
        console.error('Displaying error:', message);
        
        document.querySelectorAll('.stat-value').forEach(el => {
            if (el.textContent === 'Loading...' || el.classList.contains('loading')) {
                el.textContent = 'Error';
                el.classList.add('error');
                el.classList.remove('loading');
            }
        });
        
        document.querySelectorAll('.stat-card.loading').forEach(el => {
            el.classList.add('error');
            el.classList.remove('loading');
        });
    }
}

// Create global instance
window.statsDashboard = null;

// Initialize dashboard when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        window.statsDashboard = new StatsDashboard();
    });
} else {
    window.statsDashboard = new StatsDashboard();
}
