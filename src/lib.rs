//! SigmaOS Chromium Launcher & Hardening Profile Subsystem.
//!
//! Principles: Secure Defaults, Process Isolation, Defensive Programming,
//! Command-Query Separation, Single Responsibility, Arrange-Act-Assert.

pub mod flags;
pub mod profile;
pub mod launcher;

pub use flags::{SecurityFlags, FeatureFlags};
pub use profile::{BrowserProfile, ProfileKind};
pub use launcher::{ChromiumLauncher, LaunchConfig};
