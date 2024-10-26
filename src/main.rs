use clap::Parser;
use scan_fmt::{scan_fmt, scan_fmt_some};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Deck File
    #[arg(short)]
    input: std::path::PathBuf,
}

#[derive(Debug)]
struct Card {
	name: String,
	set: String,
}

fn main() {
    let args = Args::parse();

	std::fs::read_to_string(&args.input)
		.expect("Failed to read file")
		.lines()
		.for_each(|line| {
			// 1 Adaptive Automaton (brr) 64
			// 1x Phyrexian Arena (mkc) 132 [Maybeboard{noDeck}{noPrice},Draw,Maybeboard Premium{noDeck}]
			// 1x Westvale Abbey // Ormendahl, Profane Prince (soi) 281 [Land]

			let maybe_x = "{*[x]}";
			let set_name = r#"{/(.*)\s\(/}"#;
			let full_match = format!("{{d}}{maybe_x} {set_name} {{}}");

			let (_, name, set_name) = scan_fmt_some!{
				line,
				&full_match,
				u64, String, String
			};

			println!("{:?} - {:?}", name, set_name);
		});
}

// https://api.scryfall.com/cards/named?&format=image&image=png&exact=Adaptive Automaton&set=brr
// async fn get_card_image()
