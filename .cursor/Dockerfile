FROM ethpandaops/circleci-python-rust:3.12.4

ENV DEBIAN_FRONTEND=noninteractive TZ=UTC \
    PIP_DISABLE_PIP_VERSION_CHECK=1 PIP_NO_CACHE_DIR=1 \
    PYTHONUNBUFFERED=1 PYTHONDONTWRITEBYTECODE=1 \
    CARGO_NET_RETRY=10 CARGO_NET_TIMEOUT=30 RUST_BACKTRACE=1

# --- System / Bevy runtime deps only ---
RUN apt-get update -qq && apt-get install -yqq --no-install-recommends \
    libudev-dev libasound2-dev libx11-dev libxi-dev libgl1-mesa-dev \
    libglu1-mesa-dev libxcursor-dev libxrandr-dev libxinerama-dev \
    libwayland-dev libxkbcommon-dev libvulkan-dev mesa-vulkan-drivers \
    ca-certificates curl git jq unzip procps htop xvfb x11-utils scrot \
    postgresql-client redis-tools ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# --- Rust toolchain housekeeping ---
# The base already has rust/cargo. Just ensure components.
RUN rustup component add rustfmt clippy rust-analyzer

# --- cargo-binstall pinned (faster installs) ---
ARG CARGO_BINSTALL_VER=1.10.9
RUN curl -fsSL -o /tmp/binstall.tgz \
      "https://github.com/cargo-bins/cargo-binstall/releases/download/v${CARGO_BINSTALL_VER}/cargo-binstall-x86_64-unknown-linux-gnu.tgz" \
 && tar -C /usr/local/bin -xzf /tmp/binstall.tgz cargo-binstall && rm /tmp/binstall.tgz

# --- Rust CLIs (binstall when possible, fall back to build) ---
RUN cargo binstall --no-confirm --no-symlinks cargo-watch cargo-edit cargo-audit cargo-outdated \
 || true \
 && (cargo binstall --no-confirm --no-symlinks sqlx-cli \
     || cargo install sqlx-cli --no-default-features --features rustls,postgres)
