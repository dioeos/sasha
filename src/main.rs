use std::fs::{self, File};
use std::io::{Result, prelude::*};
use std::path::Path;
use std::thread;
use std::time::Duration;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()>{
     let path = Path::new("/tmp/sasha/foo.txt");

     let subscriber = FmtSubscriber::builder()
         .with_max_level(Level::TRACE)
         .finish();
     
     if let Some(parent) = path.parent() {
         fs::create_dir_all(parent)?;
     }
     tracing::subscriber::set_global_default(subscriber)
         .expect("Failed to set tracing subscriber!");


    let mut file = File::create("/tmp/sasha/foo.txt")?;
    loop {
        file.write_all(b"Hello. I'm Sasha, a Rust daemon\n")?;
        file.flush()?;
        info!("Sasha heartbeat...");
        thread::sleep(Duration::from_secs(1));
    }
}
