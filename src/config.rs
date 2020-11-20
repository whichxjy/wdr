use std::fs;
use std::path::Path;

lazy_static! {
    pub static ref DOWNLOADED_PATH: &'static Path = {
        let downloaded_path = Path::new("downloaded");
        fs::create_dir_all(downloaded_path).expect("Fail to create downloaded directory");
        downloaded_path
    };
}
