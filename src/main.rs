use std::env;
use std::process;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        exit_with_usage_guide();
    }

    let text = args[2].clone();

    #[allow(non_snake_case)]
    let method = match Method::from_str(&args[1]) {
        Ok(Method) => Method,
        Err(_err) => {
            exit_with_usage_guide();
            return;
        }
    };

    println!("text: {}\nmethod: {:?}", text, method);
}

#[derive(Debug)]
enum Method {
    DaleChallScore,
    ColemanLiauIndex,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DaleChallScore" => Ok(Method::DaleChallScore),
            "ColemanLiauIndex" => Ok(Method::ColemanLiauIndex),
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
