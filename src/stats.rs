use std::collections::HashMap;
use std::fs;

use bytesize::ByteSize;

pub struct CPUInfo {
    model_name: String,
    cpu_mhz: f64,
}

pub struct MemInfo<T> {
    total: T,
    free: T,
    avail: T,
    cached: T,
    buffers: T,
}

pub fn get_cpu_info() -> Vec<CPUInfo> {
    let data = fs::read_to_string("/proc/cpuinfo").unwrap();
    let blocks = data
        .split("\n")
        .filter(|elm| elm.starts_with("model name") || elm.starts_with("cpu MHz"))
        .map(|elm| elm.split(": ").nth(1).unwrap())
        .collect::<Vec<&str>>();
    let mut cpus = vec![];

    for block in blocks.chunks(2) {
        let cpu = CPUInfo {
            model_name: String::from(block[0]),
            cpu_mhz: block[1].parse::<f64>().unwrap(),
        };
        cpus.push(cpu);
    }

    cpus
}

pub fn get_mem_info() -> MemInfo<ByteSize> {
    let data = fs::read_to_string("/proc/meminfo").unwrap();
    let mem: HashMap<String, u64> = data
        .split("\n")
        .map(|kv| kv.split_whitespace().take(2).collect::<Vec<&str>>())
        .filter(|elm| elm.len() > 0)
        .map(|elm| {
            let mut key = elm[0].to_string();
            key.pop();
            let val = elm[1].parse::<u64>().unwrap();
            (key, val)
        })
        .collect();

    MemInfo {
        total: ByteSize::kb(*mem.get("MemTotal").unwrap()),
        free: ByteSize::kb(*mem.get("MemFree").unwrap()),
        avail: ByteSize::kb(*mem.get("MemAvailable").unwrap()),
        cached: ByteSize::kb(*mem.get("Cached").unwrap()),
        buffers: ByteSize::kb(*mem.get("Buffers").unwrap()),
    }
}
