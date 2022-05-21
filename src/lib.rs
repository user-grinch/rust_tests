use toy_arms;
use std::os::raw::c_char;

toy_arms::create_entrypoint!(main_thread);

fn main_thread() {
    loop {
        if toy_arms::detect_keypress(toy_arms::VirtualKeyCode::VK_INSERT) {
            unsafe {
                // void __cdecl SetHelpMessage(const char*, bool, bool, bool)
                type FHelpMSG = extern "cdecl" fn(*const c_char, c_char, c_char, c_char);
                let func: FHelpMSG = std::mem::transmute(0x588BE0); 
                func(b"Test Message\0".as_ptr() as *const c_char, 0, 0, 0);
            }
        }
    }
}