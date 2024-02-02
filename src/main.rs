use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "echor")]
#[command(version = "0.1.0")]
#[command(author = "Radish-Miyazaki <y.hidaka.kobe@gmail.com>")]
struct Args {
    #[arg(value_name = "TEXT", help = "Input text", required = true)]
    text: Vec<String>,
    #[arg(short = 'n', help = "Do not print newline")]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    print!("{}{}", args.text.join(" "), if args.omit_newline { "" } else { "\n" })
}
