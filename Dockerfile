# Use the official Rust image as a base
FROM rust:latest

# Install Bevy's dependencies for Linux (Debian-based)
# This includes libraries for windowing (X11), audio (ALSA), and input devices.
RUN apt-get update && apt-get install -y \
    libx11-dev \
    libxrandr-dev \
    libxi-dev \
    libxcursor-dev \
    libxinerama-dev \
    libasound2-dev \
    libudev-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js and npm to run the Claude CLI
# Using NodeSource repository for a recent version
RUN apt-get update && apt-get install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_lts.x | bash - && \
    apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

# Install the Claude CLI globally
RUN npm install -g @anthropic-ai/claude-code

# ユーザーとグループ作成
RUN useradd -m -u 1000 -s /bin/bash claude_user

# 作業ディレクトリ作成と権限付与
RUN mkdir -p /workspace && \
    chown -R claude_user:claude_user /workspace && \
    chmod -R 755 /workspace

USER claude_user

# Switch to the new user
USER claude_user

# Set the working directory to match docker-compose.yml
WORKDIR /workspace

