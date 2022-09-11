use std::sync::Arc;

use embedded_svc::wifi::*;
use esp_idf_svc::{netif::EspNetifStack, nvs::EspDefaultNvs, sysloop::EspSysLoopStack, wifi::*};

#[allow(unused)]
pub fn init_wifi(ssid: String, pass: String) {
    let netif_stack = Arc::new(EspNetifStack::new().unwrap());
    let sys_loop_stack = Arc::new(EspSysLoopStack::new().unwrap());
    let default_nvs = Arc::new(EspDefaultNvs::new().unwrap());
    let mut wifi = EspWifi::new(netif_stack, sys_loop_stack, default_nvs).unwrap();

    let ap_infos = wifi.scan().unwrap();

    let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

    let channel = if let Some(ours) = ours {
        Some(ours.channel)
    } else {
        None
    };

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid,
        password: pass,
        channel,
        ..Default::default()
    }))
    .unwrap();

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(_))),
        _,
    ) = status
    {}
}
