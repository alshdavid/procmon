#![deny(unused_crate_dependencies)]

mod args;
mod plot;
mod reporter;

use clap::Parser;
use plot::generate_plot;
use std::env;
use std::process::Command;
use std::process::Stdio;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use sysinfo::Pid;
use sysinfo::ProcessExt;
use sysinfo::System;
use sysinfo::SystemExt;

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

      while sys.refresh_process(pid) {
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
        time: start_time,
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

  generate_plot(&args.report_path, &reporter.rows.read().unwrap()).unwrap();
}

// fn main() {
//   let report_file = cfg::get_report_file_path();
//   let polling_interval = cfg::get_polling_interval();
//   let target_pid = cfg::get_pid();
//   let mem_units = cfg::get_mem_units();
//   let time_units = cfg::get_time_units();
//   let columns = cfg::get_columns();

//   let mut handles = Vec::<JoinHandle<()>>::new();
//   let duration: Arc<Mutex<Option<Duration>>> = Arc::new(Mutex::new(None));
//   let (sender, receiver) = channel::<(u32, Option<Duration>)>();

//   let buffer = Arc::new(Mutex::new(Vec::<String>::new()));

//   if report_file.to_string() != "stdout" {
//     println!("[procmon] Polling:  {:?}", polling_interval);
//     println!("[procmon] Report:   {}", report_file.to_string());
//   }

//   // Monitor
//   {
//     let report_file = report_file.clone();
//     let buffer = buffer.clone();
//     let columns = columns.clone();
//     let time_units = time_units.clone();

//     handles.push(thread::spawn(move || {
//       let (pid, start_time) = receiver.recv().unwrap();
//       // Please note that we use "new_all" to ensure that all list of
//       // components, network interfaces, disks and users are already filled!

//       if report_file.to_string() != "stdout" {
//         println!("[procmon] PID:      {}", pid);
//       }

//       let mut sys = System::new_all();
//       let pid = Pid::from(pid as usize);

//       buffer.lock().unwrap().push(columns.get_header());

//       while sys.refresh_process(pid) {
//         let now = SystemTime::now()
//           .duration_since(UNIX_EPOCH)
//           .expect("Can't get the time");

//         let p = sys.process(pid).expect("Can't get process info");

//         let disk = p.disk_usage();
//         let memory = p.memory() / mem_units as u64;
//         let cpu = (p.cpu_usage()).round() as u64;

//         let start_time_ms = match start_time {
//           Some(t) => t.as_millis(),
//           None => (p.start_time() * 1000) as u128,
//         };

//         let run_time = match time_units {
//           cfg::TimeUnits::Seconds => (now.as_millis() - start_time_ms) as f64 / 1000 as f64,
//           cfg::TimeUnits::Milliseconds => (now.as_millis() - start_time_ms) as f64,
//         };

//         buffer.lock().unwrap().push(columns.new_string(
//           run_time,
//           cpu,
//           memory,
//           disk.written_bytes,
//           disk.read_bytes,
//         ));

//         thread::sleep(polling_interval);
//       }
//     }));
//   }

//   // Process
//   {
//     let duration = duration.clone();

//     handles.push(thread::spawn(move || {
//       if let Some(pid) = target_pid {
//         println!("[procmon] Using existing PID");
//         sender.send((pid, None)).unwrap();
//         return;
//       }

//       let args: Vec<String> = env::args().collect();
//       let command_segments_raw = &args[1..];
//       let mut segments: Vec<String> = command_segments_raw.to_vec();

//       let first = segments.remove(0);

//       let mut command = Command::new(first);
//       command.args(segments);

//       command.current_dir(env::current_dir().unwrap());

//       command.stdout(Stdio::inherit());
//       command.stdin(Stdio::inherit());
//       command.stderr(Stdio::inherit());

//       let start_time = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .expect("Can't get the time");

//       let mut child = command.spawn().unwrap();
//       sender.send((child.id(), Some(start_time))).unwrap();
//       child.wait().unwrap();

//       let end_time = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .expect("Can't get the time");

//       let time_elapsed = end_time - start_time;
//       let mut d = duration.lock().unwrap();
//       *d = Some(time_elapsed);
//     }));
//   }

//   for handle in handles {
//     handle.join().unwrap();
//   }

//   let duration = duration.lock().unwrap().unwrap();
//   let total_duration = match time_units {
//     cfg::TimeUnits::Seconds => duration.as_secs() as f64,
//     cfg::TimeUnits::Milliseconds => duration.as_millis() as f64,
//   };

//   buffer
//     .lock()
//     .unwrap()
//     .push(columns.new_string(total_duration as f64, 0, 0, 0, 0));
//   fs::write(report_file.to_string(), buffer.lock().unwrap().join("\n")).unwrap();

//   println!("[procmon] Report:   {}", report_file.to_string());

//   if duration.as_secs() < 10 {
//     println!("[procmon] Duration: {}ms", duration.as_millis());
//   } else {
//     println!("[procmon] Duration: {}s", duration.as_secs());
//   }
// }
