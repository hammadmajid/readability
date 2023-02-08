fn main() {
    if std::env::args().len() != 3 {
        logger(
            "Test readability score of your content\n".to_owned()
                + "\nUsage: readability \"Text string\" [Method]\n"
                + "\nText string:\n\tThe string of text you want to test.\n"
                + "\nMethod:\n\tThe test method you want to use, possible values are:\n"
                + "\t\tDale-Chall Score\n\t\tColeman-Liau index",
        );
        std::process::exit(1);
    }
}

fn logger(message: String) {
    println!("{message}");
}
