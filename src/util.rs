use std::fs;
use std::error::Error;

// Helper function to read a file
pub fn read_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

// Helper function to choose a random line
pub fn choose_random_line(content: &str) -> String {
    let lines: Vec<&str> = content.lines()
        .collect();
    let random_index = rand::random::<usize>() % lines.len();
    lines[random_index].to_string().replace("\\n", "\n")
}

// Helper function to choose multiple random, non repeating lines
pub fn choose_multiple_random_lines(content: &str, line_count: i32) -> Vec<String> {
    let mut used_lines_indicies : Vec<usize> = Vec::new();
    let mut chosen_lines : Vec<String> = Vec::new();

    let lines : Vec<&str> = content.lines().collect();

    for _i in 0..line_count {
        let mut random_index : usize = rand::random::<usize>() % lines.len();

        while used_lines_indicies.contains(&random_index) {
            random_index = rand::random::<usize>() % lines.len();
        }

        used_lines_indicies.push(random_index);
        chosen_lines.push(lines[random_index].to_string().replace("\\n", "\n"));

    }

    chosen_lines
}
