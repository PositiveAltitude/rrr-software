use thiserror::Error;
use embedded_svc::io::Write;
use esp_idf_hal::reset::restart;
use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use embedded_svc::http::client::Client;
use esp_idf_svc::ota::EspOta;
use esp_idf_sys::EspError;
use log::{error, info};
use crate::ota::OtaError::UrlUnavailable;


pub struct OtaDriver {
    ota: EspOta,
}

#[derive(Error, Debug)]
pub enum OtaError {
    #[error("Url is unavailable")]
    UrlUnavailable,
    #[error("Firmware download was interrupted")]
    ConnectionInterrupted,
    #[error("ESP-IDF stack error")]
    EspBackendFailure,
    #[error("Firmware does not fit the partition")]
    FileTooBig
}


impl OtaDriver {
    pub fn new() -> Result<Self, EspError> {
        let ota = EspOta::new();
        ota.map(|ota| { Self { ota } })
    }

    //TODO: handle size too big issue
    pub fn run(&mut self, url: &str) -> Result<usize, OtaError>{

        use OtaError::*;

        let mut upd = self.ota.initiate_update().map_err(|_| EspBackendFailure)?;

        let cfg = Configuration::default();
        let conn = EspHttpConnection::new(&cfg).map_err(|_| EspBackendFailure)?;
        let mut client = Client::<EspHttpConnection>::wrap(conn);
        let mut resp = client
            .get(url).map_err(|_| UrlUnavailable)?
            .submit().map_err(|_| UrlUnavailable)?;

        let mut data_transferred = 0;

        loop {
            let mut buf = [0u8; 100];
            let size = resp.read(buf.as_mut_slice()).unwrap();
            data_transferred = data_transferred + size;
            upd.write_all(&buf[0..size]).unwrap();
            if size < 100 {break}
        }

        upd.complete().map_err(|_| EspBackendFailure)?;

        Ok(data_transferred)
    }

    pub fn restart(self) {
        restart();
    }
}