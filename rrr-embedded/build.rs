use std::fs::{create_dir_all, File, read_dir, remove_dir_all, DirEntry};
use std::io::{BufReader, copy};
use std::path::{Path, PathBuf};
use trunk_build_time::cmd::build;
use trunk_build_time::config;
use embuild::{
    build::LinkArgs,
};
use flate2::Compression;
use tokio;
use flate2::write::GzEncoder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../");

    let mut cfg = config::ConfigOptsBuild::default();
    cfg.release = true;
    cfg.target = Some(PathBuf::from("../rrr-frontend/index.html"));
    cfg.filehash = Some(false);
    println!("{:?}", cfg);
    build::Build { build: cfg }.run(None).await.unwrap();

    let _ = remove_dir_all("../rrr-frontend/dist-gz");
    create_dir_all("../rrr-frontend/dist-gz").unwrap();


    fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> std::io::Result<()> {
        if dir.is_dir() {
            for entry in read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, cb)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }

    //TODO: dirty code here

    visit_dirs(Path::new("../rrr-frontend/dist/"), &|file_path: &DirEntry| {
        let source_path = file_path.path().as_os_str().to_owned();
        let mut input = BufReader::new(File::open(&source_path).unwrap());
        create_dir_all(Path::new(&source_path.to_str().unwrap()
            .replace("rrr-frontend/dist", "rrr-frontend/dist-gz")).parent().unwrap()).unwrap();
        let output = File::create(Path::new(&source_path.to_str().unwrap()
            .replace("rrr-frontend/dist", "rrr-frontend/dist-gz"))).unwrap();
        let mut encoder = GzEncoder::new(output, Compression::default());
        copy(&mut input, &mut encoder).unwrap();
        encoder.finish().unwrap();
        ()
    })?;

    LinkArgs::output_propagated("ESP_IDF")?;
    Ok(())
}