use std::{path::PathBuf, sync::LazyLock};

use clap::Parser;
use scan_fmt::scan_fmt_some;
use tap::Tap;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Deck File
    #[arg(short)]
    input: std::path::PathBuf,

    /// Output Directory
    #[arg(short)]
    output: Option<std::path::PathBuf>,
}

#[derive(Debug)]
struct Card {
	name: String,
	set: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
	// let input = args.input;
	let output = if let Some(o) = args.output { o } else { PathBuf::from("out") };

	std::fs::create_dir_all(&output).unwrap();
	let out = output.clone().tap_mut(|path| path.push("Adaptive Automaton")).tap_mut(|path| { path.set_extension("png"); });
	let img = get_card_image("Adaptive Automaton", "brr").await.unwrap();
	std::fs::write(out, img).unwrap();

	// std::fs::read_to_string(&args.input)
		// .expect("Failed to read file")
		// .lines()
		// .for_each(|line| {
			// // 1 Adaptive Automaton (brr) 64
			// // 1x Phyrexian Arena (mkc) 132 [Maybeboard{noDeck}{noPrice},Draw,Maybeboard Premium{noDeck}]
			// // 1x Westvale Abbey // Ormendahl, Profane Prince (soi) 281 [Land]

			// let maybe_x = "{*[x]}";
			// let set_name = r#"{/(.*)\s\(/}"#;
			// let full_match = format!("{{d}}{maybe_x} {set_name} {{}}");

			// let (_, name, set_name) = scan_fmt_some!{
				// line,
				// &full_match,
				// u64, String, String
			// };

			// println!("{:?} - {:?}", name, set_name);
		// });
}

// https://api.scryfall.com/cards/named?&format=image&image=png&exact=Adaptive Automaton&set=brr
//
// We kindly ask that you insert 50 – 100 milliseconds of delay between
// the requests you send to the server at api.scryfall.com.
// (i.e., 10 requests per second on average).
async fn get_card_image(name: &str, set: &str) -> anyhow::Result<bytes::Bytes> {
	static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

	Ok(CLIENT.get("https://api.scryfall.com/cards/named")
		.query(&[
			("format", "image"),
			("version", "png"),
			("exact", name),
			("set", set),
		])
		.header(reqwest::header::USER_AGENT, "yank-images")
		.header(reqwest::header::ACCEPT, "*/*")
		.send().await?
		.error_for_status()?
		.bytes().await?)
}
