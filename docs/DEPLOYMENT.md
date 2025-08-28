# Linux Hardware Database - Deployment Guide

This guide covers all deployment options for the Linux Hardware Database (lx-hw-db) project.

## Overview

The lx-hw-db project provides multiple deployment methods to suit different use cases:

- **CLI Tools**: Hardware detection and indexing utilities
- **GUI Applications**: GTK4 and Qt6 graphical interfaces  
- **Web Interface**: Browser-based hardware database and search
- **Container Deployment**: Docker and Podman containers
- **Package Distribution**: Native packages for major Linux distributions

## Quick Start

### Automated Installation

The easiest way to install lx-hw-db is using our universal installation script:

```bash
curl -fsSL https://raw.githubusercontent.com/lx-hw-db/lx-hw-db/main/install.sh | bash
```

This script will:
1. Detect your distribution
2. Choose the best installation method
3. Install all dependencies
4. Set up shell completions

### Manual Installation Options

#### Option 1: Distribution Packages (Recommended)

**Ubuntu/Debian:**
```bash
curl -fsSL https://raw.githubusercontent.com/lx-hw-db/lx-hw-db/main/scripts/install-ubuntu.sh | bash
```

**Fedora/RHEL/CentOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/lx-hw-db/lx-hw-db/main/scripts/install-fedora.sh | bash
```

**Arch Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/lx-hw-db/lx-hw-db/main/scripts/install-arch.sh | bash
```

**NixOS:**
```bash
curl -fsSL https://raw.githubusercontent.com/lx-hw-db/lx-hw-db/main/scripts/install-nixos.sh | bash
```

#### Option 2: Pre-built Binaries

Download the latest release for your architecture:

```bash
# x86_64 GNU libc
wget https://github.com/lx-hw-db/lx-hw-db/releases/latest/download/lx-hw-db-v0.1.0-x86_64-unknown-linux-gnu.tar.gz

# ARM64 GNU libc  
wget https://github.com/lx-hw-db/lx-hw-db/releases/latest/download/lx-hw-db-v0.1.0-aarch64-unknown-linux-gnu.tar.gz

# x86_64 musl (static)
wget https://github.com/lx-hw-db/lx-hw-db/releases/latest/download/lx-hw-db-v0.1.0-x86_64-unknown-linux-musl.tar.gz

# Extract and install
tar -xzf lx-hw-db-*.tar.gz
sudo cp lx-hw-detect lx-hw-indexer /usr/local/bin/
```

#### Option 3: Cargo (Build from Source)

```bash
# Install from crates.io
cargo install lx-hw-detect

# Or install from Git
cargo install --git https://github.com/lx-hw-db/lx-hw-db --bin lx-hw-detect --bin lx-hw-indexer
```

#### Option 4: Flatpak

```bash
flatpak install flathub org.lxhwdb.LxHwDb
```

#### Option 5: AppImage

```bash
# Download AppImage
wget https://github.com/lx-hw-db/lx-hw-db/releases/latest/download/lx-hw-db-0.1.0-x86_64.AppImage

# Make executable and run
chmod +x lx-hw-db-0.1.0-x86_64.AppImage
./lx-hw-db-0.1.0-x86_64.AppImage
```

## Container Deployment

### Docker

#### CLI Tools
```bash
# Pull CLI image
docker pull ghcr.io/lx-hw-db/lx-hw-db:latest-cli

# Run hardware detection
docker run --rm --privileged \
  -v /sys:/sys:ro \
  -v /proc:/proc:ro \
  -v /dev:/dev:ro \
  -v "$PWD":/data \
  ghcr.io/lx-hw-db/lx-hw-db:latest-cli detect --output /data
```

#### Web Interface
```bash
# Pull web interface image
docker pull ghcr.io/lx-hw-db/lx-hw-db:latest-web

# Run web interface
docker run -d --name lx-hw-db-web \
  -p 8000:8000 \
  -v "$PWD/data":/var/www/lx-hw-db/data \
  ghcr.io/lx-hw-db/lx-hw-db:latest-web
```

#### Using Docker Compose

```bash
# Clone repository
git clone https://github.com/lx-hw-db/lx-hw-db.git
cd lx-hw-db

# Start all services
docker compose up -d

# Or use production configuration
cd web
docker compose -f docker-compose.production.yml up -d
```

### Podman

```bash
# Same commands work with Podman
podman pull ghcr.io/lx-hw-db/lx-hw-db:latest-web
podman run -d --name lx-hw-db-web -p 8000:8000 ghcr.io/lx-hw-db/lx-hw-db:latest-web
```

## Web Interface Deployment

### Standalone Python Server

```bash
# Download web interface bundle
wget https://github.com/lx-hw-db/lx-hw-db/releases/latest/download/lx-hw-db-web-0.1.0.tar.gz
tar -xzf lx-hw-db-web-0.1.0.tar.gz
cd lx-hw-db-web-0.1.0

# Install and start
sudo bash install.sh

# Manual start
python3 serve.py --host 0.0.0.0 --port 8000
```

### Apache/Nginx Reverse Proxy

#### Apache Configuration

```apache
<VirtualHost *:80>
    ServerName lx-hw-db.example.com
    
    ProxyPass / http://127.0.0.1:8000/
    ProxyPassReverse / http://127.0.0.1:8000/
    ProxyPreserveHost On
    
    # Optional: Serve static files directly
    DocumentRoot /var/www/lx-hw-db
    ProxyPass /css !
    ProxyPass /js !
    ProxyPass /data !
</VirtualHost>
```

#### Nginx Configuration

```nginx
server {
    listen 80;
    server_name lx-hw-db.example.com;
    
    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
    
    # Optional: Serve static files directly
    location ~* \.(css|js|png|jpg|jpeg|gif|ico|svg)$ {
        root /var/www/lx-hw-db;
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

### Systemd Service

```ini
# /etc/systemd/system/lx-hw-db-web.service
[Unit]
Description=Linux Hardware Database Web Interface
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/var/www/lx-hw-db
ExecStart=/usr/bin/python3 serve.py --host 0.0.0.0 --port 8000
Restart=always
RestartSec=3

# Security settings
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
ReadWritePaths=/var/www/lx-hw-db/data

[Install]
WantedBy=multi-user.target
```

```bash
# Enable and start service
sudo systemctl enable lx-hw-db-web
sudo systemctl start lx-hw-db-web
```

## Development Deployment

### Local Development

```bash
# Clone repository
git clone https://github.com/lx-hw-db/lx-hw-db.git
cd lx-hw-db

# Using Nix (recommended)
nix develop
cargo run --bin lx-hw-detect -- --help

# Or install dependencies manually
# See docs/INSTALL.md for distribution-specific instructions

# Start web interface
cd web
python3 serve.py --port 3000
```

### Development Container

```bash
# Build development container
docker build --target dev -t lx-hw-db:dev .

# Run development environment
docker run -it --rm \
  -v "$PWD":/workspace \
  lx-hw-db:dev

# Or use Docker Compose
docker compose -f docker-compose.yml run lx-hw-db-dev
```

## Production Deployment Considerations

### Security

1. **Container Security**
   - Use non-root users
   - Enable read-only root filesystem where possible
   - Apply resource limits
   - Use security profiles (AppArmor/SELinux)

2. **Web Interface Security**
   - Enable HTTPS with TLS certificates
   - Configure proper CORS headers
   - Use reverse proxy for SSL termination
   - Implement rate limiting

3. **Data Privacy**
   - Configure privacy levels appropriately
   - Rotate anonymization salts regularly
   - Monitor for data leakage
   - Implement data retention policies

### Performance

1. **Web Interface Optimization**
   - Enable gzip compression
   - Configure proper caching headers
   - Use CDN for static assets
   - Implement database indexing

2. **Container Optimization**
   - Use multi-stage builds
   - Minimize image layers
   - Use appropriate base images
   - Configure resource limits

### Monitoring

1. **Health Checks**
   - Web interface endpoints
   - Container health status
   - Database connectivity
   - Disk space monitoring

2. **Logging**
   - Centralized log collection
   - Error tracking
   - Performance metrics
   - Security audit logs

### Backup and Recovery

1. **Data Backup**
   - Hardware reports database
   - Configuration files
   - Generated indices
   - User-submitted data

2. **Recovery Procedures**
   - Database restoration
   - Service recovery
   - Disaster recovery planning
   - Data migration procedures

## Troubleshooting

### Common Issues

1. **Permission Errors**
   ```bash
   # Fix hardware detection permissions
   sudo usermod -a -G dialout $USER
   sudo chmod +r /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor
   ```

2. **Missing Dependencies**
   ```bash
   # Install hardware detection tools
   sudo apt install lshw dmidecode pciutils usbutils  # Ubuntu/Debian
   sudo dnf install lshw dmidecode pciutils usbutils  # Fedora
   sudo pacman -S lshw dmidecode pciutils usbutils    # Arch
   ```

3. **GUI Issues**
   ```bash
   # Install GUI libraries
   sudo apt install libgtk-4-1 libadwaita-1-0  # Ubuntu/Debian
   sudo dnf install gtk4 libadwaita             # Fedora
   sudo pacman -S gtk4 libadwaita               # Arch
   ```

4. **Container Issues**
   ```bash
   # Enable privileged mode for hardware access
   docker run --privileged ...
   
   # Mount required directories
   -v /sys:/sys:ro -v /proc:/proc:ro -v /dev:/dev:ro
   ```

### Getting Help

- **Documentation**: https://github.com/lx-hw-db/lx-hw-db/tree/main/docs
- **Issues**: https://github.com/lx-hw-db/lx-hw-db/issues
- **Discussions**: https://github.com/lx-hw-db/lx-hw-db/discussions
- **Matrix Chat**: #lx-hw-db:matrix.org

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for information about contributing to the project, including:

- Development setup
- Testing procedures
- Pull request process
- Community guidelines

## License

Linux Hardware Database is licensed under the AGPL-3.0-or-later license. See [LICENSE](../LICENSE) for full terms.