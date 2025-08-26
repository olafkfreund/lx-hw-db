
// Main JavaScript for Linux Hardware Compatibility Database
document.addEventListener('DOMContentLoaded', async () => {
    console.log('Linux Hardware Compatibility Database loaded');

    // Wait for database indexer to complete
    document.addEventListener('databaseIndexed', async () => {
        // Initialize search functionality
        await initializeSearch();
        
        // Initialize statistics dashboard
        await initializeStatsDashboard();
    });

    // Initialize UI components
    initializeUIComponents();
});

/**
 * Initialize search functionality
 */
async function initializeSearch() {
    try {
        // Initialize search engine
        await window.hardwareSearch.initialize();
        
        // Initialize search UI
        await window.hardwareSearchUI.initialize();
        
        console.log('Search system initialized successfully');
    } catch (error) {
        console.error('Failed to initialize search system:', error);
        
        // Show error to user
        const searchContainer = document.querySelector('.search-container');
        if (searchContainer) {
            const errorDiv = document.createElement('div');
            errorDiv.className = 'search-init-error';
            errorDiv.innerHTML = `
                <p>⚠️ Search functionality temporarily unavailable</p>
                <small>Please refresh the page to try again</small>
            `;
            searchContainer.appendChild(errorDiv);
        }
    }
}

/**
 * Initialize statistics dashboard
 */
async function initializeStatsDashboard() {
    try {
        await window.statsDashboard.initialize();
        console.log('Statistics dashboard initialized successfully');
    } catch (error) {
        console.error('Failed to initialize statistics dashboard:', error);
    }
}

/**
 * Update statistics display
 */
function updateStatistics(stats) {
    // Update total reports count
    const totalElement = document.querySelector('[data-stat="total-reports"]');
    if (totalElement) {
        animateCounter(totalElement, stats.totalReports);
    }

    // Update distribution counts
    const distributions = Object.entries(stats.distributions)
        .sort(([,a], [,b]) => b - a)
        .slice(0, 5);
    
    console.log('Top distributions:', distributions);

    // Update vendor counts
    const cpuVendors = Object.entries(stats.vendors.cpu)
        .sort(([,a], [,b]) => b - a);
    
    const gpuVendors = Object.entries(stats.vendors.gpu)
        .sort(([,a], [,b]) => b - a);

    console.log('CPU vendors:', cpuVendors);
    console.log('GPU vendors:', gpuVendors);
}

/**
 * Animate counter to target value
 */
function animateCounter(element, target) {
    const start = 0;
    const duration = 2000; // 2 seconds
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
 * Initialize other UI components
 */
function initializeUIComponents() {
    // Smooth scrolling for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });

    // Copy buttons for code examples
    document.querySelectorAll('.copy-btn').forEach(btn => {
        btn.addEventListener('click', async function() {
            const targetId = this.dataset.target;
            const targetElement = document.querySelector(targetId);
            
            if (targetElement) {
                try {
                    await navigator.clipboard.writeText(targetElement.textContent);
                    
                    // Visual feedback
                    const originalText = this.textContent;
                    this.textContent = 'Copied!';
                    this.classList.add('copied');
                    
                    setTimeout(() => {
                        this.textContent = originalText;
                        this.classList.remove('copied');
                    }, 2000);
                } catch (err) {
                    console.error('Failed to copy text:', err);
                }
            }
        });
    });

    // Initialize ripple effect for buttons
    document.querySelectorAll('.btn, .feature-card').forEach(element => {
        element.addEventListener('click', function(e) {
            const ripple = document.createElement('span');
            const rect = this.getBoundingClientRect();
            const size = Math.max(rect.width, rect.height);
            const x = e.clientX - rect.left - size / 2;
            const y = e.clientY - rect.top - size / 2;
            
            ripple.style.width = ripple.style.height = size + 'px';
            ripple.style.left = x + 'px';
            ripple.style.top = y + 'px';
            ripple.classList.add('ripple');
            
            this.appendChild(ripple);
            
            setTimeout(() => {
                ripple.remove();
            }, 600);
        });
    });

    // Initialize navbar scroll behavior
    let lastScrollTop = 0;
    const navbar = document.querySelector('.navbar');
    
    if (navbar) {
        window.addEventListener('scroll', function() {
            const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
            
            if (scrollTop > lastScrollTop && scrollTop > 100) {
                // Scrolling down
                navbar.style.transform = 'translateY(-100%)';
            } else {
                // Scrolling up
                navbar.style.transform = 'translateY(0)';
            }
            
            lastScrollTop = scrollTop <= 0 ? 0 : scrollTop;
        });
    }
}
