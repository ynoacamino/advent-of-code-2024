pub mod utils {
    pub fn read_input(path: &str) -> Vec<String> {
        let current_path = std::env::current_dir().unwrap();
        let file_path = current_path.join("src/inputs/").join(path);

        println!("Reading file: {:?}", file_path);

        let input = std::fs::read_to_string(file_path).unwrap();
        input.lines().map(|s| s.to_string()).collect()
    }
}
