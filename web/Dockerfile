# Multi-stage build for Linux Hardware Database Web Interface
FROM node:18-alpine as builder

WORKDIR /build

# Copy package files
COPY package.json ./
COPY css/ css/
COPY js/ js/

# Install build dependencies and build
RUN npm install --only=dev
RUN npm run build

# Production stage
FROM python:3.11-alpine

# Install system dependencies
RUN apk add --no-cache \
    curl \
    && rm -rf /var/cache/apk/*

# Create non-root user
RUN addgroup -g 1000 lxhw \
    && adduser -u 1000 -G lxhw -s /bin/sh -D lxhw

# Set working directory
WORKDIR /var/www/lx-hw-db

# Copy web interface files
COPY --chown=lxhw:lxhw . .

# Copy built assets from builder stage
COPY --from=builder --chown=lxhw:lxhw /build/dist/ ./dist/

# Create required directories
RUN mkdir -p data indices statistics hardware-reports \
    && chown -R lxhw:lxhw .

# Switch to non-root user
USER lxhw

# Expose port
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/ || exit 1

# Run the web server
CMD ["python3", "serve.py", "--host", "0.0.0.0", "--port", "8000"]