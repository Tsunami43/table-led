use esp_idf_svc::wifi::*;
use esp_idf_svc::netif::*;
use esp_idf_svc::eventloop::*;
use esp_idf_sys as _;

fn connect_wifi(ssid: &str, password: &str) -> Result<Box<EspWifi>, anyhow::Error> {
    let sysloop = EspSystemEventLoop::take()?;
    let mut wifi = EspWifi::new(EspNetif::new()?, sysloop.clone(), None)?;
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: ssid.into(),
        password: password.into(),
        ..Default::default()
    }))?;

    wifi.start()?;
    wifi.connect()?;
    wifi.wait_netif_up()?;
    Ok(Box::new(wifi))
}
