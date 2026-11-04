use sigmaos_dpkg::{Config, ControlFile, DpkgComponent, DpkgDatabase, PackageStatus, Component};

#[test]
fn test_config_default_aaa() {
    let c = Config::default();
    assert!(c.enabled);
    assert!(!c.db_path.is_empty());
}

#[test]
fn test_control_file_valid_aaa() {
    let ctrl = ControlFile::new("vim", "9.0");
    assert!(ctrl.is_valid());
    assert_eq!(ctrl.package, "vim");
    assert_eq!(ctrl.architecture, "amd64");
}

#[test]
fn test_control_file_invalid_empty_aaa() {
    let ctrl = ControlFile::new("", "");
    assert!(!ctrl.is_valid());
}

#[test]
fn test_dpkg_database_register_aaa() {
    let mut db = DpkgDatabase::new();
    db.register(ControlFile::new("vim", "9.0"), PackageStatus::Installed);
    db.register(ControlFile::new("curl", "8.0"), PackageStatus::Installed);
    assert_eq!(db.installed_count(), 2);
    assert_eq!(db.count(), 2);
}

#[test]
fn test_dpkg_database_status_aaa() {
    let mut db = DpkgDatabase::new();
    db.register(ControlFile::new("broken-pkg", "1.0"), PackageStatus::Broken);
    assert_eq!(db.status("broken-pkg"), Some(&PackageStatus::Broken));
    assert_eq!(db.installed_count(), 0);
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = DpkgComponent::new(Config::default());
    assert!(c.initialize().is_ok());
    assert!(c.health_check().is_ok());
    assert!(c.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let c = DpkgComponent::new(Config::default());
    assert_eq!(c.name(), "sigmaos-dpkg");
}
