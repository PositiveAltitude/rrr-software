use std::sync::{Arc, Mutex};
use embedded_svc::wifi::{Configuration, AccessPointConfiguration, ClientConfiguration, AuthMethod};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use log::info;
use anyhow::Result;
use crate::api::*;


pub struct WiFi<'a> {
    wifi: BlockingWifi<EspWifi<'a>>,
    state: Arc<Mutex<State>>,
}

impl<'a> WiFi<'a> {
    pub fn new(
        configuration: WifiConnectionConfiguration,
        modem: impl Peripheral<P=esp_idf_hal::modem::Modem> + 'static,
        sysloop: EspSystemEventLoop,
        state: Arc<Mutex<State>>,
    ) -> Result<Self> {
        let esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
        let mut wifi = BlockingWifi::wrap(esp_wifi, sysloop)?;

        let cfg = match configuration.clone() {
            WifiConnectionConfiguration {
                connection_type: WifiConnectionType::StartAccessPoint,
                credentials: WifiCredentials { ssid, password }
            } => {
                Configuration::AccessPoint(
                    AccessPointConfiguration {
                        ssid: heapless::String::from(ssid.as_str()),
                        channel: 1,
                        password: heapless::String::from(password.as_str()),
                        auth_method: AuthMethod::WPA2Personal,
                        ..Default::default()
                    })
            }
            WifiConnectionConfiguration {
                connection_type: WifiConnectionType::ConnectToExternal,
                credentials: WifiCredentials { ssid, password }
            } => {
                Configuration::Client(
                    ClientConfiguration {
                        ssid: heapless::String::from(ssid.as_str()),
                        password: heapless::String::from(password.as_str()),
                        channel: None,
                        ..Default::default()
                    },
                )
            }
        };

        let client_configuration_result = wifi.set_configuration(&cfg);

        let connection_result = client_configuration_result.and_then(|_| {
            wifi.start()?;
            wifi.connect()?;
            info!("WIFI Connect -- OK");
            let mut configuration = configuration.clone();
            configuration.credentials.password = "".into();
            state.lock().unwrap().wifi_state = configuration;
            Ok(())
        });

        match connection_result {
            Ok(_) => (),
            Err(_) => {
                info!("WIFI Connect -- FAIL");
                let ap_configuration_result = wifi.set_configuration(&Configuration::AccessPoint(
                    AccessPointConfiguration {
                        ssid: "RRR-wifi".into(),
                        channel: 1,
                        ..Default::default()
                    },
                ));
                ap_configuration_result.and_then(|_| {
                    wifi.start()?;
                    info!("WIFI AP Start -- OK");
                    state.lock().unwrap().wifi_state =
                        WifiConnectionConfiguration {
                            connection_type: WifiConnectionType::StartAccessPoint,
                            credentials: WifiCredentials {
                                ssid: "RRR-wifi".into(),
                                password: "".into(),
                            },
                        };
                    Ok(())
                })?;
            }
        };

        wifi.wait_netif_up()?;
        let ip_info = wifi.wifi().ap_netif().get_ip_info()?;
        info!("DHCP -- OK");
        info!("DHCP info: {:?}", ip_info);
        Ok(Self { wifi, state })
    }
}