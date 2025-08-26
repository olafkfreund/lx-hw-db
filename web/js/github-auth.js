/**
 * GitHub Authentication and Attribution System
 * Handles OAuth login and contributor attribution for hardware tips
 */
class GitHubAuth {
    constructor() {
        this.clientId = 'your-github-oauth-app-id'; // Configure with actual OAuth app
        this.isAuthenticated = false;
        this.userInfo = null;
        this.init();
    }

    init() {
        // Check for existing auth token in localStorage
        const token = localStorage.getItem('github_token');
        const userInfo = localStorage.getItem('github_user');
        
        if (token && userInfo) {
            this.isAuthenticated = true;
            this.userInfo = JSON.parse(userInfo);
            this.updateAuthUI();
        }

        // Check for OAuth callback
        this.handleOAuthCallback();
        this.setupEventListeners();
    }

    setupEventListeners() {
        // GitHub login button
        const loginBtn = document.getElementById('github-login-btn');
        if (loginBtn) {
            loginBtn.addEventListener('click', () => this.initiateLogin());
        }

        // Logout button
        const logoutBtn = document.getElementById('github-logout-btn');
        if (logoutBtn) {
            logoutBtn.addEventListener('click', () => this.logout());
        }

        // Attribution toggle
        const attributionToggle = document.getElementById('attribution-toggle');
        if (attributionToggle) {
            attributionToggle.addEventListener('change', (e) => {
                this.toggleAttribution(e.target.checked);
            });
        }
    }

    initiateLogin() {
        // Redirect to GitHub OAuth
        const scope = 'read:user user:email';
        const redirectUri = encodeURIComponent(window.location.origin + '/auth-callback');
        const state = this.generateRandomState();
        localStorage.setItem('oauth_state', state);

        const githubUrl = `https://github.com/login/oauth/authorize?client_id=${this.clientId}&redirect_uri=${redirectUri}&scope=${scope}&state=${state}`;
        
        // For demo purposes, we'll simulate the auth flow
        this.simulateGitHubAuth();
    }

    // Simulate GitHub authentication for demo
    simulateGitHubAuth() {
        const demoUsers = [
            {
                login: 'linux-enthusiast',
                name: 'Linux Enthusiast',
                email: 'enthusiast@example.com',
                avatar_url: 'https://avatars.githubusercontent.com/u/1?v=4',
                bio: 'Open source contributor',
                public_repos: 42,
                followers: 123
            },
            {
                login: 'hardware-guru',
                name: 'Hardware Guru',
                email: 'guru@example.com',
                avatar_url: 'https://avatars.githubusercontent.com/u/2?v=4',
                bio: 'System administrator and hardware expert',
                public_repos: 28,
                followers: 89
            },
            {
                login: 'nixos-wizard',
                name: 'NixOS Wizard',
                email: 'wizard@example.com',
                avatar_url: 'https://avatars.githubusercontent.com/u/3?v=4',
                bio: 'NixOS configuration specialist',
                public_repos: 15,
                followers: 67
            }
        ];

        // Simulate random user selection for demo
        const randomUser = demoUsers[Math.floor(Math.random() * demoUsers.length)];
        
        // Store auth info
        localStorage.setItem('github_token', 'demo_token_' + Date.now());
        localStorage.setItem('github_user', JSON.stringify(randomUser));
        
        this.isAuthenticated = true;
        this.userInfo = randomUser;
        this.updateAuthUI();
        
        // Show success message
        this.showNotification('Successfully signed in with GitHub!', 'success');
    }

    handleOAuthCallback() {
        const urlParams = new URLSearchParams(window.location.search);
        const code = urlParams.get('code');
        const state = urlParams.get('state');
        
        if (code && state) {
            const storedState = localStorage.getItem('oauth_state');
            if (state === storedState) {
                this.exchangeCodeForToken(code);
            }
            // Clean up URL
            window.history.replaceState({}, document.title, window.location.pathname);
        }
    }

    async exchangeCodeForToken(code) {
        try {
            // In production, this would be handled by your backend
            // For demo, we'll simulate the process
            this.simulateGitHubAuth();
        } catch (error) {
            console.error('GitHub authentication failed:', error);
            this.showNotification('GitHub authentication failed. Please try again.', 'error');
        }
    }

    logout() {
        localStorage.removeItem('github_token');
        localStorage.removeItem('github_user');
        localStorage.removeItem('attribution_enabled');
        
        this.isAuthenticated = false;
        this.userInfo = null;
        this.updateAuthUI();
        
        this.showNotification('Successfully signed out', 'info');
    }

    updateAuthUI() {
        const loginSection = document.getElementById('github-login-section');
        const userSection = document.getElementById('github-user-section');
        const attributionSection = document.getElementById('attribution-section');
        
        if (this.isAuthenticated && this.userInfo) {
            // Hide login, show user info
            if (loginSection) loginSection.style.display = 'none';
            if (userSection) {
                userSection.style.display = 'block';
                this.updateUserDisplay();
            }
            if (attributionSection) attributionSection.style.display = 'block';
        } else {
            // Show login, hide user info
            if (loginSection) loginSection.style.display = 'block';
            if (userSection) userSection.style.display = 'none';
            if (attributionSection) attributionSection.style.display = 'none';
        }
    }

    updateUserDisplay() {
        const avatar = document.getElementById('user-avatar');
        const name = document.getElementById('user-name');
        const username = document.getElementById('user-username');
        const stats = document.getElementById('user-stats');
        
        if (avatar) avatar.src = this.userInfo.avatar_url;
        if (name) name.textContent = this.userInfo.name || this.userInfo.login;
        if (username) username.textContent = `@${this.userInfo.login}`;
        if (stats) {
            stats.innerHTML = `
                <span>📦 ${this.userInfo.public_repos} repos</span>
                <span>👥 ${this.userInfo.followers} followers</span>
            `;
        }
    }

    toggleAttribution(enabled) {
        localStorage.setItem('attribution_enabled', enabled.toString());
        
        if (enabled) {
            this.showNotification('Your GitHub profile will be credited for contributions', 'info');
        } else {
            this.showNotification('Contributions will be anonymous', 'info');
        }
    }

    isAttributionEnabled() {
        return localStorage.getItem('attribution_enabled') === 'true';
    }

    getContributorInfo() {
        if (!this.isAuthenticated || !this.isAttributionEnabled()) {
            return null;
        }
        
        return {
            username: this.userInfo.login,
            name: this.userInfo.name,
            avatar_url: this.userInfo.avatar_url,
            profile_url: `https://github.com/${this.userInfo.login}`
        };
    }

    generateRandomState() {
        return Math.random().toString(36).substring(2, 15) + 
               Math.random().toString(36).substring(2, 15);
    }

    showNotification(message, type = 'info') {
        // Create notification element
        const notification = document.createElement('div');
        notification.className = `notification notification-${type}`;
        notification.innerHTML = `
            <div class="notification-content">
                <span class="notification-icon">
                    ${type === 'success' ? '✅' : type === 'error' ? '❌' : 'ℹ️'}
                </span>
                <span class="notification-message">${message}</span>
                <button class="notification-close" onclick="this.parentElement.parentElement.remove()">×</button>
            </div>
        `;
        
        // Add to page
        document.body.appendChild(notification);
        
        // Auto remove after 5 seconds
        setTimeout(() => {
            if (notification.parentElement) {
                notification.remove();
            }
        }, 5000);
        
        // Animate in
        setTimeout(() => {
            notification.classList.add('show');
        }, 10);
    }
}

// Create global instance
window.githubAuth = new GitHubAuth();

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = GitHubAuth;
}