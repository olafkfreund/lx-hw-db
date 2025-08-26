
// Statistics Dashboard for Linux Hardware Compatibility Database
class StatsDashboard {
    constructor() {
        this.apiBase = './api/v1/stats/';
        this.init();
    }
    
    async init() {
        await this.loadOverviewStats();
    }
    
    async loadOverviewStats() {
        try {
            const response = await fetch(`${this.apiBase}overview.json`);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data = await response.json();
            this.displayOverviewStats(data.data);
        } catch (error) {
            console.error('Error loading stats:', error);
            this.displayError('Failed to load statistics');
        }
    }
    
    displayOverviewStats(stats) {
        // Update hardware reports count
        const reportsElement = document.querySelector('[data-stat="reports"]');
        if (reportsElement) {
            reportsElement.textContent = stats.total_reports || 0;
        }
        
        // Update unique systems count
        const systemsElement = document.querySelector('[data-stat="systems"]');
        if (systemsElement) {
            systemsElement.textContent = stats.unique_systems || 0;
        }
        
        // Update vendors count
        const vendorsElement = document.querySelector('[data-stat="vendors"]');
        if (vendorsElement) {
            vendorsElement.textContent = stats.total_vendors || 0;
        }
        
        // Update kernel versions count
        const kernelsElement = document.querySelector('[data-stat="kernels"]');
        if (kernelsElement) {
            kernelsElement.textContent = stats.kernel_versions || 0;
        }
        
        // Remove loading text
        document.querySelectorAll('.loading').forEach(el => {
            el.textContent = '';
            el.classList.remove('loading');
        });
    }
    
    displayError(message) {
        document.querySelectorAll('.loading').forEach(el => {
            el.textContent = 'Error loading data';
            el.classList.add('error');
        });
    }
}

// Initialize dashboard when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
    new StatsDashboard();
});
