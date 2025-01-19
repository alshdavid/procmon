use core::time;
use std::env;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use clap::ValueEnum;
use serde::Serialize;

#[derive(Parser, Debug, Clone)]
pub struct Args {
  /// Command to run
  pub command: Vec<String>,

  /// Output path for the generated report
  #[arg(short = 'r', long = "report",  env = "PM_REPORT", value_parser = parse_file_path, default_value = "procmon")]
  pub report_path: PathBuf,

  /// How often to probe the process for details in milliseconds
  #[arg(short = 'i', long = "interval", env = "PM_INTERVAL", value_parser = parse_duration, default_value = "250")]
  pub poll_interval: time::Duration,

  /// What units to use for recording memory
  #[arg(
    short = 'm',
    long = "mem-units",
    env = "PM_MEM_UNITS",
    value_enum,
    default_value = "mb"
  )]
  pub mem_units: MemoryUnits,

  /// What units to use for recording time
  #[arg(
    short = 't',
    long = "time-units",
    env = "PM_TIME_UNITS",
    value_enum,
    default_value = "ms"
  )]
  pub time_units: TimeUnits,

  /// Override report file if exists
  #[arg(long = "no-override-report")]
  pub no_override_report: bool,

  /// Don't measure CPU usage
  #[arg(long = "no-cpu", env = "PM_NO_CPU")]
  pub no_measure_cpu: bool,

  /// Don't measure memory usage
  #[arg(long = "no-memory", env = "PM_NO_MEMORY")]
  pub no_measure_mem: bool,

  /// Don't measure disk usage
  #[arg(long = "no-disk", env = "PM_NO_DISK")]
  pub no_measure_disk: bool,
}

fn parse_file_path(arg: &str) -> Result<PathBuf, std::num::ParseIntError> {
  let mut target_path = PathBuf::from(arg);
  if target_path.is_relative() {
    target_path = env::current_dir().unwrap().join(target_path);
  }
  Ok(target_path)
}

fn parse_duration(arg: &str) -> Result<time::Duration, std::num::ParseIntError> {
  let seconds = arg.parse()?;
  Ok(std::time::Duration::from_millis(seconds))
}

#[derive(Default, Debug, Clone, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum MemoryUnits {
  #[default]
  Mb,
  Kb,
  B,
}

impl MemoryUnits {
  pub fn get_unit(&self) -> String {
    match self {
      MemoryUnits::Mb => "mb".to_string(),
      MemoryUnits::Kb => "kb".to_string(),
      MemoryUnits::B => "b".to_string(),
    }
  }
}

#[derive(Default, Debug, Clone, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum TimeUnits {
  S,
  #[default]
  Ms,
}

impl TimeUnits {
  pub fn to_f64(
    &self,
    t: Duration,
  ) -> f64 {
    match self {
      TimeUnits::S => t.as_millis() as f64 / 1000_f64,
      TimeUnits::Ms => t.as_millis() as f64,
    }
  }

  pub fn get_unit(&self) -> String {
    match self {
      TimeUnits::S => "s".to_string(),
      TimeUnits::Ms => "ms".to_string(),
    }
  }
}
