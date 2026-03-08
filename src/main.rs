#![deny(unused_crate_dependencies)]

mod args;
mod reporter;

use clap::Parser;
use std::env;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use sysinfo::Pid;
use sysinfo::ProcessesToUpdate;
use sysinfo::System;

use crate::args::Args;
use crate::reporter::Columns;
use crate::reporter::Reporter;
use crate::reporter::Row;

fn main() {
  let args = Args::parse();

  let reporter = match Reporter::new(
    &args.report_path,
    &!args.no_override_report,
    Columns {
      cpu: !args.no_measure_cpu,
      memory: !args.no_measure_mem,
      disk: !args.no_measure_disk,
    },
    args.clone(),
  ) {
    Ok(v) => v,
    Err(msg) => {
      println!("Error: {}", msg);
      return;
    }
  };

  let (sender, receiver) = channel::<(u32, Duration)>();

  // Monitor
  let h0 = {
    let command = args.clone();
    let reporter = reporter.clone();

    thread::spawn(move || {
      let (pid, start_time) = receiver.recv().unwrap();
      let pid = Pid::from(pid as usize);

      // Please note that we use "new_all" to ensure that all list of
      // components, network interfaces, disks and users are already filled!
      let mut sys = System::new_all();

      loop {
        if sys.refresh_processes(ProcessesToUpdate::Some(&[pid]), true) == 0 {
          break;
        }
        let now = SystemTime::now()
          .duration_since(UNIX_EPOCH)
          .expect("Can't get the time");

        let p = sys.process(pid).expect("Can't get process info");
        let mut row = Row::default();

        if !command.no_measure_cpu {
          let cpu = p.cpu_usage();
          row.cpu = Some(cpu.round() as u64);
        }

        if !command.no_measure_mem {
          let memory = p.memory();
          row.memory = Some(memory);
        }

        if !command.no_measure_disk {
          let disk = p.disk_usage();
          row.disk_read = Some(disk.read_bytes);
          row.disk_write = Some(disk.written_bytes);
        }

        row.time = now - start_time;

        reporter.write(row);
        thread::sleep(command.poll_interval);
      }
    })
  };

  // Process
  let h1 = {
    let mut args = args.clone();
    let reporter = reporter.clone();

    thread::spawn(move || {
      let mut command = {
        let first = args.command.remove(0);

        let mut command = Command::new(first);
        command.args(args.command);

        command.current_dir(env::current_dir().unwrap());

        command.stdout(Stdio::inherit());
        command.stdin(Stdio::inherit());
        command.stderr(Stdio::inherit());
        command
      };

      let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Can't get the time");

      reporter.write(Row {
        time: Duration::from_millis(0),
        memory: Some(0),
        cpu: Some(0),
        disk_read: Some(0),
        disk_write: Some(0),
      });

      let mut child = command.spawn().unwrap();
      sender.send((child.id(), start_time)).unwrap();
      child.wait().unwrap();

      let end_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Can't get the time");

      reporter.write(Row {
        time: end_time - start_time,
        memory: Some(0),
        cpu: Some(0),
        disk_read: Some(0),
        disk_write: Some(0),
      });
    })
  };

  h1.join().unwrap();
  h0.join().unwrap();
}
