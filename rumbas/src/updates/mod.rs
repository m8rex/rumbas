use crate::support::rc::RC;
use semver::{Version, VersionReq};

mod zero_five;
mod zero_four_zero;

pub fn update(current_rc: RC) -> Option<RC> {
    let current_version = current_rc.version();
    let new_version = if current_version == Version::new(0, 4, 0) {
        Some(zero_four_zero::update())
    } else if VersionReq::parse("0.5.*")
        .expect("this to be a valid version requirements")
        .matches(&current_version)
    {
        Some(zero_five::update())
    } else if current_version == Version::new(0, 6, 0) || current_version == Version::new(0, 6, 1) {
        Some(Version::new(0, 6, 2))
    } else {
        None
    };

    new_version.map(|n| current_rc.with_version(n))
}
