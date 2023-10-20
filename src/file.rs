use std::{ffi::OsStr, path::Path};

//TODO: Implement dynamic path for config file
pub fn get_config_path() -> &'static OsStr {
    // ~/.envn
    let path = std::path::Path::new("/home/noobscience/.envn");
    if !path.exists() {
        let _ = std::fs::create_dir::<_>(path);
    }

    path.as_os_str()
}

pub fn file_exists(path: &Path) -> bool {
    path.exists()
}

pub fn write_file(path: &Path, content: String) -> bool {
    //TODO: Add warning if file exists
    let _ = std::fs::write(path, content);
    return true;
}
