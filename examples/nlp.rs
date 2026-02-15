use gananayantra::nlp::text_analysis::{jaccard_similarity, reading_time_minutes};

fn main() {
    println!("--- NLP Calculators ---");

    // Reading time
    let word_count = 1500;
    let avg_wpm = 200.0;
    match reading_time_minutes(word_count, avg_wpm) {
        Ok(time) => println!("Reading time for {} words at {} WPM: {:.2} minutes", word_count, avg_wpm, time),
        Err(e) => println!("Error: {}", e),
    }

    // Similarity
    let set_a = 100;
    let set_b = 80;
    let intersection = 50;
    match jaccard_similarity(set_a, set_b, intersection) {
        Ok(sim) => println!("Jaccard Similarity: {:.2}", sim),
        Err(e) => println!("Error: {}", e),
    }
}
