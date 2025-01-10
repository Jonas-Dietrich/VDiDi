use std::fs;
use std::error::Error;

// Helper function to read a file
pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

// Helper function to choose a random line
pub fn choose_random_line(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let random_index = rand::random::<usize>() % lines.len();
    lines[random_index].to_string()
}
