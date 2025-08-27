
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
        
        await this.loadOverviewStats();
        this.initializeCompatibilityChart();
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
        console.log('Displaying stats:', stats);
        
        // Update hardware reports count
        const reportsElement = document.querySelector('[data-stat="reports"]');
        if (reportsElement) {
            reportsElement.textContent = stats.total_reports || 0;
            const card = reportsElement.closest('.stat-card');
            if (card) card.classList.remove('loading');
        }

        // Update unique systems count
        const systemsElement = document.querySelector('[data-stat="systems"]');
        if (systemsElement) {
            systemsElement.textContent = stats.unique_systems || 0;
            const card = systemsElement.closest('.stat-card');
            if (card) card.classList.remove('loading');
        }

        // Update vendors count
        const vendorsElement = document.querySelector('[data-stat="vendors"]');
        if (vendorsElement) {
            vendorsElement.textContent = stats.total_vendors || 0;
            const card = vendorsElement.closest('.stat-card');
            if (card) card.classList.remove('loading');
        }

        // Update kernel versions count
        const kernelsElement = document.querySelector('[data-stat="kernels"]');
        if (kernelsElement) {
            kernelsElement.textContent = stats.kernel_versions || 0;
            const card = kernelsElement.closest('.stat-card');
            if (card) card.classList.remove('loading');
        }

        console.log('Statistics updated successfully');
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
                        <div class="compat-value" id="compat-unknown">1</div>
                    </div>
                </div>
                <div class="chart-placeholder">
                    <p>Compatibility distribution will be visualized here as more data is collected.</p>
                </div>
            </div>
        `;
    }

    displayError(message) {
        console.error('Stats display error:', message);
        document.querySelectorAll('[data-stat]').forEach(el => {
            el.textContent = '—';
        });
        document.querySelectorAll('.stat-card.loading').forEach(el => {
            el.classList.remove('loading');
            el.classList.add('error');
        });
    }
}

// Initialize dashboard immediately - simplified approach
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        console.log('DOM loaded, initializing StatsDashboard immediately');
        window.statsDashboard = new StatsDashboard();
    });
} else {
    console.log('DOM already loaded, initializing StatsDashboard immediately');
    window.statsDashboard = new StatsDashboard();
}
