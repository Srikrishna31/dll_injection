use windows::{core::*, Win32::UI::WindowsAndMessaging::MessageBoxA};
use windows::{Win32::Foundation::*, Win32::System::SystemServices::*};
/// DLLMain
/// Whenever Windows loads a DLL, it checks to see if it exports a function named `DllMain`. If so, the operating sytem
/// calls the function with a `DLL_PROCESS_ATTACH` or `DLL_PROCESS_DETACH` when attaching or detaching the DLL to processes.
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => attach(),
        DLL_PROCESS_DETACH => detach(),
        _ => (),
    }

    true
}

fn attach() {
    unsafe {
        MessageBoxA(HWND(0), s!("ZOMG!"), s!("hello.dll"), Default::default());
    };
}

fn detach() {
    unsafe {
        MessageBoxA(HWND(0), s!("GOODBYE!"), s!("hello.dll"), Default::default());
    };
}
#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
