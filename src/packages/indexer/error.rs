use std::path::PathBuf;

pub enum Error {
    /// the specified root directory for discovering packages does not exist
    IndexMissingDir { path: PathBuf },
    /// std::fs returned an error
    FsError { path: PathBuf, reason: String },
    /// multiple errors occured when creating a new PackageIndex
    NewIndex { errors: Vec<Self> },
    /// parsing error for the manifest file
    ManifestParse { path: PathBuf, reason: String },
    /// a package exists but includes no versions (a package must have at least one version)
    NoVersions { name: String, package_root: PathBuf },
    /// a package manifest has different name than its directory name
    NameMismatch {
        expected: String,
        got: String,
        package_root: PathBuf,
    },
    /// a package manifest has different version that its directory name
    VersionMismatch {
        expected: String,
        got: String,
        version_root: PathBuf,
    },
    /// version string cannot be parsed
    BadVersionString { got: String, reason: String },
    /// the same package is defined in multiple locations
    MultipleDefinitions { paths: Vec<PathBuf> },
}
