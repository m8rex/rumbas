use crate::support::rc::RC;
use semver::{Version, VersionReq};

mod zero_five;
mod zero_seven_one;

pub fn update(current_rc: RC) -> Option<RC> {
    let current_version = current_rc.version();
    let new_version = if current_version == Version::new(0, 4, 0) {
        log::error!(
            "This rumbas repo is to old to update with this version. Please use rumbas 0.6.3"
        );
        None
    } else if VersionReq::parse("0.5.*")
        .expect("this to be a valid version requirements")
        .matches(&current_version)
    {
        Some(zero_five::update())
    } else if current_version == Version::new(0, 6, 0)
        || current_version == Version::new(0, 6, 1)
        || current_version == Version::new(0, 6, 2)
    {
        Some(Version::new(0, 6, 3))
    } else if current_version == Version::new(0, 6, 3) {
        log::warn!("You will need to manually update you rumbas repo to 0.7.0");
        log::warn!("Please move all templates files to the regular folders and update all paths.");
        Some(Version::new(0, 7, 0))
    } else if current_version == Version::new(0, 7, 0) {
        Some(Version::new(0, 7, 1))
    } else if current_version == Version::new(0, 7, 1) {
        Some(zero_seven_one::update())
    } else {
        None
    };

    new_version.map(|n| current_rc.with_version(n))
}
