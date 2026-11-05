#[derive(Debug, Clone)]
pub struct OfficePackage {
    pub name: String,
    pub version: String,
    pub license: String,
    pub sha256: Option<String>,
}

impl OfficePackage {
    pub fn new(name: impl Into<String>, version: impl Into<String>, license: impl Into<String>) -> Self {
        Self { name: name.into(), version: version.into(), license: license.into(), sha256: None }
    }
    pub fn with_sha256(mut self, hash: impl Into<String>) -> Self { self.sha256 = Some(hash.into()); self }
    pub fn is_verified(&self) -> bool { self.sha256.is_some() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verification_check() {
        let p = OfficePackage::new("libreoffice", "24.8", "MPL-2.0").with_sha256("abc123hash");
        assert!(p.is_verified());
    }
}
