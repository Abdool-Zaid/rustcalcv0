pub fn return_input() -> String {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => line.trim().to_owned(),
        Err(error) => panic!("Failed to read input: {}", error),
    }
}
