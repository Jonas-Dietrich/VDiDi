use crate::util::choose_random_line;
use crate::util::read_file;
use crate::util::choose_multiple_random_lines;

pub fn generate_random_mail() -> Result<String, Box<dyn std::error::Error>> {

    let content_lines = read_file("./data/content.txt")?;
    let subject_lines = read_file("./data/subjects.txt")?;
    let greeting_lines = read_file("./data/greetings.txt")?;
    let ending_lines = read_file("./data/endings.txt")?;

    let mut full_mail : String = String::new();

    let subject = choose_random_line(&subject_lines);

    let greeting = choose_random_line(&greeting_lines);

    let ending = choose_random_line(&ending_lines);

    let content = choose_multiple_random_lines(&content_lines, 10);

    full_mail.push_str(&subject);
    full_mail.push_str("\n");
    full_mail.push_str("\n");
    full_mail.push_str(&greeting);
    full_mail.push_str("\n");
    for line in content {
        full_mail.push_str(&line);
        full_mail.push_str("\n");
    }
    full_mail.push_str("\n");
    full_mail.push_str(&ending);

    
    Ok(full_mail)
}
