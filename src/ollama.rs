use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use crate::util::read_file;
use crate::util::choose_random_line;

pub async fn generate_mail() -> Result<String, Box<dyn std::error::Error>> {
    // Reading the data
    let greetings = read_file("./data/greetings.txt")?;
    let subjects = read_file("./data/subjects.txt")?;
    let endings = read_file("./data/endings.txt")?;

    // Choosing a random line from the data
    let greeting = choose_random_line(&greetings);
    let subject = choose_random_line(&subjects);
    let ending = choose_random_line(&endings);

    // Generate a new ollama Client and open a connection
    let ollama = Ollama::new("http://localhost".to_string(), 11434);
    let model = "dolphin-mistral:latest".to_string();

    let prompt = format!(
        "Schreibe eine unformale, kurze E-Mail (PER DU, kein Sie, Ihnen usw.) auf Deutsch, die folgende Struktur und Inhalt hat:\n\n\
        1. Betreff: {}\n\
        2. Begrüßung: {}\n\
        3. Text der E-Mail: Erfinde hier etwas Text\n\
        4. Verabschiedung: {}\n\n\
        Achte darauf, dass die E-Mail freundlich und informell ist und MAXIMAL 100 Wörter lang ist. Der Text sollte sinnlos und unzusammenhängend wirken.\
        Verwende nach dem Betreff eine Leerzeile und formatiere die E-Mail sauber mit Absätzen.\
        Die Mail sollte versandfertig sein und keine unvollständigen Dinge beinhalten. Gib auch NUR die Mail zurück, keinen weiteren Text.",
        subject, greeting, ending
    );
    
    // Get a response
    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    match res {
        Ok(response) => Ok(response.response),
        Err(err) => Err(Box::new(err)),
    }
}
