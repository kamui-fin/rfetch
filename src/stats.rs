extern crate minreq;

use bytesize::ByteSize;
use chrono::prelude::{DateTime, Local};
use isolang::Language;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use std::{collections::HashMap, net::Ipv4Addr};

// TODO:
// Replace unwraps with proper error handling
// Re-consider uses of `String` in some scenarios
// Try to utilize async and multi-threading for better performance

pub enum IpType {
    Public,
    Private,
}

pub struct CPUInfo {
    pub model_name: String,
    pub cpu_mhz: f64,
}

pub struct MemInfo<T> {
    pub total: T,
    pub avail: T,
    pub cached: T,
    pub buffers: T,
    pub used: T,
}

pub struct MountInfo {
    pub device: String,
    pub m_pnt: String,
    pub fs_type: String,
}

pub struct SysInfo {
    pub uptime: Duration,
    pub process_num: u16,
}

pub struct UserInfo {
    pub name: String,
    pub home: PathBuf,
    pub shell: PathBuf,
}

pub struct MachineInfo {
    pub arch: String,
    pub kernel: String,
    pub nodename: String,
}

pub struct Distro {
    pub name: String,
    pub color: String,
}

pub struct LocaleInfo {
    pub locale: String,
    pub hr_lang: String,
}

pub struct FsInfo<T> {
    pub total_size: T,
    pub free: T,
    pub used: T,
}

pub struct Color(pub String);

pub struct DeviceInfo(pub String);

pub struct Temp(pub i32);

// Utility functions

fn get_env(key: String) -> String {
    std::env::var(key).unwrap()
}

// Functions for getting statistics and information about the system
// Used by the rfetch frontend (might seperate this into another crate)

pub fn cpu_info() -> Vec<CPUInfo> {
    let data = fs::read_to_string("/proc/cpuinfo").unwrap();
    let blocks = data
        .split("\n")
        .filter(|elm| elm.starts_with("model name") || elm.starts_with("cpu MHz"))
        .map(|elm| elm.split(": ").nth(1).unwrap())
        .collect::<Vec<&str>>();

    let cpus = blocks
        .chunks(2)
        .map(|ck| CPUInfo {
            model_name: String::from(ck[0]),
            cpu_mhz: ck[1].parse::<f64>().unwrap(),
        })
        .collect::<Vec<CPUInfo>>();

    cpus
}

pub fn mem_info() -> MemInfo<ByteSize> {
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

    let total = mem["MemTotal"];
    let avail = mem["MemAvailable"];
    let cached = mem["Cached"];
    let buffers = mem["Buffers"];
    let used = total - avail;

    MemInfo {
        total: ByteSize::kb(total),
        avail: ByteSize::kb(avail),
        cached: ByteSize::kb(cached),
        buffers: ByteSize::kb(buffers),
        used: ByteSize::kb(used),
    }
}

pub fn user_info() -> UserInfo {
    let user = nix::unistd::User::from_uid(nix::unistd::getuid())
        .unwrap()
        .unwrap();
    UserInfo {
        name: user.name,
        home: user.dir,
        shell: user.shell,
    }
}

pub fn machine_info() -> MachineInfo {
    let mach = nix::sys::utsname::uname();
    MachineInfo {
        arch: String::from(mach.machine()),
        kernel: String::from(mach.release()),
        nodename: String::from(mach.nodename()),
    }
}

pub fn color_scheme() -> Vec<Color> {
    (40..=47)
        .chain(100..=107)
        .map(|cl| Color(format!("\x1B[{}m", cl)))
        .collect::<Vec<Color>>()
}

pub fn distro() -> Distro {
    let os_release = fs::read_to_string("/etc/os-release").unwrap();
    let os_release: HashMap<String, String> = os_release
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|elm| {
            let data = elm.split("=").collect::<Vec<&str>>();
            (data[0].to_string(), data[1].to_string())
        })
        .collect();

    Distro {
        name: os_release["NAME"].clone(),
        color: os_release["ANSI_COLOR"].clone(),
    }
}

pub fn sysinfo() -> SysInfo {
    let sinf = nix::sys::sysinfo::sysinfo().unwrap();
    SysInfo {
        uptime: sinf.uptime(),
        process_num: sinf.process_count(),
    }
}

pub fn current_datetime() -> DateTime<Local> {
    Local::now()
}

pub fn locale() -> LocaleInfo {
    let locale = get_env(String::from("LANG"));
    let hr_lang = Language::from_locale(locale.as_str())
        .unwrap()
        .to_name()
        .to_string();
    LocaleInfo { locale, hr_lang }
}

pub fn get_mounts() -> Vec<MountInfo> {
    let proc_mnts_data = fs::read_to_string("/proc/mounts")
        .unwrap()
        .split("\n")
        .filter(|elm| elm.len() > 0)
        .map(|line| {
            let clms: Vec<&str> = line.split_whitespace().collect();
            MountInfo {
                device: String::from(clms[0]),
                m_pnt: String::from(clms[1]),
                fs_type: String::from(clms[2]),
            }
        })
        .collect::<Vec<MountInfo>>();
    proc_mnts_data
}

pub fn disk_usage(path: &str) -> FsInfo<ByteSize> {
    let p_stat = nix::sys::statfs::statfs(path).unwrap();

    let free = p_stat.block_size() as u64 * p_stat.blocks_available();
    let total = p_stat.block_size() as u64 * p_stat.blocks();
    let used = total - free;

    FsInfo {
        free: ByteSize::b(free),
        used: ByteSize::b(used),
        total_size: ByteSize::b(total),
    }
}

pub fn device() -> DeviceInfo {
    DeviceInfo(
        fs::read_to_string("/sys/class/dmi/id/product_name")
            .unwrap()
            .trim()
            .to_string(),
    )
}

pub fn get_temp() -> Temp {
    let temp_data = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp").unwrap();
    Temp(temp_data.trim().parse::<i32>().unwrap())
}

pub fn ip(iptype: IpType) -> Option<Ipv4Addr> {
    match iptype {
        IpType::Public => {
            let response = minreq::get("http://ifconfig.me").send().unwrap();
            response.as_str().unwrap().parse::<Ipv4Addr>().ok()
        }
        IpType::Private => {
            let addrs = nix::ifaddrs::getifaddrs().unwrap();
            for ifaddr in addrs {
                if let Some(x) = ifaddr.address {
                    if let nix::sys::socket::AddressFamily::Inet = x.family() {
                        let addr = x.to_string().split(":").next().unwrap().to_string();
                        if addr.starts_with("192.168.1.") {
                            return addr.parse::<Ipv4Addr>().ok();
                        }
                    }
                }
            }
            None
        }
    }
}

// A few more feature ideas

// pub fn packages() {}
// pub fn gpu_info() {}
// pub fn default_apps() {}
// pub fn current_track() {}
// pub fn wm_name() {}
// pub fn theme_info() {}
// pub fn display_info() {}
// pub fn init_system() {}
// pub fn volume() {}
// pub fn in_speed() {}
