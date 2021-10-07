use bindings::Windows::Win32::System::Power::DEVICE_NOTIFY_CALLBACK;
use bindings::Windows::Win32::System::Power::PDEVICE_NOTIFY_CALLBACK_ROUTINE;
use bindings::Windows::Win32::UI::WindowsAndMessaging::PBT_APMRESUMEAUTOMATIC;
use bindings::Windows::Win32::UI::WindowsAndMessaging::PBT_APMRESUMESUSPEND;
use bindings::Windows::Win32::UI::WindowsAndMessaging::PBT_APMSUSPEND;
use std::ffi::c_void;
use std::ptr::null_mut;

fn main() {
    // 参考: https://stackoverflow.com/questions/11394625/powerregistersuspendresumenotification-provided-callback-function-doesnt-work
    // 电源管理事件: https://docs.microsoft.com/en-us/windows/win32/power/power-management-events

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

    #[repr(C)]
    pub struct DeviceNotifySubscribeParameters {
        pub callback: PDEVICE_NOTIFY_CALLBACK_ROUTINE,
        pub context: *mut ::std::ffi::c_void,
    }
    let mut recipient = DeviceNotifySubscribeParameters {
        callback: power_callback,
        context: null_mut(),
    };

    let mut registrationhandle = null_mut();
    let ret = unsafe {
        let recipient = &mut recipient as *mut DeviceNotifySubscribeParameters as *mut c_void;

        #[link(name = "POWRPROF")]
        extern "system" {
            fn PowerRegisterSuspendResumeNotification(
                flags: u32,
                recipient: *mut c_void,
                registrationhandle: *mut *mut c_void,
            ) -> u32;
        }

        PowerRegisterSuspendResumeNotification(
            DEVICE_NOTIFY_CALLBACK.0,
            recipient,
            &mut registrationhandle as *mut *mut c_void,
        )
    };

    if ret != 0 {
        println!("PowerRegisterSuspendResumeNotification failed");
        return;
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
