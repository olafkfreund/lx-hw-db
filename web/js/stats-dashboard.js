
// Statistics Dashboard for Linux Hardware Compatibility Database
class StatsDashboard {
    constructor() {
        this.apiBase = './api/v1/stats/';
        this.init();
    }

    async init() {
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

        console.log('Stats updated:', {
            reports: stats.total_reports,
            systems: stats.unique_systems,
            vendors: stats.total_vendors,
            kernels: stats.kernel_versions
        });

        // Update compatibility chart with new data
        this.updateCompatibilityChart(stats.compatibility_overview);
    }

    initializeCompatibilityChart() {
        const chartContainer = document.querySelector('#compatibility-chart');
        if (!chartContainer) return;

        chartContainer.innerHTML = `
            <div class="compatibility-chart-loading">
                <div class="loading-spinner"></div>
                <p>Loading compatibility overview...</p>
            </div>
        `;
    }

    updateCompatibilityChart(compatibilityData) {
        const chartContainer = document.querySelector('#compatibility-chart');
        if (!chartContainer || !compatibilityData) return;

        // Calculate total and percentages
        const entries = Object.entries(compatibilityData);
        const total = entries.reduce((sum, [, count]) => sum + count, 0);

        if (total === 0) {
            chartContainer.innerHTML = `
                <div class="compatibility-empty">
                    <p>No compatibility data available yet.</p>
                </div>
            `;
            return;
        }

        // Create simple bar chart
        const maxCount = Math.max(...entries.map(([, count]) => count));
        
        chartContainer.innerHTML = `
            <div class="compatibility-bars">
                ${entries.map(([status, count]) => {
                    const percentage = ((count / total) * 100).toFixed(1);
                    const barWidth = (count / maxCount) * 100;
                    const statusClass = status.toLowerCase().replace(/\s+/g, '-');
                    
                    return `
                        <div class="compatibility-bar-item">
                            <div class="compatibility-bar-label">
                                <span class="status-name">${status}</span>
                                <span class="status-stats">${count} (${percentage}%)</span>
                            </div>
                            <div class="compatibility-bar-track">
                                <div class="compatibility-bar-fill compatibility-${statusClass}" 
                                     style="width: ${barWidth}%"></div>
                            </div>
                        </div>
                    `;
                }).join('')}
            </div>
        `;
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
