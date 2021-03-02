use bytesize::ByteSize;
use std::collections::HashMap;
use std::ffi::CStr;
use std::fs;
use std::time::Duration;

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

pub struct SysInfo {
    uptime: Duration,
    process_num: u16,
}

pub struct UserInfo {
    name: String,
    home: String,
    shell: String,
}

pub struct MachineInfo {
    arch: String,
    kernel: String,
    hostname: String,
}

pub struct Color(pub String);

pub struct Distro(pub String);

fn c_ptr_to_string(ptr: *const i8) -> String {
    unsafe { CStr::from_ptr(ptr).to_owned().into_string().unwrap() }
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
        total: ByteSize::kb(mem["MemTotal"]),
        free: ByteSize::kb(mem["MemFree"]),
        avail: ByteSize::kb(mem["MemAvailable"]),
        cached: ByteSize::kb(mem["Cached"]),
        buffers: ByteSize::kb(mem["Buffers"]),
    }
}

pub fn get_user_info() -> UserInfo {
    unsafe {
        let pw_ent = *libc::getpwuid(libc::getuid());
        UserInfo {
            name: c_ptr_to_string(pw_ent.pw_name),
            home: c_ptr_to_string(pw_ent.pw_dir),
            shell: c_ptr_to_string(pw_ent.pw_shell),
        }
    }
}

pub fn get_machine_info() -> MachineInfo {
    unsafe {
        let mut buffer = std::mem::MaybeUninit::<libc::utsname>::uninit();
        libc::uname(buffer.as_mut_ptr());
        buffer.assume_init();
        let buffer = *buffer.as_ptr();

        MachineInfo {
            arch: c_ptr_to_string(buffer.machine.as_ptr()),
            kernel: c_ptr_to_string(buffer.release.as_ptr()),
            hostname: c_ptr_to_string(buffer.nodename.as_ptr()),
        }
    }
}

pub fn get_color_scheme() -> Vec<Color> {
    let mut colors: Vec<Color> = vec![];

    for i in (30..=37).chain(40..=47) {
        colors.push(Color(format!("\x1B[{}m", i)));
    }

    colors
}

pub fn get_distro() -> Distro {
    let os_release = fs::read_to_string("/etc/os-release").unwrap();
    let os_release: HashMap<String, String> = os_release
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|elm| {
            let data = elm.split("=").collect::<Vec<&str>>();
            (data[0].to_string(), data[1].to_string())
        })
        .collect();

    Distro(os_release.get("NAME").unwrap().to_string())
}

pub fn get_sysinfo() -> SysInfo {
    unsafe {
        let mut inf = std::mem::MaybeUninit::<libc::sysinfo>::uninit();
        libc::sysinfo(inf.as_mut_ptr());
        inf.assume_init();
        let inf = *inf.as_ptr()

        let uptime = Duration::new(inf.uptime as u64, 0);
        let process_num = inf.procs;

        SysInfo {
            uptime,
            process_num,
        }
    }
}
