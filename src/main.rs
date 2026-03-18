use sysinfo::System;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    let interval = Duration::from_secs(1);
    let mut next_time = Instant::now() + interval;
    let mut sys = System::new_all();

    sys.refresh_all();
    let cpu_count = sys.cpus().len();

    print!("\x1B[?25l");

    loop {
        sys.refresh_all();

        let cpus = sys.cpus();
        let global: f32 = cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32;

        println!("Global CPU usage: {:.1}%", global);

        for cpu in cpus {
            println!("CPU {:>4} usage: {:.1}%", cpu.name(), cpu.cpu_usage());
        }

        let lines = 1 + cpu_count;
        print!("\x1B[{}A", lines);

        let now = Instant::now();
        if next_time > now {
            sleep(next_time - now);
        }
        next_time += interval;
    }
}