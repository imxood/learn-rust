use bindings::Windows::Win32::System::Power::PowerRegisterSuspendResumeNotification;
use bindings::Windows::Win32::System::Power::DEVICE_NOTIFY_CALLBACK;
use bindings::Windows::Win32::System::Power::DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS;
use bindings::Windows::Win32::System::Power::HPOWERNOTIFY;
use std::ffi::c_void;
use std::ptr::null_mut;
use windows::IntoParam;

#[test]
fn test_message_box() -> windows::Result<()> {
    use bindings::Windows::Win32::UI::WindowsAndMessaging::*;
    unsafe {
        MessageBoxA(None, "Text", "Caption", MB_OK);
    }
    Ok(())
}

#[test]
fn test_power() {

    // 参考: https://stackoverflow.com/questions/11394625/powerregistersuspendresumenotification-provided-callback-function-doesnt-work

    use std::time::Duration;

    unsafe extern "system" fn power_callback(
        context: *mut c_void,
        power_type: u32,
        setting: *mut c_void,
    ) -> u32 {
        println!("power_callback");
        0
    }

    let params = DEVICE_NOTIFY_SUBSCRIBE_PARAMETERS {
        Callback: Some(power_callback),
        Context: null_mut(),
    };

    let mut handle = HPOWERNOTIFY::default();
    unsafe {
        PowerRegisterSuspendResumeNotification(
            DEVICE_NOTIFY_CALLBACK.0,
            &params,
            &mut handle.0 as *mut c_void as *mut *mut c_void,
        );
    }

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
