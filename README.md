# SigmaOS Chromium

A Rust library for assembling Chromium command lines with a named user data
directory, selected privacy and feature flags, and an optional app window URL.
It is a command builder: it does not spawn Chromium, create profile
directories, or validate that the selected flags are supported by the
installed browser.

## Library surface

- `BrowserProfile` maps a profile name under a caller supplied base directory.
  Incognito and isolated ephemeral profiles add Chromium's `--incognito` flag.
- `SecurityFlags::hardened()` supplies the library's default flag set.
- `FeatureFlags` formats caller selected enabled and disabled Chromium
  features.
- `ChromiumLauncher::build_command_line()` returns argv elements, keeping the
  binary, profile, flags, and target URL as separate strings.

```rust
use sigmaos_chromium::{BrowserProfile, ChromiumLauncher, LaunchConfig, ProfileKind};

let profile = BrowserProfile::new("work", ProfileKind::Standard, "/home/user/.config/sigmaos/chromium");
let mut launcher = ChromiumLauncher::new(LaunchConfig::default(), profile);
launcher.set_target_url("https://example.org");
let argv = launcher.build_command_line();
```

## Security scope

The defaults are a configurable collection of Chromium switches, not a security
boundary or hardened browser distribution. Review switches against the exact
Chromium build in use; feature support and security effects can change between
versions. The caller remains responsible for process execution, filesystem
permissions, profile lifecycle, and policy enforcement.

## Development

This crate has no third party dependencies. Run `cargo check` to compile the
library and `cargo test` to run its unit tests.
