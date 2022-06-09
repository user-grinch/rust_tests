use std::thread;
use std::os::raw::c_char;
use windows::{
    Win32::Foundation::*, Win32::System::SystemServices::*,
};

fn plugin_thread() 
{
    type FHelpMSG = extern "cdecl" fn(*const c_char, c_char, c_char, c_char);
    loop 
    {
        unsafe 
        {
            // void __cdecl SetHelpMessage(const char*, bool, bool, bool)
            let func: FHelpMSG = std::mem::transmute(0x588BE0); 
            func(b"Test Message\0".as_ptr() as *const c_char, 0, 0, 0);
        }
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[no_mangle]
extern "stdcall" fn DllMain( _ : HINSTANCE, reason: u32, _: u32) -> bool 
{
    if reason == DLL_PROCESS_ATTACH 
    {
        thread::spawn(plugin_thread);
    }

    return true;  
}

