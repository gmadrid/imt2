use walkdir::DirEntry;

pub fn is_hidden(e: &DirEntry) -> bool {
    let name = e.path().file_name();
    name.map_or(false, |n| n.to_string_lossy().starts_with('.'))
}
