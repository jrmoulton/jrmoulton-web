fn main() {
    std::fs::create_dir_all("build/styles").unwrap();
    std::process::Command::new("sass")
        .arg("styles/common.scss")
        .arg("build/styles/common.css")
        .spawn()
        .unwrap();
}
