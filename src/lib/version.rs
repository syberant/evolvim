const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

#[derive(Serialize, Deserialize)]
pub struct Version {
    major: String,
    minor: String,
    patch: String,
}

impl Version {
    pub fn current_version() -> Self {
        Version {
            major: String::from(VERSION_MAJOR),
            minor: String::from(VERSION_MINOR),
            patch: String::from(VERSION_PATCH),
        }
    }

    pub fn is_compatible_with_current(&self) -> bool {
        if self.major != VERSION_MAJOR {
            // False if there is a difference in the major version
            false
        } else if self.minor != VERSION_MINOR {
            // This is currently an unstable crate so breaking changes will bump the minor version.
            //
            // If this crate ever becomes stable this will need to change.
            false
        } else {
            true
        }
    }
}

use std::fmt;
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
