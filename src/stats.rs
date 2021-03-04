use bytesize::ByteSize;
use chrono::prelude::{DateTime, Local};
use isolang::Language;
use std::ffi::CStr;
use std::fs;
use std::time::Duration;
use std::{collections::HashMap, ffi::CString};

// TODO:
// Replace unwraps with proper error handling
// Re-consider uses of `String` in some scenarios
// Try to utilize async and multi-threading for better performance

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

pub struct SysInfo {
    pub uptime: Duration,
    pub process_num: u16,
}
pub struct UserInfo {
    pub name: String,
    pub home: String,
    pub shell: String,
}

pub struct MachineInfo {
    pub arch: String,
    pub kernel: String,
    pub hostname: String,
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
    pub mount_point: String,
    pub filesys: String,
    pub total_size: T,
    pub free: T,
}

pub struct Color(pub String);

// Utility functions

fn c_ptr_to_string(ptr: *const i8) -> String {
    unsafe { CStr::from_ptr(ptr).to_owned().into_string().unwrap() }
}

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
    unsafe {
        let pw_ent = *libc::getpwuid(libc::getuid());
        UserInfo {
            name: c_ptr_to_string(pw_ent.pw_name),
            home: c_ptr_to_string(pw_ent.pw_dir),
            shell: c_ptr_to_string(pw_ent.pw_shell),
        }
    }
}

pub fn machine_info() -> MachineInfo {
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
    unsafe {
        let mut inf = std::mem::MaybeUninit::<libc::sysinfo>::uninit();
        libc::sysinfo(inf.as_mut_ptr());
        inf.assume_init();
        let inf = *inf.as_ptr();

        let uptime = Duration::new(inf.uptime as u64, 0);
        let process_num = inf.procs;

        SysInfo {
            uptime,
            process_num,
        }
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

pub fn disk_usage() -> Vec<FsInfo<ByteSize>> {
    unsafe {
        let mut disk_stats: Vec<FsInfo<ByteSize>> = vec![];

        let mtab = CString::new("/etc/mtab").unwrap();
        let perm = CString::new("r").unwrap();
        let mnts = libc::setmntent(mtab.as_ptr(), perm.as_ptr());

        let mut mntent = libc::getmntent(mnts);

        while !mntent.is_null() {
            let mnt_pnt = (*mntent).mnt_dir;
            let filesys = (*mntent).mnt_type;
            let mut stat_buf = std::mem::MaybeUninit::<libc::statfs>::uninit();
            libc::statfs(mnt_pnt, stat_buf.as_mut_ptr());
            stat_buf.assume_init();
            let stat_buf = *stat_buf.as_ptr();
            let free = stat_buf.f_bsize as u64 * stat_buf.f_bavail;
            let total = stat_buf.f_bsize as u64 * stat_buf.f_blocks;

            disk_stats.push(FsInfo {
                mount_point: c_ptr_to_string(mnt_pnt),
                filesys: c_ptr_to_string(filesys),
                total_size: ByteSize::b(total),
                free: ByteSize::b(free),
            });

            mntent = libc::getmntent(mnts);
        }

        libc::endmntent(mnts);
        disk_stats
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
// pub fn ip() {}
// pub fn in_speed() {}
