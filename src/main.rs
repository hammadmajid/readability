mod calc;

use clap::{Parser, ValueEnum};

fn main() {
    let cli = Cli::parse();
    let grade: i8;

    match cli.method {
        Method::DaleChallScore => grade = calc::dale_chall_score(cli.text),
        Method::ColemanLiauIndex => grade = calc::coleman_liau_index(cli.text),
        Method::AutomatedReadabilityIndex => grade = calc::automated_readability_index(cli.text),
        Method::FleschKincaidGradeLevel => grade = calc::flesch_kincaid_grade_level(cli.text),
        Method::FleschKincaidReadingEase => grade = calc::flesch_kincaid_reading_ease(cli.text),
        Method::GunningFog => grade = calc::gunning_fog(cli.text),
        Method::Lix => grade = calc::lix(cli.text),
        Method::Smog => grade = calc::smog(cli.text),
    };

    println!("Grade: {}", grade);
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
