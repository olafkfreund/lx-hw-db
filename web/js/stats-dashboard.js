
// Statistics Dashboard for Linux Hardware Compatibility Database
class StatsDashboard {
    constructor() {
        this.apiBase = './api/v1/stats/';
        this.retryCount = 0;
        this.maxRetries = 3;
        this.init();
    }

    async init() {
        console.log('Initializing StatsDashboard...');
        
        // Wait a bit for DOM to be fully ready
        await new Promise(resolve => setTimeout(resolve, 100));
        
        await this.loadOverviewStats();
        await this.loadTrends();
        this.initializeCompatibilityChart();
    }

    async loadOverviewStats() {
        try {
            console.log('Loading overview stats from:', `${this.apiBase}overview.json`);
            const response = await fetch(`${this.apiBase}overview.json`);
            
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            
            const data = await response.json();
            console.log('Overview stats loaded:', data);
            
            if (data && data.data) {
                this.displayOverviewStats(data.data);
            } else {
                console.error('Invalid data structure:', data);
                this.displayError('Invalid data format');
            }
        } catch (error) {
            console.error('Error loading overview stats:', error);
            
            if (this.retryCount < this.maxRetries) {
                this.retryCount++;
                console.log(`Retrying... (${this.retryCount}/${this.maxRetries})`);
                setTimeout(() => this.loadOverviewStats(), 1000 * this.retryCount);
            } else {
                this.displayError('Failed to load statistics');
            }
        }
    }

    async loadTrends() {
        try {
            const response = await fetch(`${this.apiBase}trends.json`);
            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }
            const data = await response.json();
            console.log('Trends data loaded:', data);
        } catch (error) {
            console.error('Error loading trends:', error);
        }
    }

    displayOverviewStats(stats) {
        console.log('Displaying overview stats:', stats);
        
        // Update hardware reports count
        const reportsElement = document.querySelector('[data-stat="reports"]');
        if (reportsElement) {
            reportsElement.textContent = this.formatNumber(stats.total_reports || 0);
            // Remove loading class from parent card
            const card = reportsElement.closest('.stat-card');
            if (card) card.classList.remove('loading');
        }

        // Update unique systems count
        const systemsElement = document.querySelector('[data-stat="systems"]');
        if (systemsElement) {
            systemsElement.textContent = this.formatNumber(stats.unique_systems || 0);
            const card = systemsElement.closest('.stat-card');
            if (card) card.classList.remove('loading');
        }

        // Update vendors count
        const vendorsElement = document.querySelector('[data-stat="vendors"]');
        if (vendorsElement) {
            vendorsElement.textContent = this.formatNumber(stats.total_vendors || 0);
            const card = vendorsElement.closest('.stat-card');
            if (card) card.classList.remove('loading');
        }

        // Update kernel versions count
        const kernelsElement = document.querySelector('[data-stat="kernels"]');
        if (kernelsElement) {
            kernelsElement.textContent = this.formatNumber(stats.kernel_versions || 0);
            const card = kernelsElement.closest('.stat-card');
            if (card) card.classList.remove('loading');
        }

        // Update compatibility overview if available
        if (stats.compatibility_overview) {
            this.updateCompatibilityChart(stats.compatibility_overview);
        }
    }

    initializeCompatibilityChart() {
        const chartContainer = document.getElementById('compatibility-chart');
        if (!chartContainer) {
            console.log('Compatibility chart container not found');
            return;
        }

        // Create a simple compatibility overview
        chartContainer.innerHTML = `
            <div class="compatibility-overview-content">
                <div class="compatibility-stats">
                    <div class="compat-stat">
                        <div class="compat-label">Full Support</div>
                        <div class="compat-value" id="compat-full">0</div>
                    </div>
                    <div class="compat-stat">
                        <div class="compat-label">Partial Support</div>
                        <div class="compat-value" id="compat-partial">0</div>
                    </div>
                    <div class="compat-stat">
                        <div class="compat-label">Limited Support</div>
                        <div class="compat-value" id="compat-limited">0</div>
                    </div>
                    <div class="compat-stat">
                        <div class="compat-label">Unknown</div>
                        <div class="compat-value" id="compat-unknown">0</div>
                    </div>
                </div>
                <div class="chart-placeholder">
                    <p>Compatibility distribution chart will be displayed here as more hardware reports are collected.</p>
                </div>
            </div>
        `;
    }

    updateCompatibilityChart(compatData) {
        console.log('Updating compatibility chart with:', compatData);
        
        // Update individual compatibility values
        const fullElement = document.getElementById('compat-full');
        if (fullElement) fullElement.textContent = compatData.full || 0;
        
        const partialElement = document.getElementById('compat-partial');
        if (partialElement) partialElement.textContent = compatData.partial || 0;
        
        const limitedElement = document.getElementById('compat-limited');
        if (limitedElement) limitedElement.textContent = compatData.limited || 0;
        
        const unknownElement = document.getElementById('compat-unknown');
        if (unknownElement) unknownElement.textContent = compatData.Unknown || compatData.unknown || 0;
    }

    formatNumber(num) {
        if (num >= 1000000) {
            return (num / 1000000).toFixed(1) + 'M';
        } else if (num >= 1000) {
            return (num / 1000).toFixed(1) + 'K';
        }
        return num.toString();
    }

    displayError(message) {
        console.error('Display error:', message);
        
        document.querySelectorAll('[data-stat]').forEach(el => {
            el.textContent = '—';
        });
        
        document.querySelectorAll('.stat-card.loading').forEach(el => {
            el.classList.remove('loading');
            el.classList.add('error');
        });
        
        // Show error in compatibility chart
        const chartContainer = document.getElementById('compatibility-chart');
        if (chartContainer) {
            chartContainer.innerHTML = `<p class="error-message">${message}</p>`;
        }
    }
}

// Initialize dashboard when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        console.log('DOM loaded, initializing StatsDashboard');
        window.statsDashboard = new StatsDashboard();
    });
} else {
    console.log('DOM already loaded, initializing StatsDashboard');
    window.statsDashboard = new StatsDashboard();
}
