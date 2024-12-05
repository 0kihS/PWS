use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::error::Error;

fn splits(text: &str) -> Vec<String> {
    // Splits de tekst uit in woorden
    text.split_whitespace()
        .filter_map(|token| {
            let trimmed = token.trim_matches(|c: char| !c.is_alphanumeric());
            if !trimmed.is_empty() {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Input van een JSON bestand
    let input_path = "messages.json"; // Update this with your actual file path
    let output_path = "preprocessed_data.txt"; // Output plain text file

    // Open het input bestand
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    // Parse de JSON data
    let json_data: Value = serde_json::from_reader(reader)?;

    // Navigeer naar de array met berichten
    if let Some(messages) = json_data.get("messages").and_then(|v| v.as_array()) {
        let output_file = File::create(output_path)?;
        let mut writer = BufWriter::new(output_file);

        for message in messages {
            // Neem alles in het "content" veld (hier zijn de daadwerkelijke berichten)
            if let Some(content) = message.get("content").and_then(|v| v.as_str()) {
                // Splits de zinnen uit
                let tokens = splits(content);
                // Voeg alle losse woorden samen in een nieuw bestand, telkens op een nieuwe regel.
                writeln!(writer, "{}", tokens.join(" "))?;
            }
        }

        println!("Data verwerkt! Output kan worden gevonden bij: {}", output_path);
    } else {
        eprintln!("Oeps!: 'messages' array is niet gevonden.");
    }

    Ok(())
}
