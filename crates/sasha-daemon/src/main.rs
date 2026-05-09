use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Result, prelude::*};
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::os::unix::net::UnixStream;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use dotenvy::dotenv;

fn main() -> Result<()>{
     let subscriber = FmtSubscriber::builder()
         .with_max_level(Level::TRACE)
         .finish();
     tracing::subscriber::set_global_default(subscriber)
         .expect("Failed to set tracing subscriber!");

     info!("Starting sasha daemon...");
     dotenv().expect("Failed to load .env file");
     info!("Successfully loaded environment variables...");

     let niri_socket_path = std::env::var("NIRI_SOCKET").expect("NIRI_SOCKET is not set");

     let path = Path::new("/tmp/sasha/foo.txt");
     let writer_stream = UnixStream::connect(niri_socket_path)?;
     let reader_stream = writer_stream.try_clone()?;

     let mut reader = BufReader::new(reader_stream);
     let mut writer = BufWriter::new(writer_stream);

     if let Some(parent) = path.parent() {
         fs::create_dir_all(parent)?;
     }

    // let mut file = File::create("/tmp/sasha/foo.txt")?;
    loop {
        writer.write_all(b"\"FocusedWindow\"\n")?;
        writer.flush()?;

        let mut response = String::new();
        reader.read_line(&mut response)?;

        info!("{response}");
        thread::sleep(Duration::from_secs(1));
    }
}
