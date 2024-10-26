use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Deck File
    #[arg(short)]
    input: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("Hello, world!: {args:#?}");
}

// https://api.scryfall.com/cards/named?&format=image&image=png&exact=Adaptive Automaton&set=brr
// async fn get_card_image()
