use std::fs;

struct CPUInfo {
    model_name: String,
    cpu_mhz: f64,
}

fn get_cpu_info() -> Vec<CPUInfo> {
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
