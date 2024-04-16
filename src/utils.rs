use camino::{Utf8Path, Utf8PathBuf};

pub fn home_path<P: AsRef<Utf8Path>>(path: P) -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(dirs::home_dir().expect("Home Directory Required"))
        .expect("Conversion Expected")
        .join(path)
}

pub fn homebrew_path<P: AsRef<Utf8Path>>(path: P) -> Utf8PathBuf {
    Utf8PathBuf::from("/opt/homebrew/bin").join(path)
}