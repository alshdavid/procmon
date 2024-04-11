use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug)]
pub struct Columns {
  pub memory: bool,
  pub cpu: bool,
  pub disk: bool,
}

#[derive(Default, Debug)]
pub struct Row {
  pub time: f64,
  pub memory: Option<u64>,
  pub cpu: Option<u64>,
  pub disk_read: Option<u64>,
  pub disk_write: Option<u64>,
}

pub struct Reporter {
  report_file: File,
  columns: Columns,
  pub rows: RwLock<Vec<Row>>,
}

impl Reporter {
  pub fn new(
    report_file: &Path,
    override_report_file: &bool,
    columns: Columns,
    mem_unit: &str,
    time_unit: &str,
  ) -> Result<Arc<Self>, String> {
    if report_file.exists() {
      if *override_report_file {
        fs::remove_file(report_file).unwrap();
      } else {
        return Err(format!(
          "Report file {} already exists",
          report_file.to_str().unwrap()
        ));
      }
    }
    let report_file = OpenOptions::new()
      .write(true)
      .append(true)
      .create_new(true)
      .open(report_file)
      .unwrap();

    let mut header = vec![format!("time_{}", time_unit)];
    if columns.cpu {
      header.push("cpu".to_string());
    }
    if columns.memory {
      header.push(format!("memory_{}", mem_unit));
    }
    if columns.disk {
      header.push("disk_read".to_string());
      header.push("disk_write".to_string());
    }

    let reporter = Self {
      report_file,
      columns,
      rows: RwLock::new(vec![]),
    };

    reporter.write_to_report_file(&header);

    Ok(Arc::new(reporter))
  }

  pub fn write(
    &self,
    row: Row,
  ) {
    let mut line = vec![format!("{:.3}", row.time)];

    if self.columns.cpu {
      if let Some(cpu) = row.cpu {
        line.push(format!("{}", cpu));
      }
    }

    if self.columns.memory {
      if let Some(memory) = row.memory {
        line.push(format!("{}", memory));
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
