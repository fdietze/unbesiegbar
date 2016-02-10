use std::path::Path;
use std::fs::File;
use std::io::{BufRead,BufReader};


pub struct CpuUsage {
    prev: Vec<CpuState>
}

#[derive(Debug)]
struct CpuState{total:u64, idle:u64}

pub type Usage = Vec<f32>;

impl CpuUsage {
    pub fn new() -> CpuUsage {
        CpuUsage {
            prev: CpuTimes::get_states(),
        }
    }

    pub fn get(&mut self) -> Usage {
        let current = CpuTimes::get_states();

        let usage:Usage = current.iter().zip(self.prev.iter()).map(|(c,p)| {
            let diff_total = c.total - p.total;
            let diff_idle = c.idle - p.idle;
            if diff_total > 0 {
                (diff_total - diff_idle) as f32 / diff_total as f32
            } else {0f32}
        } as f32).collect();

        self.prev = current;

        usage
    }
}

#[derive(Debug)]
struct CpuTimes {
    user: u64,
    nice: u64,
    system: u64,
    idle: u64,
    iowait: u64,
    irq: u64,
    softirq: u64,
    steal: u64,
    guest: u64,
    guest_nice: u64,
}

impl CpuTimes {
    fn from_line(line: &str) -> CpuTimes {
        let parts: Vec<_> = line.split_whitespace()
            .skip(1)
            .map(|elem| elem.parse::<u64>().unwrap_or(0))
            .collect();

        CpuTimes{
            user: parts[0],
            nice: parts[1],
            system: parts[2],
            idle: parts[3],
            iowait: parts[4],
            irq: parts[5],
            softirq: parts[6],
            steal: parts[7],
            guest: parts[8],
            guest_nice: parts[9],
        }
    }

    fn to_state(&self) -> CpuState {
        CpuState {total: self.user + self.nice + self.system + self.idle, idle: self.idle}
    }

    pub fn get_states() -> Vec<CpuState> {
        let fh = File::open(Path::new("/proc/stat")).unwrap();
        let reader = BufReader::new(fh);
        let mut cpus: Vec<CpuState> = Vec::new();
        let mut lines = reader.lines();
        // Skip first line since it's just totals
        lines.next();

        for line in lines {
            let line = line.unwrap();
            if !line.starts_with("cpu") {
                break;
            }
            cpus.push(CpuTimes::from_line(&line).to_state());
        }

        cpus
    }
}
