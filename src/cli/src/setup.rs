pub fn setup() {
    let infralink_directory = dirs::home_dir().unwrap().join(".infralink");

    if !infralink_directory.exists() {
        std::fs::create_dir(&infralink_directory).unwrap();
    }
}
