#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub static mut USE_AIR_SIDE_SPECIAL: [bool; 8] = [true; 8];
pub static mut USE_UP_SPECIAL: [bool; 8] = [true; 8];
// Be careful when you hook this if you are hooking it for more than one mod. You should only hook a function once
#[skyline::hook(replace = StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_hook(boma: &mut smash::app::BattleObjectModuleAccessor, status_kind: i32, unk: bool) -> u64{
    if smash::app::utility::get_kind(boma) == *FIGHTER_KIND_GANON && status_kind == FIGHTER_STATUS_KIND_SPECIAL_S {
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
      if !USE_AIR_SIDE_SPECIAL[entry_id as usize] {
        return 0;
      }
  }
  return original!()(boma, status_kind, unk);
}

mod ganondorf;

#[skyline::main(name = "SFG")]
pub fn main() {
  unsafe {
    skyline::install_hook!(change_status_hook);
    ganondorf::install();
  }
}