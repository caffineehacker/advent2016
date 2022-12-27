use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    door_id: String,
}

fn main() {
    let args = Args::parse();

    let multiprogress = indicatif::MultiProgress::new();
    let part_1progress = multiprogress.add(indicatif::ProgressBar::new_spinner());
    let part_2progress = multiprogress.add(indicatif::ProgressBar::new_spinner());

    let mut i = 0;
    let mut part1_codes_found = "".to_string();
    let mut part2_code: Vec<char> = vec!['_'; 8];
    loop {
        if part1_codes_found.len() < 8 {
            part_1progress.set_message(format!(
                "Part 1: checking {}, found {}",
                i,
                part1_codes_found.len()
            ));
        }
        part_2progress.set_message(format!(
            "Part 2: checking {}, found {}",
            i,
            part2_code.iter().collect::<String>()
        ));
        let hash = format!(
            "{:x}",
            md5::compute(args.door_id.clone() + i.to_string().as_str())
        );

        if hash.starts_with("00000") {
            if part1_codes_found.len() < 8 {
                part1_codes_found += &hash[5..6];
                if part1_codes_found.len() == 8 {
                    part_1progress.finish_with_message(format!("Part 1: {}", part1_codes_found));
                }
            }

            let index = hash[5..6].parse::<usize>();
            if let Ok(index) = index {
                if index < 8 && part2_code[index] == '_' {
                    part2_code[index] = hash.chars().nth(6).unwrap();
                    if !part2_code.contains(&'_') {
                        part_2progress.finish_with_message(format!(
                            "Part 2: {}",
                            part2_code.iter().collect::<String>()
                        ));
                        break;
                    }
                }
            }
        }

        i += 1;
    }
}
