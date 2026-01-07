//! the indexer creates an index of all packages
//! the file structure is as follows
//!
//! so far we are only concered with component package,
//! each component package contains multiple components
//!
//! in manifest.toml, provides specifies what components are included in package
//! in the same directory, it must contain the componentname.[dll/dylib/so]
//! which is the dynamic library file supported by the operating system
//!
//! repo-root/
//! ├── package1/
//! │   ├── 0.1.0/
//! │   │   └── package.toml
//! │   └── 0.1.1/
//! │       └── package.toml
//! └── package2/
//!     ├── 0.1.0/
//!     │   └── package.toml
//!     └── 0.1.1/
//!         └── package.toml
//! repo2-root/
//! └── package3/
//!     └── 0.1.0/
//!         └── package.toml
//!
//! package.toml specs is found in component/package_manifest

pub mod component;
mod deps_resolvable;
mod deps_resolver;
mod error;
pub use error::Error;
