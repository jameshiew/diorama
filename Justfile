run scene *args:
    BEVY_ASSET_ROOT=examples/{{scene}} cargo run \
        {{args}} \
        --example {{scene}}

xvfb-run := if os() == 'linux' {
  'xvfb-run'
} else {
  ''
}

screenshot-and-exit scene:
    BEVY_ASSET_ROOT=examples/{{scene}} {{xvfb-run}} cargo run \
        --features bevy/bevy_ci_testing,bevy/png \
        --example {{scene}}

fmt:
    cargo +nightly fmt \
        --all

fmt-check:
    cargo +nightly fmt \
        --all \
        -- --check

check:
    cargo check \
        --all-targets

clippy:
    cargo clippy \
        --all-targets \
        -- -D warnings

doc *args:
    RUSTDOCFLAGS="-Dwarnings" cargo doc \
        --no-deps \
        {{args}}

build *args:
    cargo build \
        --all-targets \
        {{args}}

test:
    cargo nextest run

install-cargo-tools-essential:
    cargo install --locked cargo-binstall
    cargo binstall --no-confirm cargo-hack
    cargo binstall --no-confirm cargo-nextest

install-cargo-tools: install-cargo-tools-essential
    cargo binstall --no-confirm cargo-machete
    cargo binstall --no-confirm cargo-audit
    cargo binstall --no-confirm wasm-server-runner
    cargo install --git https://github.com/TheBevyFlock/bevy_cli --branch main --locked bevy_cli

install-debian-deps:
    sudo apt update && sudo apt-get install -y --no-install-recommends \
        g++ \
        pkg-config \
        libx11-dev \
        libasound2-dev \
        libudev-dev \
        libxkbcommon-x11-0 \
        libwayland-dev \
        libxkbcommon-dev \
        xorg \
        libxkbcommon-dev \
        libxkbcommon-x11-0 \
        xvfb \
        libgl1-mesa-dri \
        libxcb-xfixes0-dev \
        mesa-vulkan-drivers
