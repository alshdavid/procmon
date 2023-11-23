use csv::Writer;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::channel;
use std::thread;
use std::time;
use std::time::Duration;
use std::time::Instant;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use sysinfo::Pid;
use sysinfo::ProcessExt;
use sysinfo::System;
use sysinfo::SystemExt;

#[derive(serde::Serialize)]
struct Row {
    time: u64,
    memory: u64,
    cpu: f32,
    disk_read: u64,
    disk_write: u64,
}

fn main() {
    let file_name = chrono::offset::Local::now().format("%Y%m%d-%H%M%S.csv").to_string();
    let mut report_file = env::current_dir().unwrap().join(PathBuf::from(file_name));
    let duration: Arc<Mutex<Option<Duration>>> = Arc::new(Mutex::new(None));

    let report_result = env::var("PM_REPORT");
    if let Ok(report) = report_result {
        report_file = PathBuf::from(report);
        if report_file.is_relative() {
            report_file = env::current_dir().unwrap().join(report_file);
        }
    };
    let report_file_path = report_file.to_str().unwrap().to_string();
    println!("[procmon] Report:   {}", report_file_path);

    let (s, r) = channel::<u32>();

    let h0 = thread::spawn(move || {
        let pid = r.recv().unwrap();
        // Please note that we use "new_all" to ensure that all list of
        // components, network interfaces, disks and users are already filled!

        println!("[procmon] PID:      {}", pid);

        let one_second = time::Duration::from_secs(1);
        let mut sys = System::new_all();

        let mut wtr = Writer::from_path(report_file.clone()).expect("Can't create CSV writer");
        let pid = Pid::from(pid as usize);

        while sys.refresh_process(pid) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Can't get the time");
            let p = sys.process(pid).expect("Can't get process info");

            let disk = p.disk_usage();
            let run_time = now.as_secs() - p.start_time();

            wtr.serialize(Row {
                time: run_time,
                memory: p.memory(),
                cpu: p.cpu_usage(),
                disk_read: disk.read_bytes,
                disk_write: disk.written_bytes,
            })
            .expect("Failed serializing row");

            wtr.flush().expect("Can't flush CSV");
            thread::sleep(one_second);
        }
    });
    
    let duration_thread = duration.clone();
    let h1 = thread::spawn(move || {
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


        let start = Instant::now();

        let mut child = command.spawn().unwrap();
        s.send(child.id()).unwrap();
        child.wait().unwrap();
        
        let mut d = duration_thread.lock().unwrap();
        *d = Some(start.elapsed());
    });

    h1.join().unwrap();
    h0.join().unwrap();

    println!("[procmon] Report:   {}", report_file_path);
    println!("[procmon] Duration: {}s", duration.lock().unwrap().unwrap().as_secs());
}
