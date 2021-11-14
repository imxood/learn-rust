use embedded_svc::wifi::{
    AccessPointConfiguration, ApIpStatus, ApStatus, ClientConfiguration, ClientConnectionStatus,
    ClientIpStatus, ClientStatus, Configuration, Status, Wifi,
};
use esp_idf_svc::wifi::EspWifi;

pub fn wifi_connect(wifi: &mut EspWifi, ssid: &str, pass: &str) {
    wifi.set_configuration(&Configuration::Mixed(
        ClientConfiguration {
            ssid: ssid.into(),
            password: pass.into(),
            channel: None,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: 1,
            ..Default::default()
        },
    ))
    .unwrap();

    println!("Wifi configuration set, about to get status");

    let status = wifi.get_status();
    println!("Wifi status: {:?}", &status);
}

pub fn wifi_scan() {
    // let ap_infos = wifi.scan()?;

    // let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

    // let channel = if let Some(ours) = ours {
    //     println!(
    //         "Found configured access point {} on channel {}",
    //         ssid, ours.channel
    //     );
    //     Some(ours.channel)
    // } else {
    //     println!(
    //         "Configured access point {} not found during scanning, will go with unknown channel",
    //         ssid
    //     );
    //     None
    // };
    // Ok(())
}
