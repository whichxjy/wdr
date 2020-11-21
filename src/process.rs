use crate::config::WORKSPACE_PATH;
use crate::model::Resource;
use std::fs::OpenOptions;
use std::io::{Result as IOResult, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::process::{Child, Command};
use std::str;
use url::Url;

fn run_cmd_in_workspace(cmd: &str) -> IOResult<Child> {
    let (program, option) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    Command::new(program)
        .current_dir(WORKSPACE_PATH.to_str().unwrap())
        .args(&[option, cmd])
        .spawn()
}

custom_error! {
    pub ProcessError
    Prepare = "Fail to prepare process",
    Run = "Fail to run process"
}

pub type ProcessResult<T> = Result<T, ProcessError>;

pub struct Process<'a> {
    name: &'a str,
    resource: &'a Resource,
    cmd_child: Option<Child>,
}

impl<'a> Process<'a> {
    pub fn new(name: &'a str, resource: &'a Resource) -> Self {
        Process {
            name,
            resource,
            cmd_child: None,
        }
    }

    pub fn prepare(&self) -> ProcessResult<()> {
        wdr_info!("Start download from {}", self.resource.link);

        let url = match Url::parse(&self.resource.link) {
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
                wdr_error!("Fail to parse filename from {}", self.resource.link);
                return Err(ProcessError::Prepare);
            }
        };

        let full_path = WORKSPACE_PATH.join(filename);
        wdr_info!("Full path of target: {}", full_path.to_str().unwrap());

        let res = match reqwest::blocking::get(&self.resource.link) {
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

        wdr_info!("{} is prepared now", self.name);

        Ok(())
    }

    pub fn run(&mut self) -> ProcessResult<()> {
        match run_cmd_in_workspace("echo what") {
            Ok(cmd_child) => {
                self.cmd_child = Some(cmd_child);
                Ok(())
            }
            _ => Err(ProcessError::Run),
        }
    }

    pub fn kill(&mut self) {
        let cmd_child = match &mut self.cmd_child {
            Some(cmd_child) => cmd_child,
            None => {
                wdr_warn!("No cmd child for {}", self.name);
                return;
            }
        };

        match cmd_child.kill() {
            Ok(()) => wdr_info!("Process {} was killed", self.name),
            Err(err) => wdr_error!("Fail to kill {}: {}", self.name, err),
        };
    }
}
