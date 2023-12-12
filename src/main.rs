mod cfg;

use csv::Writer;
use std::env;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use sysinfo::Pid;
use sysinfo::ProcessExt;
use sysinfo::System;
use sysinfo::SystemExt;

#[derive(serde::Serialize)]
struct Row {
    time: u128,
    memory: u64,
    cpu: u32,
    disk_read: u64,
    disk_write: u64,
}

fn main() {
    let report_file = cfg::get_report_file_path();
    let polling_interval = cfg::get_polling_interval();
    let target_pid = cfg::get_pid();
    let mem_units = cfg::get_mem_units();

    let mut handles = Vec::<JoinHandle<()>>::new();
    let duration: Arc<Mutex<Option<Duration>>> = Arc::new(Mutex::new(None));
    let (sender, receiver) = channel::<(u32, Option<Duration>)>();

    let wtr = Arc::new(Mutex::new(Writer::from_writer(report_file.writer())));

    if report_file.to_string() != "stdout" {
        println!("[procmon] Polling:  {:?}", polling_interval);
        println!("[procmon] Report:   {}", report_file.to_string());
    }

    // Monitor
    {
        let report_file = report_file.clone();
        let wtr = wtr.clone();

        handles.push(thread::spawn(move || {
            let (pid, start_time) = receiver.recv().unwrap();
            // Please note that we use "new_all" to ensure that all list of
            // components, network interfaces, disks and users are already filled!

            if report_file.to_string() != "stdout" {
                println!("[procmon] PID:      {}", pid);
            }

            let mut sys = System::new_all();
            let pid = Pid::from(pid as usize);
            let mut wtr = wtr.lock().unwrap();

            while sys.refresh_process(pid) {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Can't get the time");

                let p = sys.process(pid).expect("Can't get process info");
                
                let disk = p.disk_usage();
                let memory = p.memory() / mem_units as u64;
                let cpu = (p.cpu_usage() * 100.0).round() as u32;

                let start_time_ms = match start_time {
                    Some(t) => t.as_millis(),
                    None => (p.start_time() * 1000) as u128,
                };
                
                let run_time = if polling_interval.as_millis() > 1000 {
                    now.as_millis() - start_time_ms / 1000
                } else {
                    now.as_millis() - start_time_ms
                };

                wtr.serialize(Row {
                    time: run_time,
                    memory,
                    cpu,
                    disk_read: disk.read_bytes,
                    disk_write: disk.written_bytes,
                })
                .expect("Failed serializing row");

                wtr.flush().expect("Can't flush CSV");
                thread::sleep(polling_interval);
            }
        }));
    }
    
    // Process
    {
        let duration = duration.clone();

        handles.push(thread::spawn(move || {
            if let Some(pid) = target_pid {
                println!("[procmon] Using existing PID");
                sender.send((pid, None)).unwrap();
                return;
            }

            let args: Vec<String> = env::args().collect();
            let command_segments_raw = &args[1..];
            let mut segments: Vec<String> = command_segments_raw.to_vec();
            
            let first = segments.remove(0);

            let mut command = Command::new(first);
            command.args(segments);

            command.current_dir(env::current_dir().unwrap());

            command.stdout(Stdio::inherit());
            command.stdin(Stdio::inherit());
            command.stderr(Stdio::inherit());


            let start_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Can't get the time");

            let mut child = command.spawn().unwrap();
            sender.send((child.id(), Some(start_time))).unwrap();
            child.wait().unwrap();

            let end_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Can't get the time");

            let time_elapsed = end_time - start_time;
            let mut d = duration.lock().unwrap();
            *d = Some(time_elapsed);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = duration.lock().unwrap().unwrap();
    let total_duration = if polling_interval.as_millis() > 1000 {
        duration.as_secs() as u128
    } else {
        duration.as_millis() as u128
    };

    wtr.lock().unwrap().serialize(Row {
        time: total_duration,
        memory: 0,
        cpu: 0 as u32,
        disk_read: 0,
        disk_write: 0,
    })
    .expect("Failed serializing row");

    if report_file.to_string() == "stdout" {
        return;
    }

    println!("[procmon] Report:   {}", report_file.to_string());

    if duration.as_secs() < 10 {
        println!("[procmon] Duration: {}ms", duration.as_millis());
    } else {
        println!("[procmon] Duration: {}s", duration.as_secs());
    }
}
