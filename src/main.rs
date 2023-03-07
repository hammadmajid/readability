use std::env;
use std::process;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        exit_with_usage_guide();
    }

    let text = args[2].clone();

    let method = Method::from_str(&args[1]).unwrap_or_else(|error| {
        exit_with_usage_guide();

        // the below line is unreachable because of the above fn call
        // wrote it to prevent compiler from complaining
        panic!("Error: {:?}", error);
    });

    println!("text: {}\nmethod: {:?}", text, method);
}

#[derive(Debug)]
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

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DaleChallScore" => Ok(Method::DaleChallScore),
            "ColemanLiauIndex" => Ok(Method::ColemanLiauIndex),
            "AutomatedReadabilityIndex" => Ok(Method::AutomatedReadabilityIndex),
            "FleschKincaidGradeLevel" => Ok(Method::FleschKincaidGradeLevel),
            "FleschKincaidReadingEase" => Ok(Method::FleschKincaidReadingEase),
            "GunningFog" => Ok(Method::GunningFog),
            "Lix" => Ok(Method::Lix),
            "Smog" => Ok(Method::Smog),
            _ => Err(())
        }
    }
}

fn exit_with_usage_guide() {
    println!("\
    Usage: cargo run [method] [text]

        [method]    The test method you want to use, possible values are:
                        DaleChallScore
                        ColemanLiauIndex

        [text]      The string of text you want to test.
    ");
    process::exit(1);
}
