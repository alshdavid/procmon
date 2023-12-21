use std::env;
use std::path::PathBuf;
use std::time;

#[derive(Clone)]
pub enum ReportPath {
    PathBuf(PathBuf),
    Stdout,
}

impl ReportPath {
    pub fn to_string(&self) -> String {
        match self {
            ReportPath::PathBuf(p) => {
                return p.to_str().unwrap().to_string();
            },
            ReportPath::Stdout => {
                return "stdout".to_string();
            }
        }
    }
}

pub fn get_report_file_path() -> ReportPath {
    let file_name = chrono::offset::Local::now().format("%Y%m%d-%H%M%S.csv").to_string();
    let mut report_file = env::current_dir().unwrap().join(PathBuf::from(file_name));
    let report_result = env::var("PM_REPORT");
    if let Ok(report) = report_result {
        if report == "stdout" {
            return ReportPath::Stdout;
        }
        report_file = PathBuf::from(report);
        if report_file.is_relative() {
            report_file = env::current_dir().unwrap().join(report_file);
        }
    };
    return ReportPath::PathBuf(report_file);
}

pub fn get_polling_interval() -> time::Duration {
    let result = env::var("PM_POLL_INTERVAL");
    if let Ok(result) = result {
        if let Ok(result_int) = result.parse::<u64>() {
            return time::Duration::from_millis(result_int);
        }
    }
    return time::Duration::from_secs(1);
}

pub fn get_pid() -> Option<u32> {
    let result = env::var("PM_PID");
    if let Ok(result) = result {
        if let Ok(result_int) = result.parse::<u32>() {
            return Some(result_int);
        }
    }
    return None;
}

pub fn get_mem_units() -> usize {
    let result = env::var("PM_MEM_UNITS");
    if let Ok(result) = result {
        if result.to_lowercase() == "mb" {
            return 1048576;
        }
        if result.to_lowercase() == "kb" {
            return 1024;
        }
        if result.to_lowercase() == "b" {
            return 0;
        }
    }
    return 1048576;
}

#[derive(Debug, Clone)]
pub enum TimeUnits {
    Seconds,
    Milliseconds,
}

pub fn get_time_units() -> TimeUnits {
    let result = env::var("PM_TIME_UNITS");
    if let Ok(result) = result {
        if result.to_lowercase() == "s" {
            return TimeUnits::Seconds;
        }
        if result.to_lowercase() == "ms" {
            return TimeUnits::Milliseconds;
        }
    }
    return TimeUnits::Milliseconds;
}


#[derive(Clone)]
pub struct Columns {
    pub cpu: bool,
    pub memory: bool,
    pub disk_write: bool,
    pub disk_read: bool,
}

impl Columns {
    pub fn new_string(&self, time: f64, cpu: u64, memory: u64, disk_write: u64, disk_read: u64) -> String {
        let mut output = Vec::<String>::new();

        output.push(format!("{}", time));

        if self.cpu {
            output.push(format!("{}", cpu));
        }
        if self.memory {
            output.push(format!("{}", memory));
        }
        if self.disk_write {
            output.push(format!("{}", disk_write));
        }
        if self.disk_read {
            output.push(format!("{}", disk_read));
        }

        return output.join(",")
    }

    pub fn get_header(&self) -> String {
        let mut output = Vec::<String>::new();

        output.push(String::from("time"));

        if self.cpu {
            output.push(String::from("cpu"));
        }
        if self.memory {
            output.push(String::from("memory"));
        }
        if self.disk_write {
            output.push(String::from("disk_write"));
        }
        if self.disk_read {
            output.push(String::from("disk_read"));
        }

        return output.join(",")
    }
}

pub fn get_columns() -> Columns {
    let result = env::var("PM_TRACK");
    if let Ok(result) = result {
        return Columns{
            cpu: result.contains("cpu"),
            memory: result.contains("memory"),
            disk_write: result.contains("disk_write"),
            disk_read: result.contains("disk_read"),
        };
    }
    return Columns{
        cpu: true,
        memory: true,
        disk_write: true,
        disk_read: true,
    };
}
