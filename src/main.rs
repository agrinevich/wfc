use clap::Parser;
use log::{error, info};

#[derive(Parser)]
struct Args {
    /// Path to TXT file
    #[arg(short, long)]
    file: std::path::PathBuf,

    /// Number of result rows
    #[arg(short, long, default_value_t = 10)]
    num: usize,

    /// Min length of word
    #[arg(long, default_value_t = 4)]
    wlmin: usize,

    /// Max length of word
    #[arg(long, default_value_t = 100)]
    wlmax: usize,
}

fn main() {
    env_logger::init();
    info!("Start");

    let args = Args::parse();

    let result = std::fs::read_to_string(&args.file);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            error!(
                "Failed to open file: {}, error: {:?}",
                &args.file.display(),
                error
            );
            std::process::exit(2);
        }
    };

    let words_freq = wfc::count_word_frequency(&content, args.wlmin, args.wlmax);

    let mut words: Vec<&String> = words_freq.keys().collect();

    words.sort_by(|a, b| {
        words_freq
            .get(*a)
            .copied()
            .unwrap()
            .cmp(&words_freq.get(*b).copied().unwrap())
    });

    let mut n: usize = 1;
    while n <= args.num {
        let cur_word = words[words.len() - n];

        println!(
            "{}:\t{}",
            cur_word,
            words_freq.get(cur_word).copied().unwrap()
        );

        n += 1;
    }

    info!("Finish");
}
