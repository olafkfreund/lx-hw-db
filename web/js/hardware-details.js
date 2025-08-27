/**
 * Hardware Details Modal
 * Displays detailed information about hardware components
 */

class HardwareDetails {
    constructor() {
        this.modal = null;
        this.init();
    }
    
    init() {
        // Create modal container if it doesn't exist
        if (!document.getElementById('hardware-details-modal')) {
            this.createModal();
        }
        
        // Add event delegation for View Details buttons
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('view-details-btn') || 
                e.target.classList.contains('view-report-details') ||
                e.target.textContent === 'View Details') {
                e.preventDefault();
                console.log('View Details button clicked:', e.target);
                const hardwareId = e.target.dataset.hardwareId || 
                                 e.target.dataset.reportId ||
                                 e.target.closest('[data-hardware-id]')?.dataset.hardwareId ||
                                 e.target.closest('[data-report-id]')?.dataset.reportId;
                console.log('Hardware/Report ID found:', hardwareId);
                if (hardwareId) {
                    this.showDetails(hardwareId);
                } else {
                    console.warn('No hardware ID found for View Details button');
                }
            }
        });
    }
    
    createModal() {
        const modal = document.createElement('div');
        modal.id = 'hardware-details-modal';
        modal.className = 'modal';
        modal.setAttribute('role', 'dialog');
        modal.setAttribute('aria-labelledby', 'modal-title');
        modal.setAttribute('aria-describedby', 'hardware-details-body');
        modal.setAttribute('aria-modal', 'true');
        modal.innerHTML = `
            <div class="modal-overlay" aria-hidden="true"></div>
            <div class="modal-content hardware-details-content">
                <button class="modal-close" aria-label="Close hardware details dialog">×</button>
                <div class="modal-header">
                    <h2 id="modal-title">Hardware Details</h2>
                </div>
                <div class="modal-body" id="hardware-details-body">
                    <!-- Details will be inserted here -->
                </div>
                <div class="modal-footer">
                    <button class="btn-primary copy-specs-btn" aria-label="Copy hardware specifications to clipboard">
                        📋 Copy Specifications
                    </button>
                    <button class="btn-secondary submit-report-btn" aria-label="Submit a compatibility report for this hardware">
                        📤 Submit Compatibility Report
                    </button>
                    <button class="btn-secondary close-modal-btn" aria-label="Close hardware details dialog">
                        Close
                    </button>
                </div>
            </div>
        `;
        
        document.body.appendChild(modal);
        this.modal = modal;
        
        // Add event listeners
        modal.querySelector('.modal-overlay').addEventListener('click', () => this.hideModal());
        modal.querySelector('.modal-close').addEventListener('click', () => this.hideModal());
        modal.querySelector('.close-modal-btn').addEventListener('click', () => this.hideModal());
        modal.querySelector('.copy-specs-btn').addEventListener('click', () => this.copySpecs());
        modal.querySelector('.submit-report-btn').addEventListener('click', () => this.submitReport());
        
        // Add keyboard event listener for ESC key and focus trap
        modal.addEventListener('keydown', (e) => this.handleKeyDown(e));
    }
    
    async showDetails(hardwareId) {
        console.log('showDetails called with hardwareId:', hardwareId);
        console.log('hardwareId type:', typeof hardwareId);
        
        if (!hardwareId) {
            this.showError('No hardware ID provided');
            return;
        }

        // Store the currently focused element to restore later
        this.previouslyFocusedElement = document.activeElement;
        
        // Show modal with loading state first
        this.showLoadingState();
        
        try {
            // Load hardware data
            const hardware = await this.getHardwareData(hardwareId);
            console.log('getHardwareData returned:', hardware);
            
            if (!hardware) {
                console.warn('No hardware data found for ID:', hardwareId);
                this.showError('Hardware information not found for ID: ' + hardwareId);
                return;
            }
            
            // Populate modal with hardware details
            const body = document.getElementById('hardware-details-body');
            body.innerHTML = this.renderHardwareDetails(hardware);
            
            // Update modal header with hardware name
            const header = this.modal.querySelector('.modal-header h2');
            if (header && hardware.name) {
                header.textContent = hardware.name;
            }
            
        } catch (error) {
            console.error('Error loading hardware details:', error);
            this.showError('Failed to load hardware information. Please try again.');
        }
    }

    showLoadingState() {
        const body = document.getElementById('hardware-details-body');
        body.innerHTML = `
            <div class="modal-loading-state">
                <div class="loading-spinner"></div>
                <p>Loading hardware details...</p>
            </div>
        `;
        
        // Reset header
        const header = this.modal.querySelector('.modal-header h2');
        if (header) {
            header.textContent = 'Hardware Details';
        }
        
        // Show modal
        this.modal.classList.add('show');
        document.body.style.overflow = 'hidden';
    }

    showError(message) {
        const body = document.getElementById('hardware-details-body');
        body.innerHTML = `
            <div class="modal-error-state">
                <div class="error-icon">⚠️</div>
                <h3>Error Loading Details</h3>
                <p>${message}</p>
                <button class="btn-primary retry-btn" onclick="this.closest('.modal').style.display='none'">
                    Close
                </button>
            </div>
        `;
        
        // Show modal if not already shown
        if (!this.modal.classList.contains('show')) {
            this.modal.classList.add('show');
            document.body.style.overflow = 'hidden';
            this.setFocus();
        }
    }

    handleKeyDown(e) {
        if (e.key === 'Escape') {
            this.hideModal();
        } else if (e.key === 'Tab') {
            this.trapFocus(e);
        }
    }

    trapFocus(e) {
        const focusableElements = this.modal.querySelectorAll(
            'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        );
        const firstFocusable = focusableElements[0];
        const lastFocusable = focusableElements[focusableElements.length - 1];

        if (e.shiftKey) {
            // Shift + Tab
            if (document.activeElement === firstFocusable) {
                e.preventDefault();
                lastFocusable.focus();
            }
        } else {
            // Tab
            if (document.activeElement === lastFocusable) {
                e.preventDefault();
                firstFocusable.focus();
            }
        }
    }

    setFocus() {
        // Focus on the close button by default for better accessibility
        const closeButton = this.modal.querySelector('.modal-close');
        if (closeButton) {
            closeButton.focus();
        }
    }
    
    async getHardwareData(hardwareId) {
        console.log('getHardwareData called with:', hardwareId);
        
        try {
            // Try to get data from the loaded database
            if (window.dataLoader && window.dataLoader.hardwareData) {
                console.log('Checking dataLoader...');
                const hardware = window.dataLoader.hardwareData.hardware.find(h => h.id === hardwareId);
                if (hardware) {
                    console.log('Found hardware in dataLoader:', hardware);
                    return hardware;
                }
                console.log('Hardware not found in dataLoader');
            } else {
                console.log('No dataLoader available');
            }
            
            // Fallback: load from file
            console.log('Trying to fetch hardware-database.json...');
            const response = await fetch('./data/hardware-database.json');
            if (response.ok) {
                const data = await response.json();
                const hardware = data.hardware.find(h => h.id === hardwareId);
                if (hardware) {
                    console.log('Found hardware in file:', hardware);
                    return hardware;
                }
                console.log('Hardware not found in file');
            } else {
                console.log('Could not fetch hardware-database.json, status:', response.status);
            }
        } catch (error) {
            console.error('Error loading hardware data:', error);
        }
        
        // Return mock data if nothing found (for demonstration)
        console.log('Falling back to mock data...');
        const mockData = this.getMockHardware(hardwareId);
        console.log('Mock data result:', mockData);
        return mockData;
    }
    
    getMockHardware(hardwareId) {
        // Mock data for demonstration
        const mockData = {
            'realtek_rtl8125': {
                id: 'realtek_rtl8125',
                name: 'Realtek RTL8125 2.5GbE Controller',
                category: 'network',
                manufacturer: 'Realtek',
                model: 'RTL8125',
                pci_id: '10ec:8125',
                compatibility_status: 'excellent',
                last_tested: '2025-08-25',
                kernel_support: '5.9+',
                driver: 'r8169',
                specifications: {
                    speed: '2.5 Gbps',
                    interface: 'PCIe 2.0 x1',
                    features: ['Wake-on-LAN', 'TSO', 'RSS', 'VLAN'],
                    power: '2.5W typical'
                },
                compatibility: {
                    debian: {
                        status: 'excellent',
                        kernel_min: '5.9',
                        packages: ['firmware-realtek'],
                        notes: 'Works out of the box with kernel 5.9+'
                    },
                    ubuntu: {
                        status: 'excellent',
                        kernel_min: '5.8',
                        packages: ['linux-firmware'],
                        notes: 'Native support in Ubuntu 20.10+'
                    },
                    arch: {
                        status: 'excellent',
                        kernel_min: '5.9',
                        packages: ['linux-firmware'],
                        notes: 'Full support with latest kernel'
                    },
                    fedora: {
                        status: 'excellent',
                        kernel_min: '5.9',
                        packages: ['linux-firmware'],
                        notes: 'Works perfectly since Fedora 33'
                    }
                },
                known_issues: [
                    'Some users report wake-on-LAN issues with kernels before 6.0',
                    'Performance may be limited on older PCIe 1.0 slots'
                ],
                community_rating: 4.7,
                reports_count: 156
            },
            'intel_ax211': {
                id: 'intel_ax211',
                name: 'Intel AX211 WiFi 6E',
                category: 'network',
                manufacturer: 'Intel',
                model: 'AX211',
                pci_id: '8086:51f0',
                compatibility_status: 'excellent',
                last_tested: '2025-08-25',
                kernel_support: '5.12+',
                driver: 'iwlwifi',
                specifications: {
                    standard: 'WiFi 6E (802.11ax)',
                    bands: '2.4GHz, 5GHz, 6GHz',
                    max_speed: '2.4 Gbps',
                    bluetooth: '5.3',
                    interface: 'M.2 2230 (E-key)'
                },
                compatibility: {
                    debian: {
                        status: 'excellent',
                        kernel_min: '5.14',
                        packages: ['firmware-iwlwifi'],
                        notes: 'Requires non-free firmware'
                    },
                    ubuntu: {
                        status: 'excellent',
                        kernel_min: '5.13',
                        packages: ['linux-firmware'],
                        notes: 'Works out of the box in Ubuntu 22.04+'
                    },
                    arch: {
                        status: 'excellent',
                        kernel_min: '5.12',
                        packages: ['linux-firmware'],
                        notes: 'Full support with latest firmware'
                    },
                    fedora: {
                        status: 'excellent',
                        kernel_min: '5.14',
                        packages: ['iwlax2xx-firmware'],
                        notes: 'Native support since Fedora 35'
                    }
                },
                known_issues: [
                    '6GHz band requires regulatory database update',
                    'Bluetooth may conflict with USB 3.0 devices'
                ],
                community_rating: 4.8,
                reports_count: 243
            },
            'samsung_980_pro': {
                id: 'samsung_980_pro',
                name: 'Samsung 980 PRO NVMe SSD',
                category: 'storage',
                manufacturer: 'Samsung',
                model: '980 PRO',
                pci_id: '144d:a808',
                compatibility_status: 'excellent',
                last_tested: '2025-08-26',
                kernel_support: '4.4+',
                driver: 'nvme',
                specifications: {
                    capacity: '1TB',
                    interface: 'PCIe 4.0 x4',
                    sequential_read: '7,000 MB/s',
                    sequential_write: '5,000 MB/s',
                    form_factor: 'M.2 2280'
                },
                compatibility: {
                    nixos: {
                        status: 'excellent',
                        kernel_min: '4.4',
                        packages: ['linux'],
                        notes: 'Perfect support in NixOS with native NVMe driver'
                    },
                    arch: {
                        status: 'excellent',
                        kernel_min: '4.4',
                        packages: ['linux'],
                        notes: 'Excellent support in Arch Linux'
                    },
                    ubuntu: {
                        status: 'excellent',
                        kernel_min: '4.4',
                        packages: ['linux-image-generic'],
                        notes: 'Works out of the box'
                    }
                },
                known_issues: [
                    'Some older motherboards may require BIOS update for PCIe 4.0'
                ],
                community_rating: 4.9,
                reports_count: 445
            },
            'samsung_990_pro': {
                id: 'samsung_990_pro',
                name: 'Samsung 990 PRO NVMe SSD',
                category: 'storage',
                manufacturer: 'Samsung',
                model: '990 PRO',
                pci_id: '144d:a80a',
                compatibility_status: 'excellent',
                last_tested: '2025-08-25',
                kernel_support: '5.0+',
                driver: 'nvme',
                specifications: {
                    capacity: '2TB',
                    interface: 'PCIe 4.0 x4',
                    sequential_read: '7,450 MB/s',
                    sequential_write: '6,900 MB/s',
                    form_factor: 'M.2 2280'
                },
                compatibility: {
                    nixos: {
                        status: 'excellent',
                        kernel_min: '5.0',
                        packages: ['linux'],
                        notes: 'Excellent performance in NixOS 22.11+'
                    },
                    arch: {
                        status: 'excellent',
                        kernel_min: '5.0',
                        packages: ['linux'],
                        notes: 'Top performance on Arch Linux'
                    },
                    ubuntu: {
                        status: 'excellent',
                        kernel_min: '5.0',
                        packages: ['linux-image-generic'],
                        notes: 'Optimal performance on Ubuntu 20.04+'
                    }
                },
                known_issues: [
                    'Temperature monitoring requires lm-sensors configuration'
                ],
                community_rating: 4.8,
                reports_count: 278
            },
            // Generic fallback entries for common IDs
            '1': {
                id: '1',
                name: 'Sample Hardware Component',
                category: 'generic',
                manufacturer: 'Generic',
                model: 'Sample Model',
                compatibility_status: 'good',
                last_tested: '2025-08-27',
                kernel_support: '5.4+',
                driver: 'generic',
                specifications: {
                    type: 'Generic hardware component',
                    interface: 'Standard'
                },
                compatibility: {
                    linux: {
                        status: 'good',
                        kernel_min: '5.4',
                        packages: ['linux-generic'],
                        notes: 'Works with most Linux distributions'
                    }
                },
                known_issues: [],
                community_rating: 4.0,
                reports_count: 12
            }
        };
        
        return mockData[hardwareId] || null;
    }
    
    renderHardwareDetails(hardware) {
        return `
            <div class="hardware-detail-full">
                <div class="detail-header">
                    <h3>${hardware.name}</h3>
                    <span class="compatibility-badge ${hardware.compatibility_status}">
                        ${hardware.compatibility_status}
                    </span>
                </div>
                
                <div class="detail-sections">
                    <div class="detail-section">
                        <h4>General Information</h4>
                        <table class="detail-table">
                            <tr><td>Manufacturer:</td><td>${hardware.manufacturer}</td></tr>
                            <tr><td>Model:</td><td>${hardware.model}</td></tr>
                            <tr><td>Category:</td><td>${hardware.category}</td></tr>
                            <tr><td>PCI ID:</td><td><code>${hardware.pci_id || 'N/A'}</code></td></tr>
                            <tr><td>Driver:</td><td><code>${hardware.driver || 'N/A'}</code></td></tr>
                            <tr><td>Kernel Support:</td><td>${hardware.kernel_support || 'Unknown'}</td></tr>
                            <tr><td>Last Tested:</td><td>${hardware.last_tested}</td></tr>
                            <tr><td>Community Rating:</td><td>⭐ ${hardware.community_rating || 'N/A'}/5</td></tr>
                            <tr><td>Reports:</td><td>${hardware.reports_count || 0} submissions</td></tr>
                        </table>
                    </div>
                    
                    ${hardware.specifications ? `
                    <div class="detail-section">
                        <h4>Specifications</h4>
                        <table class="detail-table">
                            ${Object.entries(hardware.specifications).map(([key, value]) => `
                                <tr>
                                    <td>${key.replace(/_/g, ' ').charAt(0).toUpperCase() + key.slice(1)}:</td>
                                    <td>${Array.isArray(value) ? value.join(', ') : value}</td>
                                </tr>
                            `).join('')}
                        </table>
                    </div>
                    ` : ''}
                    
                    <div class="detail-section">
                        <h4>Linux Distribution Compatibility</h4>
                        <div class="distro-compat-grid">
                            ${Object.entries(hardware.compatibility || {}).map(([distro, info]) => `
                                <div class="distro-compat-card">
                                    <h5>${distro.charAt(0).toUpperCase() + distro.slice(1)}</h5>
                                    <span class="compatibility-badge ${info.status}">${info.status}</span>
                                    <div class="compat-details">
                                        <p><strong>Min Kernel:</strong> ${info.kernel_min || 'N/A'}</p>
                                        <p><strong>Packages:</strong> ${info.packages ? info.packages.join(', ') : 'None'}</p>
                                        <p><strong>Notes:</strong> ${info.notes || 'No additional notes'}</p>
                                    </div>
                                </div>
                            `).join('')}
                        </div>
                    </div>
                    
                    ${hardware.known_issues && hardware.known_issues.length > 0 ? `
                    <div class="detail-section">
                        <h4>Known Issues</h4>
                        <ul class="issues-list">
                            ${hardware.known_issues.map(issue => `<li>${issue}</li>`).join('')}
                        </ul>
                    </div>
                    ` : ''}
                </div>
            </div>
        `;
    }
    
    hideModal() {
        this.modal.classList.remove('show');
        document.body.style.overflow = '';
        
        // Restore focus to the element that opened the modal
        if (this.previouslyFocusedElement) {
            this.previouslyFocusedElement.focus();
            this.previouslyFocusedElement = null;
        }
    }
    
    copySpecs() {
        const body = document.getElementById('hardware-details-body');
        const text = body.innerText;
        
        navigator.clipboard.writeText(text).then(() => {
            this.showNotification('Specifications copied to clipboard!');
        }).catch(err => {
            console.error('Failed to copy:', err);
            this.showNotification('Failed to copy specifications', 'error');
        });
    }
    
    submitReport() {
        // Open submission form or redirect to GitHub
        this.showNotification('Opening submission form...');
        setTimeout(() => {
            window.open('https://github.com/lx-hw-db/lx-hw-db/issues/new', '_blank');
        }, 1000);
    }
    
    showNotification(message, type = 'success') {
        const notification = document.createElement('div');
        notification.className = `notification ${type}`;
        notification.textContent = message;
        document.body.appendChild(notification);
        
        setTimeout(() => {
            notification.classList.add('show');
        }, 10);
        
        setTimeout(() => {
            notification.classList.remove('show');
            setTimeout(() => notification.remove(), 300);
        }, 3000);
    }
    
    showError(message) {
        const body = document.getElementById('hardware-details-body');
        body.innerHTML = `
            <div class="error-message">
                <p>❌ ${message}</p>
            </div>
        `;
        this.modal.classList.add('show');
    }
}

// Initialize when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        window.hardwareDetails = new HardwareDetails();
    });
} else {
    window.hardwareDetails = new HardwareDetails();
}