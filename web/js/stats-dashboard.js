
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
        console.log('About to update compatibility chart with:', stats.compatibility_overview);
        this.updateCompatibilityChart(stats.compatibility_overview);
        
        // Add a test to verify the function works with known data
        console.log('Testing chart with hardcoded data...');
        setTimeout(() => {
            this.updateCompatibilityChart({"Unknown": 1, "Working": 2, "Partial": 1});
        }, 1000);
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

    updateCompatibilityChart(compatibilityData, retryCount = 0) {
        console.log('updateCompatibilityChart called with:', compatibilityData);
        console.log('Document ready state:', document.readyState);
        console.log('All elements with id:', Array.from(document.querySelectorAll('[id]')).map(el => el.id));
        
        // Debug the compatibility section specifically
        const matrixSection = document.querySelector('.compatibility-matrix-section');
        console.log('Matrix section found:', matrixSection);
        if (matrixSection) {
            console.log('Matrix section innerHTML:', matrixSection.innerHTML);
            console.log('All elements inside matrix section:', Array.from(matrixSection.querySelectorAll('*')).map(el => `${el.tagName}${el.id ? '#' + el.id : ''}${el.className ? '.' + el.className.replace(/ /g, '.') : ''}`));
        }
        
        let chartContainer = document.querySelector('#compatibility-chart');
        console.log('Chart container found:', chartContainer);
        
        if (!chartContainer) {
            if (retryCount < 5) { // Try 5 times first
                console.warn(`Chart container not found yet, retrying in 500ms... (${retryCount + 1}/5)`);
                setTimeout(() => this.updateCompatibilityChart(compatibilityData, retryCount + 1), 500);
                return;
            } else {
                // After retries failed, create our own chart container
                console.warn('Chart container not found after retries. Creating our own container.');
                const matrixSection = document.querySelector('.compatibility-matrix-section');
                if (matrixSection) {
                    const compatibilityOverview = matrixSection.querySelector('.compatibility-overview');
                    if (compatibilityOverview) {
                        // Create and insert the chart container
                        const newChartContainer = document.createElement('div');
                        newChartContainer.id = 'compatibility-chart';
                        newChartContainer.className = 'chart-container';
                        compatibilityOverview.appendChild(newChartContainer);
                        chartContainer = newChartContainer;
                        console.log('Created new chart container:', chartContainer);
                    }
                }
                
                if (!chartContainer) {
                    console.error('Could not create chart container. Chart will not be displayed.');
                    console.error('Available sections:', Array.from(document.querySelectorAll('section')).map(s => s.className || s.id));
                    return;
                }
            }
        }
        
        if (!compatibilityData) {
            console.error('No compatibility data provided!');
            return;
        }

        try {
            // Calculate total and percentages
            const entries = Object.entries(compatibilityData);
            console.log('Compatibility entries:', entries);
            const total = entries.reduce((sum, [, count]) => sum + count, 0);
            console.log('Total count:', total);

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
            console.log('Max count:', maxCount);
            
            const chartHTML = `
                <div class="compatibility-bars">
                    ${entries.map(([status, count]) => {
                        const percentage = ((count / total) * 100).toFixed(1);
                        const barWidth = (count / maxCount) * 100;
                        const statusClass = status.toLowerCase().replace(/\s+/g, '-');
                        console.log(`Rendering bar for ${status}: ${count} (${percentage}%, width: ${barWidth}%)`);
                        
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
            
            console.log('Generated chart HTML:', chartHTML);
            chartContainer.innerHTML = chartHTML;
            console.log('Chart HTML inserted successfully');
            
        } catch (error) {
            console.error('Error in updateCompatibilityChart:', error);
            chartContainer.innerHTML = `
                <div class="compatibility-error">
                    <p>Error rendering chart: ${error.message}</p>
                </div>
            `;
        }
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
