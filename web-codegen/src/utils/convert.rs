use std::borrow::Cow;
use std::path::Component::Normal;
use std::path::PathBuf;

// 文件路径转模块路径
pub fn path2module_path(path: &PathBuf) -> String {
    let mut path = path.clone();
    path.set_extension("");
    if path.ends_with("mod") {
        path.pop();
    }

    let mut iter = path.components();

    // 忽略crate src
    if let Some(Normal(first)) = iter.next()
        && first.to_string_lossy().contains("-")
    {
        iter.next();
    }

    let mut modules = vec![];
    while let Some(Normal(name)) = iter.next() {
        modules.push(name.to_string_lossy());
    }

    modules.join("::")
}
