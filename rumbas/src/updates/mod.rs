use crate::support::rc::RC;

mod zero_four_zero;

pub fn update(current_rc: RC) -> Option<RC> {
    let new_version = match &current_rc.version()[..] {
        "0.4.0" => Some(zero_four_zero::update()),
        _ => None,
    };

    new_version.map(|n| current_rc.with_version(n))
}
