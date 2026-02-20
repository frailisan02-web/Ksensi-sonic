// A/rust/src/lib.rs
// Minimal cdylib placeholder used by workflow A
use std::ffi::{CStr};
use std::os::raw::{c_char, c_int};
use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Default)]
struct AimbotConfig { enabled: bool, precision: i32, auxilio_type: String, aimbot_type: String }

lazy_static! {
    static ref CONFIG: Mutex<AimbotConfig> = Mutex::new(AimbotConfig::default());
}

unsafe fn cstr_to_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() { return None; }
    CStr::from_ptr(ptr).to_str().ok().map(|s| s.to_owned())
}

#[no_mangle]
pub extern "C" fn set_aimbot_enabled(enabled: bool) {
    let mut cfg = CONFIG.lock().unwrap();
    cfg.enabled = enabled;
    println!('[zsensi_bridge] enabled -> {}', enabled);
}

#[no_mangle]
pub extern "C" fn set_precision_value(precision: c_int) {
    let mut cfg = CONFIG.lock().unwrap();
    cfg.precision = precision as i32;
    println!('[zsensi_bridge] precision -> {}', cfg.precision);
}

#[no_mangle]
pub extern "C" fn set_auxilio_type(aux: *const c_char) {
    let mut cfg = CONFIG.lock().unwrap();
    unsafe { if let Some(s) = cstr_to_string(aux) { cfg.auxilio_type = s; } }
}

#[no_mangle]
pub extern "C" fn inject() {
    let cfg = CONFIG.lock().unwrap();
    println!('[zsensi_bridge] inject -> enabled={}, precision={}, auxilio={}');
}
