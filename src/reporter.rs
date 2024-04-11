use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;

use crate::args::Args;
use crate::args::MemoryUnits;

#[derive(Debug)]
pub struct Columns {
  pub memory: bool,
  pub cpu: bool,
  pub disk: bool,
}

#[derive(Default, Debug)]
pub struct Row {
  pub time: Duration,
  pub memory: Option<u64>,
  pub cpu: Option<u64>,
  pub disk_read: Option<u64>,
  pub disk_write: Option<u64>,
}

pub struct Reporter {
  report_file: File,
  columns: Columns,
  args: Args,
  pub rows: RwLock<Vec<Row>>,
}

impl Reporter {
  pub fn new(
    report_folder: &Path,
    override_report: &bool,
    columns: Columns,
    args: Args,
  ) -> Result<Arc<Self>, String> {
    if report_folder.exists() {
      if *override_report {
        fs::remove_dir_all(report_folder).unwrap();
      } else {
        return Err(format!(
          "Report folder {} already exists",
          report_folder.to_str().unwrap()
        ));
      }
    }

    fs::create_dir(report_folder).unwrap();

    let report_file = OpenOptions::new()
      .write(true)
      .append(true)
      .create_new(true)
      .open(report_folder.join("report.csv"))
      .unwrap();

    let mut header = vec![format!("time_{}", args.time_units.get_unit())];
    if columns.cpu {
      header.push("cpu".to_string());
    }
    if columns.memory {
      header.push(format!("memory_{}", args.mem_units.get_unit()));
    }
    if columns.disk {
      header.push("disk_read".to_string());
      header.push("disk_write".to_string());
    }

    let reporter = Self {
      report_file,
      columns,
      args,
      rows: RwLock::new(vec![]),
    };

    reporter.write_to_report_file(&header);

    Ok(Arc::new(reporter))
  }

  pub fn write(
    &self,
    row: Row,
  ) {
    let mut line = vec![];

    line.push({
      match self.args.time_units {
        crate::args::TimeUnits::S => format!("{:.3}", self.args.time_units.to_f64(row.time)),
        crate::args::TimeUnits::Ms => format!("{}", row.time.as_millis()),
      }
    });

    if self.columns.cpu {
      if let Some(cpu) = row.cpu {
        line.push(format!("{}", cpu));
      }
    }

    if self.columns.memory {
      if let Some(memory) = row.memory {
        match self.args.mem_units {
          MemoryUnits::Mb => {
            line.push(format!("{}", memory / 1048576 as u64));
          }
          MemoryUnits::Kb => {
            line.push(format!("{}", memory / 1024 as u64));
          }
          MemoryUnits::B => {
            line.push(format!("{}", memory));
          }
        }
      }
    }

    if self.columns.disk {
      if let Some(disk_read) = row.disk_read {
        line.push(format!("{}", disk_read));
      }
      if let Some(disk_write) = row.disk_write {
        line.push(format!("{}", disk_write));
      }
    }

    self.rows.write().unwrap().push(row);
    self.write_to_report_file(&line);
  }

  fn write_to_report_file(
    &self,
    text: &[String],
  ) {
    let mut output = String::new();
    for (i, col) in text.iter().enumerate() {
      output.push_str(&format!("{}", col));
      if i != text.len() - 1 {
        output.push_str(&format!(","));
      }
    }
    writeln!(&self.report_file, "{}", output).expect("Unable to write to file");
  }
}
