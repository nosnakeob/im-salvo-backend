use std::path::PathBuf;

pub fn path2module_path(path: &mut PathBuf) -> String {
    path.set_extension("");
    if path.file_stem() == Some("mod".as_ref()) {
        path.pop();
    };
    let path = match path.strip_prefix("src") {
        Ok(res) => res,
        Err(_) => path
    };
    path.iter().map(|x| x.to_string_lossy()).collect::<Vec<_>>().join("::")
}