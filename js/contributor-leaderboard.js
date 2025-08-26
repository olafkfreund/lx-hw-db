/**
 * Contributor Leaderboard and Scoring System
 * Recognition system for community contributors with badges and achievements
 */

class ContributorLeaderboard {
    constructor() {
        this.isInitialized = false;
        this.contributors = new Map();
        this.achievements = new Map();
        this.scoringRules = this.setupScoringRules();
        this.badges = this.setupBadges();
        this.leaderboardPeriods = ['all-time', 'this-month', 'this-week'];
    }

    /**
     * Initialize the contributor leaderboard system
     */
    async initialize() {
        console.log('Initializing contributor leaderboard system...');
        
        await this.loadContributorData();
        this.calculateAllScores();
        this.setupLeaderboardInterface();
        this.isInitialized = true;
        
        console.log('Contributor leaderboard system initialized successfully');
    }

    /**
     * Set up scoring rules for different activities
     */
    setupScoringRules() {
        return {
            // Tip submission and quality
            tip_submitted: 10,
            tip_approved: 25,
            tip_featured: 100,
            tip_upvote: 2,
            tip_downvote: -1,
            high_quality_tip: 50, // Rating >= 4.5
            
            // Community engagement
            helpful_review: 5,
            detailed_feedback: 10,
            bug_report: 15,
            
            // Special contributions
            first_contribution: 50,
            hardware_expert: 200, // 10+ tips for same hardware
            distribution_guru: 150, // Tips across 3+ distributions
            
            // Moderation activities
            moderation_vote: 3,
            accurate_moderation: 10,
            
            // Time-based bonuses
            early_adopter: 500, // First 100 contributors
            veteran: 1000, // Active for 6+ months
            consistent_contributor: 100 // Contributions every month
        };
    }

    /**
     * Set up achievement badges
     */
    setupBadges() {
        return {
            // Contribution Badges
            'first-steps': {
                name: 'First Steps',
                description: 'Made your first contribution',
                icon: '🌱',
                color: 'green',
                requirement: { type: 'tips_contributed', count: 1 }
            },
            'helping-hand': {
                name: 'Helping Hand',
                description: 'Contributed 5 helpful configuration tips',
                icon: '🤝',
                color: 'blue',
                requirement: { type: 'tips_approved', count: 5 }
            },
            'expert-contributor': {
                name: 'Expert Contributor',
                description: 'Contributed 25+ high-quality tips',
                icon: '⭐',
                color: 'gold',
                requirement: { type: 'tips_approved', count: 25 }
            },
            'tip-master': {
                name: 'Tip Master',
                description: 'Contributed 100+ configuration tips',
                icon: '👑',
                color: 'purple',
                requirement: { type: 'tips_approved', count: 100 }
            },
            
            // Quality Badges
            'quality-focused': {
                name: 'Quality Focused',
                description: 'Average tip rating of 4.0+',
                icon: '💎',
                color: 'diamond',
                requirement: { type: 'average_rating', value: 4.0 }
            },
            'perfectionist': {
                name: 'Perfectionist',
                description: 'Average tip rating of 4.8+',
                icon: '✨',
                color: 'platinum',
                requirement: { type: 'average_rating', value: 4.8 }
            },
            
            // Hardware Expertise
            'gpu-guru': {
                name: 'GPU Guru',
                description: '10+ graphics card configuration tips',
                icon: '🎮',
                color: 'red',
                requirement: { type: 'component_expert', component: 'gpu', count: 10 }
            },
            'cpu-champion': {
                name: 'CPU Champion',
                description: '10+ processor optimization tips',
                icon: '🖥️',
                color: 'orange',
                requirement: { type: 'component_expert', component: 'cpu', count: 10 }
            },
            'network-ninja': {
                name: 'Network Ninja',
                description: '10+ network configuration tips',
                icon: '🌐',
                color: 'teal',
                requirement: { type: 'component_expert', component: 'network', count: 10 }
            },
            
            // Distribution Mastery
            'arch-architect': {
                name: 'Arch Architect',
                description: '15+ Arch Linux configuration tips',
                icon: '🏔️',
                color: 'blue',
                requirement: { type: 'distribution_expert', distribution: 'arch', count: 15 }
            },
            'debian-developer': {
                name: 'Debian Developer',
                description: '15+ Debian/Ubuntu configuration tips',
                icon: '🌀',
                color: 'red',
                requirement: { type: 'distribution_expert', distribution: 'debian', count: 15 }
            },
            'nixos-wizard': {
                name: 'NixOS Wizard',
                description: '10+ NixOS configuration tips',
                icon: '❄️',
                color: 'blue',
                requirement: { type: 'distribution_expert', distribution: 'nixos', count: 10 }
            },
            'distro-hopper': {
                name: 'Distro Hopper',
                description: 'Tips across 4+ different distributions',
                icon: '🦘',
                color: 'rainbow',
                requirement: { type: 'multi_distribution', count: 4 }
            },
            
            // Community Engagement
            'community-champion': {
                name: 'Community Champion',
                description: 'Highly active in community reviews',
                icon: '🏆',
                color: 'gold',
                requirement: { type: 'moderation_votes', count: 50 }
            },
            'mentor': {
                name: 'Mentor',
                description: 'Helped review and improve 25+ tips',
                icon: '🎓',
                color: 'purple',
                requirement: { type: 'helpful_reviews', count: 25 }
            },
            
            // Special Recognition
            'trailblazer': {
                name: 'Trailblazer',
                description: 'Among the first 50 contributors',
                icon: '🚀',
                color: 'special',
                requirement: { type: 'early_contributor', rank: 50 }
            },
            'legend': {
                name: 'Legend',
                description: 'Top 10 all-time contributor',
                icon: '🌟',
                color: 'legendary',
                requirement: { type: 'top_contributor', rank: 10 }
            }
        };
    }

    /**
     * Load contributor data (mock data for demo)
     */
    async loadContributorData() {
        // In production, this would load from the database
        // For demo, we'll create sample contributor data
        const sampleContributors = [
            {
                id: 'linux-enthusiast',
                username: 'linux-enthusiast',
                name: 'Linux Enthusiast',
                avatar_url: 'https://avatars.githubusercontent.com/u/1?v=4',
                profile_url: 'https://github.com/linux-enthusiast',
                joinDate: '2024-08-15',
                stats: {
                    tips_contributed: 45,
                    tips_approved: 42,
                    tips_featured: 3,
                    total_upvotes: 187,
                    total_downvotes: 8,
                    average_rating: 4.6,
                    moderation_votes: 23,
                    helpful_reviews: 15,
                    components_covered: ['cpu', 'gpu', 'network'],
                    distributions_covered: ['debian', 'arch', 'fedora'],
                    consecutive_months_active: 6
                }
            },
            {
                id: 'hardware-guru',
                username: 'hardware-guru',
                name: 'Hardware Guru',
                avatar_url: 'https://avatars.githubusercontent.com/u/2?v=4',
                profile_url: 'https://github.com/hardware-guru',
                joinDate: '2024-07-22',
                stats: {
                    tips_contributed: 67,
                    tips_approved: 64,
                    tips_featured: 5,
                    total_upvotes: 298,
                    total_downvotes: 12,
                    average_rating: 4.8,
                    moderation_votes: 45,
                    helpful_reviews: 28,
                    components_covered: ['gpu', 'cpu', 'memory', 'storage'],
                    distributions_covered: ['arch', 'nixos', 'gentoo'],
                    consecutive_months_active: 7
                }
            },
            {
                id: 'nixos-wizard',
                username: 'nixos-wizard',
                name: 'NixOS Wizard',
                avatar_url: 'https://avatars.githubusercontent.com/u/3?v=4',
                profile_url: 'https://github.com/nixos-wizard',
                joinDate: '2024-09-10',
                stats: {
                    tips_contributed: 32,
                    tips_approved: 30,
                    tips_featured: 8,
                    total_upvotes: 156,
                    total_downvotes: 3,
                    average_rating: 4.9,
                    moderation_votes: 18,
                    helpful_reviews: 12,
                    components_covered: ['cpu', 'gpu', 'network', 'system'],
                    distributions_covered: ['nixos'],
                    consecutive_months_active: 4
                }
            },
            {
                id: 'community-helper',
                username: 'community-helper',
                name: 'Community Helper',
                avatar_url: 'https://avatars.githubusercontent.com/u/4?v=4',
                profile_url: 'https://github.com/community-helper',
                joinDate: '2024-06-05',
                stats: {
                    tips_contributed: 28,
                    tips_approved: 26,
                    tips_featured: 1,
                    total_upvotes: 112,
                    total_downvotes: 5,
                    average_rating: 4.3,
                    moderation_votes: 67,
                    helpful_reviews: 34,
                    components_covered: ['network', 'audio', 'input'],
                    distributions_covered: ['debian', 'fedora'],
                    consecutive_months_active: 8
                }
            },
            {
                id: 'newbie-contributor',
                username: 'newbie-contributor',
                name: 'Newbie Contributor',
                avatar_url: 'https://avatars.githubusercontent.com/u/5?v=4',
                profile_url: 'https://github.com/newbie-contributor',
                joinDate: '2025-01-20',
                stats: {
                    tips_contributed: 3,
                    tips_approved: 2,
                    tips_featured: 0,
                    total_upvotes: 8,
                    total_downvotes: 1,
                    average_rating: 3.8,
                    moderation_votes: 1,
                    helpful_reviews: 0,
                    components_covered: ['gpu'],
                    distributions_covered: ['debian'],
                    consecutive_months_active: 1
                }
            }
        ];

        // Load contributors into the system
        sampleContributors.forEach(contributor => {
            this.contributors.set(contributor.id, contributor);
        });
    }

    /**
     * Calculate scores for all contributors
     */
    calculateAllScores() {
        this.contributors.forEach((contributor, id) => {
            const score = this.calculateContributorScore(contributor);
            const achievements = this.calculateAchievements(contributor);
            
            contributor.totalScore = score;
            contributor.achievements = achievements;
            contributor.rank = 0; // Will be set after sorting
            
            this.contributors.set(id, contributor);
        });

        // Calculate ranks
        this.updateRanks();
    }

    /**
     * Calculate score for a single contributor
     */
    calculateContributorScore(contributor) {
        const stats = contributor.stats;
        const rules = this.scoringRules;
        let score = 0;

        // Base tip contributions
        score += stats.tips_contributed * rules.tip_submitted;
        score += stats.tips_approved * rules.tip_approved;
        score += stats.tips_featured * rules.tip_featured;
        score += stats.total_upvotes * rules.tip_upvote;
        score += stats.total_downvotes * rules.tip_downvote;

        // Quality bonus
        if (stats.average_rating >= 4.5) {
            score += rules.high_quality_tip;
        }

        // Community engagement
        score += stats.moderation_votes * rules.moderation_vote;
        score += stats.helpful_reviews * rules.helpful_review;

        // Expertise bonuses
        stats.components_covered.forEach(component => {
            const componentTips = Math.floor(stats.tips_approved / stats.components_covered.length);
            if (componentTips >= 10) {
                score += rules.hardware_expert;
            }
        });

        if (stats.distributions_covered.length >= 3) {
            score += rules.distribution_guru;
        }

        // Time-based bonuses
        if (stats.consecutive_months_active >= 6) {
            score += rules.veteran;
        }

        if (stats.consecutive_months_active >= 3) {
            score += rules.consistent_contributor;
        }

        // First contribution bonus
        if (stats.tips_contributed >= 1) {
            score += rules.first_contribution;
        }

        return Math.max(0, score);
    }

    /**
     * Calculate achievements for a contributor
     */
    calculateAchievements(contributor) {
        const achievements = [];
        const stats = contributor.stats;

        Object.entries(this.badges).forEach(([badgeId, badge]) => {
            const req = badge.requirement;
            let earned = false;

            switch (req.type) {
                case 'tips_contributed':
                    earned = stats.tips_contributed >= req.count;
                    break;
                case 'tips_approved':
                    earned = stats.tips_approved >= req.count;
                    break;
                case 'average_rating':
                    earned = stats.average_rating >= req.value;
                    break;
                case 'component_expert':
                    const componentTips = Math.floor(stats.tips_approved / stats.components_covered.length);
                    earned = stats.components_covered.includes(req.component) && componentTips >= req.count;
                    break;
                case 'distribution_expert':
                    const distroTips = Math.floor(stats.tips_approved / stats.distributions_covered.length);
                    earned = stats.distributions_covered.includes(req.distribution) && distroTips >= req.count;
                    break;
                case 'multi_distribution':
                    earned = stats.distributions_covered.length >= req.count;
                    break;
                case 'moderation_votes':
                    earned = stats.moderation_votes >= req.count;
                    break;
                case 'helpful_reviews':
                    earned = stats.helpful_reviews >= req.count;
                    break;
                case 'early_contributor':
                    // This would be calculated based on join date/order
                    earned = this.isEarlyContributor(contributor, req.rank);
                    break;
                case 'top_contributor':
                    // This will be calculated after ranking
                    earned = false; // Set later
                    break;
            }

            if (earned) {
                achievements.push({
                    id: badgeId,
                    ...badge,
                    earnedDate: this.getAchievementEarnedDate(contributor, badgeId)
                });
            }
        });

        return achievements;
    }

    /**
     * Check if contributor is an early adopter
     */
    isEarlyContributor(contributor, maxRank) {
        const allContributors = Array.from(this.contributors.values())
            .sort((a, b) => new Date(a.joinDate) - new Date(b.joinDate));
        
        const index = allContributors.findIndex(c => c.id === contributor.id);
        return index < maxRank;
    }

    /**
     * Get earned date for achievement (mock)
     */
    getAchievementEarnedDate(contributor, badgeId) {
        // In production, this would track actual earn dates
        return new Date().toISOString().split('T')[0];
    }

    /**
     * Update contributor ranks
     */
    updateRanks() {
        const sortedContributors = Array.from(this.contributors.values())
            .sort((a, b) => b.totalScore - a.totalScore);

        sortedContributors.forEach((contributor, index) => {
            contributor.rank = index + 1;
            
            // Update top contributor achievements
            if (contributor.rank <= 10) {
                const topBadge = contributor.achievements.find(a => a.id === 'legend');
                if (!topBadge && contributor.rank <= 10) {
                    contributor.achievements.push({
                        id: 'legend',
                        ...this.badges['legend'],
                        earnedDate: new Date().toISOString().split('T')[0]
                    });
                }
            }
            
            this.contributors.set(contributor.id, contributor);
        });
    }

    /**
     * Set up leaderboard interface
     */
    setupLeaderboardInterface() {
        // Add leaderboard section to the main interface
        this.createLeaderboardSection();
        this.setupEventListeners();
    }

    /**
     * Create leaderboard section in the main page
     */
    createLeaderboardSection() {
        // Add leaderboard link to navigation
        const nav = document.querySelector('.header-nav');
        if (nav && !nav.querySelector('.leaderboard-link')) {
            const leaderboardLink = document.createElement('a');
            leaderboardLink.href = '#leaderboard';
            leaderboardLink.className = 'nav-link leaderboard-link';
            leaderboardLink.textContent = '🏆 Leaderboard';
            nav.appendChild(leaderboardLink);
        }

        // Create leaderboard section
        const mainContent = document.querySelector('.main-content');
        if (mainContent && !mainContent.querySelector('#leaderboard')) {
            const leaderboardSection = document.createElement('section');
            leaderboardSection.id = 'leaderboard';
            leaderboardSection.className = 'leaderboard-section';
            leaderboardSection.innerHTML = this.createLeaderboardHTML();
            
            // Insert after stats section
            const statsSection = mainContent.querySelector('#stats');
            if (statsSection) {
                statsSection.insertAdjacentElement('afterend', leaderboardSection);
            } else {
                mainContent.appendChild(leaderboardSection);
            }
        }
    }

    /**
     * Create leaderboard HTML
     */
    createLeaderboardHTML() {
        const topContributors = this.getTopContributors(10);
        
        return `
            <div class="container">
                <div class="leaderboard-intro">
                    <h2>🏆 Community Leaderboard</h2>
                    <p>Recognizing our amazing community contributors who help make Linux hardware compatibility better for everyone.</p>
                </div>

                <div class="leaderboard-controls">
                    <div class="period-tabs">
                        <button class="period-tab active" data-period="all-time">All Time</button>
                        <button class="period-tab" data-period="this-month">This Month</button>
                        <button class="period-tab" data-period="this-week">This Week</button>
                    </div>
                    
                    <div class="leaderboard-stats">
                        <div class="stat-item">
                            <span class="stat-number">${this.contributors.size}</span>
                            <span class="stat-label">Contributors</span>
                        </div>
                        <div class="stat-item">
                            <span class="stat-number">${this.getTotalTips()}</span>
                            <span class="stat-label">Tips Shared</span>
                        </div>
                        <div class="stat-item">
                            <span class="stat-number">${this.getTotalAchievements()}</span>
                            <span class="stat-label">Achievements</span>
                        </div>
                    </div>
                </div>

                <div class="leaderboard-content">
                    <div class="top-contributors">
                        <h3>🌟 Hall of Fame</h3>
                        <div class="podium">
                            ${this.createPodium(topContributors.slice(0, 3))}
                        </div>
                    </div>

                    <div class="contributor-rankings">
                        <h3>📊 Full Rankings</h3>
                        <div class="rankings-list">
                            ${topContributors.map(contributor => this.createContributorCard(contributor)).join('')}
                        </div>
                    </div>

                    <div class="achievements-showcase">
                        <h3>🏅 Recent Achievements</h3>
                        <div class="recent-achievements">
                            ${this.createRecentAchievements()}
                        </div>
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * Create podium for top 3 contributors
     */
    createPodium(topThree) {
        if (topThree.length < 3) return '';

        const [first, second, third] = topThree;
        
        return `
            <div class="podium-container">
                <div class="podium-position second">
                    <div class="position-number">2</div>
                    <div class="contributor-avatar">
                        <img src="${second.avatar_url}" alt="${second.name}" />
                        <div class="crown silver-crown">🥈</div>
                    </div>
                    <div class="contributor-info">
                        <h4>${second.name}</h4>
                        <div class="score">${second.totalScore.toLocaleString()} pts</div>
                        <div class="achievements-count">${second.achievements.length} badges</div>
                    </div>
                </div>
                
                <div class="podium-position first">
                    <div class="position-number">1</div>
                    <div class="contributor-avatar">
                        <img src="${first.avatar_url}" alt="${first.name}" />
                        <div class="crown gold-crown">👑</div>
                    </div>
                    <div class="contributor-info">
                        <h4>${first.name}</h4>
                        <div class="score">${first.totalScore.toLocaleString()} pts</div>
                        <div class="achievements-count">${first.achievements.length} badges</div>
                    </div>
                </div>
                
                <div class="podium-position third">
                    <div class="position-number">3</div>
                    <div class="contributor-avatar">
                        <img src="${third.avatar_url}" alt="${third.name}" />
                        <div class="crown bronze-crown">🥉</div>
                    </div>
                    <div class="contributor-info">
                        <h4>${third.name}</h4>
                        <div class="score">${third.totalScore.toLocaleString()} pts</div>
                        <div class="achievements-count">${third.achievements.length} badges</div>
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * Create contributor card
     */
    createContributorCard(contributor) {
        const topBadges = contributor.achievements
            .sort((a, b) => this.getBadgeImportance(b) - this.getBadgeImportance(a))
            .slice(0, 3);

        return `
            <div class="contributor-card" data-contributor-id="${contributor.id}">
                <div class="rank-badge rank-${contributor.rank <= 3 ? contributor.rank : 'other'}">
                    #${contributor.rank}
                </div>
                
                <div class="contributor-header">
                    <img src="${contributor.avatar_url}" alt="${contributor.name}" class="contributor-avatar-small" />
                    <div class="contributor-details">
                        <h4 class="contributor-name">
                            <a href="${contributor.profile_url}" target="_blank" rel="noopener">
                                ${contributor.name}
                            </a>
                        </h4>
                        <div class="contributor-username">@${contributor.username}</div>
                        <div class="contributor-joined">Joined ${this.formatDate(contributor.joinDate)}</div>
                    </div>
                    <div class="contributor-score">
                        <div class="score-number">${contributor.totalScore.toLocaleString()}</div>
                        <div class="score-label">points</div>
                    </div>
                </div>

                <div class="contributor-stats">
                    <div class="stat-group">
                        <div class="stat">
                            <span class="stat-value">${contributor.stats.tips_approved}</span>
                            <span class="stat-name">Tips</span>
                        </div>
                        <div class="stat">
                            <span class="stat-value">${contributor.stats.average_rating.toFixed(1)}</span>
                            <span class="stat-name">Rating</span>
                        </div>
                        <div class="stat">
                            <span class="stat-value">${contributor.achievements.length}</span>
                            <span class="stat-name">Badges</span>
                        </div>
                        <div class="stat">
                            <span class="stat-value">${contributor.stats.total_upvotes}</span>
                            <span class="stat-name">Upvotes</span>
                        </div>
                    </div>
                </div>

                <div class="contributor-badges">
                    ${topBadges.map(badge => `
                        <div class="achievement-badge ${badge.color}" title="${badge.description}">
                            <span class="badge-icon">${badge.icon}</span>
                            <span class="badge-name">${badge.name}</span>
                        </div>
                    `).join('')}
                    ${contributor.achievements.length > 3 ? 
                        `<div class="more-badges">+${contributor.achievements.length - 3} more</div>` : ''
                    }
                </div>
            </div>
        `;
    }

    /**
     * Create recent achievements display
     */
    createRecentAchievements() {
        const recentAchievements = [];
        
        // Collect recent achievements from all contributors
        this.contributors.forEach(contributor => {
            contributor.achievements.forEach(achievement => {
                recentAchievements.push({
                    ...achievement,
                    contributor: contributor
                });
            });
        });

        // Sort by earned date (mock - in production would be actual dates)
        recentAchievements.sort((a, b) => new Date(b.earnedDate) - new Date(a.earnedDate));

        return recentAchievements.slice(0, 6).map(achievement => `
            <div class="achievement-item">
                <img src="${achievement.contributor.avatar_url}" alt="${achievement.contributor.name}" class="achievement-avatar" />
                <div class="achievement-details">
                    <div class="achievement-badge-mini ${achievement.color}">
                        <span class="badge-icon">${achievement.icon}</span>
                        <span class="badge-name">${achievement.name}</span>
                    </div>
                    <div class="achievement-contributor">
                        <strong>${achievement.contributor.name}</strong> earned this ${this.formatDate(achievement.earnedDate)}
                    </div>
                    <div class="achievement-description">${achievement.description}</div>
                </div>
            </div>
        `).join('');
    }

    /**
     * Set up event listeners
     */
    setupEventListeners() {
        // Period tab switching
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('period-tab')) {
                document.querySelectorAll('.period-tab').forEach(tab => tab.classList.remove('active'));
                e.target.classList.add('active');
                
                const period = e.target.dataset.period;
                this.updateLeaderboardForPeriod(period);
            }
        });

        // Contributor card clicks
        document.addEventListener('click', (e) => {
            if (e.target.closest('.contributor-card')) {
                const card = e.target.closest('.contributor-card');
                const contributorId = card.dataset.contributorId;
                this.showContributorProfile(contributorId);
            }
        });

        // Navigation link
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('leaderboard-link')) {
                e.preventDefault();
                document.querySelector('#leaderboard').scrollIntoView({ behavior: 'smooth' });
            }
        });
    }

    /**
     * Update leaderboard for specific time period
     */
    updateLeaderboardForPeriod(period) {
        // In production, this would filter by date
        console.log(`Switching to ${period} leaderboard`);
        // For now, keep showing all-time data
    }

    /**
     * Show detailed contributor profile
     */
    showContributorProfile(contributorId) {
        const contributor = this.contributors.get(contributorId);
        if (!contributor) return;

        const profileModal = document.createElement('div');
        profileModal.className = 'contributor-profile-modal';
        profileModal.innerHTML = `
            <div class="profile-modal-content">
                <div class="profile-modal-header">
                    <h2>👤 Contributor Profile</h2>
                    <button class="profile-modal-close" aria-label="Close">&times;</button>
                </div>
                
                <div class="profile-modal-body">
                    <div class="profile-header">
                        <img src="${contributor.avatar_url}" alt="${contributor.name}" class="profile-avatar" />
                        <div class="profile-info">
                            <h3>${contributor.name}</h3>
                            <div class="profile-username">@${contributor.username}</div>
                            <div class="profile-rank">Rank #${contributor.rank} • ${contributor.totalScore.toLocaleString()} points</div>
                            <a href="${contributor.profile_url}" target="_blank" class="profile-github-link">
                                View GitHub Profile →
                            </a>
                        </div>
                    </div>

                    <div class="profile-stats-grid">
                        <div class="profile-stat">
                            <div class="stat-number">${contributor.stats.tips_contributed}</div>
                            <div class="stat-label">Tips Contributed</div>
                        </div>
                        <div class="profile-stat">
                            <div class="stat-number">${contributor.stats.tips_approved}</div>
                            <div class="stat-label">Tips Approved</div>
                        </div>
                        <div class="profile-stat">
                            <div class="stat-number">${contributor.stats.average_rating.toFixed(1)}</div>
                            <div class="stat-label">Average Rating</div>
                        </div>
                        <div class="profile-stat">
                            <div class="stat-number">${contributor.stats.total_upvotes}</div>
                            <div class="stat-label">Total Upvotes</div>
                        </div>
                        <div class="profile-stat">
                            <div class="stat-number">${contributor.stats.moderation_votes}</div>
                            <div class="stat-label">Reviews Given</div>
                        </div>
                        <div class="profile-stat">
                            <div class="stat-number">${contributor.stats.consecutive_months_active}</div>
                            <div class="stat-label">Months Active</div>
                        </div>
                    </div>

                    <div class="profile-achievements">
                        <h4>🏅 Achievements (${contributor.achievements.length})</h4>
                        <div class="achievements-grid">
                            ${contributor.achievements.map(achievement => `
                                <div class="achievement-card ${achievement.color}">
                                    <div class="achievement-icon">${achievement.icon}</div>
                                    <div class="achievement-name">${achievement.name}</div>
                                    <div class="achievement-desc">${achievement.description}</div>
                                    <div class="achievement-date">Earned ${this.formatDate(achievement.earnedDate)}</div>
                                </div>
                            `).join('')}
                        </div>
                    </div>

                    <div class="profile-expertise">
                        <h4>🔧 Expertise Areas</h4>
                        <div class="expertise-tags">
                            ${contributor.stats.components_covered.map(component => `
                                <span class="expertise-tag">${this.getComponentDisplayName(component)}</span>
                            `).join('')}
                        </div>
                        <div class="distributions-covered">
                            ${contributor.stats.distributions_covered.map(distro => `
                                <span class="distribution-tag">${this.getDistributionDisplayName(distro)}</span>
                            `).join('')}
                        </div>
                    </div>
                </div>
            </div>
        `;

        document.body.appendChild(profileModal);
        profileModal.style.display = 'flex';

        // Set up modal events
        profileModal.querySelector('.profile-modal-close').addEventListener('click', () => {
            profileModal.remove();
        });

        profileModal.addEventListener('click', (e) => {
            if (e.target === profileModal) {
                profileModal.remove();
            }
        });
    }

    /**
     * Get top contributors
     */
    getTopContributors(limit = 10) {
        return Array.from(this.contributors.values())
            .sort((a, b) => b.totalScore - a.totalScore)
            .slice(0, limit);
    }

    /**
     * Get total tips across all contributors
     */
    getTotalTips() {
        return Array.from(this.contributors.values())
            .reduce((total, contributor) => total + contributor.stats.tips_approved, 0);
    }

    /**
     * Get total achievements across all contributors
     */
    getTotalAchievements() {
        return Array.from(this.contributors.values())
            .reduce((total, contributor) => total + contributor.achievements.length, 0);
    }

    /**
     * Get badge importance for sorting
     */
    getBadgeImportance(badge) {
        const importance = {
            'legendary': 100,
            'special': 90,
            'platinum': 80,
            'gold': 70,
            'purple': 60,
            'diamond': 55,
            'rainbow': 50,
            'blue': 40,
            'red': 35,
            'orange': 30,
            'teal': 25,
            'green': 20
        };
        return importance[badge.color] || 10;
    }

    /**
     * Format date for display
     */
    formatDate(dateString) {
        const date = new Date(dateString);
        return date.toLocaleDateString('en-US', { 
            year: 'numeric', 
            month: 'short',
            day: 'numeric'
        });
    }

    /**
     * Get component display name
     */
    getComponentDisplayName(component) {
        const names = {
            'cpu': 'CPU',
            'gpu': 'Graphics',
            'memory': 'Memory',
            'storage': 'Storage',
            'network': 'Network',
            'audio': 'Audio',
            'input': 'Input',
            'display': 'Display',
            'system': 'System'
        };
        return names[component] || component;
    }

    /**
     * Get distribution display name
     */
    getDistributionDisplayName(distro) {
        const names = {
            'debian': 'Debian/Ubuntu',
            'arch': 'Arch Linux',
            'fedora': 'Fedora',
            'nixos': 'NixOS',
            'gentoo': 'Gentoo',
            'opensuse': 'openSUSE',
            'alpine': 'Alpine'
        };
        return names[distro] || distro;
    }

    /**
     * Add points to contributor (for real-time updates)
     */
    addContributorPoints(contributorId, points, reason) {
        const contributor = this.contributors.get(contributorId);
        if (contributor) {
            contributor.totalScore += points;
            console.log(`Added ${points} points to ${contributor.name} for ${reason}`);
            
            // Recalculate ranks
            this.updateRanks();
            
            // Show achievement notification if applicable
            this.checkForNewAchievements(contributor);
        }
    }

    /**
     * Check for new achievements after score update
     */
    checkForNewAchievements(contributor) {
        const previousAchievements = contributor.achievements.length;
        const newAchievements = this.calculateAchievements(contributor);
        
        if (newAchievements.length > previousAchievements) {
            const newBadges = newAchievements.slice(previousAchievements);
            newBadges.forEach(badge => {
                this.showAchievementNotification(contributor, badge);
            });
        }
        
        contributor.achievements = newAchievements;
        this.contributors.set(contributor.id, contributor);
    }

    /**
     * Show achievement notification
     */
    showAchievementNotification(contributor, badge) {
        const notification = document.createElement('div');
        notification.className = 'achievement-notification';
        notification.innerHTML = `
            <div class="achievement-notification-content">
                <div class="achievement-icon">${badge.icon}</div>
                <div class="achievement-text">
                    <h4>New Achievement!</h4>
                    <p><strong>${contributor.name}</strong> earned the <strong>${badge.name}</strong> badge!</p>
                    <small>${badge.description}</small>
                </div>
            </div>
        `;

        document.body.appendChild(notification);
        
        // Animate in
        setTimeout(() => notification.classList.add('show'), 100);
        
        // Remove after 5 seconds
        setTimeout(() => {
            notification.classList.remove('show');
            setTimeout(() => notification.remove(), 300);
        }, 5000);
    }
}

// Global instance
window.contributorLeaderboard = new ContributorLeaderboard();

// Auto-initialize when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
    await window.contributorLeaderboard.initialize();
});

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = ContributorLeaderboard;
}