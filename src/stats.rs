extern crate minreq;

use bytesize::ByteSize;
use chrono::prelude::{DateTime, Local};
use isolang::Language;
#[cfg(feature = "battery")]
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use std::{collections::HashMap, net::Ipv4Addr};

// TODO:
// Try to utilize async and multi-threading for better performance

pub enum IpType {
    Public,
    Private,
}

pub struct CpuInfo {
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

#[cfg(feature = "battery")]
pub struct BatteryInfo {
    pub status: BatteryStatus,
    pub percent: u8,
}

#[cfg(feature = "battery")]
pub enum BatteryStatus {
    Charging,
    Discharging,
    Full,
}

#[cfg(feature = "battery")]
impl Display for BatteryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Charging => "charging",
                Self::Discharging => "discharging",
                Self::Full => "full",
            }
        )
    }
}

// Utility functions

pub fn get_env(key: &str) -> Option<String> {
    std::env::var(key).ok()
}

// Functions for getting statistics and information about the system
// Used by the rfetch frontend (might seperate this into another crate)

pub fn cpu_info() -> Option<Vec<CpuInfo>> {
    let data = fs::read_to_string("/proc/cpuinfo").ok()?;

    let blocks = data
        .split('\n')
        .filter(|elm| elm.starts_with("model name") || elm.starts_with("cpu MHz"))
        .map(|elm| elm.split(": ").nth(1))
        .collect::<Vec<Option<&str>>>();

    blocks
        .chunks(2)
        .map(|ck| -> Option<CpuInfo> {
            Some(CpuInfo {
                model_name: String::from(ck[0]?),
                cpu_mhz: ck[1]?.parse::<f64>().ok()?,
            })
        })
        .collect()
}

pub fn mem_info() -> Option<MemInfo<ByteSize>> {
    let data = fs::read_to_string("/proc/meminfo").ok()?;
    let mem = data
        .split('\n')
        .map(|kv| kv.split_whitespace().take(2).collect::<Vec<&str>>())
        .filter(|elm| !elm.is_empty())
        .map(|elm| -> (String, Option<u64>) {
            let mut key = elm[0].to_string();
            key.pop();
            let val = elm[1].parse::<u64>().ok();
            (key, val)
        })
        .collect::<HashMap<String, Option<u64>>>();

    let total = mem["MemTotal"];
    let avail = mem["MemAvailable"];
    let cached = mem["Cached"];
    let buffers = mem["Buffers"];
    let used = total? - avail?;

    Some(MemInfo {
        total: ByteSize::kb(total?),
        avail: ByteSize::kb(avail?),
        cached: ByteSize::kb(cached?),
        buffers: ByteSize::kb(buffers?),
        used: ByteSize::kb(used),
    })
}

pub fn user_info() -> Option<UserInfo> {
    let user = nix::unistd::User::from_uid(nix::unistd::getuid()).ok()??;
    Some(UserInfo {
        name: user.name,
        home: user.dir,
        shell: user.shell,
    })
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

pub fn distro() -> Option<Distro> {
    let os_release = fs::read_to_string("/etc/os-release").ok()?;
    let os_release: HashMap<String, String> = os_release
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|elm| {
            let data = elm
                .split('=')
                .map(|s| s.to_string().replace("\"", ""))
                .collect::<Vec<String>>();
            (data[0].clone(), data[1].clone())
        })
        .collect();

    Some(Distro {
        name: os_release["NAME"].clone(),
        color: os_release["ANSI_COLOR"].clone(),
    })
}

pub fn sysinfo() -> Option<SysInfo> {
    let sinf = nix::sys::sysinfo::sysinfo().ok()?;
    Some(SysInfo {
        uptime: sinf.uptime(),
        process_num: sinf.process_count(),
    })
}

pub fn current_datetime() -> DateTime<Local> {
    Local::now()
}

pub fn locale() -> Option<LocaleInfo> {
    let locale = get_env("LANG")?;
    let hr_lang = Language::from_locale(locale.as_str())?
        .to_name()
        .to_string();
    Some(LocaleInfo { locale, hr_lang })
}

pub fn disk_usage(path: &str) -> Option<FsInfo<ByteSize>> {
    let p_stat = nix::sys::statfs::statfs(path).ok()?;
    let free = p_stat.block_size() as u64 * p_stat.blocks_available();
    let total = p_stat.block_size() as u64 * p_stat.blocks();
    let used = total - free;

    Some(FsInfo {
        free: ByteSize::b(free),
        used: ByteSize::b(used),
        total_size: ByteSize::b(total),
    })
}

pub fn device() -> Option<DeviceInfo> {
    let data = fs::read_to_string("/sys/class/dmi/id/product_name").ok()?;
    Some(DeviceInfo(data.trim().to_string()))
}

pub fn get_temp() -> Option<Temp> {
    let data = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp").ok()?;
    Some(Temp(data.trim().parse::<i32>().ok()?))
}

pub fn ip(iptype: IpType) -> Option<Ipv4Addr> {
    match iptype {
        IpType::Public => {
            let response = minreq::get("http://ifconfig.me").send();
            let res = response.ok()?;
            let res_str = res.as_str().ok()?;
            return res_str.parse::<Ipv4Addr>().ok();
        }
        IpType::Private => {
            let addrs = nix::ifaddrs::getifaddrs().ok()?;

            for ifaddr in addrs {
                if let Some(x) = ifaddr.address {
                    if let nix::sys::socket::AddressFamily::Inet = x.family() {
                        let x = x.to_string();
                        let addr = x.split(':').next()?.parse::<Ipv4Addr>().ok()?;
                        if addr.is_private() {
                            return Some(addr);
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn packages(distro: &str) -> Option<usize> {
    match distro {
        "Arch Linux" => {
            let output = std::process::Command::new("pacman")
                .arg("-Qq")
                .output()
                .ok()?;
            Some(String::from_utf8_lossy(&output.stdout).split('\n').count())
        }
        "Gentoo" => {
            let mut count = 0;
            for entry in fs::read_dir("/var/db/pkg").ok()? {
                let entry = entry.unwrap();
                let metadata = fs::metadata(entry.path()).ok()?;

                if metadata.is_dir() {
                    count += fs::read_dir(entry.path()).ok()?.count();
                }
            }
            Some(count)
        }
        _ => None,
    }
}

#[cfg(feature = "battery")]
pub fn battery_info() -> Option<BatteryInfo> {
    use std::process::Command;
    let mut state: BatteryInfo = BatteryInfo {
        status: BatteryStatus::Full,
        percent: 0,
    };

    let raw_path = Command::new("upower").args(["-e"]).output().unwrap().stdout;

    let paths = String::from_utf8(raw_path).expect("couldn't get battery system info.");
    let path: &str = paths
        .split('\n')
        .filter(|s| s.contains("BAT"))
        .last()
        .expect("no battery system installed.");

    let raw_stats = Command::new("upower")
        .args(["-i", path])
        .output()
        .unwrap()
        .stdout;
    let stat_string = String::from_utf8(raw_stats).expect("");
    let stats: Vec<String> = stat_string
        .split('\n')
        .filter(|l| l.contains("percentage") | l.contains("state"))
        .map(|l| l.split(':').last().unwrap_or("").trim().to_string())
        .collect();

    for mut s in stats.into_iter() {
        let str = s.clone();
        match &str[..] {
            "charging" => state.status = BatteryStatus::Charging,
            "discharging" => state.status = BatteryStatus::Discharging,
            _ if s.pop() == Some('%') => state.percent = s.parse().unwrap_or(0),
            _ => {}
        }
    }

    Some(state)
}
