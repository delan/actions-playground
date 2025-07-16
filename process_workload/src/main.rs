use std::{env, error::Error, process::Command, thread::sleep, time::Duration};

use reqwest::blocking::get;

fn main() -> Result<(), Box<dyn Error>> {
    // usage: process_workload
    //    or: process_workload child
    if env::args().nth(1).as_deref() == Some("child") {
        for _ in 0..10 {
            sleep(Duration::from_secs(1));
            get("https://ci0.servo.org/secret")?;
        }
    } else {
        Command::new("target/debug/process_workload")
            .arg("child")
            .spawn()?;
    }

    Ok(())
}
