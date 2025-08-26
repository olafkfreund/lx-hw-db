/**
 * Linux Hardware Compatibility Database - Statistics Dashboard
 * 
 * Dashboard component for displaying database statistics and trends
 */

class StatsDashboard {
    constructor(searchEngine, options = {}) {
        this.searchEngine = searchEngine;
        this.options = {
            containerId: options.containerId || 'stats-container',
            chartContainerId: options.chartContainerId || 'compatibility-chart',
            enableCharts: options.enableCharts !== false,
            refreshInterval: options.refreshInterval || 300000, // 5 minutes
            ...options
        };
        
        this.stats = null;
        this.refreshTimer = null;
        
        this.initializeDashboard();
    }
    
    /**
     * Initialize the statistics dashboard
     */
    async initializeDashboard() {
        await this.loadStatistics();
        this.renderStatistics();
        
        if (this.options.enableCharts) {
            this.renderCompatibilityChart();
        }
        
        // Set up auto-refresh
        if (this.options.refreshInterval > 0) {
            this.refreshTimer = setInterval(() => {
                this.refreshStatistics();
            }, this.options.refreshInterval);
        }
    }
    
    /**
     * Load statistics from the search engine
     */
    async loadStatistics() {
        try {
            this.stats = await this.searchEngine.getStatistics();
            return this.stats;
        } catch (error) {
            console.error('Failed to load statistics:', error);
            this.showError('Failed to load statistics');
            throw error;
        }
    }
    
    /**
     * Refresh statistics data
     */
    async refreshStatistics() {
        try {
            await this.loadStatistics();
            this.renderStatistics();
            
            if (this.options.enableCharts) {
                this.renderCompatibilityChart();
            }
        } catch (error) {
            console.warn('Failed to refresh statistics:', error);
        }
    }
    
    /**
     * Render the main statistics cards
     */
    renderStatistics() {
        const container = document.getElementById(this.options.containerId);
        if (!container || !this.stats) return;
        
        const statsCards = [
            {
                value: this.formatNumber(this.stats.total_reports || 0),
                label: 'Hardware Reports',
                icon: '📊',
                color: 'var(--color-primary)'
            },
            {
                value: this.formatNumber(this.stats.unique_systems || 0),
                label: 'Unique Systems',
                icon: '💻',
                color: 'var(--color-info)'
            },
            {
                value: this.formatNumber(this.stats.total_vendors || 0),
                label: 'Hardware Vendors',
                icon: '🏭',
                color: 'var(--color-success)'
            },
            {
                value: this.formatNumber(this.stats.kernel_versions || 0),
                label: 'Kernel Versions',
                icon: '🐧',
                color: 'var(--color-warning)'
            }
        ];
        
        container.innerHTML = statsCards.map(card => `
            <div class="stat-card" style="border-left: 4px solid ${card.color}">
                <div class="stat-header">
                    <span class="stat-icon">${card.icon}</span>
                </div>
                <div class="stat-value" style="color: ${card.color}">${card.value}</div>
                <div class="stat-label">${card.label}</div>
                ${this.renderStatTrend(card.label.toLowerCase())}
            </div>
        `).join('');
        
        // Add additional stats if available
        this.renderAdditionalStats(container);
    }
    
    /**
     * Render additional statistics
     */
    renderAdditionalStats(container) {
        if (!this.stats) return;
        
        // Add component types and distributions stats
        const additionalStats = [];
        
        if (this.stats.component_types) {
            additionalStats.push({
                value: this.formatNumber(this.stats.component_types),
                label: 'Component Types',
                icon: '⚡',
                color: 'var(--color-secondary)'
            });
        }
        
        if (this.stats.distributions) {
            additionalStats.push({
                value: this.formatNumber(this.stats.distributions),
                label: 'Linux Distributions',
                icon: '🐧',
                color: 'var(--color-primary)'
            });
        }
        
        // Add health score if available
        if (this.stats.health_score && typeof this.stats.health_score === 'function') {
            const healthScore = this.stats.health_score();
            additionalStats.push({
                value: `${healthScore}/100`,
                label: 'Database Health',
                icon: healthScore >= 80 ? '💚' : healthScore >= 60 ? '💛' : '❤️',
                color: healthScore >= 80 ? 'var(--color-success)' : 
                       healthScore >= 60 ? 'var(--color-warning)' : 'var(--color-error)'
            });
        }
        
        if (additionalStats.length > 0) {
            const additionalHTML = additionalStats.map(card => `
                <div class="stat-card additional-stat" style="border-left: 4px solid ${card.color}">
                    <div class="stat-header">
                        <span class="stat-icon">${card.icon}</span>
                    </div>
                    <div class="stat-value" style="color: ${card.color}">${card.value}</div>
                    <div class="stat-label">${card.label}</div>
                </div>
            `).join('');
            
            container.insertAdjacentHTML('beforeend', additionalHTML);
        }
    }
    
    /**
     * Render trend indicator for a stat
     */
    renderStatTrend(statType) {
        if (!this.stats.growth_stats || this.stats.growth_stats.length < 2) {
            return '';
        }
        
        const trend = this.calculateTrend(statType);
        if (!trend) return '';
        
        const trendClass = trend.direction === 'up' ? 'trend-up' : 
                          trend.direction === 'down' ? 'trend-down' : 'trend-stable';
        
        const trendIcon = trend.direction === 'up' ? '↗️' : 
                         trend.direction === 'down' ? '↘️' : '→';
        
        return `
            <div class="stat-trend ${trendClass}">
                <span class="trend-icon">${trendIcon}</span>
                <span class="trend-value">${trend.percentage}%</span>
            </div>
        `;
    }
    
    /**
     * Render compatibility overview chart
     */
    renderCompatibilityChart() {
        const container = document.getElementById(this.options.chartContainerId);
        if (!container || !this.stats || !this.stats.compatibility_overview) return;
        
        const compatibilityData = this.stats.compatibility_overview;
        const total = Object.values(compatibilityData).reduce((sum, count) => sum + count, 0);
        
        if (total === 0) {
            container.innerHTML = '<p class="no-data">No compatibility data available</p>';
            return;
        }
        
        // Create simple bar chart
        const chartHTML = Object.entries(compatibilityData)
            .sort(([, a], [, b]) => b - a)
            .map(([status, count]) => {
                const percentage = ((count / total) * 100).toFixed(1);
                const statusClass = status.toLowerCase();
                
                return `
                    <div class="chart-bar" data-status="${status}">
                        <div class="chart-label">
                            <span class="status-badge ${statusClass}">${this.formatCompatibilityStatus(status)}</span>
                            <span class="count">${this.formatNumber(count)} reports</span>
                        </div>
                        <div class="chart-bar-container">
                            <div class="chart-bar-fill ${statusClass}" style="width: ${percentage}%"></div>
                        </div>
                        <div class="chart-percentage">${percentage}%</div>
                    </div>
                `;
            })
            .join('');
        
        container.innerHTML = `
            <div class="compatibility-chart">
                <div class="chart-title">
                    <h4>Compatibility Distribution</h4>
                    <p class="chart-subtitle">Based on ${this.formatNumber(total)} reports</p>
                </div>
                <div class="chart-bars">
                    ${chartHTML}
                </div>
                ${this.renderTopHardwareList()}
            </div>
        `;
        
        // Add chart interactions
        this.bindChartEvents();
    }
    
    /**
     * Render top hardware list
     */
    renderTopHardwareList() {
        if (!this.stats.top_hardware || this.stats.top_hardware.length === 0) {
            return '';
        }
        
        const topHardware = this.stats.top_hardware.slice(0, 5);
        
        const hardwareHTML = topHardware.map((hw, index) => `
            <div class="top-hardware-item">
                <div class="rank">${index + 1}</div>
                <div class="hardware-info">
                    <div class="hardware-name">${this.escapeHtml(hw.vendor)} ${this.escapeHtml(hw.model)}</div>
                    <div class="hardware-meta">
                        ${this.formatNumber(hw.report_count)} reports • 
                        ${hw.avg_compatibility.toFixed(1)} compatibility score
                    </div>
                </div>
                <div class="compatibility-badge ${this.getCompatibilityClass(hw.avg_compatibility)}">
                    ${this.getCompatibilityText(hw.avg_compatibility)}
                </div>
            </div>
        `).join('');
        
        return `
            <div class="top-hardware-section">
                <h4>Most Reported Hardware</h4>
                <div class="top-hardware-list">
                    ${hardwareHTML}
                </div>
            </div>
        `;
    }
    
    /**
     * Bind chart interaction events
     */
    bindChartEvents() {
        const chartBars = document.querySelectorAll('.chart-bar');
        
        chartBars.forEach(bar => {
            bar.addEventListener('mouseenter', this.handleChartBarHover.bind(this));
            bar.addEventListener('mouseleave', this.handleChartBarLeave.bind(this));
            bar.addEventListener('click', this.handleChartBarClick.bind(this));
        });
    }
    
    /**
     * Handle chart bar hover
     */
    handleChartBarHover(event) {
        const bar = event.currentTarget;
        bar.classList.add('hovered');
        
        // Show tooltip with additional info
        this.showChartTooltip(bar);
    }
    
    /**
     * Handle chart bar leave
     */
    handleChartBarLeave(event) {
        const bar = event.currentTarget;
        bar.classList.remove('hovered');
        this.hideChartTooltip();
    }
    
    /**
     * Handle chart bar click
     */
    handleChartBarClick(event) {
        const bar = event.currentTarget;
        const status = bar.dataset.status;
        
        // Trigger search for this compatibility status
        if (this.searchEngine && status) {
            const searchEvent = new CustomEvent('statsFilterSearch', {
                detail: { filter: 'compatibility', value: status.toLowerCase() }
            });
            document.dispatchEvent(searchEvent);
        }
    }
    
    /**
     * Show chart tooltip
     */
    showChartTooltip(bar) {
        // Implementation for showing detailed tooltip
        // This would show additional information about the compatibility status
    }
    
    /**
     * Hide chart tooltip
     */
    hideChartTooltip() {
        // Implementation for hiding tooltip
    }
    
    /**
     * Show error message
     */
    showError(message) {
        const container = document.getElementById(this.options.containerId);
        if (container) {
            container.innerHTML = `
                <div class="stats-error">
                    <div class="error-icon">⚠️</div>
                    <div class="error-message">${message}</div>
                    <button class="retry-button" onclick="location.reload()">Retry</button>
                </div>
            `;
        }
    }
    
    // Utility methods
    
    formatNumber(num) {
        if (num >= 1000000) {
            return (num / 1000000).toFixed(1) + 'M';
        } else if (num >= 1000) {
            return (num / 1000).toFixed(1) + 'K';
        }
        return num.toLocaleString();
    }
    
    formatCompatibilityStatus(status) {
        return status.replace(/([A-Z])/g, ' $1')
            .replace(/^./, str => str.toUpperCase())
            .trim();
    }
    
    getCompatibilityClass(score) {
        if (score >= 90) return 'excellent';
        if (score >= 75) return 'good';
        if (score >= 50) return 'fair';
        return 'poor';
    }
    
    getCompatibilityText(score) {
        if (score >= 90) return 'Excellent';
        if (score >= 75) return 'Good';
        if (score >= 50) return 'Fair';
        if (score > 0) return 'Poor';
        return 'Unknown';
    }
    
    calculateTrend(statType) {
        if (!this.stats.growth_stats || this.stats.growth_stats.length < 2) {
            return null;
        }
        
        const recent = this.stats.growth_stats.slice(-2);
        const [previous, current] = recent;
        
        if (!previous.total_reports || !current.total_reports) {
            return null;
        }
        
        const change = ((current.total_reports - previous.total_reports) / previous.total_reports) * 100;
        
        return {
            direction: change > 1 ? 'up' : change < -1 ? 'down' : 'stable',
            percentage: Math.abs(change).toFixed(1)
        };
    }
    
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
    
    /**
     * Destroy the dashboard and clean up
     */
    destroy() {
        if (this.refreshTimer) {
            clearInterval(this.refreshTimer);
            this.refreshTimer = null;
        }
    }
}

// Additional CSS for the stats dashboard (to be added to styles.css)
const additionalStatsCSS = `
    .stat-card.additional-stat {
        opacity: 0.9;
    }
    
    .stat-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: var(--space-sm);
    }
    
    .stat-icon {
        font-size: 1.5rem;
    }
    
    .stat-trend {
        display: flex;
        align-items: center;
        gap: var(--space-xs);
        font-size: 0.75rem;
        font-weight: 500;
        margin-top: var(--space-sm);
    }
    
    .trend-up { color: var(--color-success); }
    .trend-down { color: var(--color-error); }
    .trend-stable { color: var(--text-tertiary); }
    
    .compatibility-chart {
        width: 100%;
    }
    
    .chart-title {
        text-align: center;
        margin-bottom: var(--space-lg);
    }
    
    .chart-subtitle {
        color: var(--text-secondary);
        font-size: 0.875rem;
        margin: 0;
    }
    
    .chart-bars {
        display: grid;
        gap: var(--space-md);
    }
    
    .chart-bar {
        display: grid;
        grid-template-columns: 200px 1fr 60px;
        align-items: center;
        gap: var(--space-md);
        padding: var(--space-sm);
        border-radius: var(--radius-md);
        transition: background-color var(--transition-fast);
        cursor: pointer;
    }
    
    .chart-bar:hover,
    .chart-bar.hovered {
        background-color: var(--bg-secondary);
    }
    
    .chart-label {
        display: flex;
        flex-direction: column;
        gap: var(--space-xs);
    }
    
    .status-badge {
        font-size: 0.75rem;
        font-weight: 500;
        padding: var(--space-xs) var(--space-sm);
        border-radius: var(--radius-full);
        text-transform: uppercase;
    }
    
    .count {
        font-size: 0.75rem;
        color: var(--text-secondary);
    }
    
    .chart-bar-container {
        height: 8px;
        background-color: var(--bg-tertiary);
        border-radius: var(--radius-full);
        overflow: hidden;
    }
    
    .chart-bar-fill {
        height: 100%;
        border-radius: var(--radius-full);
        transition: width var(--transition-slow);
    }
    
    .chart-bar-fill.excellent { background-color: var(--color-success); }
    .chart-bar-fill.good { background-color: var(--color-info); }
    .chart-bar-fill.fair { background-color: var(--color-warning); }
    .chart-bar-fill.poor { background-color: var(--color-error); }
    .chart-bar-fill.unknown { background-color: var(--color-secondary); }
    
    .chart-percentage {
        font-size: 0.875rem;
        font-weight: 500;
        text-align: right;
    }
    
    .top-hardware-section {
        margin-top: var(--space-2xl);
    }
    
    .top-hardware-list {
        display: grid;
        gap: var(--space-sm);
        margin-top: var(--space-md);
    }
    
    .top-hardware-item {
        display: flex;
        align-items: center;
        gap: var(--space-md);
        padding: var(--space-sm) var(--space-md);
        background-color: var(--bg-secondary);
        border-radius: var(--radius-md);
        transition: background-color var(--transition-fast);
    }
    
    .top-hardware-item:hover {
        background-color: var(--bg-tertiary);
    }
    
    .rank {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 1.5rem;
        height: 1.5rem;
        background-color: var(--color-primary);
        color: var(--text-inverse);
        border-radius: var(--radius-full);
        font-size: 0.75rem;
        font-weight: 700;
        flex-shrink: 0;
    }
    
    .hardware-info {
        flex: 1;
    }
    
    .hardware-name {
        font-weight: 500;
        margin-bottom: var(--space-xs);
    }
    
    .hardware-meta {
        font-size: 0.75rem;
        color: var(--text-secondary);
    }
    
    .stats-error {
        text-align: center;
        padding: var(--space-2xl);
        color: var(--text-secondary);
    }
    
    .error-icon {
        font-size: 2rem;
        margin-bottom: var(--space-md);
    }
    
    .retry-button {
        margin-top: var(--space-md);
        padding: var(--space-sm) var(--space-md);
        background-color: var(--color-primary);
        color: var(--text-inverse);
        border: none;
        border-radius: var(--radius-md);
        cursor: pointer;
        transition: background-color var(--transition-fast);
    }
    
    .retry-button:hover {
        background-color: var(--color-primary-hover);
    }
    
    .no-data {
        text-align: center;
        color: var(--text-secondary);
        font-style: italic;
        padding: var(--space-xl);
    }
    
    @media (max-width: 768px) {
        .chart-bar {
            grid-template-columns: 1fr;
            gap: var(--space-sm);
        }
        
        .chart-percentage {
            text-align: left;
        }
    }
`;

// Inject additional CSS
if (typeof document !== 'undefined') {
    const style = document.createElement('style');
    style.textContent = additionalStatsCSS;
    document.head.appendChild(style);
}

// Export for both browser and Node.js
if (typeof module !== 'undefined' && module.exports) {
    module.exports = StatsDashboard;
} else if (typeof window !== 'undefined') {
    window.StatsDashboard = StatsDashboard;
}