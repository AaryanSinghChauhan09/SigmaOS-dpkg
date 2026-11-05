// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-dpkg — Debian package manager core, inspired by dpkg.
//!
//! Models two dpkg fundamentals: `.deb` package filenames
//! (`name_version_architecture.deb`) and the status database format
//! (`Package:`/`Version:`/`Status:` stanzas). Fail-fast on malformed input.

use std::fmt;

/// Errors parsing package filenames or status entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DpkgError {
    /// Filename does not match `name_version_arch.deb`.
    BadFilename(String),
    /// Status stanza missing a required field.
    MissingField(&'static str),
    /// Status stanza has an unknown `Status:` value.
    BadStatus(String),
}

impl fmt::Display for DpkgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadFilename(n) => write!(f, "bad package filename: {n}"),
            Self::MissingField(field) => write!(f, "missing field: {field}"),
            Self::BadStatus(s) => write!(f, "unknown status: {s}"),
        }
    }
}

impl std::error::Error for DpkgError {}

/// A parsed `.deb` package filename.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebPackage {
    /// Source package name.
    pub name: String,
    /// Upstream version string.
    pub version: String,
    /// Target architecture (e.g. `amd64`, `all`).
    pub arch: String,
}

impl DebPackage {
    /// Parse `name_version_arch.deb`. Version may itself contain hyphens is
    /// not allowed upstream; we split on the last two `-` separators before
    /// `.deb`, matching dpkg's convention.
    pub fn parse_filename(filename: &str) -> Result<Self, DpkgError> {
        let stem = filename.strip_suffix(".deb").ok_or_else(|| {
            DpkgError::BadFilename(filename.to_string())
        })?;
        let mut parts = stem.rsplitn(3, '-');
        let arch = parts.next().ok_or_else(|| DpkgError::BadFilename(filename.into()))?;
        let version = parts.next().ok_or_else(|| DpkgError::BadFilename(filename.into()))?;
        let name = parts.next().ok_or_else(|| DpkgError::BadFilename(filename.into()))?;
        if name.is_empty() || version.is_empty() || arch.is_empty() {
            return Err(DpkgError::BadFilename(filename.to_string()));
        }
        Ok(Self {
            name: name.to_string(),
            version: version.to_string(),
            arch: arch.to_string(),
        })
    }
}

/// dpkg status values, per `/var/lib/dpkg/status`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageStatus {
    /// Installed and configured.
    Installed,
    /// Unpacked but not configured.
    Unpacked,
    /// Config-files remain after removal.
    ConfigFiles,
    /// Half-installed state.
    HalfInstalled,
    /// Half-configured state.
    HalfConfigured,
    /// Triggers awaited.
    TriggersAwaited,
    /// Triggers pending.
    TriggersPending,
}

impl PackageStatus {
    fn parse(s: &str) -> Result<Self, DpkgError> {
        match s {
            "install ok installed" => Ok(Self::Installed),
            "install ok unpacked" => Ok(Self::Unpacked),
            "deinstall ok config-files" => Ok(Self::ConfigFiles),
            "install ok half-installed" => Ok(Self::HalfInstalled),
            "install ok half-configured" => Ok(Self::HalfConfigured),
            "install ok triggers-awaited" => Ok(Self::TriggersAwaited),
            "install ok triggers-pending" => Ok(Self::TriggersPending),
            other => Err(DpkgError::BadStatus(other.to_string())),
        }
    }
}

/// One entry from the dpkg status database.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusEntry {
    /// Binary package name.
    pub package: String,
    /// Installed version.
    pub version: String,
    /// Lifecycle status.
    pub status: PackageStatus,
}

/// Parse a single status stanza (fields separated by `\n`, stanza ends at
/// blank line or EOF). Requires `Package:`, `Version:`, `Status:`.
pub fn parse_status_stanza(stanza: &str) -> Result<StatusEntry, DpkgError> {
    let mut package = None;
    let mut version = None;
    let mut status = None;
    for line in stanza.lines() {
        if let Some(v) = line.strip_prefix("Package: ") {
            package = Some(v.trim().to_string());
        } else if let Some(v) = line.strip_prefix("Version: ") {
            version = Some(v.trim().to_string());
        } else if let Some(v) = line.strip_prefix("Status: ") {
            status = Some(PackageStatus::parse(v.trim())?);
        }
    }
    Ok(StatusEntry {
        package: package.ok_or(DpkgError::MissingField("Package"))?,
        version: version.ok_or(DpkgError::MissingField("Version"))?,
        status: status.ok_or(DpkgError::MissingField("Status"))?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_deb_filename() {
        let p = DebPackage::parse_filename("bash_5.2.15-1_amd64.deb").unwrap();
        assert_eq!(p.name, "bash");
        assert_eq!(p.version, "5.2.15-1");
        assert_eq!(p.arch, "amd64");
    }

    #[test]
    fn reject_non_deb() {
        assert!(matches!(
            DebPackage::parse_filename("foo.tar.gz"),
            Err(DpkgError::BadFilename(_))
        ));
    }

    #[test]
    fn parse_status_stanza() {
        let stanza = "Package: bash\nVersion: 5.2.15-1\nStatus: install ok installed\n";
        let e = parse_status_stanza(stanza).unwrap();
        assert_eq!(e.package, "bash");
        assert_eq!(e.status, PackageStatus::Installed);
    }

    #[test]
    fn missing_field_fails() {
        assert_eq!(
            parse_status_stanza("Package: x\nVersion: 1\n"),
            Err(DpkgError::MissingField("Status"))
        );
    }

    #[test]
    fn unknown_status_fails() {
        let stanza = "Package: x\nVersion: 1\nStatus: bogus state\n";
        assert!(matches!(
            parse_status_stanza(stanza),
            Err(DpkgError::BadStatus(_))
        ));
    }
}
