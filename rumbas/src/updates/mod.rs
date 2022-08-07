use crate::support::rc::RC;
use semver::{Version, VersionReq};

mod zero_four_zero;
mod zero_five;

pub fn update(current_rc: RC) -> Option<RC> {
    let current_version = current_rc.version();
    let new_version = if current_version ==  Version::new(0,4,0)  {
         Some(zero_four_zero::update())
        } else if VersionReq::parse("0.5.*").expect("this to be a valid version requirements").matches(&current_version) {
            Some(zero_five::update())
        }
        else {
            None
        };

    new_version.map(|n| current_rc.with_version(n))
}
