use anyhow::Result;
use esp_idf_svc::nvs::{EspDefaultNvs, EspDefaultNvsPartition, EspNvs, NvsDefault, NvsPartitionId};
use log::info;
use rrr_api::*;

pub struct Nvs {
    espnvs: EspNvs<NvsDefault>,
}

const WIFI_SSID_NAME: &str = "wifi_ssid";
const WIFI_SSID_LENGTH_NAME: &str = "wifi_ssid_l";
const WIFI_PASSWORD_NAME: &str = "wifi_pass";
const WIFI_PASSWORD_LENGTH_NAME: &str = "wifi_pass_l";
const WIFI_SET_NAME: &str = "wifi_set";


impl Nvs {
    pub fn new() -> Result<(Nvs)> {
        let nvs =
            EspDefaultNvs::new(EspDefaultNvsPartition::take()?, "", true)?;
        Ok(Self { espnvs: nvs })
    }

    pub fn set_wifi_connection(&mut self, wifi: WifiCredentials) -> Result<()> {
        self.espnvs.set_u8(WIFI_SET_NAME, 0)?;
        self.espnvs.set_blob(WIFI_SSID_NAME, wifi.ssid.as_bytes())?;
        let ssid_length = wifi.password.len() as u8;
        self.espnvs.set_u8(WIFI_SSID_LENGTH_NAME, wifi.ssid.len() as u8)?;
        self.espnvs.set_blob(WIFI_PASSWORD_NAME, wifi.password.as_bytes())?;
        let pass_length = wifi.password.len() as u8;
        self.espnvs.set_u8(WIFI_PASSWORD_LENGTH_NAME, wifi.password.len() as u8)?;
        self.espnvs.set_u8(WIFI_SET_NAME, 1)?;
        Ok(())
    }

    pub fn get_wifi_connection(&mut self) -> Result<Option<WifiCredentials>> {
        match self.espnvs.get_u8(WIFI_SET_NAME)? {
            None => { Ok(None) }
            Some(0) => { Ok(None) }
            Some(_) => {
                let ssid_length = self.espnvs.get_u8(WIFI_SSID_LENGTH_NAME)?;
                let pass_length = self.espnvs.get_u8(WIFI_PASSWORD_LENGTH_NAME)?;

                match (ssid_length, pass_length) {
                    (Some(sl), Some(pl)) => {
                        let mut ssid = vec![0u8; sl as usize];
                        let mut pass = vec![0u8; pl as usize];
                        self.espnvs.get_blob(WIFI_SSID_NAME, ssid.as_mut_slice())?;
                        self.espnvs.get_blob(WIFI_PASSWORD_NAME, pass.as_mut_slice())?;

                        let ssid = String::from_utf8(ssid)?;
                        let password = String::from_utf8(pass)?;

                        Ok(Some(WifiCredentials { ssid, password }))
                    }
                    _ => { Ok(None) }
                }
            }
        }
    }

    pub fn wipe_data(&mut self) -> Result<()> {
        self.espnvs.remove(WIFI_SET_NAME)?;
        self.espnvs.remove(WIFI_SSID_NAME)?;
        self.espnvs.remove(WIFI_SSID_LENGTH_NAME)?;
        self.espnvs.remove(WIFI_PASSWORD_NAME)?;
        self.espnvs.remove(WIFI_PASSWORD_LENGTH_NAME)?;

        Ok(())
    }
}
