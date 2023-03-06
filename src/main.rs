use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        exit_with_usage_guide();
    }

    let _text = args[2].clone();
    // let method = // TODO(hammadmajid): extract the [1] argument
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