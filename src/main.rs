mod calc;

use clap::{Parser, ValueEnum};

fn main() {
    let cli = Cli::parse();

    match cli.method {
        Method::DaleChallScore => calc::dale_chall_score(cli.text),
        Method::ColemanLiauIndex => calc::coleman_liau_index(cli.text),
        Method::AutomatedReadabilityIndex => calc::automated_readability_index(cli.text),
        Method::FleschKincaidGradeLevel => calc::flesch_kincaid_grade_level(cli.text),
        Method::FleschKincaidReadingEase => calc::flesch_kincaid_reading_ease(cli.text),
        Method::GunningFog => calc::gunning_fog(cli.text),
        Method::Lix => calc::lix(cli.text),
        Method::Smog => calc::smog(cli.text),
    }
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
