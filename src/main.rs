extern crate rustc_serialize;

use std::time::Duration;
use std::thread;

mod output;
mod cpu_usage;
mod command;
mod time;

use output::*;

// https://github.com/i3/i3status/tree/master/src
fn main() {
    let output = I3barOutput;
    print!("{}", output.start());

    let mut cpu_usage = cpu_usage::CpuUsage::new();

    loop{
        let cpu = cpu_usage.get().iter().map(|usage| format!("{:5.1}",usage*100f32)).collect::<Vec<String>>().join(" ");
        let date = command::command("date -u");
        let localtime = time::get_time("%H %M %S");

        let outs = vec![format!("cpu {}", cpu), format!("date {}", date), format!("time {}", localtime)];
        let bar = output.chunk(outs);

        println!("{}",bar);
        thread::sleep(Duration::from_millis(1000));
    }
}
