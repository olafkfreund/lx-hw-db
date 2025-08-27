
// Statistics Dashboard for Linux Hardware Compatibility Database
class StatsDashboard {
    constructor() {
        this.apiBase = './api/v1/stats/';
        this.init();
    }

    async init() {
        console.log('Initializing StatsDashboard...');
        
        // Wait a bit for DOM to be fully ready
        await new Promise(resolve => setTimeout(resolve, 100));
        
        // Check if DataLoader has already populated statistics
        const reportsElement = document.querySelector('[data-stat="reports"]');
        if (reportsElement && reportsElement.textContent !== 'Loading...' && reportsElement.textContent !== '') {
            console.log('Statistics already loaded by DataLoader, skipping API calls');
            this.initializeCompatibilityChart();
            return;
        }
        
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

// Initialize dashboard when DataLoader has finished loading
document.addEventListener('app-data-ready', () => {
    console.log('App data ready, initializing StatsDashboard');
    window.statsDashboard = new StatsDashboard();
});

// Fallback: Initialize after a delay if DataLoader doesn't fire the event
setTimeout(() => {
    if (!window.statsDashboard) {
        console.log('DataLoader event not received, initializing StatsDashboard anyway');
        window.statsDashboard = new StatsDashboard();
    }
}, 5000);
