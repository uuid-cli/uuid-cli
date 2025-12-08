# uuid-cli

Small CLI to generate UUID values.

## How to use

```sh
# If failure to install the latest version
# brew uninstall uuid-cli
# brew untap guzhongren/uuid-cli

brew tap guzhongren/uuid-cli
brew install uuid-cli

# Test
uuid-cli
# 019afb4e-d48f-7573-acac-0b01d48a24db
```

## Quick Start

- Build from source:

```sh
cargo build --release
```

- Run tests:

```sh
cargo test
```

## Usage

```sh
# generate one UUID
cargo run --release -- --count 1

# generate 5, uppercase, no hyphens
cargo run --release -- -n 5 --upper --no-hyphen

# run installed binary
# uuid-cli --count 10
```

## Cross-compilation / CI

The repository uses GitHub Actions to build for multiple targets. CI packages artifacts using the `PROJECT_NAME` and git tag (or short sha), producing names like `uuid-cli-<tag|sha>-<target>.tar.gz` which are uploaded and (on tagged releases) published to GitHub Releases.

If you need a local cross-build setup, consider using `cross` or setting up the appropriate toolchains for your target.

## Contributing

- Build and run tests locally with `cargo build --release` and `cargo test`.
- Open issues or PRs for bugs and improvements. Follow the repository's contribution guidelines when available.

## Releases

- Update the version in `Cargo.toml` under cli
- publish via GitHub Actions with bellow command:
  ```sh
  git tag v1.0.0
  git push origin --tags
  ```
- Update the `homebrew-uuid-cli/Formula/uuid-cli.rb` of [homebrew-uuid-cli](https://github.com/guzhongren/homebrew-uuid-cli/blob/main/Formula/uuid-cli.rb)
All the `url` and `sha256` should be retrived from the [release page](https://github.com/guzhongren/uuid-cli/releases).


<details>
<summary>Release info</summary>
Prebuilt release artifacts are published on GitHub Releases. Artifact naming follows this pattern:

- `uuid-cli-<tag>-<target>.tar.gz` (example: `uuid-cli-v0.1.7-aarch64-apple-darwin.tar.gz`)
- Debian packages (when built for a tag) are named: `uuid-cli_<version>_<arch>.deb` (example: `uuid-cli_0.1.7_arm64.deb`).

Each tarball contains the `uuid-cli` binary, `README.md`, `LICENSE`, and optionally completions and a manpage when available.

</details>


## License

See the `LICENSE` file in the repository.

---

If you'd like, I can add a short example of installing a release tarball or a quick troubleshooting note for CI builds.
