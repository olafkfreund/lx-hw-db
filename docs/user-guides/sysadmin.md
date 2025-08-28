# System Administrator Guide

A comprehensive guide for IT professionals, DevOps engineers, and system administrators deploying Linux systems at scale.

## 🎯 Who This Guide Is For

This guide is designed for:
- **IT Administrators**: Managing enterprise Linux deployments
- **DevOps Engineers**: Automating infrastructure with hardware considerations  
- **Cloud Engineers**: Hybrid cloud deployments with on-premises hardware
- **HPC Administrators**: High-performance computing cluster management
- **Infrastructure Architects**: Planning large-scale Linux deployments

## 🏢 Enterprise Use Cases

### Hardware Procurement Planning

The database helps with strategic hardware purchasing decisions:

**🔍 Fleet Analysis**
- Standardize on well-supported hardware across deployments
- Identify hardware with excellent long-term kernel support
- Plan for end-of-life hardware replacement cycles
- Evaluate total cost of ownership including support complexity

**📊 Risk Assessment**
- Identify hardware with known compatibility issues
- Plan mitigations for problematic but required hardware
- Document workarounds and configuration requirements
- Track hardware support across kernel version lifecycles

### Deployment Automation

**🚀 Infrastructure as Code Integration**
- Generate Ansible playbooks from hardware compatibility data
- Create Terraform modules with hardware-specific configurations
- Build container images with required drivers and firmware
- Automate kernel parameter optimization based on detected hardware

**⚙️ Configuration Management**
- Maintain hardware-specific configuration profiles
- Automate driver installation and firmware updates
- Standardize kernel parameters across similar hardware
- Implement configuration drift detection and remediation

---

## 🏗️ Architecture for Enterprise Deployment

### Centralized Hardware Database

Deploy a private instance for organizational use while contributing anonymized data to the public database.

#### Deployment Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Enterprise Network                        │
│                                                             │
│  ┌───────────────┐    ┌─────────────────┐    ┌─────────────┐ │
│  │   Detection   │    │   Private DB    │    │  Analytics  │ │
│  │    Agents     │───▶│    Instance     │───▶│  Dashboard  │ │
│  │               │    │                 │    │             │ │
│  └───────────────┘    └─────────────────┘    └─────────────┘ │
│           │                     │                            │
│           │            ┌────────▼────────┐                   │
│           │            │  Anonymization  │                   │
│           └───────────▶│    Gateway      │                   │
│                        │                 │                   │
│                        └────────┬────────┘                   │
└─────────────────────────────────┼──────────────────────────────┘
                                  │
                     ┌────────────▼────────────┐
                     │    Public Database      │
                     │   (Community Shared)    │
                     └─────────────────────────┘
```

#### Private Instance Setup

**Docker Deployment (Recommended)**
```yaml
# docker-compose.yml
version: '3.8'
services:
  lx-hw-db:
    image: lx-hw-db:latest
    ports:
      - "8080:8000"
    volumes:
      - ./data:/app/data
      - ./config:/app/config
    environment:
      - LX_HW_DB_MODE=enterprise
      - LX_HW_DB_PRIVACY_LEVEL=strict
      - LX_HW_DB_PUBLIC_SYNC=true
    restart: unless-stopped

  database:
    image: postgres:15
    environment:
      POSTGRES_DB: lxhwdb
      POSTGRES_USER: lxhwdb
      POSTGRES_PASSWORD_FILE: /run/secrets/db_password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    secrets:
      - db_password

volumes:
  postgres_data:

secrets:
  db_password:
    file: ./secrets/db_password.txt
```

**Kubernetes Deployment**
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: lx-hw-db
  namespace: infrastructure
spec:
  replicas: 3
  selector:
    matchLabels:
      app: lx-hw-db
  template:
    metadata:
      labels:
        app: lx-hw-db
    spec:
      containers:
      - name: lx-hw-db
        image: lx-hw-db:latest
        ports:
        - containerPort: 8000
        env:
        - name: LX_HW_DB_MODE
          value: "enterprise"
        - name: LX_HW_DB_DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: lx-hw-db-secrets
              key: database-url
        volumeMounts:
        - name: config
          mountPath: /app/config
        - name: data
          mountPath: /app/data
      volumes:
      - name: config
        configMap:
          name: lx-hw-db-config
      - name: data
        persistentVolumeClaim:
          claimName: lx-hw-db-data
```

### Automated Hardware Discovery

Deploy detection agents across your infrastructure for continuous hardware inventory.

#### Agent Configuration

**Ansible Deployment**
```yaml
---
- name: Deploy hardware detection agents
  hosts: all
  become: yes
  vars:
    lx_hw_detect_version: "latest"
    collection_interval: "24h"
    privacy_level: "enhanced"
    
  tasks:
    - name: Install lx-hw-detect
      package:
        name: lx-hw-detect
        state: present
        
    - name: Configure detection agent
      template:
        src: agent-config.toml.j2
        dest: /etc/lx-hw-detect/config.toml
        mode: '0644'
        
    - name: Install systemd service
      template:
        src: lx-hw-agent.service.j2
        dest: /etc/systemd/system/lx-hw-agent.service
        
    - name: Enable and start service
      systemd:
        name: lx-hw-agent
        enabled: yes
        state: started
        daemon_reload: yes
```

**Agent Configuration Template (agent-config.toml.j2)**
```toml
[agent]
mode = "continuous"
interval = "{{ collection_interval }}"
server_url = "https://hardware-db.internal.company.com"

[privacy]
level = "{{ privacy_level }}"
anonymize_serial_numbers = true
rotate_salts = true
submission_consent = true

[detection]
tools = ["lshw", "dmidecode", "lspci", "lsusb"]
include_kernel_modules = true
check_compatibility = true

[reporting]
format = "json"
include_performance_data = false
compress_reports = true

[enterprise]
department = "{{ ansible_hostname | regex_replace('^([a-z]+)-.*', '\\1') }}"
location = "{{ datacenter_location | default('unknown') }}"
environment = "{{ environment_type | default('production') }}"
```

#### Continuous Monitoring

**System Service Configuration**
```ini
# /etc/systemd/system/lx-hw-agent.service
[Unit]
Description=Linux Hardware Detection Agent
After=network.target
Wants=network.target

[Service]
Type=simple
User=lx-hw-detect
Group=lx-hw-detect
ExecStart=/usr/bin/lx-hw-detect agent --config /etc/lx-hw-detect/config.toml
Restart=always
RestartSec=300
StandardOutput=journal
StandardError=journal

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
PrivateTmp=true
ProtectKernelTunables=true
ProtectControlGroups=true
RestrictSUIDSGID=true
ReadWritePaths=/var/lib/lx-hw-detect

[Install]
WantedBy=multi-user.target
```

---

## 🔧 Hardware-Specific Enterprise Guidance

### Server Hardware Validation

#### CPU and Memory Testing

**Enterprise CPU Compatibility**
```bash
# Comprehensive CPU detection and validation
lx-hw-detect detect --component cpu --enterprise-report

# Check for enterprise features
lx-hw-detect check --features "virtualization,security,performance"

# Validate memory configuration
lx-hw-detect validate --component memory --check-ecc
```

**Memory Validation Script**
```bash
#!/bin/bash
# validate-memory.sh - Enterprise memory validation

set -euo pipefail

REPORT_FILE="memory-validation-$(hostname)-$(date +%Y%m%d).json"

echo "🔍 Starting enterprise memory validation..."

# Generate detailed memory report
lx-hw-detect detect --component memory --format json > "$REPORT_FILE"

# Extract memory information
TOTAL_MEMORY=$(jq '.devices[] | select(.component_type == "Memory") | .specifications.total_size' "$REPORT_FILE")
ECC_STATUS=$(jq '.devices[] | select(.component_type == "Memory") | .specifications.ecc_capable' "$REPORT_FILE")
MEMORY_SPEED=$(jq '.devices[] | select(.component_type == "Memory") | .specifications.speed' "$REPORT_FILE")

echo "📊 Memory Configuration:"
echo "  Total Memory: $TOTAL_MEMORY"
echo "  ECC Capable: $ECC_STATUS"
echo "  Memory Speed: $MEMORY_SPEED"

# Validate against enterprise requirements
if [[ "$ECC_STATUS" == "false" ]]; then
    echo "⚠️  WARNING: ECC memory not detected - not recommended for production"
    exit 1
fi

if [[ $(echo "$TOTAL_MEMORY < 32" | bc) -eq 1 ]]; then
    echo "⚠️  WARNING: Less than 32GB memory - may not meet enterprise requirements"
fi

echo "✅ Memory validation completed successfully"
```

#### Storage Array Configuration

**Enterprise Storage Detection**
```bash
# Detect storage arrays and RAID configurations
lx-hw-detect detect --component storage --include-raid

# Check for hardware RAID compatibility
lx-hw-detect check --raid-controllers

# Validate storage performance characteristics
lx-hw-detect analyze --storage-performance
```

**RAID Configuration Validation**
```yaml
# Ansible playbook for RAID validation
---
- name: Validate RAID configuration
  hosts: storage_servers
  tasks:
    - name: Detect RAID controllers
      shell: lx-hw-detect detect --component raid --format json
      register: raid_detection
      
    - name: Parse RAID information
      set_fact:
        raid_controllers: "{{ (raid_detection.stdout | from_json).devices | selectattr('component_type', 'eq', 'RAID Controller') | list }}"
        
    - name: Validate RAID requirements
      fail:
        msg: "RAID controller not detected or not supported"
      when: raid_controllers | length == 0
      
    - name: Generate RAID configuration
      template:
        src: raid-config.j2
        dest: "/etc/raid/{{ inventory_hostname }}-config.conf"
      when: raid_controllers | length > 0
```

### Network Infrastructure Integration

#### Network Hardware Standardization

**Network Adapter Validation**
```bash
# Enterprise network adapter detection
lx-hw-detect detect --component network --enterprise-features

# Check for SR-IOV and virtualization support
lx-hw-detect check --network-features "sriov,dpdk,roce"

# Validate network driver compatibility across kernel versions
lx-hw-detect analyze --network-drivers --kernel-range 5.15-6.6
```

**Network Configuration Automation**
```yaml
# Ansible role for network hardware configuration
---
- name: Configure enterprise network adapters
  hosts: all
  vars:
    required_network_features: ['sriov', 'multi_queue', 'jumbo_frames']
    
  tasks:
    - name: Detect network hardware
      shell: lx-hw-detect detect --component network --format json
      register: network_hardware
      
    - name: Validate network features
      assert:
        that:
          - item.features | intersect(required_network_features) | length == required_network_features | length
        fail_msg: "Network adapter {{ item.name }} missing required features"
      loop: "{{ (network_hardware.stdout | from_json).devices }}"
      when: item.component_type == "Network Adapter"
      
    - name: Generate network configuration
      template:
        src: network-config.j2
        dest: "/etc/netplan/{{ item.interface }}.yaml"
      loop: "{{ (network_hardware.stdout | from_json).devices }}"
      when: item.component_type == "Network Adapter"
      notify: Apply netplan
      
  handlers:
    - name: Apply netplan
      command: netplan apply
```

### GPU Computing Clusters

#### GPU Cluster Validation

**NVIDIA GPU Cluster Setup**
```bash
# Detect GPU configurations across cluster
pdsh -w compute[01-64] "lx-hw-detect detect --component gpu --cluster-info"

# Validate GPU driver consistency
pdsh -w compute[01-64] "lx-hw-detect check --gpu-drivers --cluster-validation"

# Generate SLURM GPU configuration
lx-hw-detect generate-config --type slurm --component gpu --cluster compute[01-64]
```

**GPU Health Monitoring**
```python
#!/usr/bin/env python3
# gpu-cluster-monitor.py - GPU cluster monitoring

import json
import subprocess
import sys
from datetime import datetime

def check_gpu_health(nodes):
    """Check GPU health across cluster nodes"""
    results = {}
    
    for node in nodes:
        try:
            # Run hardware detection on remote node
            cmd = f"ssh {node} lx-hw-detect detect --component gpu --format json"
            result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
            
            if result.returncode == 0:
                gpu_data = json.loads(result.stdout)
                results[node] = {
                    'status': 'healthy',
                    'gpus': gpu_data['devices'],
                    'checked_at': datetime.now().isoformat()
                }
            else:
                results[node] = {
                    'status': 'error',
                    'error': result.stderr,
                    'checked_at': datetime.now().isoformat()
                }
                
        except Exception as e:
            results[node] = {
                'status': 'unreachable',
                'error': str(e),
                'checked_at': datetime.now().isoformat()
            }
    
    return results

def generate_cluster_report(results):
    """Generate cluster health report"""
    healthy_nodes = sum(1 for r in results.values() if r['status'] == 'healthy')
    total_nodes = len(results)
    
    print(f"🖥️  GPU Cluster Health Report")
    print(f"📊 Healthy Nodes: {healthy_nodes}/{total_nodes}")
    print(f"⏰ Generated: {datetime.now()}")
    print()
    
    for node, data in results.items():
        if data['status'] == 'healthy':
            gpu_count = len([d for d in data['gpus'] if d['component_type'] == 'GPU'])
            print(f"✅ {node}: {gpu_count} GPUs detected")
        else:
            print(f"❌ {node}: {data['status']} - {data.get('error', 'Unknown error')}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: gpu-cluster-monitor.py <node1> [node2] ...")
        sys.exit(1)
        
    nodes = sys.argv[1:]
    results = check_gpu_health(nodes)
    generate_cluster_report(results)
```

---

## 📊 Enterprise Analytics and Reporting

### Hardware Inventory Dashboard

Create comprehensive dashboards for hardware tracking and lifecycle management.

#### Grafana Dashboard Configuration

**Hardware Metrics Collection**
```yaml
# prometheus-hw-exporter.yml
apiVersion: v1
kind: ConfigMap
metadata:
  name: hardware-exporter-config
data:
  config.yml: |
    collectors:
      - name: lx_hw_detect
        command: ["/usr/bin/lx-hw-detect", "detect", "--format", "json"]
        interval: 300s
        timeout: 60s
        
    metrics:
      - name: hardware_compatibility_score
        help: "Hardware compatibility score (0-100)"
        type: gauge
        labels: [hostname, component_type, vendor, model]
        
      - name: hardware_device_count
        help: "Number of detected hardware devices"
        type: gauge
        labels: [hostname, component_type]
        
      - name: hardware_driver_status
        help: "Driver status (0=missing, 1=loaded, 2=optimal)"
        type: gauge
        labels: [hostname, device_id, driver_name]
```

**Dashboard Queries**
```promql
# Hardware compatibility overview
avg by (component_type) (hardware_compatibility_score)

# Devices with compatibility issues
hardware_compatibility_score < 70

# Driver status across fleet
sum by (driver_name) (hardware_driver_status == 0)

# Hardware age distribution
histogram_quantile(0.95, rate(hardware_device_age_seconds_bucket[5m]))
```

#### Automated Reporting

**Weekly Hardware Report Script**
```bash
#!/bin/bash
# weekly-hardware-report.sh - Generate enterprise hardware reports

set -euo pipefail

REPORT_DIR="/var/lib/lx-hw-db/reports"
WEEK=$(date +%Y-W%U)
REPORT_FILE="$REPORT_DIR/hardware-report-$WEEK.json"

mkdir -p "$REPORT_DIR"

echo "📊 Generating weekly hardware report for $WEEK..."

# Collect data from all managed systems
SYSTEMS=($(curl -s "http://cmdb.internal/api/systems?os=linux" | jq -r '.[].hostname'))

{
    echo "{"
    echo "  \"report_metadata\": {"
    echo "    \"generated_at\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\","
    echo "    \"week\": \"$WEEK\","
    echo "    \"total_systems\": ${#SYSTEMS[@]}"
    echo "  },"
    echo "  \"systems\": ["
    
    for i in "${!SYSTEMS[@]}"; do
        system="${SYSTEMS[$i]}"
        echo "    {"
        echo "      \"hostname\": \"$system\","
        
        # Fetch hardware data
        if ssh "$system" "lx-hw-detect detect --format json" 2>/dev/null; then
            echo "      \"status\": \"success\""
        else
            echo "      \"status\": \"error\","
            echo "      \"error\": \"Failed to collect hardware data\""
        fi
        
        if [[ $i -lt $((${#SYSTEMS[@]} - 1)) ]]; then
            echo "    },"
        else
            echo "    }"
        fi
    done
    
    echo "  ]"
    echo "}"
} > "$REPORT_FILE"

echo "✅ Report generated: $REPORT_FILE"

# Send report to stakeholders
mail -s "Weekly Hardware Report - $WEEK" \
     -a "$REPORT_FILE" \
     infrastructure-team@company.com \
     < /dev/null
```

### Lifecycle Management

#### Hardware End-of-Life Tracking

**EOL Detection Script**
```python
#!/usr/bin/env python3
# hardware-eol-tracker.py - Track hardware approaching end-of-life

import json
import requests
from datetime import datetime, timedelta
from collections import defaultdict

class HardwareEOLTracker:
    def __init__(self, hardware_db_url):
        self.db_url = hardware_db_url
        self.eol_database = self.load_eol_database()
        
    def load_eol_database(self):
        """Load hardware EOL information from various sources"""
        # In practice, this would load from multiple vendor APIs
        # and maintain a database of EOL dates
        return {
            # Format: vendor_id:device_id -> eol_date
            "8086:1533": "2025-12-31",  # Intel I210 Gigabit Network
            "10de:1380": "2024-06-30",  # NVIDIA GTX 750Ti
            # ... more entries
        }
    
    def check_system_eol_status(self, hardware_report):
        """Check EOL status for all hardware in a system"""
        eol_warnings = []
        critical_eol = []
        
        for device in hardware_report.get('devices', []):
            device_id = device.get('device_id')
            if device_id in self.eol_database:
                eol_date = datetime.strptime(self.eol_database[device_id], "%Y-%m-%d")
                days_until_eol = (eol_date - datetime.now()).days
                
                if days_until_eol < 0:
                    critical_eol.append({
                        'device': device,
                        'eol_date': eol_date,
                        'days_past_eol': abs(days_until_eol)
                    })
                elif days_until_eol < 365:
                    eol_warnings.append({
                        'device': device,
                        'eol_date': eol_date,
                        'days_until_eol': days_until_eol
                    })
        
        return {
            'warnings': eol_warnings,
            'critical': critical_eol
        }
    
    def generate_fleet_eol_report(self, systems):
        """Generate EOL report for entire fleet"""
        fleet_report = {
            'summary': {
                'total_systems': len(systems),
                'systems_with_eol_hardware': 0,
                'systems_with_critical_eol': 0
            },
            'details': []
        }
        
        for system in systems:
            # Fetch hardware report for system
            hardware_report = self.fetch_hardware_report(system)
            eol_status = self.check_system_eol_status(hardware_report)
            
            if eol_status['warnings'] or eol_status['critical']:
                fleet_report['summary']['systems_with_eol_hardware'] += 1
                
            if eol_status['critical']:
                fleet_report['summary']['systems_with_critical_eol'] += 1
                
            fleet_report['details'].append({
                'system': system,
                'eol_status': eol_status
            })
        
        return fleet_report
    
    def fetch_hardware_report(self, system):
        """Fetch hardware report from system"""
        # Implementation would fetch from your hardware database
        # This is a placeholder
        return {'devices': []}

# Usage example
if __name__ == "__main__":
    tracker = HardwareEOLTracker("https://hardware-db.internal.company.com")
    systems = ["server-01", "server-02", "workstation-03"]
    
    report = tracker.generate_fleet_eol_report(systems)
    
    print(json.dumps(report, indent=2, default=str))
```

---

## 🔐 Enterprise Privacy and Security

### Privacy Controls for Organizations

#### Organizational Privacy Policies

**Enterprise Privacy Configuration**
```toml
# /etc/lx-hw-detect/enterprise-policy.toml
[privacy]
# Organizational privacy level (overrides user settings)
enforce_minimum_level = "enhanced"
allow_user_override = false

# Data retention policies
max_data_age = "2y"
automatic_cleanup = true
cleanup_schedule = "monthly"

# Anonymization policies
require_anonymization = true
custom_salt_rotation = "weekly"
differential_privacy_epsilon = 0.5

[data_handling]
# Internal data sharing policies
allow_internal_sharing = true
require_department_approval = true
audit_data_access = true

# External data sharing
allow_public_contribution = true
require_legal_review = false  # Set to true for highly regulated environments
anonymization_review = true

[compliance]
# Compliance frameworks
gdpr_compliance = true
hipaa_compliance = false  # Set to true if handling healthcare systems
sox_compliance = true     # Set to true for financial systems

# Audit requirements
audit_log_retention = "7y"
require_audit_trails = true
encrypted_audit_logs = true
```

#### Data Governance Framework

**Data Classification System**
```yaml
# data-classification.yml
hardware_data_classification:
  public:
    - component_categories
    - vendor_information
    - general_specifications
    - compatibility_matrices
    
  internal:
    - system_configurations
    - performance_metrics
    - deployment_patterns
    - usage_statistics
    
  confidential:
    - detailed_system_topology
    - security_configurations
    - vendor_relationships
    - cost_information
    
  restricted:
    - serial_numbers (never collected)
    - personal_identifiers (never collected)
    - network_credentials (never collected)
    - encryption_keys (never collected)

data_handling_procedures:
  public:
    storage: "public_repositories"
    sharing: "unrestricted"
    retention: "indefinite"
    
  internal:
    storage: "internal_databases"
    sharing: "authorized_personnel"
    retention: "as_per_policy"
    
  confidential:
    storage: "encrypted_databases"
    sharing: "need_to_know"
    retention: "business_requirement"
    encryption: "required"
    
  restricted:
    collection: "prohibited"
    storage: "not_applicable"
    sharing: "not_applicable"
```

### Security Hardening

#### Secure Deployment Configuration

**Container Security**
```dockerfile
# Dockerfile.enterprise
FROM alpine:3.18 AS builder
RUN apk add --no-cache rust cargo
COPY . /src
WORKDIR /src
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/lx-hw-detect /lx-hw-detect
COPY --from=alpine:3.18 /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=alpine:3.18 /etc/passwd /etc/passwd

# Run as non-root user
USER 65534:65534

# Security metadata
LABEL security.scan.date="2025-08-27"
LABEL security.vulnerability.scan="passed"
LABEL security.compliance.level="enterprise"

ENTRYPOINT ["/lx-hw-detect"]
```

**Kubernetes Security Policy**
```yaml
apiVersion: policy/v1beta1
kind: PodSecurityPolicy
metadata:
  name: lx-hw-db-psp
spec:
  privileged: false
  allowPrivilegeEscalation: false
  requiredDropCapabilities:
    - ALL
  volumes:
    - 'configMap'
    - 'emptyDir'
    - 'projected'
    - 'secret'
    - 'downwardAPI'
    - 'persistentVolumeClaim'
  runAsUser:
    rule: 'MustRunAsNonRoot'
  runAsGroup:
    rule: 'MustRunAs'
    ranges:
      - min: 1000
        max: 65535
  seLinux:
    rule: 'RunAsAny'
  fsGroup:
    rule: 'RunAsAny'
  readOnlyRootFilesystem: true
```

---

## 🚀 Automation and Integration

### CI/CD Pipeline Integration

#### Hardware Validation in Deployment Pipelines

**GitLab CI Hardware Validation**
```yaml
# .gitlab-ci.yml
stages:
  - hardware_validation
  - deploy
  - post_deploy_validation

variables:
  HARDWARE_VALIDATION_IMAGE: "lx-hw-db:validation-latest"

hardware_compatibility_check:
  stage: hardware_validation
  image: $HARDWARE_VALIDATION_IMAGE
  script:
    - echo "🔍 Validating hardware compatibility for deployment targets"
    - |
      for target in $DEPLOYMENT_TARGETS; do
        echo "Checking $target..."
        lx-hw-detect remote-check --host $target --requirements deployment-requirements.yml
        if [ $? -ne 0 ]; then
          echo "❌ Hardware compatibility check failed for $target"
          exit 1
        fi
      done
    - echo "✅ All hardware compatibility checks passed"
  only:
    - master
    - staging

deploy_application:
  stage: deploy
  dependencies:
    - hardware_compatibility_check
  script:
    - echo "🚀 Deploying application to validated hardware"
    - ansible-playbook -i inventory deploy.yml
  only:
    - master
    - staging

validate_deployment:
  stage: post_deploy_validation
  script:
    - echo "🧪 Validating deployment on actual hardware"
    - |
      for target in $DEPLOYMENT_TARGETS; do
        lx-hw-detect remote-validate --host $target --deployment-id $CI_PIPELINE_ID
      done
  only:
    - master
    - staging
```

**Hardware Requirements Specification**
```yaml
# deployment-requirements.yml
deployment_requirements:
  minimum_specs:
    cpu:
      cores: 8
      architecture: "x86_64"
      features: ["sse4.2", "avx2"]
      
    memory:
      size_gb: 32
      ecc_capable: true
      
    storage:
      root_disk:
        size_gb: 500
        type: "ssd"
        interface: "nvme"
      data_disk:
        size_gb: 2000
        type: "ssd"
        raid_level: 1
        
    network:
      adapters: 2
      speed_gbps: 10
      features: ["sriov"]
      
  compatibility_requirements:
    kernel_version: ">=5.15"
    driver_status: "optimal"
    known_issues: "none"
    
  performance_requirements:
    compatibility_score: ">85"
    benchmark_results:
      disk_iops: ">50000"
      network_throughput: ">9.5Gbps"
      memory_bandwidth: ">50GB/s"
```

### Infrastructure Automation

#### Terraform Integration

**Hardware-Aware Infrastructure**
```hcl
# hardware-aware-infrastructure.tf

data "lx_hw_db_compatibility" "compute_nodes" {
  for_each = var.compute_node_types
  
  hardware_requirements = {
    cpu_cores     = each.value.cpu_cores
    memory_gb     = each.value.memory_gb
    gpu_required  = each.value.gpu_required
    network_speed = each.value.network_speed
  }
  
  compatibility_threshold = 85
  kernel_version         = var.target_kernel_version
}

resource "aws_instance" "compute_nodes" {
  for_each = data.lx_hw_db_compatibility.compute_nodes
  
  # Select instance type based on compatibility data
  instance_type = each.value.recommended_instance_types[0]
  ami          = data.aws_ami.linux[each.value.optimal_distribution].id
  
  # Configure based on hardware recommendations
  user_data = templatefile("user_data.sh", {
    hardware_config = each.value.configuration_recommendations
    driver_packages = each.value.required_packages
    kernel_params   = each.value.kernel_parameters
  })
  
  tags = {
    Name = "compute-${each.key}"
    HardwareCompatibilityScore = each.value.compatibility_score
    OptimizedFor = each.value.use_case
  }
}

# Custom Terraform provider for hardware database
terraform {
  required_providers {
    lx_hw_db = {
      source  = "lx-hw-db/lx-hw-db"
      version = "~> 1.0"
    }
  }
}
```

**Ansible Dynamic Inventory Integration**
```python
#!/usr/bin/env python3
# ansible-hardware-inventory.py - Dynamic inventory based on hardware compatibility

import json
import requests
import sys
import argparse

class HardwareInventory:
    def __init__(self, hardware_db_url):
        self.hardware_db_url = hardware_db_url
        
    def get_inventory(self):
        """Generate Ansible inventory based on hardware compatibility"""
        inventory = {
            '_meta': {
                'hostvars': {}
            }
        }
        
        # Fetch all systems from hardware database
        systems = self.fetch_systems()
        
        for system in systems:
            hostname = system['hostname']
            hardware_data = system['hardware_data']
            
            # Categorize systems by hardware compatibility
            compatibility_score = self.calculate_compatibility_score(hardware_data)
            
            if compatibility_score >= 90:
                group = 'high_compatibility'
            elif compatibility_score >= 75:
                group = 'medium_compatibility'
            else:
                group = 'low_compatibility'
                
            if group not in inventory:
                inventory[group] = {'hosts': []}
                
            inventory[group]['hosts'].append(hostname)
            
            # Add hardware-specific variables
            inventory['_meta']['hostvars'][hostname] = {
                'hardware_compatibility_score': compatibility_score,
                'cpu_vendor': hardware_data.get('cpu', {}).get('vendor'),
                'gpu_present': bool(hardware_data.get('gpus')),
                'memory_gb': hardware_data.get('memory', {}).get('total_gb'),
                'storage_type': hardware_data.get('storage', {}).get('primary_type'),
                'network_adapters': len(hardware_data.get('network', [])),
                'recommended_kernel_params': self.get_kernel_params(hardware_data),
                'required_packages': self.get_required_packages(hardware_data)
            }
            
        return inventory
    
    def fetch_systems(self):
        """Fetch system data from hardware database API"""
        response = requests.get(f"{self.hardware_db_url}/api/v1/systems")
        return response.json()['data']
    
    def calculate_compatibility_score(self, hardware_data):
        """Calculate overall hardware compatibility score"""
        scores = []
        for component in hardware_data.get('components', []):
            scores.append(component.get('compatibility_score', 0))
        return sum(scores) / len(scores) if scores else 0
    
    def get_kernel_params(self, hardware_data):
        """Get recommended kernel parameters for hardware"""
        params = []
        
        # Add GPU-specific parameters
        if hardware_data.get('gpus'):
            for gpu in hardware_data['gpus']:
                if gpu['vendor'] == 'nvidia':
                    params.extend(['nvidia_drm.modeset=1', 'nvidia.NVreg_PreserveVideoMemoryAllocations=1'])
                elif gpu['vendor'] == 'amd':
                    params.append('amdgpu.si_support=1')
        
        # Add CPU-specific parameters
        cpu = hardware_data.get('cpu', {})
        if cpu.get('vendor') == 'intel':
            params.append('intel_idle.max_cstate=1')
        elif cpu.get('vendor') == 'amd':
            params.append('processor.max_cstate=1')
            
        return params
    
    def get_required_packages(self, hardware_data):
        """Get required packages for optimal hardware support"""
        packages = ['lshw', 'dmidecode', 'pciutils', 'usbutils']
        
        # Add hardware-specific packages
        if hardware_data.get('gpus'):
            for gpu in hardware_data['gpus']:
                if gpu['vendor'] == 'nvidia':
                    packages.extend(['nvidia-driver-535', 'nvidia-cuda-toolkit'])
                elif gpu['vendor'] == 'amd':
                    packages.extend(['mesa-vulkan-drivers', 'rocm-utils'])
                    
        return packages

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('--list', action='store_true', help='List all hosts')
    parser.add_argument('--host', help='Get variables for specific host')
    args = parser.parse_args()
    
    inventory_manager = HardwareInventory("https://hardware-db.internal.company.com")
    
    if args.list:
        inventory = inventory_manager.get_inventory()
        print(json.dumps(inventory, indent=2))
    elif args.host:
        inventory = inventory_manager.get_inventory()
        host_vars = inventory['_meta']['hostvars'].get(args.host, {})
        print(json.dumps(host_vars, indent=2))
    else:
        parser.print_help()
```

---

## 📈 Monitoring and Maintenance

### Enterprise Monitoring Integration

#### Prometheus Integration

**Hardware Metrics Exporter**
```go
// hardware-prometheus-exporter.go
package main

import (
    "encoding/json"
    "log"
    "net/http"
    "os/exec"
    "time"

    "github.com/prometheus/client_golang/prometheus"
    "github.com/prometheus/client_golang/prometheus/promhttp"
)

var (
    hardwareCompatibilityScore = prometheus.NewGaugeVec(
        prometheus.GaugeOpts{
            Name: "hardware_compatibility_score",
            Help: "Hardware compatibility score from 0 to 100",
        },
        []string{"hostname", "component_type", "vendor", "model"},
    )

    hardwareDeviceCount = prometheus.NewGaugeVec(
        prometheus.GaugeOpts{
            Name: "hardware_device_count",
            Help: "Number of hardware devices detected",
        },
        []string{"hostname", "component_type"},
    )

    hardwareDriverStatus = prometheus.NewGaugeVec(
        prometheus.GaugeOpts{
            Name: "hardware_driver_status",
            Help: "Driver status: 0=missing, 1=loaded, 2=optimal",
        },
        []string{"hostname", "device_id", "driver_name"},
    )
)

type HardwareReport struct {
    Devices []Device `json:"devices"`
}

type Device struct {
    ComponentType       string  `json:"component_type"`
    Vendor             string  `json:"vendor"`
    Model              string  `json:"model"`
    DeviceID           string  `json:"device_id"`
    CompatibilityScore float64 `json:"compatibility_score"`
    DriverStatus       int     `json:"driver_status"`
    DriverName         string  `json:"driver_name"`
}

func init() {
    prometheus.MustRegister(hardwareCompatibilityScore)
    prometheus.MustRegister(hardwareDeviceCount)
    prometheus.MustRegister(hardwareDriverStatus)
}

func collectHardwareMetrics() {
    hostname, _ := os.Hostname()
    
    // Run hardware detection
    cmd := exec.Command("lx-hw-detect", "detect", "--format", "json")
    output, err := cmd.Output()
    if err != nil {
        log.Printf("Failed to collect hardware data: %v", err)
        return
    }

    var report HardwareReport
    if err := json.Unmarshal(output, &report); err != nil {
        log.Printf("Failed to parse hardware report: %v", err)
        return
    }

    // Update metrics
    deviceCounts := make(map[string]int)
    
    for _, device := range report.Devices {
        // Compatibility score
        hardwareCompatibilityScore.WithLabelValues(
            hostname,
            device.ComponentType,
            device.Vendor,
            device.Model,
        ).Set(device.CompatibilityScore)

        // Device count
        deviceCounts[device.ComponentType]++

        // Driver status
        hardwareDriverStatus.WithLabelValues(
            hostname,
            device.DeviceID,
            device.DriverName,
        ).Set(float64(device.DriverStatus))
    }

    // Update device counts
    for componentType, count := range deviceCounts {
        hardwareDeviceCount.WithLabelValues(hostname, componentType).Set(float64(count))
    }
}

func main() {
    // Collect metrics every 5 minutes
    go func() {
        for {
            collectHardwareMetrics()
            time.Sleep(5 * time.Minute)
        }
    }()

    // Serve metrics
    http.Handle("/metrics", promhttp.Handler())
    log.Println("Hardware metrics exporter listening on :8080")
    log.Fatal(http.ListenAndServe(":8080", nil))
}
```

#### Nagios/Icinga Integration

**Hardware Monitoring Plugin**
```bash
#!/bin/bash
# check_hardware_compatibility.sh - Nagios plugin for hardware compatibility

set -euo pipefail

SCRIPT_NAME=$(basename "$0")
VERSION="1.0.0"

# Default thresholds
WARNING_THRESHOLD=75
CRITICAL_THRESHOLD=50
TIMEOUT=30

# Plugin return codes
STATE_OK=0
STATE_WARNING=1
STATE_CRITICAL=2
STATE_UNKNOWN=3

usage() {
    cat << EOF
$SCRIPT_NAME v$VERSION

Usage: $SCRIPT_NAME [OPTIONS]

Options:
  -w, --warning THRESHOLD    Warning threshold for compatibility score (default: $WARNING_THRESHOLD)
  -c, --critical THRESHOLD   Critical threshold for compatibility score (default: $CRITICAL_THRESHOLD)
  -t, --timeout SECONDS      Timeout for hardware detection (default: $TIMEOUT)
  -h, --help                 Show this help message
  -V, --version             Show version information

Examples:
  $SCRIPT_NAME -w 80 -c 60
  $SCRIPT_NAME --warning 85 --critical 70 --timeout 45

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -w|--warning)
            WARNING_THRESHOLD="$2"
            shift 2
            ;;
        -c|--critical)
            CRITICAL_THRESHOLD="$2"
            shift 2
            ;;
        -t|--timeout)
            TIMEOUT="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit $STATE_OK
            ;;
        -V|--version)
            echo "$SCRIPT_NAME v$VERSION"
            exit $STATE_OK
            ;;
        *)
            echo "Unknown option: $1" >&2
            usage >&2
            exit $STATE_UNKNOWN
            ;;
    esac
done

# Validate thresholds
if [[ $WARNING_THRESHOLD -le $CRITICAL_THRESHOLD ]]; then
    echo "UNKNOWN - Warning threshold must be greater than critical threshold" >&2
    exit $STATE_UNKNOWN
fi

# Main monitoring function
check_hardware_compatibility() {
    local temp_file
    temp_file=$(mktemp)
    trap "rm -f $temp_file" EXIT

    # Run hardware detection with timeout
    if timeout "$TIMEOUT" lx-hw-detect detect --format json > "$temp_file" 2>/dev/null; then
        # Parse compatibility scores
        local scores
        scores=$(jq -r '.devices[].compatibility_score // 0' "$temp_file")
        
        if [[ -z "$scores" ]]; then
            echo "UNKNOWN - No hardware compatibility data available"
            exit $STATE_UNKNOWN
        fi

        # Calculate average compatibility score
        local total_score=0
        local device_count=0
        
        while IFS= read -r score; do
            if [[ -n "$score" && "$score" != "null" ]]; then
                total_score=$(echo "$total_score + $score" | bc -l)
                device_count=$((device_count + 1))
            fi
        done <<< "$scores"

        if [[ $device_count -eq 0 ]]; then
            echo "UNKNOWN - No devices with compatibility scores found"
            exit $STATE_UNKNOWN
        fi

        local avg_score
        avg_score=$(echo "scale=1; $total_score / $device_count" | bc -l)

        # Count problematic devices
        local critical_devices
        critical_devices=$(jq -r ".devices[] | select(.compatibility_score < $CRITICAL_THRESHOLD) | .component_type + \": \" + .vendor + \" \" + .model" "$temp_file" | wc -l)
        
        local warning_devices
        warning_devices=$(jq -r ".devices[] | select(.compatibility_score < $WARNING_THRESHOLD and .compatibility_score >= $CRITICAL_THRESHOLD) | .component_type + \": \" + .vendor + \" \" + .model" "$temp_file" | wc -l)

        # Performance data
        local perf_data="avg_score=${avg_score};${WARNING_THRESHOLD};${CRITICAL_THRESHOLD};0;100 critical_devices=${critical_devices};;;0;${device_count} warning_devices=${warning_devices};;;0;${device_count}"

        # Determine status and output
        if [[ $(echo "$avg_score < $CRITICAL_THRESHOLD" | bc -l) -eq 1 ]] || [[ $critical_devices -gt 0 ]]; then
            echo "CRITICAL - Average hardware compatibility: ${avg_score}%, ${critical_devices} critical devices | $perf_data"
            exit $STATE_CRITICAL
        elif [[ $(echo "$avg_score < $WARNING_THRESHOLD" | bc -l) -eq 1 ]] || [[ $warning_devices -gt 0 ]]; then
            echo "WARNING - Average hardware compatibility: ${avg_score}%, ${warning_devices} devices need attention | $perf_data"
            exit $STATE_WARNING
        else
            echo "OK - Average hardware compatibility: ${avg_score}%, all devices compatible | $perf_data"
            exit $STATE_OK
        fi
    else
        echo "UNKNOWN - Hardware detection failed or timed out after ${TIMEOUT} seconds"
        exit $STATE_UNKNOWN
    fi
}

# Check dependencies
if ! command -v lx-hw-detect &> /dev/null; then
    echo "UNKNOWN - lx-hw-detect command not found"
    exit $STATE_UNKNOWN
fi

if ! command -v jq &> /dev/null; then
    echo "UNKNOWN - jq command not found (required for JSON parsing)"
    exit $STATE_UNKNOWN
fi

if ! command -v bc &> /dev/null; then
    echo "UNKNOWN - bc command not found (required for calculations)"
    exit $STATE_UNKNOWN
fi

# Run the check
check_hardware_compatibility
```

---

## 🔧 Advanced Enterprise Features

### Custom Hardware Profiles

Create standardized hardware profiles for different organizational needs.

```yaml
# enterprise-hardware-profiles.yml
hardware_profiles:
  standard_workstation:
    description: "Standard office workstation profile"
    requirements:
      cpu:
        cores_min: 4
        threads_min: 8
        architecture: "x86_64"
        vendors: ["intel", "amd"]
        
      memory:
        size_gb_min: 16
        type: "ddr4"
        ecc_required: false
        
      storage:
        root_disk:
          size_gb_min: 256
          type: "ssd"
        
      graphics:
        integrated_acceptable: true
        discrete_preferred: false
        
      network:
        ethernet_required: true
        wifi_required: true
        speed_mbps_min: 1000

  development_workstation:
    description: "Developer workstation with high performance requirements"
    requirements:
      cpu:
        cores_min: 8
        threads_min: 16
        base_clock_ghz_min: 3.0
        vendors: ["intel", "amd"]
        
      memory:
        size_gb_min: 32
        type: "ddr4"
        speed_mhz_min: 3200
        
      storage:
        root_disk:
          size_gb_min: 512
          type: "nvme_ssd"
          speed_gbps_min: 3.0
        data_disk:
          size_gb_min: 1000
          type: "ssd"
          
      graphics:
        discrete_required: true
        vram_gb_min: 4
        
  server_compute:
    description: "Compute server for data processing and virtualization"
    requirements:
      cpu:
        cores_min: 16
        threads_min: 32
        virtualization_support: true
        vendors: ["intel", "amd"]
        
      memory:
        size_gb_min: 128
        ecc_required: true
        registered_dimms: true
        
      storage:
        boot_disk:
          size_gb_min: 120
          type: "ssd"
          raid_level: 1
        data_disks:
          count_min: 4
          size_gb_min: 2000
          type: "enterprise_ssd"
          raid_level: 10
          
      network:
        adapters_min: 2
        speed_gbps_min: 10
        redundancy_required: true
        
  gpu_compute_node:
    description: "GPU compute node for machine learning and rendering"
    requirements:
      cpu:
        cores_min: 16
        threads_min: 32
        pcie_lanes_min: 64
        
      memory:
        size_gb_min: 256
        ecc_required: true
        speed_mhz_min: 3200
        
      gpu:
        count_min: 2
        vram_gb_min: 24
        compute_capability_min: 8.6
        nvlink_support: true
        
      storage:
        nvme_count_min: 2
        total_capacity_tb_min: 8
        raid_support: true
        
      cooling:
        liquid_cooling_required: true
        thermal_design_power_w_max: 400
```

### Compliance and Audit Framework

**Compliance Monitoring Script**
```python
#!/usr/bin/env python3
# compliance-monitor.py - Monitor hardware compliance across enterprise

import json
import yaml
import logging
import smtplib
from datetime import datetime, timedelta
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart

class ComplianceMonitor:
    def __init__(self, config_file):
        with open(config_file, 'r') as f:
            self.config = yaml.safe_load(f)
        
        self.setup_logging()
        
    def setup_logging(self):
        """Setup logging configuration"""
        logging.basicConfig(
            level=logging.INFO,
            format='%(asctime)s - %(levelname)s - %(message)s',
            handlers=[
                logging.FileHandler('/var/log/hardware-compliance.log'),
                logging.StreamHandler()
            ]
        )
        self.logger = logging.getLogger(__name__)
        
    def check_gdpr_compliance(self, hardware_reports):
        """Check GDPR compliance for hardware data collection"""
        violations = []
        
        for report in hardware_reports:
            # Check for PII in hardware data
            if self.contains_pii(report):
                violations.append({
                    'system': report.get('system_id'),
                    'violation': 'Personal information detected in hardware report',
                    'severity': 'critical'
                })
            
            # Check anonymization
            if not self.is_properly_anonymized(report):
                violations.append({
                    'system': report.get('system_id'),
                    'violation': 'Hardware identifiers not properly anonymized',
                    'severity': 'high'
                })
                
            # Check retention policy
            report_age = self.get_report_age(report)
            if report_age > timedelta(days=self.config['retention_days']):
                violations.append({
                    'system': report.get('system_id'),
                    'violation': f'Report older than retention policy ({report_age.days} days)',
                    'severity': 'medium'
                })
        
        return violations
    
    def check_sox_compliance(self, financial_systems):
        """Check SOX compliance for financial systems"""
        violations = []
        
        for system in financial_systems:
            # Check for approved hardware only
            if not self.is_approved_hardware(system['hardware']):
                violations.append({
                    'system': system['system_id'],
                    'violation': 'Unapproved hardware detected on financial system',
                    'severity': 'critical'
                })
            
            # Check audit trail
            if not self.has_audit_trail(system):
                violations.append({
                    'system': system['system_id'],
                    'violation': 'Missing hardware change audit trail',
                    'severity': 'high'
                })
        
        return violations
    
    def generate_compliance_report(self):
        """Generate comprehensive compliance report"""
        report = {
            'generated_at': datetime.now().isoformat(),
            'compliance_period': {
                'start': (datetime.now() - timedelta(days=30)).isoformat(),
                'end': datetime.now().isoformat()
            },
            'violations': {
                'gdpr': [],
                'sox': [],
                'internal': []
            },
            'summary': {}
        }
        
        # Collect compliance data
        hardware_reports = self.fetch_hardware_reports()
        financial_systems = self.fetch_financial_systems()
        
        # Run compliance checks
        report['violations']['gdpr'] = self.check_gdpr_compliance(hardware_reports)
        report['violations']['sox'] = self.check_sox_compliance(financial_systems)
        
        # Generate summary
        total_violations = sum(len(v) for v in report['violations'].values())
        critical_violations = sum(
            len([x for x in v if x.get('severity') == 'critical'])
            for v in report['violations'].values()
        )
        
        report['summary'] = {
            'total_violations': total_violations,
            'critical_violations': critical_violations,
            'compliance_score': max(0, 100 - (total_violations * 5)),
            'status': 'compliant' if critical_violations == 0 else 'non-compliant'
        }
        
        return report
    
    def send_compliance_alerts(self, report):
        """Send compliance alerts to stakeholders"""
        if report['summary']['critical_violations'] > 0:
            self.send_email(
                self.config['compliance_team_email'],
                "CRITICAL: Hardware Compliance Violations Detected",
                self.format_compliance_email(report)
            )
            
    def format_compliance_email(self, report):
        """Format compliance report for email"""
        html = f"""
        <html>
        <body>
        <h2>Hardware Compliance Report</h2>
        <p><strong>Generated:</strong> {report['generated_at']}</p>
        <p><strong>Status:</strong> {report['summary']['status'].upper()}</p>
        
        <h3>Summary</h3>
        <ul>
        <li>Total Violations: {report['summary']['total_violations']}</li>
        <li>Critical Violations: {report['summary']['critical_violations']}</li>
        <li>Compliance Score: {report['summary']['compliance_score']}/100</li>
        </ul>
        
        <h3>Critical Violations</h3>
        """
        
        for compliance_type, violations in report['violations'].items():
            critical_violations = [v for v in violations if v.get('severity') == 'critical']
            if critical_violations:
                html += f"<h4>{compliance_type.upper()}</h4><ul>"
                for violation in critical_violations:
                    html += f"<li><strong>{violation['system']}:</strong> {violation['violation']}</li>"
                html += "</ul>"
        
        html += "</body></html>"
        return html

# Usage
if __name__ == "__main__":
    monitor = ComplianceMonitor('/etc/compliance-monitor.yml')
    report = monitor.generate_compliance_report()
    
    # Save report
    with open(f'/var/lib/compliance/report-{datetime.now().strftime("%Y%m%d")}.json', 'w') as f:
        json.dump(report, f, indent=2)
    
    # Send alerts if needed
    monitor.send_compliance_alerts(report)
    
    print(f"Compliance check completed. Status: {report['summary']['status']}")
```

---

## ✅ System Administrator Checklist

After implementing the guidance in this document, you should have:

### Infrastructure Setup
- [ ] Private hardware database instance deployed
- [ ] Detection agents installed across infrastructure
- [ ] Monitoring and alerting configured
- [ ] Backup and disaster recovery implemented

### Automation Integration
- [ ] CI/CD pipelines include hardware validation
- [ ] Infrastructure as Code templates created
- [ ] Ansible/Terraform integration configured
- [ ] Dynamic inventory based on hardware compatibility

### Compliance and Security
- [ ] Privacy policies implemented and enforced
- [ ] Compliance monitoring automated
- [ ] Security hardening applied
- [ ] Audit trails configured

### Operational Processes
- [ ] Hardware lifecycle management processes
- [ ] EOL tracking and replacement planning
- [ ] Performance monitoring and optimization
- [ ] Documentation and runbooks updated

---

**🎉 Congratulations!** You now have a comprehensive enterprise-grade hardware compatibility management system that provides visibility, automation, and compliance for your Linux infrastructure.

For additional support or advanced configurations, consult the [Enterprise Support Documentation](../support/enterprise.md) or reach out to the community for assistance.

*This guide is maintained by the lx-hw-db community with input from enterprise users. Last updated: 2025-08-27*