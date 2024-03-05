mod replay;
mod util;

use clap::Parser;

#[derive(Parser)]
#[command(version, about = "A tool for extracting osu! replay file format data.", long_about=None)]
struct Cli {
    /// input replay files directory
    #[arg(required = true)]
    input: std::path::PathBuf,

    /// output file
    #[arg(short('o'), long)]
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let osr_files = util::collect_files(&args.input);

    let mut output_csv: std::path::PathBuf = args.output;
    output_csv.set_extension("csv");

    let csv_exists: bool = output_csv.exists();
    let csv_file = if csv_exists {
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(output_csv)
    } else {
        std::fs::File::create(output_csv)
    };

    let mut csv_writer = match csv_file {
        Ok(file) => csv::WriterBuilder::new().from_writer(file),
        Err(e) => panic!("{e}"),
    };

    if !csv_exists {
        let _ = csv_writer.write_record(&[
            "beatmap_hash",
            "count300",
            "count100",
            "count50",
            "count_miss",
            "score",
            "max_combo",
            "perfect_combo",
            // "replay_data",
        ]);
    }

    let mut parsed_replays_count: u32 = 0;
    match osr_files {
        Ok(files) => {
            for osr in &files {
                match std::fs::File::open(osr) {
                    Ok(mut replay) => {
                        let game_mode: u8 = util::read_byte(&mut replay).unwrap();
                        if game_mode != 0 {
                            // println!("Skipping {osr:?}");
                            continue;
                        }
                        let version: u32 =
                            u32::from_le_bytes(util::read_integer(&mut replay).unwrap());
                        let beatmap_hash: String = match util::read_string(&mut replay) {
                            Ok(beatmap_hash) => beatmap_hash,
                            Err(_) => {
                                // println!("Skipping {osr:?}");
                                continue;
                            }
                        };
                        let player_name: String = match util::read_string(&mut replay) {
                            Ok(player_name) => player_name,
                            Err(_) => {
                                // println!("Skipping {osr:?}");
                                continue;
                            }
                        };
                        let replay_hash: String = match util::read_string(&mut replay) {
                            Ok(replay_hash) => replay_hash,
                            Err(_) => {
                                // println!("Skipping {osr:?}");
                                continue;
                            }
                        };
                        let count300: u16 =
                            u16::from_le_bytes(util::read_short(&mut replay).unwrap());
                        let count100: u16 =
                            u16::from_le_bytes(util::read_short(&mut replay).unwrap());
                        let count50: u16 =
                            u16::from_le_bytes(util::read_short(&mut replay).unwrap());
                        let count_geki: u16 =
                            u16::from_le_bytes(util::read_short(&mut replay).unwrap());
                        let count_katu: u16 =
                            u16::from_le_bytes(util::read_short(&mut replay).unwrap());
                        let count_miss: u16 =
                            u16::from_le_bytes(util::read_short(&mut replay).unwrap());
                        let score: u32 =
                            u32::from_le_bytes(util::read_integer(&mut replay).unwrap());
                        let max_combo: u16 =
                            u16::from_le_bytes(util::read_short(&mut replay).unwrap());
                        let perfect_combo: bool = util::read_bool(&mut replay).unwrap();
                        let mods: u32 =
                            u32::from_le_bytes(util::read_integer(&mut replay).unwrap());
                        let hp: String = match util::read_string(&mut replay) {
                            Ok(hp) => hp,
                            Err(_) => {
                                // println!("Skipping {osr:?}");
                                continue;
                            }
                        };
                        let timestamp: u64 =
                            u64::from_le_bytes(util::read_long(&mut replay).unwrap());
                        let replay_length: u32 =
                            u32::from_le_bytes(util::read_integer(&mut replay).unwrap());
                        // let replay_data: Vec<u8> =
                        //     util::read_byte_array(&mut replay, replay_length as usize).unwrap();
                        let replay_data: Vec<u8> =
                            match util::read_byte_array(&mut replay, replay_length as usize) {
                                Ok(data) => data,
                                Err(_) => continue,
                            };
                        let online_score_id: u64 =
                            u64::from_le_bytes(util::read_long(&mut replay).unwrap_or_default());

                        let replay_data: replay::ReplayData = replay::ReplayData::new(
                            game_mode,
                            version,
                            beatmap_hash,
                            player_name,
                            replay_hash,
                            count300,
                            count100,
                            count50,
                            count_geki,
                            count_katu,
                            count_miss,
                            score,
                            max_combo,
                            perfect_combo,
                            mods,
                            hp,
                            timestamp,
                            replay_length,
                            replay_data,
                            online_score_id,
                            0.0,
                        );
                        let _ = csv_writer.serialize((
                            &replay_data.beatmap_hash,
                            &replay_data.count300,
                            &replay_data.count100,
                            &replay_data.count50,
                            &replay_data.count_miss,
                            &replay_data.score,
                            &replay_data.max_combo,
                            &replay_data.perfect_combo,
                            // &replay_data.replay_data,
                        ));
                        // println!("Saved {osr:?} with game mode: {game_mode}");
                        parsed_replays_count += 1;
                    }
                    Err(error) => println!("File error: {error:?}"),
                }
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::InvalidInput => panic!("{error:?}"),
            _ => panic!("Error: {error:?}"),
        },
    }
    let _ = csv_writer.flush();
    println!("Total parsed replays: {parsed_replays_count}");
}
