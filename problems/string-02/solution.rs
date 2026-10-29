pub fn join_path(dir: String, file: String) -> String {
    if dir.is_empty() {
        return file;
    }
    dir + "/" + &file
}
