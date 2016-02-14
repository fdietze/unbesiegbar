use std::time::Duration;
use std::thread;

mod cpu_usage;
mod command;
mod time;

// https://github.com/i3/i3status/tree/master/src

fn main() {
    let mut cpu_usage = cpu_usage::CpuUsage::new();

    loop{
        let cpu = cpu_usage.get().iter().map(|usage| format!("{:5.1}",usage*100f32)).collect::<Vec<String>>().join(" ");
        let date = command::command("date -u");
        let localtime = time::get_time("%H %M %S");

        let bar = format!("cpu {} | date {} | time {}", cpu, date, localtime);

        println!("{}",bar);
        thread::sleep(Duration::from_millis(1000));
    }
}
