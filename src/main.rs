use std::fs::{self, File};
use std::io::{Result, prelude::*};
use std::path::Path;
use std::thread;
use std::time::Duration;

fn main() -> Result<()>{
     let path = Path::new("/tmp/sasha/foo.txt");

     if let Some(parent) = path.parent() {
         fs::create_dir_all(parent)?;
     }


    let mut file = File::create("/tmp/sasha/foo.txt")?;
    loop {
        file.write_all(b"Hello. I'm Sasha, a Rust daemon\n")?;
        file.flush()?;
        thread::sleep(Duration::from_secs(1));
    }
}
