# Install

How to get the `odm` binary. Product overview: root `README.md`. Version history: root `CHANGELOG.md`.

Visitor-facing install guide (same steps): project site [`install.html`](https://hembrow-innovations.github.io/odm/install.html) (source: `website/install.html`).

## Requirements

- **Runtime**: `git` on `PATH`
- **Actions** (`odm run`): Unix shell
- **Build from source**: Rust 1.70+

## GitHub Releases (primary)

1. Open the latest release: https://github.com/hembrow-innovations/odm/releases
2. Download `odm-<version>-<target>.tar.gz` for your host triple (e.g. `aarch64-apple-darwin`, `x86_64-unknown-linux-musl`).
3. Extract and place `odm` on your `PATH`:

```bash
tar xzf odm-<version>-<target>.tar.gz
# archive contains: odm, README-release.txt
sudo mv odm /usr/local/bin/   # or another directory on PATH
odm --version
```

## Build from source

```bash
git clone https://github.com/hembrow-innovations/odm.git
cd odm
cargo build -p odm --release
# → target/release/odm

cargo install --path crates/odm
# → ~/.cargo/bin/odm (typically)
```

### Release tarball locally

```bash
./scripts/release-build.sh
# → dist/odm-<version>-<host-triple>.tar.gz
```

Optional:

- **`ODM_TARGET=<triple>`** — cross/build for a specific triple (`cargo build --target`)
- **`ODM_RELEASE_PUBLISH=1`** — after build, run `gh release create` (requires authenticated `gh`)

## Verify

```bash
odm --version
odm init --help
```

## Non-goals (this release)

- Homebrew formula
- crates.io publish of the binary crate
- Signed/notarized macOS binaries
- Windows as a primary supported channel
