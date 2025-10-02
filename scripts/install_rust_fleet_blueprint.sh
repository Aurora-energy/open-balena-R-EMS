#!/usr/bin/env bash
set -euo pipefail

LOG_DIR="$(dirname "$0")/logs"
mkdir -p "$LOG_DIR"
LOG_FILE="$LOG_DIR/install_rust_fleet_blueprint.log"
touch "$LOG_FILE"

# Tee stdout/stderr to the log file for auditing while still presenting output
# to the terminal.
exec > >(tee -a "$LOG_FILE") 2>&1

echo "[INFO] $(date --iso-8601=seconds) Starting installation" 

# Utility to run commands with sudo only when available. This keeps the script
# safe on systems where sudo is not installed (e.g., root shells in containers).
run_cmd() {
    if command -v sudo >/dev/null 2>&1; then
        sudo "$@"
    else
        "$@"
    fi
}

ensure_rust() {
    if command -v rustc >/dev/null 2>&1; then
        echo "[INFO] Rust already installed: $(rustc --version)"
        return
    fi

    echo "[INFO] Installing Rust via rustup"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # shellcheck disable=SC1090
    source "$HOME/.cargo/env"
    echo "[INFO] Rust installed: $(rustc --version)"
}

ensure_docker() {
    if command -v docker >/dev/null 2>&1; then
        echo "[INFO] Docker already installed: $(docker --version)"
        return
    fi

    echo "[INFO] Installing Docker Engine via apt"
    if command -v apt-get >/dev/null 2>&1; then
        run_cmd apt-get update
        run_cmd apt-get install -y ca-certificates curl gnupg lsb-release
        if [ ! -f /etc/apt/keyrings/docker.gpg ]; then
            run_cmd install -m 0755 -d /etc/apt/keyrings
            curl -fsSL https://download.docker.com/linux/$(. /etc/os-release && echo "$ID")/gpg | run_cmd gpg --dearmor -o /etc/apt/keyrings/docker.gpg
            run_cmd chmod a+r /etc/apt/keyrings/docker.gpg
        fi
        if [ ! -f /etc/apt/sources.list.d/docker.list ]; then
            echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/$(. /etc/os-release && echo "$ID") $(lsb_release -cs) stable" | run_cmd tee /etc/apt/sources.list.d/docker.list >/dev/null
        fi
        run_cmd apt-get update
        run_cmd apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
    else
        echo "[WARN] apt-get not available; please install Docker manually"
    fi

    if command -v docker >/dev/null 2>&1; then
        echo "[INFO] Docker installed: $(docker --version)"
    else
        echo "[ERROR] Docker installation skipped or failed"
    fi
}

ensure_surreal() {
    if command -v surreal >/dev/null 2>&1; then
        echo "[INFO] SurrealDB CLI already installed: $(surreal version)"
        return
    fi

    echo "[INFO] Installing SurrealDB CLI"
    TMP_DIR="$(mktemp -d)"
    pushd "$TMP_DIR" >/dev/null
    curl -sSfL https://download.surrealdb.com/install | bash
    popd >/dev/null

    if command -v surreal >/dev/null 2>&1; then
        echo "[INFO] SurrealDB CLI installed: $(surreal version)"
    else
        echo "[ERROR] SurrealDB CLI installation failed"
    fi
}

ensure_rust
ensure_docker
ensure_surreal

echo "[INFO] $(date --iso-8601=seconds) Installation routine completed"

echo "[INFO] Validating toolchain versions"
rustc --version || true
cargo --version || true
docker --version || true
surreal version || true

echo "[INFO] $(date --iso-8601=seconds) Validation finished"
