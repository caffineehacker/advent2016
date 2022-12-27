use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    door_id: String,
}

fn main() {
    let args = Args::parse();

    let progress = indicatif::ProgressBar::new_spinner();

    let mut i = 0;
    let mut codes_found = "".to_string();
    loop {
        progress.set_message(format!("Checking {}, found {}", i, codes_found.len()));
        let hash = format!(
            "{:x}",
            md5::compute(args.door_id.clone() + i.to_string().as_str())
        );

        if hash.starts_with("00000") {
            codes_found += &hash[5..6];
            if codes_found.len() == 8 {
                println!("Part 1: {}", codes_found);
                break;
            }
        }

        i += 1;
    }
}
