use rumbas::support::rc::RC;
use rumbas::RUMBAS_VERSION;
use rumbas_support::path::RumbasPath;

pub fn check_rc(p: &RumbasPath, can_execute_in_old_version: bool) -> bool {
    // Check rc file
    let rc_res = RC::from_path(p);
    match rc_res {
        Ok(rc) => {
            let rc_version = rc.version();
            if rc_version < *RUMBAS_VERSION && !can_execute_in_old_version {
                log::error!("This repository uses an older rumbas version than the one that is compiling it ({} vs {}).", rc_version, *RUMBAS_VERSION);
                log::error!("Please execute `rumbas update-repo`.");
                false
            } else if rc_version > *RUMBAS_VERSION {
                log::error!("This repository uses a newer rumbas version than the one you are using ({} vs {}).", rc_version, *RUMBAS_VERSION);
                log::error!("Please update your rumbas version.");
                false
            } else {
                true
            }
        }
        Err(e) => {
            log::error!("Could not parse rc file: {}", e);
            false
        }
    }
}
