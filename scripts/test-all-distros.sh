#!/usr/bin/env bash
# Cross-distro test runner for Velocity Console
# Runs the Rust test suite on Ubuntu, Fedora, and Arch Linux
#
# Usage: ./scripts/test-all-distros.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "=============================================="
echo "  Velocity Console Cross-Distro Test Suite"
echo "=============================================="
echo ""

# Track results
FAILED=()
PASSED=()

# Test function
run_distro_test() {
    local distro=$1
    local dockerfile=$2
    local image_name="velocity-test-$distro"
    
    echo "----------------------------------------"
    echo "Testing on: $distro"
    echo "----------------------------------------"
    
    # Build the image
    echo "[1/2] Building Docker image..."
    if ! docker build -f "$dockerfile" -t "$image_name" . > /dev/null 2>&1; then
        echo "❌ Failed to build $distro image"
        FAILED+=("$distro (build)")
        return 1
    fi
    
    # Run tests
    echo "[2/2] Running test suite..."
    if docker run --rm -v "$(pwd):/workspace" "$image_name"; then
        echo "✅ $distro: All tests passed"
        PASSED+=("$distro")
    else
        echo "❌ $distro: Tests failed"
        FAILED+=("$distro (tests)")
        return 1
    fi
    
    echo ""
}

# Run tests on each distro
run_distro_test "ubuntu" "docker/Dockerfile.ubuntu" || true
run_distro_test "fedora" "docker/Dockerfile.fedora" || true
run_distro_test "arch" "docker/Dockerfile.arch" || true

# Summary
echo "=============================================="
echo "  Summary"
echo "=============================================="
echo ""

if [ ${#PASSED[@]} -gt 0 ]; then
    echo "✅ Passed: ${PASSED[*]}"
fi

if [ ${#FAILED[@]} -gt 0 ]; then
    echo "❌ Failed: ${FAILED[*]}"
    exit 1
fi

echo ""
echo "All distros passed! 🎉"
