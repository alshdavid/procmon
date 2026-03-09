use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;
use std::thread::JoinHandle;
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

#[derive(Clone)]
pub struct Reporter {
  tx_writer: Sender<Row>,
}

impl Reporter {
  pub fn new(
    report_file: &Path,
    override_report: &bool,
    columns: Columns,
    args: Args,
  ) -> anyhow::Result<Self> {
    if !override_report && std::fs::exists(report_file)? {
      return Err(anyhow::anyhow!("Report {:?} already exists", report_file));
    }

    if std::fs::exists(report_file)? {
      std::fs::remove_file(report_file)?;
    }

    if let Some(parent_dir) = report_file.parent()
      && !std::fs::exists(parent_dir)?
    {
      std::fs::create_dir_all(parent_dir)?;
    }

    let mut report_file = OpenOptions::new()
      .append(true)
      .create_new(true)
      .open(report_file)?;

    let (tx_writer, rx_write) = channel::<Row>();

    // Sender will fail if the thread dies, don't worry about propagating error message for now
    let _h0: JoinHandle<anyhow::Result<()>> = std::thread::spawn(move || {
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

      report_file.write_all(format!("{}\n", header.join(",")).as_bytes())?;

      while let Ok(row) = rx_write.recv() {
        let mut line = vec![];

        line.push({
          match args.time_units {
            crate::args::TimeUnits::S => format!("{:.3}", args.time_units.to_f64(row.time)),
            crate::args::TimeUnits::Ms => format!("{}", row.time.as_millis()),
          }
        });

        if columns.cpu
          && let Some(cpu) = row.cpu {
            line.push(format!("{}", cpu));
          }

        if columns.memory
          && let Some(memory) = row.memory {
            match args.mem_units {
              MemoryUnits::Mb => {
                line.push(format!("{}", memory / 1048576_u64));
              }
              MemoryUnits::Kb => {
                line.push(format!("{}", memory / 1024_u64));
              }
              MemoryUnits::B => {
                line.push(format!("{}", memory));
              }
            }
          }

        if columns.disk {
          if let Some(disk_read) = row.disk_read {
            line.push(format!("{}", disk_read));
          }
          if let Some(disk_write) = row.disk_write {
            line.push(format!("{}", disk_write));
          }
        }

        let mut output = String::new();
        for (i, col) in line.iter().enumerate() {
          output.push_str(&col.to_string());
          if i != line.len() - 1 {
            output.push(',');
          }
        }

        report_file.write_all(format!("{}\n", output).as_bytes())?;
      }

      Ok(())
    });

    Ok(Self { tx_writer })
  }

  pub fn write(
    &self,
    row: Row,
  ) -> anyhow::Result<()> {
    Ok(self.tx_writer.send(row)?)
  }
}
