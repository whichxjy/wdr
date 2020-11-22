use crossbeam::channel::Receiver;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::process::{Child, Command};
use std::str;
use std::thread;
use url::Url;

use crate::config::WORKSPACE_PATH;
use crate::model::ProcessConfig;

custom_error! {
    pub ProcessError
    Prepare = "Fail to prepare process",
    Run = "Fail to run process"
}

pub type ProcessResult<T> = Result<T, ProcessError>;

pub fn prepare(process_config: &ProcessConfig) -> ProcessResult<()> {
    wdr_info!("Start download from {}", process_config.resource);

    let url = match Url::parse(&process_config.resource) {
        Ok(url) => url,
        Err(err) => {
            wdr_error!("Invalid URL: {}", err);
            return Err(ProcessError::Prepare);
        }
    };

    let segments = url.path_segments().map(|c| c.collect::<Vec<_>>()).unwrap();
    let filename = match segments.last() {
        Some(filename) => filename,
        None => {
            wdr_error!("Fail to parse filename from {}", process_config.resource);
            return Err(ProcessError::Prepare);
        }
    };

    let full_path = WORKSPACE_PATH.join(filename);
    wdr_info!("Full path of target: {}", full_path.to_str().unwrap());

    let res = match reqwest::blocking::get(&process_config.resource) {
        Ok(res) => res,
        Err(err) => {
            wdr_error!("Fail to download: {}", err);
            return Err(ProcessError::Prepare);
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
            return Err(ProcessError::Prepare);
        }
    };

    let bytes = match res.bytes() {
        Ok(bytes) => bytes,
        Err(err) => {
            wdr_error!("Fail to read bytes from response: {}", err);
            return Err(ProcessError::Prepare);
        }
    };

    if let Err(err) = file.write_all(&bytes) {
        wdr_error!("Fail to write bytes to file: {}", err);
        return Err(ProcessError::Prepare);
    }

    wdr_info!("Process {} is ready now", process_config.name);

    Ok(())
}

pub fn run(process_config: ProcessConfig, stop_receiver: Receiver<()>) -> ProcessResult<()> {
    let log_path = WORKSPACE_PATH.join(format!("{}.log", process_config.name));

    let mut cmd_child = match run_cmd_in_workspace(&process_config.cmd, &log_path) {
        Ok(cmd_child) => cmd_child,
        _ => return Err(ProcessError::Run),
    };
    wdr_info!("Process {} is running", process_config.name);

    thread::spawn(move || {
        if stop_receiver.recv().is_ok() {
            match cmd_child.kill() {
                Ok(()) => wdr_info!("Process {} was killed", process_config.name),
                Err(err) => wdr_error!("Fail to kill {}: {}", process_config.name, err),
            };
        }
    });

    Ok(())
}

fn run_cmd_in_workspace<P: AsRef<Path>>(cmd: &str, log_path: P) -> ProcessResult<Child> {
    let (program, option) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let log_file = match OpenOptions::new().write(true).create(true).open(log_path) {
        Ok(log_file) => log_file,
        Err(err) => {
            wdr_error!("Fail to open log file: {}", err);
            return Err(ProcessError::Run);
        }
    };

    match Command::new(program)
        .current_dir(WORKSPACE_PATH.to_str().unwrap())
        .stdout(log_file)
        .args(&[option, cmd])
        .spawn()
    {
        Ok(cmd_child) => Ok(cmd_child),
        _ => Err(ProcessError::Run),
    }
}
