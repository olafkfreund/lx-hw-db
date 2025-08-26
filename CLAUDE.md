# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**lx-hw-db** is a Linux hardware probe system index - a community-driven hardware compatibility database that leverages GitHub infrastructure for markdown-based storage with privacy-preserving data collection and actionable configuration recommendations.

This project is currently in the design phase with comprehensive planning completed in `project_plan.md`. No implementation code exists yet.

## Architecture Overview

The system follows a federated architecture with these key components:

### Core Components (Planned)
- **Hardware Detection Tool**: Multi-tool approach using lshw, dmidecode, lspci, lsusb, and inxi
- **Privacy Layer**: SHA-512 hashing with rotating salts, differential privacy, k-anonymity
- **GitHub-based Storage**: Hierarchical markdown files with YAML frontmatter
- **Automated Processing**: GitHub Actions for validation, indexing, and deployment
- **Search & Discovery**: Static site generation with FlexSearch integration
- **Configuration Engine**: Personalized recommendations and driver suggestions

### Technology Stack (From Design Document)
- **Hardware Probe Tool**: Rust (single binary, maximum performance)
- **Data Processing**: Go (concurrent processing, simple deployment)  
- **Web API**: Python with FastAPI
- **Frontend**: TypeScript with React
- **Storage**: Git-based with optional PostgreSQL for advanced queries
- **Deployment**: GitHub Actions + GitHub Pages + CDN

### Privacy Architecture
- Hardware IDs undergo HMAC-SHA256 with time-rotating salts
- Differential privacy with Laplace noise (epsilon=1.0)
- K-anonymity principles ensuring configurations appear ≥k times
- Three privacy levels: Basic, Enhanced, Strict

## Repository Structure (Planned)

```
hardware/
├── cpu/
│   ├── intel/
│   └── amd/
├── gpu/
│   ├── nvidia/
│   ├── amd/
│   └── intel/
├── motherboard/
├── network/
└── peripherals/
```

Each hardware entry uses standardized markdown with YAML frontmatter containing structured metadata for compatibility status, specifications, and tested configurations.

## Development Workflow (When Implementation Begins)

### Phase 1: Core Infrastructure
1. Hardware detection CLI tool (Rust)
2. Basic privacy anonymization
3. Markdown schema validation
4. GitHub Actions automation

### Phase 2: Community Features  
1. Submission workflows
2. Validation and quality control
3. Search interface generation
4. Web frontend

### Phase 3: Advanced Features
1. Federated synchronization
2. ML-based recommendations  
3. Advanced search capabilities
4. Community governance tools

## Data Format Standards

Hardware entries follow this structure:
```yaml
---
name: "Hardware Name"
category: "cpu|gpu|motherboard|network|peripherals"
manufacturer: "Vendor"
compatibility_status: "full|partial|limited|none"
last_tested: "YYYY-MM-DD"
specs:
  # Hardware-specific specifications
compatibility:
  linux:
    kernel_version: "5.15+"
    status: "full|partial|limited|none"
    notes: "Detailed compatibility information"
tested_configurations:
  - motherboard: "Board Model"
    status: "working|issues|broken"
---
```

## Privacy and Security Considerations

- Never collect serial numbers, MAC addresses, or UUIDs in raw form
- All personal identifiers must be cryptographically hashed
- Implement differential privacy for statistical data
- Follow GDPR compliance standards
- Users maintain control over privacy levels

## Contributing Guidelines (Future)

- Hardware submissions through guided templates
- Automated validation for schema compliance
- Peer review requiring community approval
- Expert validation for component categories
- Graduated trust levels for contributors

## Project Status

**Current Phase**: Design and Planning Complete
**Next Steps**: Begin Phase 1 implementation with hardware detection tool
**License**: AGPLv3

## Agent OS Documentation

### Product Context
- **Mission & Vision:** @.agent-os/product/mission.md
- **Technical Architecture:** @.agent-os/product/tech-stack.md
- **Development Roadmap:** @.agent-os/product/roadmap.md
- **Decision History:** @.agent-os/product/decisions.md

### Development Standards
- **Code Style:** @~/.agent-os/standards/code-style.md
- **Best Practices:** @~/.agent-os/standards/best-practices.md

### Project Management
- **Active Specs:** @.agent-os/specs/
- **Spec Planning:** Use `@~/.agent-os/instructions/create-spec.md`
- **Tasks Execution:** Use `@~/.agent-os/instructions/execute-tasks.md`

## Workflow Instructions

When asked to work on this codebase:

1. **First**, check @.agent-os/product/roadmap.md for current priorities
2. **Then**, follow the appropriate instruction file:
   - For new features: @.agent-os/instructions/create-spec.md
   - For tasks execution: @.agent-os/instructions/execute-tasks.md
3. **Always**, adhere to the standards in the files listed above

## Core Design Principles

### Privacy-First Architecture
- Data minimization at collection stage
- Cryptographic anonymization with no raw identifier storage
- No user tracking across submissions
- Transparent privacy with user control

### Git-as-Database Philosophy
- Distributed storage with no single point of failure
- Version-controlled data with audit trails
- Community governance through pull requests
- Zero infrastructure cost leveraging GitHub

### Community-Driven Validation
- Peer review for all submissions
- Reputation-based trust systems
- Automated pre-validation with human oversight
- Democratic conflict resolution

### Hardware Abstraction Layers
- Capability-based hardware grouping
- Hierarchical compatibility chains
- Fuzzy matching for similar hardware
- Configuration templates from abstract profiles

### Progressive Enhancement
- Core offline functionality
- Enhanced web-based features
- Advanced ML capabilities
- Graceful degradation when features unavailable

### Federated Trust Network
- Multiple authoritative sources
- Trust hierarchies with credibility weighting
- Automatic validated synchronization
- Local autonomy with public contribution

### Configuration-as-Code
- Declarative hardware configurations
- Version-controlled settings
- Testable community-validated recommendations
- Distribution-agnostic abstractions

### Semantic Hardware Modeling
- Standardized hardware taxonomy
- Component relationship mapping
- Capability inference from specifications
- Pattern-based compatibility prediction

## Important Notes

- Product-specific files in `.agent-os/product/` override any global standards
- User's specific instructions override (or amend) instructions found in `.agent-os/specs/...`
- Always adhere to established patterns, code style, and best practices documented above
- This project emphasizes privacy-first design from the ground up
- Community governance through transparent GitHub workflows
- Federated architecture enables distributed collaboration
- Git-based storage provides natural version control and redundancy
- Focus on actionable recommendations rather than just data collection