use std::{path::PathBuf, sync::LazyLock};

use clap::Parser;
use futures::StreamExt;
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

    /// Number of simultaneous upscaling commands to run.
    #[arg(short, long)]
    futures: Option<usize>,
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
			let name_till_paren = skipped_number.chars().take_while(|c| !['(', '*', '^', '['].contains(c)).collect::<String>();
			let last_stop = name_till_paren.len();

			let set_name = if skipped_number.chars().nth(last_stop).is_some_and(|c| c == '(') {
				let (set_name, _) = skipped_number[last_stop + 1..].split_once(')').unwrap();
				Some(set_name)
			} else { None };

			let illegal_chars = [ '/', ':', '\\', '?', '*', '+', '%', '.', '"', '<', '>', '|'];
			let card_name = name_till_paren.replace(illegal_chars, "_").trim().to_owned();
			let output_path = output.clone().tap_mut(|path| path.push(card_name.clone())).tap_mut(|path| { path.set_extension("png"); });

			(card_name, set_name, output_path)
		}).collect::<Vec<_>>();

	// "We kindly ask that you insert 50 – 100 milliseconds of delay between
	// the requests you send to the server at api.scryfall.com.
	// (i.e., 10 requests per second on average)."
	for (name, set, out_path) in &cards {
		println!("Downloading: |{}|", name);
		let img = get_card_image(&name, set).await.unwrap();
		std::fs::write(&out_path, img).unwrap();
		sleep(std::time::Duration::from_millis(150)).await;
	}

	if args.upscale {
		let mut futures = Vec::new();

		for (name, _, out_path) in cards {
			std::fs::create_dir_all(format!("{}/upscaled", output.display())).unwrap();

			let output = out_path.clone()
				.tap_mut(|p| { p.pop(); })
				.tap_mut(|p| p.push("upscaled"))
				.tap_mut(|p| p.push(format!("{name}_x2_d3.png")));

			// For printing at 1200 dpi
			// 1200dpi -> 2400dpi did not seem to improve quality unlike 600 -> 1200
			// max denoise looked slightly better than no denoise
			let future = tokio::process::Command::new("waifu2x-ncnn-vulkan")
				.arg("-i").arg(&out_path)
				.arg("-o").arg(&output)
				.arg("-s").arg("4")
				.arg("-n").arg("3")
				.status();

			futures.push(future);
		}

		let stream = futures::stream::iter(futures).buffer_unordered(args.futures.unwrap_or(10));
		let results = stream.collect::<Vec<_>>().await;

		results.into_iter().for_each(|r| if r.is_err() { eprintln!("{:?}", r); });
	}
}

async fn get_card_image(name: &str, set: &Option<&str>) -> anyhow::Result<bytes::Bytes> {
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
