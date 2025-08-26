/**
 * Configuration Export System
 * Export configuration tips as shell scripts, Ansible playbooks, Docker files, etc.
 */

class TipExportSystem {
    constructor() {
        this.isInitialized = false;
        this.exportFormats = {
            'shell': {
                name: 'Shell Script',
                icon: '📄',
                extension: '.sh',
                description: 'Bash script to run all commands'
            },
            'ansible': {
                name: 'Ansible Playbook',
                icon: '🎭',
                extension: '.yml',
                description: 'Ansible playbook for automated deployment'
            },
            'dockerfile': {
                name: 'Dockerfile',
                icon: '🐳',
                extension: '.dockerfile',
                description: 'Docker container configuration'
            },
            'nixos': {
                name: 'NixOS Configuration',
                icon: '❄️',
                extension: '.nix',
                description: 'Declarative NixOS configuration'
            },
            'markdown': {
                name: 'Markdown Documentation',
                icon: '📝',
                extension: '.md',
                description: 'Human-readable documentation'
            },
            'json': {
                name: 'JSON Data',
                icon: '📊',
                extension: '.json',
                description: 'Structured data export'
            }
        };

        this.distributions = {
            'debian': { name: 'Debian/Ubuntu', packageManager: 'apt' },
            'arch': { name: 'Arch Linux', packageManager: 'pacman' },
            'fedora': { name: 'Fedora/RHEL', packageManager: 'dnf' },
            'nixos': { name: 'NixOS', packageManager: 'nix' },
            'opensuse': { name: 'openSUSE', packageManager: 'zypper' },
            'gentoo': { name: 'Gentoo', packageManager: 'portage' },
            'alpine': { name: 'Alpine Linux', packageManager: 'apk' }
        };
    }

    /**
     * Initialize the export system
     */
    async initialize() {
        console.log('Initializing tip export system...');
        
        this.setupExportInterface();
        this.isInitialized = true;
        
        console.log('Tip export system initialized successfully');
    }

    /**
     * Set up export interface
     */
    setupExportInterface() {
        // Add export functionality to configuration tips modal
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('config-tips-btn')) {
                setTimeout(() => this.addExportToModal(), 300);
            }
        });

        // Add export buttons to search results
        this.setupSearchResultExport();
    }

    /**
     * Add export functionality to configuration tips modal
     */
    addExportToModal() {
        const modal = document.querySelector('.config-tips-modal');
        if (!modal) return;

        const footer = modal.querySelector('.config-modal-footer');
        if (!footer || footer.querySelector('.export-tips-btn')) return;

        // Add export button to modal footer
        const exportBtn = document.createElement('button');
        exportBtn.className = 'btn-secondary export-tips-btn';
        exportBtn.innerHTML = '📦 Export Configuration';
        exportBtn.addEventListener('click', () => this.showExportModal());

        // Insert before the "Copy All Commands" button
        const copyAllBtn = footer.querySelector('.copy-all-btn');
        if (copyAllBtn) {
            footer.insertBefore(exportBtn, copyAllBtn);
        } else {
            footer.appendChild(exportBtn);
        }
    }

    /**
     * Set up export for search results
     */
    setupSearchResultExport() {
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('export-tip-btn')) {
                const tipId = e.target.dataset.tipId;
                this.exportSingleTip(tipId);
            }
            
            if (e.target.classList.contains('export-selected-btn')) {
                this.exportSelectedTips();
            }
        });
    }

    /**
     * Show export modal for current tips
     */
    showExportModal(tips = null) {
        // Get current tips from modal or search results
        if (!tips) {
            tips = this.getCurrentTipsFromModal();
        }

        if (!tips || tips.length === 0) {
            this.showNotification('No tips available to export', 'warning');
            return;
        }

        const exportModal = this.createExportModal(tips);
        document.body.appendChild(exportModal);
        exportModal.style.display = 'flex';
        document.body.style.overflow = 'hidden';
    }

    /**
     * Get current tips from the modal
     */
    getCurrentTipsFromModal() {
        // Try to get tips from search results first
        if (window.tipSearchSystem && window.tipSearchSystem.filteredTips) {
            return window.tipSearchSystem.filteredTips;
        }

        // Fallback to configuration tips
        if (window.configurationTips && window.configurationTips.configurationTips) {
            const allTips = [];
            const tipsByComponent = window.configurationTips.configurationTips;
            
            Object.entries(tipsByComponent).forEach(([componentKey, distributions]) => {
                Object.entries(distributions).forEach(([distroKey, tips]) => {
                    tips.forEach(tip => {
                        const [component, vendor] = componentKey.split(':');
                        allTips.push({
                            ...tip,
                            component: component,
                            vendor: vendor || '',
                            distribution: distroKey
                        });
                    });
                });
            });
            
            return allTips;
        }

        return [];
    }

    /**
     * Create export modal
     */
    createExportModal(tips) {
        const modal = document.createElement('div');
        modal.className = 'tip-export-modal';
        modal.innerHTML = `
            <div class="export-modal-content">
                <div class="export-modal-header">
                    <h2>📦 Export Configuration Tips</h2>
                    <button class="export-modal-close" aria-label="Close">&times;</button>
                </div>
                
                <div class="export-modal-body">
                    <div class="export-summary">
                        <div class="summary-stats">
                            <div class="stat-item">
                                <span class="stat-number">${tips.length}</span>
                                <span class="stat-label">Tips Selected</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-number">${this.getUniqueDistributions(tips).length}</span>
                                <span class="stat-label">Distributions</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-number">${this.getUniqueComponents(tips).length}</span>
                                <span class="stat-label">Components</span>
                            </div>
                        </div>
                    </div>

                    <div class="export-options">
                        <h3>Export Options</h3>
                        
                        <div class="export-filters">
                            <div class="filter-group">
                                <label>Select Distribution:</label>
                                <select id="export-distribution" class="export-select">
                                    <option value="all">All Distributions</option>
                                    ${this.getUniqueDistributions(tips).map(distro => 
                                        `<option value="${distro}">${this.distributions[distro]?.name || distro}</option>`
                                    ).join('')}
                                </select>
                            </div>
                            
                            <div class="filter-group">
                                <label>Hardware Components:</label>
                                <div class="component-checkboxes">
                                    <label class="component-checkbox">
                                        <input type="checkbox" value="all" checked>
                                        <span>All Components</span>
                                    </label>
                                    ${this.getUniqueComponents(tips).map(component => 
                                        `<label class="component-checkbox">
                                            <input type="checkbox" value="${component}" checked>
                                            <span>${this.getComponentDisplayName(component)}</span>
                                        </label>`
                                    ).join('')}
                                </div>
                            </div>
                        </div>

                        <div class="export-formats">
                            <h4>Export Format:</h4>
                            <div class="format-grid">
                                ${Object.entries(this.exportFormats).map(([key, format]) => `
                                    <div class="format-card" data-format="${key}">
                                        <div class="format-icon">${format.icon}</div>
                                        <div class="format-info">
                                            <h5>${format.name}</h5>
                                            <p>${format.description}</p>
                                        </div>
                                    </div>
                                `).join('')}
                            </div>
                        </div>

                        <div class="export-advanced">
                            <details class="advanced-options">
                                <summary>Advanced Options</summary>
                                <div class="advanced-content">
                                    <div class="option-group">
                                        <label class="option-checkbox">
                                            <input type="checkbox" id="include-warnings" checked>
                                            <span>Include warnings in comments</span>
                                        </label>
                                        
                                        <label class="option-checkbox">
                                            <input type="checkbox" id="include-descriptions" checked>
                                            <span>Include command descriptions</span>
                                        </label>
                                        
                                        <label class="option-checkbox">
                                            <input type="checkbox" id="include-contributors">
                                            <span>Include contributor attribution</span>
                                        </label>
                                        
                                        <label class="option-checkbox">
                                            <input type="checkbox" id="group-by-component" checked>
                                            <span>Group commands by hardware component</span>
                                        </label>
                                    </div>
                                </div>
                            </details>
                        </div>
                    </div>
                </div>
                
                <div class="export-modal-footer">
                    <button class="btn-secondary cancel-export">Cancel</button>
                    <button class="btn-secondary preview-export">👁️ Preview</button>
                    <button class="btn-primary generate-export" disabled>
                        📥 Generate Export
                    </button>
                </div>
            </div>
        `;

        this.setupExportModalEvents(modal, tips);
        return modal;
    }

    /**
     * Set up export modal events
     */
    setupExportModalEvents(modal, tips) {
        // Close modal
        modal.querySelector('.export-modal-close').addEventListener('click', () => {
            this.hideExportModal(modal);
        });

        modal.querySelector('.cancel-export').addEventListener('click', () => {
            this.hideExportModal(modal);
        });

        // Format selection
        let selectedFormat = null;
        modal.querySelectorAll('.format-card').forEach(card => {
            card.addEventListener('click', () => {
                // Remove previous selection
                modal.querySelectorAll('.format-card').forEach(c => c.classList.remove('selected'));
                
                // Select this format
                card.classList.add('selected');
                selectedFormat = card.dataset.format;
                
                // Enable generate button
                modal.querySelector('.generate-export').disabled = false;
            });
        });

        // Preview export
        modal.querySelector('.preview-export').addEventListener('click', () => {
            if (selectedFormat) {
                this.previewExport(modal, tips, selectedFormat);
            } else {
                this.showNotification('Please select an export format first', 'warning');
            }
        });

        // Generate export
        modal.querySelector('.generate-export').addEventListener('click', () => {
            if (selectedFormat) {
                this.generateExport(modal, tips, selectedFormat);
            }
        });

        // Component checkboxes
        const allCheckbox = modal.querySelector('input[value="all"]');
        const componentCheckboxes = modal.querySelectorAll('.component-checkbox input:not([value="all"])');
        
        allCheckbox.addEventListener('change', (e) => {
            componentCheckboxes.forEach(cb => cb.checked = e.target.checked);
        });

        componentCheckboxes.forEach(cb => {
            cb.addEventListener('change', () => {
                const allChecked = Array.from(componentCheckboxes).every(cb => cb.checked);
                allCheckbox.checked = allChecked;
            });
        });

        // Close on background click
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                this.hideExportModal(modal);
            }
        });
    }

    /**
     * Hide export modal
     */
    hideExportModal(modal) {
        modal.style.display = 'none';
        document.body.style.overflow = '';
        setTimeout(() => modal.remove(), 300);
    }

    /**
     * Get unique distributions from tips
     */
    getUniqueDistributions(tips) {
        return [...new Set(tips.map(tip => tip.distribution))].filter(Boolean);
    }

    /**
     * Get unique components from tips
     */
    getUniqueComponents(tips) {
        return [...new Set(tips.map(tip => tip.component))].filter(Boolean);
    }

    /**
     * Get component display name
     */
    getComponentDisplayName(component) {
        const names = {
            'cpu': 'CPU/Processor',
            'gpu': 'Graphics Card',
            'memory': 'Memory/RAM',
            'storage': 'Storage Device',
            'network': 'Network Adapter',
            'audio': 'Audio Device',
            'input': 'Input Device',
            'display': 'Display/Monitor',
            'system': 'System/BIOS'
        };
        return names[component] || component.charAt(0).toUpperCase() + component.slice(1);
    }

    /**
     * Preview export
     */
    previewExport(modal, tips, format) {
        const options = this.getExportOptions(modal);
        const filteredTips = this.filterTipsForExport(tips, options);
        const exportContent = this.generateExportContent(filteredTips, format, options);

        // Create preview modal
        const previewModal = document.createElement('div');
        previewModal.className = 'export-preview-modal';
        previewModal.innerHTML = `
            <div class="preview-modal-content">
                <div class="preview-modal-header">
                    <h2>📄 Export Preview - ${this.exportFormats[format].name}</h2>
                    <button class="preview-modal-close" aria-label="Close">&times;</button>
                </div>
                <div class="preview-modal-body">
                    <div class="preview-info">
                        <span>📊 ${filteredTips.length} tips • ${format.toUpperCase()}</span>
                        <button class="copy-preview-btn" title="Copy to clipboard">📋 Copy</button>
                    </div>
                    <pre class="export-preview"><code>${exportContent}</code></pre>
                </div>
                <div class="preview-modal-footer">
                    <button class="btn-secondary close-preview">Close Preview</button>
                    <button class="btn-primary download-from-preview">📥 Download File</button>
                </div>
            </div>
        `;

        document.body.appendChild(previewModal);
        previewModal.style.display = 'flex';

        // Set up preview modal events
        previewModal.querySelector('.preview-modal-close').addEventListener('click', () => {
            previewModal.remove();
        });

        previewModal.querySelector('.close-preview').addEventListener('click', () => {
            previewModal.remove();
        });

        previewModal.querySelector('.copy-preview-btn').addEventListener('click', async () => {
            try {
                await navigator.clipboard.writeText(exportContent);
                this.showNotification('Export content copied to clipboard!', 'success');
            } catch (error) {
                console.error('Failed to copy export:', error);
                this.showNotification('Failed to copy export content', 'error');
            }
        });

        previewModal.querySelector('.download-from-preview').addEventListener('click', () => {
            this.downloadExport(exportContent, format, 'hardware-config');
            previewModal.remove();
        });

        previewModal.addEventListener('click', (e) => {
            if (e.target === previewModal) {
                previewModal.remove();
            }
        });
    }

    /**
     * Generate export
     */
    generateExport(modal, tips, format) {
        const options = this.getExportOptions(modal);
        const filteredTips = this.filterTipsForExport(tips, options);
        const exportContent = this.generateExportContent(filteredTips, format, options);

        // Generate filename
        const timestamp = new Date().toISOString().split('T')[0];
        const filename = `lx-hw-config-${timestamp}`;

        // Download the file
        this.downloadExport(exportContent, format, filename);
        
        this.showNotification(`Export generated: ${filename}${this.exportFormats[format].extension}`, 'success');
        this.hideExportModal(modal);
    }

    /**
     * Get export options from modal
     */
    getExportOptions(modal) {
        const distribution = modal.querySelector('#export-distribution').value;
        const selectedComponents = Array.from(modal.querySelectorAll('.component-checkbox input:checked'))
            .map(cb => cb.value)
            .filter(v => v !== 'all');

        return {
            distribution: distribution === 'all' ? null : distribution,
            components: selectedComponents,
            includeWarnings: modal.querySelector('#include-warnings').checked,
            includeDescriptions: modal.querySelector('#include-descriptions').checked,
            includeContributors: modal.querySelector('#include-contributors').checked,
            groupByComponent: modal.querySelector('#group-by-component').checked
        };
    }

    /**
     * Filter tips for export based on options
     */
    filterTipsForExport(tips, options) {
        let filtered = [...tips];

        // Filter by distribution
        if (options.distribution) {
            filtered = filtered.filter(tip => tip.distribution === options.distribution);
        }

        // Filter by components
        if (options.components.length > 0) {
            filtered = filtered.filter(tip => options.components.includes(tip.component));
        }

        return filtered;
    }

    /**
     * Generate export content based on format
     */
    generateExportContent(tips, format, options) {
        switch (format) {
            case 'shell':
                return this.generateShellScript(tips, options);
            case 'ansible':
                return this.generateAnsiblePlaybook(tips, options);
            case 'dockerfile':
                return this.generateDockerfile(tips, options);
            case 'nixos':
                return this.generateNixOSConfig(tips, options);
            case 'markdown':
                return this.generateMarkdownDoc(tips, options);
            case 'json':
                return this.generateJSONExport(tips, options);
            default:
                throw new Error(`Unsupported export format: ${format}`);
        }
    }

    /**
     * Generate shell script
     */
    generateShellScript(tips, options) {
        const timestamp = new Date().toISOString();
        let script = `#!/bin/bash
# Hardware Configuration Script
# Generated by Linux Hardware Compatibility Database
# Date: ${timestamp}
# Tips: ${tips.length}

set -e  # Exit on any error

echo "🔧 Starting hardware configuration..."
echo "Generated from ${tips.length} configuration tips"
echo ""

`;

        // Group by component if requested
        if (options.groupByComponent) {
            const grouped = this.groupTipsByComponent(tips);
            
            Object.entries(grouped).forEach(([component, componentTips]) => {
                script += `# ========================================
# ${this.getComponentDisplayName(component)} Configuration
# ========================================

`;
                script += this.generateShellCommandsForTips(componentTips, options);
                script += '\n';
            });
        } else {
            script += this.generateShellCommandsForTips(tips, options);
        }

        script += `
echo ""
echo "✅ Hardware configuration completed successfully!"
echo "Please reboot your system if required by any of the changes."
`;

        return script;
    }

    /**
     * Generate shell commands for tips
     */
    generateShellCommandsForTips(tips, options) {
        let commands = '';
        
        tips.forEach((tip, index) => {
            commands += `# ${tip.title}\n`;
            if (options.includeDescriptions && tip.description) {
                commands += `# ${tip.description}\n`;
            }
            
            if (options.includeContributors && tip.contributor) {
                commands += `# Contributed by: ${tip.contributor.name || tip.contributor.username}\n`;
            }
            
            if (options.includeWarnings && tip.warnings) {
                const warnings = Array.isArray(tip.warnings) ? tip.warnings : [tip.warnings];
                warnings.forEach(warning => {
                    commands += `# ⚠️  WARNING: ${warning}\n`;
                });
            }
            
            commands += '\n';

            // Add commands
            if (tip.commands) {
                tip.commands.forEach(cmdObj => {
                    const command = typeof cmdObj === 'string' ? cmdObj : cmdObj.command;
                    if (options.includeDescriptions && cmdObj.description) {
                        commands += `# ${cmdObj.description}\n`;
                    }
                    commands += `${command}\n`;
                });
            }

            // Add configuration files
            if (tip.configuration) {
                Object.entries(tip.configuration).forEach(([filePath, content]) => {
                    commands += `
# Create configuration file: ${filePath}
cat > "${filePath}" << 'EOF'
${content}
EOF
`;
                });
            }
            
            commands += '\n';
        });

        return commands;
    }

    /**
     * Generate Ansible playbook
     */
    generateAnsiblePlaybook(tips, options) {
        const timestamp = new Date().toISOString();
        
        const playbook = {
            name: 'Hardware Configuration Playbook',
            hosts: 'localhost',
            connection: 'local',
            gather_facts: true,
            vars: {
                generated_date: timestamp,
                tip_count: tips.length
            },
            tasks: []
        };

        tips.forEach((tip, index) => {
            const task = {
                name: tip.title,
                block: []
            };

            if (tip.commands) {
                tip.commands.forEach(cmdObj => {
                    const command = typeof cmdObj === 'string' ? cmdObj : cmdObj.command;
                    task.block.push({
                        name: cmdObj.description || `Execute: ${command}`,
                        shell: command,
                        become: command.includes('sudo')
                    });
                });
            }

            if (tip.configuration) {
                Object.entries(tip.configuration).forEach(([filePath, content]) => {
                    task.block.push({
                        name: `Create configuration file: ${filePath}`,
                        copy: {
                            content: content,
                            dest: filePath,
                            backup: true
                        },
                        become: filePath.startsWith('/etc/')
                    });
                });
            }

            if (task.block.length > 0) {
                playbook.tasks.push(task);
            }
        });

        return `---
# Hardware Configuration Playbook
# Generated by Linux Hardware Compatibility Database
# Date: ${timestamp}

${this.yamlStringify([playbook])}`;
    }

    /**
     * Generate Dockerfile
     */
    generateDockerfile(tips, options) {
        const distribution = options.distribution || 'debian';
        const baseImages = {
            'debian': 'debian:bookworm',
            'arch': 'archlinux:latest',
            'fedora': 'fedora:latest',
            'alpine': 'alpine:latest'
        };

        let dockerfile = `# Hardware Configuration Dockerfile
# Generated by Linux Hardware Compatibility Database
# Base distribution: ${this.distributions[distribution]?.name || distribution}

FROM ${baseImages[distribution] || 'debian:bookworm'}

# Update package manager
`;

        // Add package manager update based on distribution
        const packageUpdates = {
            'debian': 'RUN apt-get update && apt-get upgrade -y',
            'arch': 'RUN pacman -Syu --noconfirm',
            'fedora': 'RUN dnf update -y',
            'alpine': 'RUN apk update && apk upgrade'
        };

        dockerfile += packageUpdates[distribution] || packageUpdates['debian'];
        dockerfile += '\n\n';

        // Add tips as RUN commands
        tips.forEach(tip => {
            dockerfile += `# ${tip.title}\n`;
            if (options.includeDescriptions && tip.description) {
                dockerfile += `# ${tip.description}\n`;
            }

            if (tip.commands) {
                tip.commands.forEach(cmdObj => {
                    const command = typeof cmdObj === 'string' ? cmdObj : cmdObj.command;
                    dockerfile += `RUN ${command.replace(/^sudo\s+/, '')}\n`;
                });
            }

            dockerfile += '\n';
        });

        dockerfile += `# Clean up
RUN apt-get clean || pacman -Sc --noconfirm || dnf clean all || apk cache clean

# Set working directory
WORKDIR /app

CMD ["/bin/bash"]
`;

        return dockerfile;
    }

    /**
     * Generate NixOS configuration
     */
    generateNixOSConfig(tips, options) {
        const timestamp = new Date().toISOString();
        
        let config = `# NixOS Hardware Configuration
# Generated by Linux Hardware Compatibility Database
# Date: ${timestamp}

{ config, pkgs, ... }:

{
  # Hardware-specific configuration
  
`;

        // Filter for NixOS-specific tips
        const nixosTips = tips.filter(tip => tip.distribution === 'nixos');
        
        nixosTips.forEach(tip => {
            config += `  # ${tip.title}\n`;
            if (options.includeDescriptions && tip.description) {
                config += `  # ${tip.description}\n`;
            }
            
            if (tip.configuration) {
                Object.entries(tip.configuration).forEach(([filePath, content]) => {
                    if (filePath.includes('configuration.nix')) {
                        config += `${content}\n\n`;
                    }
                });
            }
        });

        config += '}\n';
        return config;
    }

    /**
     * Generate Markdown documentation
     */
    generateMarkdownDoc(tips, options) {
        const timestamp = new Date().toISOString();
        
        let markdown = `# Hardware Configuration Guide

> Generated by Linux Hardware Compatibility Database  
> Date: ${timestamp}  
> Tips: ${tips.length}

## Overview

This document contains configuration tips for your Linux hardware setup.

`;

        if (options.groupByComponent) {
            const grouped = this.groupTipsByComponent(tips);
            
            Object.entries(grouped).forEach(([component, componentTips]) => {
                markdown += `## ${this.getComponentDisplayName(component)}\n\n`;
                
                componentTips.forEach(tip => {
                    markdown += this.formatTipAsMarkdown(tip, options);
                });
            });
        } else {
            tips.forEach(tip => {
                markdown += this.formatTipAsMarkdown(tip, options);
            });
        }

        return markdown;
    }

    /**
     * Format single tip as markdown
     */
    formatTipAsMarkdown(tip, options) {
        let markdown = `### ${tip.title}\n\n`;
        
        if (tip.description) {
            markdown += `${tip.description}\n\n`;
        }

        // Metadata
        const metadata = [];
        if (tip.distribution) {
            metadata.push(`**Distribution:** ${this.distributions[tip.distribution]?.name || tip.distribution}`);
        }
        if (tip.component) {
            metadata.push(`**Component:** ${this.getComponentDisplayName(tip.component)}`);
        }
        if (tip.rating) {
            metadata.push(`**Rating:** ${tip.rating.toFixed(1)}/5.0`);
        }

        if (metadata.length > 0) {
            markdown += metadata.join(' | ') + '\n\n';
        }

        // Commands
        if (tip.commands && tip.commands.length > 0) {
            markdown += '**Commands:**\n\n';
            tip.commands.forEach(cmdObj => {
                const command = typeof cmdObj === 'string' ? cmdObj : cmdObj.command;
                markdown += '```bash\n';
                if (options.includeDescriptions && cmdObj.description) {
                    markdown += `# ${cmdObj.description}\n`;
                }
                markdown += `${command}\n`;
                markdown += '```\n\n';
            });
        }

        // Configuration files
        if (tip.configuration) {
            markdown += '**Configuration Files:**\n\n';
            Object.entries(tip.configuration).forEach(([filePath, content]) => {
                markdown += `**${filePath}:**\n`;
                markdown += '```\n';
                markdown += content;
                markdown += '\n```\n\n';
            });
        }

        // Warnings
        if (options.includeWarnings && tip.warnings) {
            markdown += '> **⚠️ Warnings:**\n';
            const warnings = Array.isArray(tip.warnings) ? tip.warnings : [tip.warnings];
            warnings.forEach(warning => {
                markdown += `> ${warning}\n`;
            });
            markdown += '\n';
        }

        // Contributor
        if (options.includeContributors && tip.contributor) {
            markdown += `*Contributed by: [${tip.contributor.name || tip.contributor.username}](${tip.contributor.profile_url})*\n`;
        }

        markdown += '\n---\n\n';
        return markdown;
    }

    /**
     * Generate JSON export
     */
    generateJSONExport(tips, options) {
        const exportData = {
            metadata: {
                generatedAt: new Date().toISOString(),
                generatedBy: 'Linux Hardware Compatibility Database',
                tipCount: tips.length,
                exportOptions: options
            },
            tips: tips.map(tip => ({
                id: tip.id,
                title: tip.title,
                description: tip.description,
                category: tip.category,
                component: tip.component,
                vendor: tip.vendor,
                distribution: tip.distribution,
                rating: tip.rating,
                votes: tip.votes,
                commands: tip.commands,
                configuration: tip.configuration,
                warnings: tip.warnings,
                contributor: options.includeContributors ? tip.contributor : undefined,
                date: tip.date
            }))
        };

        return JSON.stringify(exportData, null, 2);
    }

    /**
     * Group tips by component
     */
    groupTipsByComponent(tips) {
        const grouped = {};
        tips.forEach(tip => {
            const component = tip.component || 'other';
            if (!grouped[component]) {
                grouped[component] = [];
            }
            grouped[component].push(tip);
        });
        return grouped;
    }

    /**
     * Download export file
     */
    downloadExport(content, format, filename) {
        const extension = this.exportFormats[format].extension;
        const blob = new Blob([content], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        
        const a = document.createElement('a');
        a.href = url;
        a.download = `${filename}${extension}`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        URL.revokeObjectURL(url);
    }

    /**
     * Simple YAML stringify for Ansible playbooks
     */
    yamlStringify(data) {
        return JSON.stringify(data, null, 2)
            .replace(/"/g, '')
            .replace(/,\n/g, '\n')
            .replace(/{\n/g, '')
            .replace(/\n}/g, '')
            .replace(/\[/g, '')
            .replace(/\]/g, '')
            .replace(/:\s+/g, ': ');
    }

    /**
     * Show notification
     */
    showNotification(message, type = 'info') {
        if (window.githubAuth) {
            window.githubAuth.showNotification(message, type);
        } else {
            console.log(`${type.toUpperCase()}: ${message}`);
        }
    }

    /**
     * Export single tip
     */
    exportSingleTip(tipId) {
        let tip = null;
        
        // Find tip in search results or configuration tips
        if (window.tipSearchSystem && window.tipSearchSystem.filteredTips) {
            tip = window.tipSearchSystem.filteredTips.find(t => t.id === tipId);
        }

        if (!tip && window.configurationTips) {
            // Search in configuration tips
            const tipsByComponent = window.configurationTips.configurationTips;
            for (const [componentKey, distributions] of Object.entries(tipsByComponent)) {
                for (const [distroKey, tips] of Object.entries(distributions)) {
                    const foundTip = tips.find(t => t.id === tipId);
                    if (foundTip) {
                        const [component, vendor] = componentKey.split(':');
                        tip = {
                            ...foundTip,
                            component: component,
                            vendor: vendor || '',
                            distribution: distroKey
                        };
                        break;
                    }
                }
                if (tip) break;
            }
        }

        if (tip) {
            this.showExportModal([tip]);
        } else {
            this.showNotification('Tip not found for export', 'error');
        }
    }

    /**
     * Export selected tips (from checkboxes in search results)
     */
    exportSelectedTips() {
        const selectedCheckboxes = document.querySelectorAll('.tip-checkbox:checked');
        const selectedIds = Array.from(selectedCheckboxes).map(cb => cb.value);
        
        if (selectedIds.length === 0) {
            this.showNotification('No tips selected for export', 'warning');
            return;
        }

        const selectedTips = [];
        if (window.tipSearchSystem && window.tipSearchSystem.filteredTips) {
            selectedTips.push(...window.tipSearchSystem.filteredTips.filter(tip => 
                selectedIds.includes(tip.id)
            ));
        }

        if (selectedTips.length > 0) {
            this.showExportModal(selectedTips);
        } else {
            this.showNotification('Selected tips not found', 'error');
        }
    }
}

// Global instance
window.tipExportSystem = new TipExportSystem();

// Auto-initialize when DOM is ready
document.addEventListener('DOMContentLoaded', async () => {
    await window.tipExportSystem.initialize();
});

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = TipExportSystem;
}