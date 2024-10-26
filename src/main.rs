use std::{path::PathBuf, sync::LazyLock};

use clap::Parser;
use tap::Tap;
use tokio::time::sleep;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Deck File
    #[arg(short)]
    input: std::path::PathBuf,

    /// Output Directory
    #[arg(short)]
    output: Option<std::path::PathBuf>,

    /// Upscale Images if you have `waifu2x-ncnn-vulkan` in path.
    #[arg(short, long)]
    upscale: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
	let output = if let Some(o) = args.output { o } else { PathBuf::from("out") };
	std::fs::create_dir_all(&output).unwrap();


	let deckfile = std::fs::read_to_string(&args.input).expect("Failed to read file");
	let cards = deckfile
		.lines()
		.map(|line| {
			let skipped_number = if let Some((_, r)) = line.split_once(' ') { r } else { line };
			let card_name = skipped_number.chars().take_while(|c| !['(', '*', '^', '['].contains(c)).collect::<String>();
			let last_stop = card_name.len();

			let set_name = if skipped_number.chars().nth(last_stop).is_some_and(|c| c == '(') {
				let (set_name, _) = skipped_number[last_stop + 1..].split_once(')').unwrap();
				Some(set_name)
			} else { None };

			(card_name.trim().to_owned(), set_name)
		});

	let mut outputs = Vec::with_capacity(deckfile.lines().count());

	// "We kindly ask that you insert 50 – 100 milliseconds of delay between
	// the requests you send to the server at api.scryfall.com.
	// (i.e., 10 requests per second on average)."
	for (name, set) in cards {
		let img = get_card_image(&name, set).await.unwrap();
		let out = output.clone().tap_mut(|path| path.push(name.clone())).tap_mut(|path| { path.set_extension("png"); });
		std::fs::write(&out, img).unwrap();
		outputs.push((name, out));
		sleep(std::time::Duration::from_millis(150)).await;
	}

	if args.upscale {
		for (name, path) in outputs {
			std::fs::create_dir_all(&format!("{}/upscaled", output.display())).unwrap();

			let output = path.clone()
				.tap_mut(|p| { p.pop(); })
				.tap_mut(|p| p.push("upscaled"))
				.tap_mut(|p| p.push(format!("{name}_x2_d3.png")));

			println!("Upscaling: {}", output.display());

			// For printing at 1200 dpi
			// 2400dpi did not seem to improve quality
			// max denoise looked slightly better than no denoise
			tokio::process::Command::new("waifu2x-ncnn-vulkan")
				.arg("-i").arg(&path)
				.arg("-o").arg(&output)
				.arg("-s").arg("4")
				.arg("-n").arg("3")
				.status().await.unwrap();
		}
	}
}

async fn get_card_image(name: &str, set: Option<&str>) -> anyhow::Result<bytes::Bytes> {
	static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

	Ok(CLIENT.get("https://api.scryfall.com/cards/named")
		.query(&[
			("format", "image"),
			("version", "png"),
			("exact", name),
			("set", set.unwrap_or_default()),
		])
		.header(reqwest::header::USER_AGENT, "yank-images")
		.header(reqwest::header::ACCEPT, "*/*")
		.send().await?
		.error_for_status()?
		.bytes().await?)
}
