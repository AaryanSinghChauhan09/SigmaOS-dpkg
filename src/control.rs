use std::collections::HashMap;
#[derive(Debug, Clone, Default)]
pub struct ControlFile {
    pub package: String,
    pub version: String,
    pub architecture: String,
    pub depends: Vec<String>,
    pub description: String,
    pub maintainer: String,
}
impl ControlFile {
    pub fn new(package: &str, version: &str) -> Self {
        Self { package: package.to_string(), version: version.to_string(), architecture: "amd64".to_string(), ..Default::default() }
    }
    pub fn add_depends(&mut self, dep: &str) { self.depends.push(dep.to_string()); }
    pub fn is_valid(&self) -> bool { !self.package.is_empty() && !self.version.is_empty() }
}
#[derive(Debug, Clone, PartialEq)]
pub enum PackageStatus { NotInstalled, Installed, Broken }
pub struct DpkgDatabase { packages: HashMap<String, (ControlFile, PackageStatus)> }
impl DpkgDatabase {
    pub fn new() -> Self { Self { packages: HashMap::new() } }
    pub fn register(&mut self, ctrl: ControlFile, status: PackageStatus) {
        self.packages.insert(ctrl.package.clone(), (ctrl, status));
    }
    pub fn status(&self, pkg: &str) -> Option<&PackageStatus> {
        self.packages.get(pkg).map(|(_, s)| s)
    }
    pub fn installed_count(&self) -> usize {
        self.packages.values().filter(|(_, s)| s == &PackageStatus::Installed).count()
    }
    pub fn count(&self) -> usize { self.packages.len() }
}
impl Default for DpkgDatabase { fn default() -> Self { Self::new() } }
