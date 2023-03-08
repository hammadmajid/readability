use clap::{Parser, ValueEnum};

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);
}

#[derive(Parser, Debug)]
struct Cli {
    /// The string of text that will be tested
    text: String,

    /// The method that will be used to test
    #[arg(value_enum)]
    method: Method,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Method {
    DaleChallScore,
    ColemanLiauIndex,
    AutomatedReadabilityIndex,
    FleschKincaidGradeLevel,
    FleschKincaidReadingEase,
    GunningFog,
    Lix,
    Smog,
}
