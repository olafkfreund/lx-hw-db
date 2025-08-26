# Technical Stack

> Last Updated: 2025-08-26
> Version: 1.0.0

## Core Technologies

### Application Framework
- **Hardware Detection Tool:** Rust
- **Version:** Latest stable
- **Language:** Rust 1.70+

### Data Processing
- **Framework:** Go
- **Version:** 1.21+
- **Purpose:** Concurrent processing and simple deployment

### Database
- **Primary:** Git-based storage with markdown files
- **Secondary:** PostgreSQL (optional for advanced queries)
- **Version:** PostgreSQL 15+
- **Storage Format:** YAML frontmatter + Markdown content

## Backend Stack

### Web API Framework
- **Framework:** Python with FastAPI
- **Version:** Python 3.11+, FastAPI 0.100+
- **Purpose:** Rapid development and rich ecosystem integration

### Privacy and Security
- **Hashing:** HMAC-SHA256 with time-rotating salts
- **Anonymization:** SHA-512 for hardware identifiers
- **Privacy:** Differential privacy with Laplace noise (epsilon=1.0)
- **Compliance:** GDPR-compliant data handling

## Frontend Stack

### JavaScript Framework
- **Framework:** TypeScript with React
- **Version:** TypeScript 5.0+, React 18+
- **Build Tool:** Vite

### Import Strategy
- **Strategy:** Node.js modules
- **Package Manager:** npm
- **Node Version:** 22 LTS

### CSS Framework
- **Framework:** TailwindCSS
- **Version:** 4.0+
- **PostCSS:** Yes

### UI Components
- **Search Interface:** FlexSearch for client-side full-text search
- **Static Generation:** Hugo or Jekyll for basic browsing
- **Advanced Search:** Algolia integration for cloud-based search

## Assets & Media

### Fonts
- **Provider:** Google Fonts
- **Loading Strategy:** Self-hosted for performance

### Icons
- **Library:** Lucide
- **Implementation:** React components

## Infrastructure

### Application Hosting
- **Primary:** GitHub Pages (static content)
- **API Hosting:** Digital Ocean App Platform
- **Region:** Multi-region deployment for global access

### Database Hosting
- **Git Storage:** GitHub repositories (primary)
- **Optional Database:** Digital Ocean Managed PostgreSQL
- **Backups:** Git history provides natural backup and version control

### Asset Storage
- **CDN:** CloudFront for global content delivery
- **Caching:** Redis for frequently accessed data
- **Static Assets:** GitHub Pages with CDN integration

## Deployment

### CI/CD Pipeline
- **Platform:** GitHub Actions
- **Triggers:** 
  - Push to main branch (production)
  - Pull requests (validation and preview)
  - Scheduled runs (federation sync)

### Automation Workflows
- **Validation:** Markdown syntax and schema compliance
- **Processing:** Hardware data anonymization and indexing
- **Quality Control:** Duplicate detection and fuzzy matching
- **Deployment:** Static site generation and CDN invalidation

### Environments
- **Production:** main branch → GitHub Pages
- **Staging:** develop branch → preview deployments
- **Review Apps:** PR-based validation environments

## Development Tools

### Hardware Detection Stack
- **lshw:** Comprehensive hardware information in JSON format
- **dmidecode:** BIOS and motherboard details
- **lspci -k:** PCI devices with kernel driver mapping
- **lsusb:** USB peripherals detection
- **inxi:** User-friendly system summaries

### Privacy Tools
- **Cryptographic Libraries:** Ring (Rust), cryptography (Python)
- **Noise Generation:** Statistical libraries for differential privacy
- **Anonymization:** Custom k-anonymity implementation

### Search and Indexing
- **Static Site Generators:** Hugo (primary), Jekyll (fallback)
- **Search Engines:** FlexSearch (client-side), Algolia (advanced)
- **Index Generation:** mddb for markdown database processing

## Federated Architecture

### Synchronization
- **Git Protocol:** Native distributed architecture
- **Frequency:** Hourly pulls from trusted sources
- **Conflict Resolution:** Timestamp-based with manual review
- **Trust Weighting:** Authoritative sources take priority

### Federation Configuration
- **Format:** YAML manifests
- **Components:** Node relationships, sync policies, validation requirements
- **Trust Levels:** Authoritative, community, experimental

## Scalability and Performance

### Performance Optimizations
- **Caching Strategy:** Multi-level caching (CDN, Redis, browser)
- **Content Delivery:** Global CDN for static assets
- **Database Optimization:** Sharding by hardware category
- **API Performance:** FastAPI with async/await patterns

### Monitoring and Analytics
- **Error Tracking:** Sentry integration
- **Performance Monitoring:** GitHub Actions metrics
- **Usage Analytics:** Privacy-respecting analytics via Plausible
- **Health Checks:** Automated endpoint monitoring

## Security Considerations

### Data Protection
- **Encryption:** TLS 1.3 for all communications
- **API Security:** Rate limiting and authentication
- **Input Validation:** Comprehensive schema validation
- **Access Control:** GitHub permissions and API tokens

### Privacy Implementation
- **Data Minimization:** Collect only necessary hardware information
- **Retention Policies:** Configurable data retention periods
- **User Rights:** GDPR-compliant data export and deletion
- **Audit Logging:** Comprehensive activity tracking

## Code Repository Structure

### Primary Repository
- **URL:** GitHub repository for main codebase
- **License:** AGPLv3
- **Branching:** GitFlow with main/develop/feature branches

### Hardware Data Repository
- **Structure:** Separate repository for hardware database
- **Organization:** Category → Manufacturer → Device hierarchy
- **Format:** Markdown files with YAML frontmatter
- **Validation:** Automated schema compliance checking