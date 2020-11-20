use std::fs;
use std::path::Path;

lazy_static! {
    pub static ref DOWNLOADED_PATH: &'static Path = {
        let downloaded_path = Path::new("downloaded");
        let _ = fs::create_dir_all(downloaded_path);
        downloaded_path
    };
}
