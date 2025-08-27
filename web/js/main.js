
// Main JavaScript for Linux Hardware Compatibility Database
document.addEventListener('DOMContentLoaded', () => {
    console.log('Linux Hardware Compatibility Database loaded');

    // Initialize global functionality
    initializeMobileNavigation();
    initializeSmoothScrolling();
    initializeHeaderScroll();
});

// Mobile Navigation Functionality
function initializeMobileNavigation() {
    const mobileMenuToggle = document.querySelector('.mobile-menu-toggle');
    const headerNav = document.querySelector('.header-nav');
    const navLinks = document.querySelectorAll('.nav-link');

    if (!mobileMenuToggle || !headerNav) {
        return; // Elements not found, skip initialization
    }

    // Toggle mobile menu
    mobileMenuToggle.addEventListener('click', () => {
        const isExpanded = mobileMenuToggle.getAttribute('aria-expanded') === 'true';
        
        // Toggle aria-expanded attribute
        mobileMenuToggle.setAttribute('aria-expanded', !isExpanded);
        
        // Toggle mobile navigation class
        headerNav.classList.toggle('mobile-nav-open', !isExpanded);
        
        console.log('Mobile menu toggled:', !isExpanded);
    });

    // Close mobile menu when clicking on nav links
    navLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            // Only close for internal links (not external GitHub link)
            if (!link.hasAttribute('target')) {
                mobileMenuToggle.setAttribute('aria-expanded', 'false');
                headerNav.classList.remove('mobile-nav-open');
                console.log('Mobile menu closed via nav link');
            }
        });
    });

    // Close mobile menu when clicking outside
    document.addEventListener('click', (e) => {
        const isClickInsideNav = headerNav.contains(e.target);
        const isClickOnToggle = mobileMenuToggle.contains(e.target);
        
        if (!isClickInsideNav && !isClickOnToggle && headerNav.classList.contains('mobile-nav-open')) {
            mobileMenuToggle.setAttribute('aria-expanded', 'false');
            headerNav.classList.remove('mobile-nav-open');
            console.log('Mobile menu closed via outside click');
        }
    });

    // Close mobile menu on escape key
    document.addEventListener('keydown', (e) => {
        if (e.key === 'Escape' && headerNav.classList.contains('mobile-nav-open')) {
            mobileMenuToggle.setAttribute('aria-expanded', 'false');
            headerNav.classList.remove('mobile-nav-open');
            mobileMenuToggle.focus();
            console.log('Mobile menu closed via escape key');
        }
    });

    // Handle window resize
    window.addEventListener('resize', () => {
        if (window.innerWidth > 768) {
            // Reset mobile menu on larger screens
            mobileMenuToggle.setAttribute('aria-expanded', 'false');
            headerNav.classList.remove('mobile-nav-open');
        }
    });
}

// Smooth Scrolling for Internal Links
function initializeSmoothScrolling() {
    const internalLinks = document.querySelectorAll('a[href^="#"]');
    
    internalLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            const targetId = link.getAttribute('href').slice(1);
            const targetElement = document.getElementById(targetId);
            
            if (targetElement) {
                e.preventDefault();
                
                // Calculate offset for fixed header
                const headerHeight = document.querySelector('.site-header')?.offsetHeight || 0;
                const targetPosition = targetElement.offsetTop - headerHeight - 20;
                
                window.scrollTo({
                    top: targetPosition,
                    behavior: 'smooth'
                });
                
                console.log('Smooth scrolling to:', targetId);
            }
        });
    });
}

// Header Scroll Effects
function initializeHeaderScroll() {
    const header = document.querySelector('.site-header');
    if (!header) return;
    
    let lastScrollY = window.scrollY;
    let ticking = false;
    
    const updateHeader = () => {
        const currentScrollY = window.scrollY;
        
        // Add/remove scrolled class based on scroll position
        if (currentScrollY > 50) {
            header.classList.add('scrolled');
        } else {
            header.classList.remove('scrolled');
        }
        
        lastScrollY = currentScrollY;
        ticking = false;
    };
    
    const onScroll = () => {
        if (!ticking) {
            requestAnimationFrame(updateHeader);
            ticking = true;
        }
    };
    
    window.addEventListener('scroll', onScroll);
}

// Legacy search function for compatibility
function initializeSearch() {
    // This function is now handled by search-ui.js
    // Keeping for backward compatibility
    console.log('Search initialization handled by search-ui.js');
}
