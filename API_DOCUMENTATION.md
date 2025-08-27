# Linux Hardware Compatibility Database API

This document describes the RESTful API endpoints available for accessing hardware compatibility data.

## Base URL

All API endpoints are relative to the base URL: `/api/v1/`

## Response Format

All API responses follow this standardized format:

```json
{
  "version": "1.0",
  "generated": "2025-08-27T11:37:16.772655756+00:00",
  "data": {
    // Endpoint-specific data
  }
}
```

## Authentication

No authentication is required. All endpoints provide read-only access to public hardware compatibility data.

## Rate Limiting

No rate limiting is enforced as all data is served as static JSON files.

## Endpoints

### API Information

#### GET /api/v1/index.json

Returns API metadata and available endpoints.

**Response:**
```json
{
  "name": "Linux Hardware Compatibility Database API",
  "version": "1.0",
  "generated": "2025-08-27T11:37:16.774006784+00:00",
  "endpoints": {
    "search": { ... },
    "stats": { ... },
    "recommendations": { ... }
  },
  "rate_limits": "None (static files)",
  "documentation": "/web/api-docs/"
}
```

### Search Endpoints

#### GET /api/v1/search/vendors.json

Returns all hardware vendors with their compatibility scores and report counts.

**Response:**
```json
{
  "data": [
    {
      "vendor": "1022",
      "total_reports": 24,
      "compatibility_score": 35.0,
      "component_types": ["PCI Device"]
    }
  ]
}
```

#### GET /api/v1/search/components.json

Returns all component types with popularity and compatibility data.

**Response:**
```json
{
  "data": [
    {
      "component_type": "PCI Device",
      "total_reports": 31,
      "top_vendors": [
        {"vendor": "1022", "count": 24},
        {"vendor": "1002", "count": 4}
      ],
      "popular_models": [...],
      "compatibility_distribution": {
        "Poor": 31
      }
    }
  ]
}
```

#### GET /api/v1/search/kernels.json

Returns kernel versions with compatibility statistics.

**Response:**
```json
{
  "data": [
    {
      "kernel_version": "6.16.0",
      "total_reports": 1,
      "compatibility_stats": {
        "poor": 1
      },
      "problematic_hardware": [
        "1002 Device 1002:1478",
        "1022 Device 1022:1480"
      ]
    }
  ]
}
```

#### GET /api/v1/search/distributions.json

Returns Linux distributions with hardware compatibility data.

**Response:**
```json
{
  "data": [
    {
      "distribution": "NixOS 25.11 (Xantusia)",
      "total_reports": 1,
      "vendor_compatibility": {
        "1022": 34.99,
        "1002": 32.81
      },
      "common_kernels": ["6.16.0"]
    }
  ]
}
```

### Statistics Endpoints

#### GET /api/v1/stats/overview.json

Returns overall database statistics.

**Response:**
```json
{
  "data": {
    "total_reports": 1,
    "unique_systems": 1,
    "total_vendors": 5,
    "component_types": 1,
    "kernel_versions": 1,
    "distributions": 1,
    "compatibility_overview": {
      "Poor": 1
    },
    "last_updated": "2025-08-27T11:37:16.770956742Z"
  }
}
```

#### GET /api/v1/stats/top-hardware.json

Returns the most reported hardware components.

**Response:**
```json
{
  "data": {
    "top_hardware": [
      {
        "vendor": "1022",
        "model": "Device 1022:57ad",
        "report_count": 1,
        "avg_compatibility": 17.5
      }
    ]
  }
}
```

#### GET /api/v1/stats/trends.json

Returns growth and trend data.

**Response:**
```json
{
  "data": {
    "growth_stats": [
      {
        "date": "2025-08-27T11:37:16.770979254Z",
        "total_reports": 1,
        "new_reports": 1
      }
    ],
    "vendor_growth": [...],
    "kernel_adoption": [...]
  }
}
```

### Recommendations Endpoints

#### GET /api/v1/recommendations/by-component/{component_type}.json

Returns hardware recommendations for specific component types.

Available component types:
- `pci-device.json`

#### GET /api/v1/recommendations/by-use-case/{use_case}.json

Returns hardware recommendations for specific use cases.

Available use cases:
- `gaming.json`
- `development.json`
- `server.json`
- `general.json`

#### GET /api/v1/recommendations/by-vendor/{vendor}.json

Returns recommendations for specific vendors (when available).

## Data Sources

### Raw Hardware Reports

Access raw hardware reports at: `/hardware-reports/`

### Search Indices

Access raw search indices at: `/indices/`

Available indices:
- `by-vendor.json` - Hardware organized by vendor
- `by-component.json` - Hardware organized by component type
- `by-kernel.json` - Hardware organized by kernel version
- `by-distribution.json` - Hardware organized by Linux distribution
- `search-terms.json` - Search terms index
- `compatibility-matrix.json` - Hardware/kernel compatibility matrix

## CORS Support

All endpoints support CORS and can be accessed from web applications.

## Caching

API responses are static files and can be cached aggressively. Recommended cache time: 1 hour.

## Updates

API data is automatically regenerated when new hardware reports are submitted through GitHub pull requests.

## Support

For API support or to report issues, please visit: https://github.com/lx-hw-db/lx-hw-db/issues