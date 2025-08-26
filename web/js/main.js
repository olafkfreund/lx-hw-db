
// Main JavaScript for Linux Hardware Compatibility Database
document.addEventListener('DOMContentLoaded', () => {
    console.log('🐧 Linux Hardware Compatibility Database loaded');
    
    // Initialize all components
    initializeNavigation();
    initializeSearch();
    initializeCopyButtons();
    loadStatistics();
    initializeScrollEffects();
});

// Smooth scrolling navigation
function initializeNavigation() {
    const navLinks = document.querySelectorAll('.nav-link[href^="#"]');
    
    navLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            e.preventDefault();
            const targetId = link.getAttribute('href');
            const targetElement = document.querySelector(targetId);
            
            if (targetElement) {
                const headerOffset = 120;
                const elementPosition = targetElement.getBoundingClientRect().top;
                const offsetPosition = elementPosition + window.pageYOffset - headerOffset;
                
                window.scrollTo({
                    top: offsetPosition,
                    behavior: 'smooth'
                });
                
                // Update active nav link
                navLinks.forEach(l => l.classList.remove('active'));
                link.classList.add('active');
            }
        });
    });
}

// Search functionality
function initializeSearch() {
    const searchInput = document.querySelector('#hardware-search');
    const searchBtn = document.querySelector('.search-btn');
    const searchResults = document.querySelector('#search-results');
    
    if (searchInput && searchBtn) {
        const performSearch = () => {
            const query = searchInput.value.trim();
            if (query) {
                console.log('🔍 Searching for:', query);
                showSearchResults(query);
            }
        };
        
        searchBtn.addEventListener('click', performSearch);
        searchInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') {
                performSearch();
            }
        });
        
        // Real-time search suggestions (placeholder)
        searchInput.addEventListener('input', (e) => {
            const query = e.target.value.trim();
            if (query.length > 2) {
                // Debounced search suggestions could go here
                console.log('💡 Suggestion query:', query);
            }
        });
    }
}

function showSearchResults(query) {
    const searchResults = document.querySelector('#search-results');
    if (searchResults) {
        searchResults.innerHTML = `
            <div style="padding: 2rem; text-align: center; color: var(--fg3);">
                <div style="font-size: 3rem; margin-bottom: 1rem;">🔍</div>
                <h3>Searching for "${query}"</h3>
                <p>Search functionality is being implemented. This is a preview of the modern interface.</p>
                <div style="margin-top: 2rem; padding: 1rem; background: var(--bg0); border-radius: var(--border-radius); border-left: 3px solid var(--primary);">
                    <strong>Coming Soon:</strong> Real-time hardware compatibility search across distributions and kernel versions.
                </div>
            </div>
        `;
        searchResults.classList.remove('hidden');
    }
}

// Copy button functionality
function initializeCopyButtons() {
    const copyButtons = document.querySelectorAll('.copy-btn');
    
    copyButtons.forEach(button => {
        button.addEventListener('click', async () => {
            const codeBlock = button.closest('.code-block');
            const code = codeBlock.querySelector('code');
            
            if (code) {
                try {
                    await navigator.clipboard.writeText(code.textContent);
                    
                    // Visual feedback
                    button.style.background = 'var(--success)';
                    setTimeout(() => {
                        button.style.background = 'var(--bg2)';
                    }, 1000);
                    
                    console.log('📋 Code copied to clipboard');
                } catch (err) {
                    console.error('Failed to copy code:', err);
                }
            }
        });
    });
}

// Load and display statistics
async function loadStatistics() {
    try {
        const response = await fetch('statistics/overview.json');
        const stats = await response.json();
        
        updateStatDisplay('reports', stats.total_reports || 0);
        updateStatDisplay('systems', stats.unique_systems || 0); 
        updateStatDisplay('vendors', stats.total_vendors || 0);
        updateStatDisplay('kernels', stats.kernel_versions || 0);
        
        console.log('📊 Statistics loaded:', stats);
    } catch (error) {
        console.warn('⚠️ Could not load statistics:', error);
        // Set default values
        updateStatDisplay('reports', 1);
        updateStatDisplay('systems', 1);
        updateStatDisplay('vendors', 0);
        updateStatDisplay('kernels', 1);
    }
}

function updateStatDisplay(statName, value) {
    const elements = document.querySelectorAll(`[data-stat="${statName}"]`);
    elements.forEach(element => {
        // Animate counter from 0 to value
        animateCounter(element, 0, value, 1000);
    });
}

function animateCounter(element, start, end, duration) {
    const startTimestamp = performance.now();
    
    const step = (timestamp) => {
        const progress = Math.min((timestamp - startTimestamp) / duration, 1);
        const current = Math.floor(progress * (end - start) + start);
        element.textContent = current.toLocaleString();
        
        if (progress < 1) {
            requestAnimationFrame(step);
        }
    };
    
    requestAnimationFrame(step);
}

// Scroll effects
function initializeScrollEffects() {
    const header = document.querySelector('.site-header');
    let lastScrollY = window.scrollY;
    
    const updateHeaderOnScroll = () => {
        const currentScrollY = window.scrollY;
        
        if (currentScrollY > 100) {
            header.style.background = 'rgba(29, 32, 33, 0.95)';
            header.style.backdropFilter = 'blur(15px)';
        } else {
            header.style.background = 'rgba(29, 32, 33, 0.9)';
            header.style.backdropFilter = 'blur(10px)';
        }
        
        // Auto-hide header on scroll down, show on scroll up
        if (currentScrollY > lastScrollY && currentScrollY > 200) {
            header.style.transform = 'translateY(-100%)';
        } else {
            header.style.transform = 'translateY(0)';
        }
        
        lastScrollY = currentScrollY;
    };
    
    let ticking = false;
    window.addEventListener('scroll', () => {
        if (!ticking) {
            requestAnimationFrame(updateHeaderOnScroll);
            ticking = true;
        }
        ticking = false;
    });
}

// Utility functions
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

// Add some visual feedback for buttons
document.addEventListener('click', (e) => {
    if (e.target.matches('.btn, .nav-link, .social-link')) {
        // Create ripple effect
        const button = e.target;
        const rect = button.getBoundingClientRect();
        const ripple = document.createElement('div');
        const size = 60;
        const x = e.clientX - rect.left - size / 2;
        const y = e.clientY - rect.top - size / 2;
        
        ripple.style.cssText = `
            position: absolute;
            border-radius: 50%;
            background: rgba(251, 189, 47, 0.3);
            transform: scale(0);
            animation: ripple 0.6s linear;
            left: ${x}px;
            top: ${y}px;
            width: ${size}px;
            height: ${size}px;
            pointer-events: none;
        `;
        
        button.style.position = 'relative';
        button.style.overflow = 'hidden';
        button.appendChild(ripple);
        
        setTimeout(() => {
            ripple.remove();
        }, 600);
    }
});

// Add ripple animation to CSS
const style = document.createElement('style');
style.textContent = `
    @keyframes ripple {
        to {
            transform: scale(4);
            opacity: 0;
        }
    }
`;
document.head.appendChild(style);

console.log('🚀 All interactive features initialized');
