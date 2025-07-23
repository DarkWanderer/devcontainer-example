FROM docker.io/library/fedora:42

# Install basic required packages
RUN dnf -y --setopt=tsflags=nodocs install rustc cargo npm git && \
    dnf clean all

# Create a non-root user with sudo privileges
RUN useradd --create-home --shell /bin/bash vscode && \
    echo "vscode ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/vscode

# Install Claude CLI
RUN mkdir -p /home/vscode/.npm-global && \
    chown vscode:vscode /home/vscode/.npm-global && \
    sudo -u vscode npm config set prefix /home/vscode/.npm-global && \
    sudo -u vscode npm install -g @anthropic-ai/claude-code

USER vscode
ENV PATH=/home/vscode/.npm-global/bin:$PATH
