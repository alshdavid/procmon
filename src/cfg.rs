use std::{env, io};
use std::fs::File;
use std::io::Write;
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

    pub fn writer(&self) -> Box<dyn Write + Send + Sync> {
        match self {
            ReportPath::PathBuf(p) => {
                return Box::new(File::create(&p).unwrap());
            },
            ReportPath::Stdout => {
                return Box::new(io::stdout());
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
