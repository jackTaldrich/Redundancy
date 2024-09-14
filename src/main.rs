use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

#[derive(Parser)]
struct Cli {
    // The path to the file to read
    #[arg(short, long)]
    file: Option<String>,

    #[arg(long)]
    include_common: bool,

    #[arg(short)]
    c: bool,

    #[arg(short, long)]
    max_length: Option<u16>,

    #[arg(short, long)]
    help: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    if args.help {
        println!(r#"
Use a file instead of paste:
--file path/to/your/file
-f path/to/your/file

Include common words in the results
--include-common
-c

How many words to include in results
--max-length <int>
-m <int>
        "#);
        return Ok(());
    }

    let input = match &args.file {
        Some(file) => {
            let mut file = File::open(file)?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            content
        }
        None => {
            let mut input = String::new();
            println!("Enter or paste text (Ctrl+D to end):");
            io::stdin().read_to_string(&mut input)?;
            input
        }
    };

    let max_length = args.max_length.unwrap_or(10);

    let word_counts = count_words(
        &input,
        args.include_common,
        args.c,
        max_length,
    );

    let mut word_frequencies: Vec<(&String, &usize)> = word_counts.iter().collect();
    word_frequencies.sort_by(|a, b| b.1.cmp(a.1));

    println!("\nLimit = {}:", max_length);
    for (word, count) in word_frequencies.iter().take(max_length as usize) {
        println!("{}: {}", word, count);
    }

    Ok(())
}

fn ignored_words() -> Vec<&'static str> {
    vec![
        "the", "and", "is", "in", "of", "to", "a", "an", "it", "for", "on", "with", "as", "by", "that", "we", "i"
    ]
}

fn count_words(
    input: &str,
    include_common_words: bool,
    case_sensitive: bool,
    max_length: u16,
) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut word_counts = HashMap::new();
    let ignored_words = ignored_words();

    for word in re.find_iter(input) {
        let mut word = word.as_str().to_string();

        if !case_sensitive {
            word = word.to_lowercase();
        }

        if word.len() <= max_length as usize && (include_common_words || !ignored_words.contains(&word.as_str())) {
            *word_counts.entry(word).or_insert(0) += 1;
        }
    }

    word_counts
}
