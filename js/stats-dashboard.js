
/**
 * Statistics Dashboard
 * Displays hardware database statistics and visualizations
 */

class StatsDashboard {
    constructor() {
        this.searchEngine = null;
        this.isInitialized = false;
        this.chartOptions = {
            responsive: true,
            maintainAspectRatio: false,
            plugins: {
                legend: {
                    display: true,
                    position: 'bottom'
                }
            }
        };
    }

    /**
     * Initialize the statistics dashboard
     */
    async initialize() {
        console.log('Initializing statistics dashboard...');
        
        // Wait for search engine to be ready
        document.addEventListener('searchEngineReady', (event) => {
            this.searchEngine = event.detail.engine;
            this.loadStatistics();
            this.isInitialized = true;
            console.log('Statistics dashboard initialized successfully');
        });

        // Initialize if search engine already exists
        if (window.hardwareSearch && window.hardwareSearch.isInitialized) {
            this.searchEngine = window.hardwareSearch;
            this.loadStatistics();
            this.isInitialized = true;
        }
    }

    /**
     * Load and display all statistics
     */
    loadStatistics() {
        if (!this.searchEngine) return;

        try {
            const stats = this.searchEngine.getStatistics();
            
            // Update basic stats cards
            this.updateBasicStats(stats);
            
            // Create visualizations
            this.createCompatibilityChart(stats.compatibility);
            this.createDistributionChart(stats.distributions);
            this.createVendorCharts(stats.vendors);
            
        } catch (error) {
            console.error('Error loading statistics:', error);
        }
    }

    /**
     * Update basic statistics cards
     */
    updateBasicStats(stats) {
        // Total reports
        const reportsElement = document.querySelector('[data-stat="reports"]');
        if (reportsElement) {
            this.animateCounter(reportsElement, stats.totalReports);
        }

        // Unique systems (approximation based on unique combinations)
        const systemsElement = document.querySelector('[data-stat="systems"]');
        if (systemsElement) {
            // Estimate unique systems based on distribution variety
            const uniqueSystems = Object.keys(stats.distributions).length * 2;
            this.animateCounter(systemsElement, uniqueSystems);
        }

        // Hardware vendors
        const vendorsElement = document.querySelector('[data-stat="vendors"]');
        if (vendorsElement) {
            const totalVendors = new Set([
                ...Object.keys(stats.vendors.cpu),
                ...Object.keys(stats.vendors.gpu)
            ]).size;
            this.animateCounter(vendorsElement, totalVendors);
        }

        // Kernel versions
        const kernelsElement = document.querySelector('[data-stat="kernels"]');
        if (kernelsElement) {
            // Count from search engine documents
            const kernelVersions = new Set();
            this.searchEngine.documents.forEach(doc => {
                kernelVersions.add(doc.system.kernel_version);
            });
            this.animateCounter(kernelsElement, kernelVersions.size);
        }

        // Remove loading class from all stat cards
        document.querySelectorAll('.stat-card').forEach(card => {
            card.classList.remove('loading');
        });
    }

    /**
     * Create compatibility status chart
     */
    createCompatibilityChart(compatibilityData) {
        const chartContainer = document.querySelector('#compatibility-chart');
        if (!chartContainer || Object.keys(compatibilityData).length === 0) return;

        // Create canvas element
        const canvas = document.createElement('canvas');
        canvas.id = 'compatibility-pie-chart';
        canvas.width = 400;
        canvas.height = 300;

        // Clear container and add canvas
        chartContainer.innerHTML = '';
        chartContainer.appendChild(canvas);

        // Prepare data for pie chart
        const labels = [];
        const data = [];
        const colors = [];
        const statusColors = {
            excellent: '#22c55e',  // Green
            good: '#3b82f6',       // Blue
            partial: '#f59e0b',    // Orange
            poor: '#ef4444'        // Red
        };

        Object.entries(compatibilityData)
            .sort(([,a], [,b]) => b - a)
            .forEach(([status, count]) => {
                labels.push(this.getStatusLabel(status));
                data.push(count);
                colors.push(statusColors[status] || '#6b7280');
            });

        // Create simple pie chart using canvas
        this.drawPieChart(canvas, {
            labels: labels,
            data: data,
            colors: colors,
            title: 'Hardware Compatibility Status'
        });
    }

    /**
     * Create distribution chart
     */
    createDistributionChart(distributionData) {
        const statsContainer = document.querySelector('#stats-container');
        if (!statsContainer || Object.keys(distributionData).length === 0) return;

        // Create distribution stats section
        const distributionSection = document.createElement('div');
        distributionSection.className = 'distribution-stats';
        distributionSection.innerHTML = `
            <h3>Top Linux Distributions</h3>
            <div class="distribution-list">
                ${Object.entries(distributionData)
                    .sort(([,a], [,b]) => b - a)
                    .slice(0, 5)
                    .map(([distro, count]) => `
                        <div class="distribution-item">
                            <span class="distro-name">${distro}</span>
                            <span class="distro-count">${count} reports</span>
                            <div class="distro-bar">
                                <div class="distro-bar-fill" style="width: ${(count / Math.max(...Object.values(distributionData))) * 100}%"></div>
                            </div>
                        </div>
                    `).join('')}
            </div>
        `;

        statsContainer.appendChild(distributionSection);
    }

    /**
     * Create vendor charts
     */
    createVendorCharts(vendorData) {
        const statsContainer = document.querySelector('#stats-container');
        if (!statsContainer) return;

        // CPU Vendors
        if (Object.keys(vendorData.cpu).length > 0) {
            const cpuSection = document.createElement('div');
            cpuSection.className = 'vendor-stats';
            cpuSection.innerHTML = `
                <h3>CPU Vendors</h3>
                <div class="vendor-list">
                    ${Object.entries(vendorData.cpu)
                        .sort(([,a], [,b]) => b - a)
                        .map(([vendor, count]) => `
                            <div class="vendor-item">
                                <span class="vendor-name">${vendor}</span>
                                <span class="vendor-count">${count}</span>
                            </div>
                        `).join('')}
                </div>
            `;
            statsContainer.appendChild(cpuSection);
        }

        // GPU Vendors  
        if (Object.keys(vendorData.gpu).length > 0) {
            const gpuSection = document.createElement('div');
            gpuSection.className = 'vendor-stats';
            gpuSection.innerHTML = `
                <h3>GPU Vendors</h3>
                <div class="vendor-list">
                    ${Object.entries(vendorData.gpu)
                        .sort(([,a], [,b]) => b - a)
                        .map(([vendor, count]) => `
                            <div class="vendor-item">
                                <span class="vendor-name">${vendor}</span>
                                <span class="vendor-count">${count}</span>
                            </div>
                        `).join('')}
                </div>
            `;
            statsContainer.appendChild(gpuSection);
        }
    }

    /**
     * Draw a simple pie chart on canvas
     */
    drawPieChart(canvas, chartData) {
        const ctx = canvas.getContext('2d');
        const centerX = canvas.width / 2;
        const centerY = canvas.height / 2;
        const radius = Math.min(centerX, centerY) - 20;

        // Calculate total
        const total = chartData.data.reduce((sum, value) => sum + value, 0);
        
        // Draw pie slices
        let currentAngle = -Math.PI / 2; // Start at top

        chartData.data.forEach((value, index) => {
            const sliceAngle = (value / total) * 2 * Math.PI;
            
            // Draw slice
            ctx.beginPath();
            ctx.moveTo(centerX, centerY);
            ctx.arc(centerX, centerY, radius, currentAngle, currentAngle + sliceAngle);
            ctx.closePath();
            ctx.fillStyle = chartData.colors[index];
            ctx.fill();
            ctx.strokeStyle = '#ffffff';
            ctx.lineWidth = 2;
            ctx.stroke();

            currentAngle += sliceAngle;
        });

        // Draw legend
        const legendX = 20;
        let legendY = 20;
        const legendItemHeight = 25;

        chartData.labels.forEach((label, index) => {
            // Legend color box
            ctx.fillStyle = chartData.colors[index];
            ctx.fillRect(legendX, legendY, 15, 15);
            ctx.strokeStyle = '#333';
            ctx.lineWidth = 1;
            ctx.strokeRect(legendX, legendY, 15, 15);

            // Legend text
            ctx.fillStyle = '#333';
            ctx.font = '14px Arial';
            ctx.fillText(`${label}: ${chartData.data[index]}`, legendX + 20, legendY + 12);

            legendY += legendItemHeight;
        });
    }

    /**
     * Animate counter to target value
     */
    animateCounter(element, target) {
        const start = 0;
        const duration = 2000;
        const startTime = Date.now();

        function update() {
            const elapsed = Date.now() - startTime;
            const progress = Math.min(elapsed / duration, 1);
            
            // Easing function for smooth animation
            const easeOut = 1 - Math.pow(1 - progress, 3);
            const current = Math.floor(start + (target - start) * easeOut);
            
            element.textContent = current.toLocaleString();
            
            if (progress < 1) {
                requestAnimationFrame(update);
            }
        }
        
        update();
    }

    /**
     * Get human-readable status label
     */
    getStatusLabel(status) {
        const labels = {
            excellent: '✅ Excellent',
            good: '👍 Good',
            partial: '⚠️ Partial',
            poor: '❌ Poor'
        };
        return labels[status] || status;
    }
}

// Global instance
window.statsDashboard = new StatsDashboard();
