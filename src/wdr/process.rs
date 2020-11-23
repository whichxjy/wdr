use crossbeam::channel::{Receiver, Sender};
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::process::{Child, Command};
use std::str;
use std::thread;
use url::Url;
use wdrlib::model::ProcessConfig;

use crate::config::WORKSPACE_PATH;

pub struct Process {
    pub config: ProcessConfig,
    pub stop_receiver: Receiver<()>,
    pub stop_done_sender: Sender<()>,
}

pub fn prepare(process_config: &ProcessConfig) -> Option<()> {
    wdr_info!("Start download from {}", process_config.resource);

    let url = match Url::parse(&process_config.resource) {
        Ok(url) => url,
        Err(err) => {
            wdr_error!("Invalid URL: {}", err);
            return None;
        }
    };

    let segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
    let filename = match segments.last() {
        Some(filename) => filename,
        None => {
            wdr_error!("Fail to parse filename from {}", process_config.resource);
            return None;
        }
    };

    let full_path = WORKSPACE_PATH.join(filename);
    wdr_info!("Local resource path: {}", full_path.to_str().unwrap());

    let res = match reqwest::blocking::get(&process_config.resource) {
        Ok(res) => res,
        Err(err) => {
            wdr_error!("Fail to download: {}", err);
            return None;
        }
    };

    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o777)
        .open(&full_path)
    {
        Ok(file) => file,
        Err(err) => {
            wdr_error!("Fail to open file {}: {}", filename, err);
            return None;
        }
    };

    let bytes = match res.bytes() {
        Ok(bytes) => bytes,
        Err(err) => {
            wdr_error!("Fail to read bytes from response: {}", err);
            return None;
        }
    };

    if let Err(err) = file.write_all(&bytes) {
        wdr_error!("Fail to write bytes to file: {}", err);
        return None;
    }

    wdr_info!("Process {} is ready now", process_config.name);

    Some(())
}

pub fn run(process: Process) -> Option<()> {
    let log_path = WORKSPACE_PATH.join(format!("{}.log", process.config.name));

    let mut cmd_child = match run_cmd_in_workspace(&process.config.cmd, &log_path) {
        Some(cmd_child) => cmd_child,
        None => return None,
    };
    wdr_info!("Process {} is running", process.config.name);

    thread::spawn(move || {
        if process.stop_receiver.recv().is_ok() {
            match cmd_child.kill() {
                Ok(()) => wdr_info!("Process {} was killed", process.config.name),
                Err(err) => wdr_error!("Fail to kill {}: {}", process.config.name, err),
            };
        }
        process.stop_done_sender.send(()).unwrap()
    });

    Some(())
}

fn run_cmd_in_workspace<P: AsRef<Path>>(cmd: &str, log_path: P) -> Option<Child> {
    let (program, option) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let log_file = match OpenOptions::new().write(true).create(true).open(log_path) {
        Ok(log_file) => log_file,
        Err(err) => {
            wdr_error!("Fail to open log file: {}", err);
            return None;
        }
    };

    match Command::new(program)
        .current_dir(WORKSPACE_PATH.to_str().unwrap())
        .stdout(log_file)
        .args(&[option, cmd])
        .spawn()
    {
        Ok(cmd_child) => Some(cmd_child),
        _ => None,
    }
}
