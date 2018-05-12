set -euxo pipefail

main() {
    cargo check --target $TARGET
    cargo check --target $TARGET --features rt
}

main
