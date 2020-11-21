use crate::config::WORKSPACE_PATH;
use crate::model::Resource;
use reqwest::blocking::Client as HttpClient;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::process::Command;
use std::str;
use url::Url;

macro_rules! run_cmd_in_workspace {
    ($x:expr) => {
        let (program, option) = if cfg!(target_os = "windows") {
            ("cmd", "/C")
        } else {
            ("sh", "-c")
        };

        Command::new(program)
            .current_dir(WORKSPACE_PATH.to_str().unwrap())
            .args(&[option, $x])
            .spawn()
            .expect("failed to execute process")
    };
}

custom_error! {
    pub ProcessError
    Prepare = "Fail to prepare process",
}

pub type ProcessResult<T> = Result<T, ProcessError>;

pub struct Process<'a> {
    resource: &'a Resource,
}

impl<'a> Process<'a> {
    pub fn new(resource: &'a Resource) -> Self {
        Process { resource }
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
        wdr_info!("Full path: {}", full_path.to_str().unwrap());

        let http_client = HttpClient::new();

        let res = match http_client.get(&self.resource.link).send() {
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

        Ok(())
    }

    pub fn run(&self) {
        run_cmd_in_workspace!("echo what");
    }
}
