# Multi-stage Docker build for Linux Hardware Database
# Supports both CLI and web interface deployments

# Build stage
FROM rust:1.75-slim as builder

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    libgtk-4-dev \
    libadwaita-1-dev \
    libglib2.0-dev \
    libcairo-gobject2 \
    libpango1.0-dev \
    libgdk-pixbuf-2.0-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Copy manifests and build dependencies first (for better caching)
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./

# Create src directory with a dummy main to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --bins
RUN rm -rf src

# Copy source code
COPY src/ src/
COPY config/ config/
COPY docs/ docs/
COPY completions/ completions/

# Build the actual applications
RUN cargo build --release --bin lx-hw-detect
RUN cargo build --release --bin lx-hw-indexer

# CLI-only runtime stage
FROM debian:bookworm-slim as cli

# Install runtime dependencies for hardware detection
RUN apt-get update && apt-get install -y \
    lshw \
    dmidecode \
    pciutils \
    usbutils \
    util-linux \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd --gid 1000 lxhw \
    && useradd --uid 1000 --gid 1000 --create-home --shell /bin/bash lxhw

# Copy binaries and configuration
COPY --from=builder /build/target/release/lx-hw-detect /usr/local/bin/
COPY --from=builder /build/target/release/lx-hw-indexer /usr/local/bin/
COPY --from=builder /build/config/default.toml /etc/lx-hw-db/

# Copy completions and documentation
COPY --from=builder /build/completions/ /usr/share/lx-hw-db/completions/
COPY --from=builder /build/docs/ /usr/share/doc/lx-hw-db/

# Set permissions
RUN chmod +x /usr/local/bin/lx-hw-detect /usr/local/bin/lx-hw-indexer

USER lxhw
WORKDIR /home/lxhw

ENTRYPOINT ["/usr/local/bin/lx-hw-detect"]
CMD ["--help"]

# Web interface stage
FROM cli as web

USER root

# Install Python for web server
RUN apt-get update && apt-get install -y \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Copy web interface files
COPY web/ /var/www/lx-hw-db/
RUN chown -R lxhw:lxhw /var/www/lx-hw-db

USER lxhw
WORKDIR /var/www/lx-hw-db

EXPOSE 8000

CMD ["python3", "serve.py", "--host", "0.0.0.0", "--port", "8000"]

# Development stage with all tools
FROM builder as dev

# Install additional development tools
RUN apt-get update && apt-get install -y \
    git \
    vim \
    curl \
    lshw \
    dmidecode \
    pciutils \
    usbutils \
    util-linux \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Install Rust development tools
RUN rustup component add rustfmt clippy rust-analyzer

# Create non-root user
RUN groupadd --gid 1000 dev \
    && useradd --uid 1000 --gid 1000 --create-home --shell /bin/bash dev

USER dev
WORKDIR /workspace

# Copy the built binaries for testing
COPY --from=builder --chown=dev:dev /build/target/release/ target/release/

CMD ["/bin/bash"]