use bindings::Windows::Win32::Foundation::HANDLE;
use bindings::Windows::Win32::System::Power::PowerRegisterSuspendResumeNotification;
use bindings::Windows::Win32::System::Power::DEVICE_NOTIFY_CALLBACK;
use bindings::Windows::Win32::System::Power::HPOWERNOTIFY;
use bindings::Windows::Win32::System::Power::PDEVICE_NOTIFY_CALLBACK_ROUTINE;
use bindings::Windows::Win32::UI::WindowsAndMessaging::PBT_APMRESUMEAUTOMATIC;
use bindings::Windows::Win32::UI::WindowsAndMessaging::PBT_APMRESUMESUSPEND;
use bindings::Windows::Win32::UI::WindowsAndMessaging::PBT_APMSUSPEND;
use std::ffi::c_void;
use std::ptr::null_mut;

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
    // 电源管理事件: https://docs.microsoft.com/en-us/windows/win32/power/power-management-events

    use std::time::Duration;

    unsafe extern "system" fn power_callback(
        _context: *mut c_void,
        power_type: u32,
        _setting: *mut c_void,
    ) -> u32 {
        match power_type {
            PBT_APMRESUMEAUTOMATIC => {
                println!("power_callback, tyle: RESUMEAUTOMATIC");
            }
            PBT_APMRESUMESUSPEND => {
                println!("power_callback, tyle: RESUMESUSPEND");
            }
            PBT_APMSUSPEND => {
                println!("power_callback, tyle: SUSPEND");
            }
            _ => {
                println!("power_callback, type: {}", power_type);
            }
        }
        0
    }

    pub struct DeviceNotifySubscribeParameters {
        pub callback: PDEVICE_NOTIFY_CALLBACK_ROUTINE,
        pub context: *mut ::std::ffi::c_void,
    }

    let mut params = DeviceNotifySubscribeParameters {
        callback: power_callback,
        context: null_mut(),
    };

    let mut handle = HPOWERNOTIFY::default();
    let ret = unsafe {
        let params =
            &mut params as *mut DeviceNotifySubscribeParameters as *mut c_void as *mut HANDLE;
        let params = *params;
        PowerRegisterSuspendResumeNotification(
            DEVICE_NOTIFY_CALLBACK.0,
            params,
            &mut handle as *mut HPOWERNOTIFY as *mut *mut c_void,
        )
    };

    if ret != 0 {
        println!("PowerRegisterSuspendResumeNotification failed");
        return;
    }

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
