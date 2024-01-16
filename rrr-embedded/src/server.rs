use crate::api;
use std::io;
use std::io::ErrorKind;

use std::sync::{Arc, Mutex};
use anyhow::Result;
use embedded_svc::http::server::{Connection, Request};
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_sys::EspError;
use include_dir::{Dir, include_dir};


static DIST: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../rrr-frontend/dist-gz/");

pub struct Server {
    server: EspHttpServer,
}

impl Server {
    pub fn new<F>(
        state: Arc<Mutex<api::State>>,
        mut command_handler: F,
    ) -> Result<Self>
        where F: Fn(&api::Command) -> Result<()> + Send + 'static
    {
        use embedded_svc::http::server::{Method};
        use embedded_svc::io::Write;
        use esp_idf_svc::http::server::{EspHttpServer};

        let mut conf = esp_idf_svc::http::server::Configuration::default();
        conf.max_resp_handlers = 100;
        conf.max_uri_handlers = 100;

        let mut server = EspHttpServer::new(&conf)?;

        fn serve_file<'a>(server: &'a mut EspHttpServer, path: &'static str, content: &'static [u8]) -> Result<(), EspError> {
            server.fn_handler(format!("/{}", path).as_ref(), Method::Get, move |req| {
                let content_type = if path.ends_with(".js") { "application/javascript" } else if path.ends_with(".wasm") { "application/wasm" } else if path.ends_with(".html") { "text/html" } else if path.ends_with(".htm") { "text/html" } else if path.ends_with(".html") { "text/html" } else if path.ends_with(".css") { "text/css" } else { "text/html" };

                req.into_response(200, None, &[
                    ("Content-Type", content_type),
                    ("Content-Encoding", "gzip"),
                    ("Access-Control-Allow-Origin", "*"),
                ])?.write_all(content)?;
                Ok(())
            })?;
            Ok(())
        }

        server
            .fn_handler("/state", Method::Get, move |req| {
                let state = state.lock().unwrap().to_owned();

                req.into_response(200, None, &[("Content-Type", "application/json"),
                    ("Access-Control-Allow-Origin", "*"),
                ])?.write_all(serde_json::to_string(&state).unwrap().as_bytes())?;
                Ok(())
            })?
            .fn_handler("/command", Method::Post, move |mut req| {
                struct ReqRead<'a, A>
                {
                    req: &'a mut Request<A>,
                }

                impl<'a, A> io::Read for ReqRead<'a, A>
                    where A: Connection
                {
                    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                        self.req.read(buf).map_err(|e| { io::Error::new(ErrorKind::BrokenPipe, "") })
                    }
                }

                let command = serde_json::from_reader::<_, api::Command>(ReqRead { req: &mut req });

                //TODO headers (cross-origin, content-type)
                match command {
                    Ok(command) => {
                        match command_handler(&command) {
                            Ok(_) => { req.into_ok_response()?; }
                            Err(_) => { req.into_status_response(500)?; }
                        }

                        Ok(())
                    }
                    Err(_) => { req.into_response(400, Some("Unable to parse command"), &[]).map(|_| ()) }
                }?;

                Ok(())
            })?
        ;

        let f = DIST.get_file("index.html").unwrap();
        serve_file(&mut server, "", f.contents())?;

        fn serve_dir(server: &mut EspHttpServer, dir: &'static Dir) -> Result<()> {
            dir.files().for_each(|f| {
                serve_file(server, f.path().to_str().unwrap(), f.contents()).unwrap();
            });
            if dir.dirs().next().is_some() { for dir in dir.dirs() { serve_dir(server, dir).unwrap(); } }
            Ok(())
        }

        DIST.files().for_each(|f| { serve_file(&mut server, f.path().to_str().unwrap(), f.contents()).unwrap(); });
        for dir in DIST.dirs() { serve_dir(&mut server, dir).unwrap(); }

        Ok(Self { server })
    }
}