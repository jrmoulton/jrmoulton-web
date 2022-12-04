fn main() {
    std::process::Command::new("sass")
        .arg("styles/common.scss")
        .arg("build/common.css")
        .spawn()
        .unwrap();
}
