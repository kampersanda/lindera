mod timer;

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use lindera::mode::Mode;
use lindera::tokenizer::DictionaryConfig;
use lindera::tokenizer::DictionaryKind;
use lindera::tokenizer::DEFAULT_DICTIONARY_KIND;
use lindera::tokenizer::{Tokenizer, TokenizerConfig};

use timer::Timer;

use clap::Parser;

const RUNS: usize = 10;
const TRIALS: usize = 10;

#[derive(Parser, Debug)]
#[clap(name = "main", about = "A program.")]
struct Args {
    /// The dictionary type.
    #[clap(
        short = 'k',
        long = "dictionary-kind",
        value_name = "DICTIONARY_KIND",
        default_value = DEFAULT_DICTIONARY_KIND
    )]
    dictionary_kind: DictionaryKind,

    #[clap(short = 's', long)]
    sentence_filename: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let dictionary_meta = DictionaryConfig {
        kind: args.dictionary_kind.clone(),
        path: None,
    };
    let config = TokenizerConfig {
        dictionary: dictionary_meta,
        user_dictionary: None,
        mode: Mode::Normal,
    };

    let lines: Vec<_> = to_lines(&args.sentence_filename).collect();
    let tokenizer = Tokenizer::with_config(config)?;

    let measure = |t: &mut Timer| {
        let mut n_words = 0;
        for _ in 0..RUNS {
            t.start();
            for line in &lines {
                let tokens = tokenizer.tokenize(line).unwrap();
                n_words += tokens.len();
            }
            t.stop();
        }
        dbg!(n_words);
    };

    let mut t = Timer::new();

    // Warmup
    t.reset();
    measure(&mut t);
    println!("Warmup: {}", t.average());

    let (mut min, mut max, mut avg) = (0.0, 0.0, 0.0);

    for _ in 0..TRIALS {
        t.reset();
        measure(&mut t);
        t.discard_min();
        t.discard_max();
        min += t.min();
        avg += t.average();
        max += t.max();
    }

    min = min / TRIALS as f64;
    avg = avg / TRIALS as f64;
    max = max / TRIALS as f64;

    println!("Number_of_sentences: {}", lines.len());
    println!(
        "Elapsed_seconds_to_tokenize_all_sentences: [{},{},{}]",
        min, avg, max
    );

    Ok(())
}

fn to_lines<P>(path: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    let buf = BufReader::new(File::open(path).unwrap());
    buf.lines().map(|line| line.unwrap())
}
