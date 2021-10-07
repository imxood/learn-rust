fn main() {
    windows::build! {
        Windows::Data::Xml::Dom::*,
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject},
        Windows::Win32::System::Power::*,
        Windows::Win32::UI::WindowsAndMessaging::MessageBoxA,
        Windows::Win32::UI::WindowsAndMessaging::PBT_APMRESUMEAUTOMATIC,
        Windows::Win32::UI::WindowsAndMessaging::PBT_APMRESUMESUSPEND,
        Windows::Win32::UI::WindowsAndMessaging::PBT_APMSUSPEND,
    };
}
