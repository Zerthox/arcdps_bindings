use std::ffi::CStr;

use crate::*;

/// A helper function to convert raw arguments to safe abstractions
#[inline(always)]
pub fn get_combat_args_from_raw<'a>(
    raw_ev: Option<&'a CombatEvent>,
    raw_src: Option<&'a RawAgent>,
    raw_dst: Option<&'a RawAgent>,
    raw_skill_name: PCCHAR,
) -> CombatEventArgs<'a> {
    CombatEventArgs {
        ev: raw_ev,
        src: raw_src.map(Into::into),
        dst: raw_dst.map(Into::into),
        skill_name: unsafe { get_str_from_pc_char(raw_skill_name) },
    }
}

/// A helper function to convert arcdps strings to [`&str`].
/// ### Remarks
/// The result is not necessarily static.
/// delta confirmed that skill names are available for the whole lifetime of the
/// plugin, but agent names are only available for the duration of the fight.
/// Reduce the lifetime in the ongoing process as needed!
#[inline(always)]
pub unsafe fn get_str_from_pc_char(src: PCCHAR) -> Option<&'static str> {
    if src.is_null() {
        None
    } else {
        Some(
            CStr::from_ptr(src as *const std::os::raw::c_char)
                .to_str()
                .unwrap_or_default(),
        )
    }
}

/// A helper function to convert raw arguments to safe abstractions
#[inline(always)]
pub fn convert_extras_user(user: &RawUserInfo) -> UserInfo {
    let name = unsafe { get_str_from_pc_char(user.account_name as _) };
    UserInfo {
        account_name: name.map(|n| n.trim_start_matches(':')),
        join_time: user.join_time,
        role: user.role,
        subgroup: user.subgroup,
        ready_status: user.ready_status,
    }
}

pub struct CombatEventArgs<'a> {
    pub ev: Option<&'a CombatEvent>,
    pub src: Option<Agent<'a>>,
    pub dst: Option<Agent<'a>>,
    pub skill_name: Option<&'static str>,
}
