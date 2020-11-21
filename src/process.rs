use std::fs::{File, OpenOptions};
use std::io::{Result as IOResult, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::process::{Child, Command};
use std::str;
use url::Url;

use crate::config::WORKSPACE_PATH;
use crate::model::ProcessConfig;

fn run_cmd_in_workspace(cmd: &str, log_file: File) -> IOResult<Child> {
    let (program, option) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    Command::new(program)
        .current_dir(WORKSPACE_PATH.to_str().unwrap())
        .stdout(log_file)
        .args(&[option, cmd])
        .spawn()
}

custom_error! {
    pub ProcessError
    Prepare = "Fail to prepare process",
    Run = "Fail to run process"
}

pub type ProcessResult<T> = Result<T, ProcessError>;

pub struct Process {
    pub config: ProcessConfig,
    cmd_child: Option<Child>,
}

impl Process {
    pub fn new(config: ProcessConfig) -> Self {
        Process {
            config,
            cmd_child: None,
        }
    }

    pub fn prepare(&self) -> ProcessResult<()> {
        wdr_info!("Start download from {}", self.config.resource);

        let url = match Url::parse(&self.config.resource) {
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
                wdr_error!("Fail to parse filename from {}", &self.config.resource);
                return Err(ProcessError::Prepare);
            }
        };

        let full_path = WORKSPACE_PATH.join(filename);
        wdr_info!("Full path of target: {}", full_path.to_str().unwrap());

        let res = match reqwest::blocking::get(&self.config.resource) {
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

        wdr_info!("Process {} is ready now", self.config.name);

        Ok(())
    }

    pub fn run(&mut self) -> ProcessResult<()> {
        let log_path = WORKSPACE_PATH.join(format!("{}.log", self.config.name));

        let log_file = match OpenOptions::new().write(true).create(true).open(&log_path) {
            Ok(log_file) => log_file,
            Err(err) => {
                wdr_error!(
                    "Fail to open log file {}: {}",
                    log_path.to_str().unwrap(),
                    err
                );
                return Err(ProcessError::Run);
            }
        };

        match run_cmd_in_workspace(&self.config.cmd, log_file) {
            Ok(cmd_child) => {
                self.cmd_child = Some(cmd_child);
                wdr_info!("Process {} is running", self.config.name);
                Ok(())
            }
            _ => Err(ProcessError::Run),
        }
    }

    pub fn stop(&mut self) {
        let cmd_child = match &mut self.cmd_child {
            Some(cmd_child) => cmd_child,
            None => {
                wdr_warn!("No cmd child for {}", self.config.name);
                return;
            }
        };

        match cmd_child.kill() {
            Ok(()) => wdr_info!("Process {} was killed", self.config.name),
            Err(err) => wdr_error!("Fail to kill {}: {}", self.config.name, err),
        };
    }
}
