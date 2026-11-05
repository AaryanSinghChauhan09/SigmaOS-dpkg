use std::fmt;

/// Represents the installation status of a package, mirroring dpkg states.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageStatus {
    /// Package has never been installed.
    NotInstalled,
    /// Installation began but was interrupted.
    HalfInstalled,
    /// Package files unpacked but not configured.
    Unpacked,
    /// Configuration started but was interrupted.
    HalfConfigured,
    /// Package is fully installed and configured.
    Installed,
    /// Only configuration files remain after removal.
    ConfigFiles,
}

impl fmt::Display for PackageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PackageStatus::NotInstalled => "not-installed",
            PackageStatus::HalfInstalled => "half-installed",
            PackageStatus::Unpacked => "unpacked",
            PackageStatus::HalfConfigured => "half-configured",
            PackageStatus::Installed => "installed",
            PackageStatus::ConfigFiles => "config-files",
        };
        f.write_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // AAA: Arrange Installed status, Act Display, Assert string.
    #[test]
    fn test_display_installed() {
        // Arrange
        let status = PackageStatus::Installed;
        // Act
        let s = status.to_string();
        // Assert
        assert_eq!(s, "installed");
    }

    // AAA: Arrange NotInstalled, Act Display, Assert string.
    #[test]
    fn test_display_not_installed() {
        let status = PackageStatus::NotInstalled;
        assert_eq!(status.to_string(), "not-installed");
    }

    // AAA: Arrange two equal statuses, Act PartialEq, Assert equal.
    #[test]
    fn test_equality() {
        assert_eq!(PackageStatus::HalfInstalled, PackageStatus::HalfInstalled);
        assert_ne!(PackageStatus::Unpacked, PackageStatus::Installed);
    }
}
