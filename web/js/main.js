/**
 * Linux Hardware Compatibility Database - Main Application
 * 
 * Main application controller that orchestrates all components
 */

class HardwareCompatibilityApp {
    constructor() {
        this.searchEngine = null;
        this.searchUI = null;
        this.statsDashboard = null;
        this.isInitialized = false;
        
        // Configuration
        this.config = {
            baseUrl: './',
            searchOptions: {
                maxResults: 50,
                enableFuzzySearch: true,
                enableTypeahead: true,
                fuzzyMatchThreshold: 0.6
            },
            uiOptions: {
                containerId: 'search-container',
                resultsPerPage: 20,
                enableHistory: true,
                enableFilters: true,
                enableExport: true
            },
            statsOptions: {
                containerId: 'stats-container',
                chartContainerId: 'compatibility-chart',
                enableCharts: true,
                refreshInterval: 300000 // 5 minutes
            }
        };
        
        this.initializeApp();
    }
    
    /**
     * Initialize the application
     */
    async initializeApp() {
        try {
            // Show loading state
            this.showGlobalLoading('Initializing hardware compatibility database...');
            
            // Initialize search engine
            this.searchEngine = new HardwareSearchEngine({
                baseUrl: this.config.baseUrl,
                ...this.config.searchOptions
            });
            
            // Wait for critical indices to load
            await this.waitForCriticalData();
            
            // Initialize UI components
            this.initializeComponents();
            
            // Set up global event listeners
            this.setupGlobalEvents();
            
            // Hide loading state
            this.hideGlobalLoading();
            
            // Mark as initialized
            this.isInitialized = true;
            
            // Trigger ready event
            this.triggerReadyEvent();
            
            console.log('✅ Hardware Compatibility Database initialized successfully');
            
        } catch (error) {
            console.error('❌ Failed to initialize application:', error);
            this.showGlobalError('Failed to initialize the database. Please refresh the page.');
        }
    }
    
    /**
     * Wait for critical data to load
     */
    async waitForCriticalData() {
        const timeout = new Promise((_, reject) => 
            setTimeout(() => reject(new Error('Timeout loading critical data')), 10000)
        );
        
        try {
            // Wait for search engine to be ready with a timeout
            await Promise.race([
                this.searchEngine.preloadCriticalIndices(),
                timeout
            ]);
            
        } catch (error) {
            console.warn('Some critical data failed to load, but continuing:', error);
            // Don't throw - allow app to continue with limited functionality
        }
    }
    
    /**
     * Initialize UI components
     */
    initializeComponents() {
        try {
            // Initialize search UI
            this.searchUI = new HardwareSearchUI(this.searchEngine, this.config.uiOptions);
            
            // Initialize statistics dashboard
            this.statsDashboard = new StatsDashboard(this.searchEngine, this.config.statsOptions);
            
            // Set up component communication
            this.setupComponentCommunication();
            
        } catch (error) {
            console.error('Failed to initialize UI components:', error);
            throw error;
        }
    }
    
    /**
     * Set up communication between components
     */
    setupComponentCommunication() {
        // Listen for search events from stats dashboard
        document.addEventListener('statsFilterSearch', (event) => {
            const { filter, value } = event.detail;
            this.handleStatsFilterSearch(filter, value);
        });
        
        // Listen for search completion to update stats if needed
        this.searchEngine.on('searchCompleted', (event) => {
            this.handleSearchCompleted(event.detail);
        });
        
        // Listen for index loading events
        this.searchEngine.on('indexLoaded', (event) => {
            this.handleIndexLoaded(event.detail);
        });
    }
    
    /**
     * Set up global event listeners
     */
    setupGlobalEvents() {
        // Keyboard shortcuts
        document.addEventListener('keydown', this.handleGlobalKeydown.bind(this));
        
        // Handle browser back/forward
        window.addEventListener('popstate', this.handlePopState.bind(this));
        
        // Handle visibility changes (for pausing/resuming auto-refresh)
        document.addEventListener('visibilitychange', this.handleVisibilityChange.bind(this));
        
        // Handle online/offline events
        window.addEventListener('online', this.handleOnline.bind(this));
        window.addEventListener('offline', this.handleOffline.bind(this));
        
        // Handle smooth scrolling for anchor links
        this.setupSmoothScrolling();
        
        // Handle theme changes
        this.setupThemeHandling();
    }
    
    /**
     * Handle filter search from stats dashboard
     */
    handleStatsFilterSearch(filter, value) {
        if (!this.searchUI) return;
        
        // Scroll to search section
        const searchSection = document.getElementById('search');
        if (searchSection) {
            searchSection.scrollIntoView({ behavior: 'smooth' });
        }
        
        // Set up the search query based on filter type
        let searchQuery = '';
        switch (filter) {
            case 'compatibility':
                searchQuery = `compatibility:${value}`;
                break;
            case 'vendor':
                searchQuery = value;
                break;
            case 'component':
                searchQuery = `type:${value}`;
                break;
            default:
                searchQuery = value;
        }
        
        // Perform the search
        setTimeout(() => {
            const searchInput = document.getElementById('hw-search-input');
            if (searchInput) {
                searchInput.value = searchQuery;
                searchInput.dispatchEvent(new Event('input'));
                this.searchUI.handleSearch();
            }
        }, 500); // Small delay to allow scroll to complete
    }
    
    /**
     * Handle search completion
     */
    handleSearchCompleted(searchResult) {
        // Update URL with search query if history is enabled
        if (this.config.uiOptions.enableHistory && searchResult.metadata.query) {
            this.updateUrlWithSearch(searchResult.metadata.query);
        }
        
        // Track search analytics (if implemented)
        this.trackSearchAnalytics(searchResult);
    }
    
    /**
     * Handle index loading events
     */
    handleIndexLoaded({ indexName, data }) {
        console.log(`📚 Loaded ${indexName} index with ${Object.keys(data).length} entries`);
        
        // Update UI state to show available features
        this.updateFeatureAvailability();
    }
    
    /**
     * Handle global keyboard shortcuts
     */
    handleGlobalKeydown(event) {
        // Ctrl/Cmd + K to focus search
        if ((event.ctrlKey || event.metaKey) && event.key === 'k') {
            event.preventDefault();
            this.focusSearch();
        }
        
        // Escape to clear search or close modals
        if (event.key === 'Escape') {
            this.handleEscapeKey();
        }
    }
    
    /**
     * Handle browser back/forward navigation
     */
    handlePopState(event) {
        if (event.state && event.state.query) {
            const searchInput = document.getElementById('hw-search-input');
            if (searchInput) {
                searchInput.value = event.state.query;
                this.searchUI.performSearch(event.state.query);
            }
        }
    }
    
    /**
     * Handle visibility change (tab switching, minimizing)
     */
    handleVisibilityChange() {
        if (document.hidden) {
            // Pause auto-refresh timers when tab is not visible
            if (this.statsDashboard && this.statsDashboard.refreshTimer) {
                clearInterval(this.statsDashboard.refreshTimer);
            }
        } else {
            // Resume auto-refresh when tab becomes visible
            if (this.statsDashboard && this.config.statsOptions.refreshInterval > 0) {
                this.statsDashboard.refreshTimer = setInterval(() => {
                    this.statsDashboard.refreshStatistics();
                }, this.config.statsOptions.refreshInterval);
            }
        }
    }
    
    /**
     * Handle online event
     */
    handleOnline() {
        console.log('🟢 Connection restored');
        this.hideConnectionMessage();
        
        // Retry loading failed indices
        if (this.searchEngine) {
            this.searchEngine.clearCache();
        }
    }
    
    /**
     * Handle offline event
     */
    handleOffline() {
        console.log('🔴 Connection lost');
        this.showConnectionMessage('You are currently offline. Some features may not work properly.');
    }
    
    /**
     * Set up smooth scrolling for navigation links
     */
    setupSmoothScrolling() {
        document.querySelectorAll('a[href^="#"]').forEach(link => {
            link.addEventListener('click', (event) => {
                const target = document.querySelector(link.getAttribute('href'));
                if (target) {
                    event.preventDefault();
                    target.scrollIntoView({
                        behavior: 'smooth',
                        block: 'start'
                    });
                    
                    // Update URL
                    history.pushState(null, '', link.getAttribute('href'));
                }
            });
        });
    }
    
    /**
     * Set up theme handling
     */
    setupThemeHandling() {
        // Listen for system theme changes
        const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
        mediaQuery.addEventListener('change', this.handleThemeChange.bind(this));
        
        // Apply initial theme
        this.handleThemeChange(mediaQuery);
    }
    
    /**
     * Handle theme changes
     */
    handleThemeChange(mediaQuery) {
        const isDark = mediaQuery.matches;
        document.documentElement.setAttribute('data-theme', isDark ? 'dark' : 'light');
        
        // Update any theme-dependent components
        this.updateComponentThemes(isDark);
    }
    
    /**
     * Focus the search input
     */
    focusSearch() {
        const searchInput = document.getElementById('hw-search-input');
        if (searchInput) {
            searchInput.focus();
            searchInput.select();
        }
    }
    
    /**
     * Handle escape key
     */
    handleEscapeKey() {
        // Clear search if search input is focused
        const searchInput = document.getElementById('hw-search-input');
        if (document.activeElement === searchInput) {
            if (this.searchUI) {
                this.searchUI.handleClear();
            }
        }
        
        // Close any open modals or dropdowns
        this.closeOpenOverlays();
    }
    
    /**
     * Show global loading state
     */
    showGlobalLoading(message) {
        const loadingHTML = `
            <div id="global-loading" class="global-loading">
                <div class="loading-content">
                    <div class="loading-spinner">
                        <svg class="animate-spin" viewBox="0 0 24 24">
                            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4" fill="none"/>
                            <path class="opacity-75" fill="currentColor" d="m4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"/>
                        </svg>
                    </div>
                    <div class="loading-message">${message}</div>
                </div>
            </div>
        `;
        
        document.body.insertAdjacentHTML('beforeend', loadingHTML);
    }
    
    /**
     * Hide global loading state
     */
    hideGlobalLoading() {
        const loading = document.getElementById('global-loading');
        if (loading) {
            loading.remove();
        }
    }
    
    /**
     * Show global error
     */
    showGlobalError(message) {
        this.hideGlobalLoading();
        
        const errorHTML = `
            <div id="global-error" class="global-error">
                <div class="error-content">
                    <div class="error-icon">⚠️</div>
                    <div class="error-message">${message}</div>
                    <button class="retry-button" onclick="location.reload()">Retry</button>
                </div>
            </div>
        `;
        
        document.body.insertAdjacentHTML('beforeend', errorHTML);
    }
    
    /**
     * Show connection message
     */
    showConnectionMessage(message) {
        // Remove existing connection message
        this.hideConnectionMessage();
        
        const messageHTML = `
            <div id="connection-message" class="connection-message">
                <div class="connection-content">
                    <span class="connection-icon">🔌</span>
                    <span class="connection-text">${message}</span>
                </div>
            </div>
        `;
        
        document.body.insertAdjacentHTML('beforeend', messageHTML);
        
        // Auto-hide after 5 seconds
        setTimeout(() => {
            this.hideConnectionMessage();
        }, 5000);
    }
    
    /**
     * Hide connection message
     */
    hideConnectionMessage() {
        const message = document.getElementById('connection-message');
        if (message) {
            message.remove();
        }
    }
    
    /**
     * Update URL with search query
     */
    updateUrlWithSearch(query) {
        const url = new URL(window.location);
        if (query) {
            url.searchParams.set('q', query);
        } else {
            url.searchParams.delete('q');
        }
        
        history.pushState({ query }, '', url);
    }
    
    /**
     * Track search analytics (placeholder)
     */
    trackSearchAnalytics(searchResult) {
        // Implementation would track search metrics
        // For now, just log to console
        if (searchResult.metadata.total > 0) {
            console.log(`🔍 Search: "${searchResult.metadata.query}" found ${searchResult.metadata.total} results`);
        }
    }
    
    /**
     * Update feature availability based on loaded indices
     */
    updateFeatureAvailability() {
        // Enable/disable features based on what data is available
        const availableIndices = Array.from(this.searchEngine.loadedIndices);
        
        // Update UI to reflect available features
        document.querySelectorAll('[data-requires-index]').forEach(element => {
            const requiredIndex = element.dataset.requiresIndex;
            element.classList.toggle('feature-available', availableIndices.includes(requiredIndex));
        });
    }
    
    /**
     * Update component themes
     */
    updateComponentThemes(isDark) {
        // Notify components of theme change
        const themeEvent = new CustomEvent('themeChanged', {
            detail: { isDark, theme: isDark ? 'dark' : 'light' }
        });
        document.dispatchEvent(themeEvent);
    }
    
    /**
     * Close open overlays
     */
    closeOpenOverlays() {
        // Hide suggestions
        const suggestions = document.getElementById('hw-suggestions');
        if (suggestions) {
            suggestions.classList.add('hidden');
        }
        
        // Close any other open dropdowns or modals
        document.querySelectorAll('.dropdown-open, .modal-open').forEach(element => {
            element.classList.remove('dropdown-open', 'modal-open');
        });
    }
    
    /**
     * Trigger application ready event
     */
    triggerReadyEvent() {
        const readyEvent = new CustomEvent('hwdbReady', {
            detail: {
                app: this,
                searchEngine: this.searchEngine,
                searchUI: this.searchUI,
                statsDashboard: this.statsDashboard
            }
        });
        document.dispatchEvent(readyEvent);
        
        // Also trigger on window for external scripts
        window.dispatchEvent(new CustomEvent('hwdbReady', {
            detail: { app: this }
        }));
    }
    
    /**
     * Get application status
     */
    getStatus() {
        return {
            isInitialized: this.isInitialized,
            searchEngine: !!this.searchEngine,
            searchUI: !!this.searchUI,
            statsDashboard: !!this.statsDashboard,
            loadedIndices: this.searchEngine ? Array.from(this.searchEngine.loadedIndices) : []
        };
    }
    
    /**
     * Destroy the application and clean up
     */
    destroy() {
        // Clean up components
        if (this.statsDashboard) {
            this.statsDashboard.destroy();
        }
        
        // Clear timers and event listeners
        document.removeEventListener('keydown', this.handleGlobalKeydown);
        window.removeEventListener('popstate', this.handlePopState);
        document.removeEventListener('visibilitychange', this.handleVisibilityChange);
        window.removeEventListener('online', this.handleOnline);
        window.removeEventListener('offline', this.handleOffline);
        
        // Clear references
        this.searchEngine = null;
        this.searchUI = null;
        this.statsDashboard = null;
        this.isInitialized = false;
    }
}

// Global loading styles
const globalLoadingCSS = `
    .global-loading,
    .global-error {
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: rgba(15, 23, 42, 0.9);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 9999;
        backdrop-filter: blur(4px);
    }
    
    .loading-content,
    .error-content {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;
        padding: 2rem;
        background-color: var(--bg-card);
        border-radius: 1rem;
        box-shadow: var(--shadow-xl);
        border: 1px solid var(--border-primary);
        max-width: 400px;
        width: 90%;
    }
    
    .loading-spinner svg {
        width: 3rem;
        height: 3rem;
        color: var(--color-primary);
    }
    
    .loading-message,
    .error-message {
        text-align: center;
        color: var(--text-primary);
        font-weight: 500;
    }
    
    .error-icon {
        font-size: 3rem;
    }
    
    .retry-button {
        padding: 0.5rem 1rem;
        background-color: var(--color-primary);
        color: var(--text-inverse);
        border: none;
        border-radius: 0.5rem;
        cursor: pointer;
        font-weight: 500;
        transition: background-color 0.15s ease;
    }
    
    .retry-button:hover {
        background-color: var(--color-primary-hover);
    }
    
    .connection-message {
        position: fixed;
        top: 1rem;
        right: 1rem;
        background-color: var(--color-warning);
        color: white;
        padding: 0.75rem 1rem;
        border-radius: 0.5rem;
        box-shadow: var(--shadow-lg);
        z-index: 1000;
        animation: slideInRight 0.3s ease;
    }
    
    .connection-content {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 0.875rem;
        font-weight: 500;
    }
    
    @keyframes slideInRight {
        from {
            transform: translateX(100%);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }
`;

// Inject global styles
if (typeof document !== 'undefined') {
    const style = document.createElement('style');
    style.textContent = globalLoadingCSS;
    document.head.appendChild(style);
}

// Initialize the app when DOM is ready
if (typeof document !== 'undefined') {
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            window.hwdbApp = new HardwareCompatibilityApp();
        });
    } else {
        window.hwdbApp = new HardwareCompatibilityApp();
    }
}

// Export for both browser and Node.js
if (typeof module !== 'undefined' && module.exports) {
    module.exports = HardwareCompatibilityApp;
} else if (typeof window !== 'undefined') {
    window.HardwareCompatibilityApp = HardwareCompatibilityApp;
}