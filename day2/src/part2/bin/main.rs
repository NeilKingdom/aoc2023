#![feature(iter_next_chunk)]

use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Default, Debug, Copy, Clone)]
struct CubeSet {
    blue_count: u8,
    green_count: u8,
    red_count: u8,
}

impl CubeSet {
    fn new(blue_count: u8, green_count: u8, red_count: u8) -> Self {
        CubeSet { red_count, blue_count, green_count } 
    }
    
    fn clear(&mut self) -> () {
        self.blue_count = 0;
        self.green_count = 0;
        self.red_count = 0;
    }

    fn inc_color_count(&mut self, num: &u8, color: &str) -> () {
        match color {
            "blue" => self.blue_count += num,
            "green" => self.green_count += num,
            "red" => self.red_count += num,
            _ => panic!("Unexpected color")
        }
    }
}

type Game = (u8, Vec<CubeSet>);

fn main() -> std::io::Result<()> {
    let fhndl = File::open("input.txt")?;
    let reader = BufReader::new(fhndl);
    let mut games: Vec<Game> = Vec::new();

    let reader = reader.lines().map(|x| {
        x.expect("Could not read line")
            .replacen(":", "", 1)
            .replace(",", "") + ";"
    });

    for line in reader {
        let mut line_iter = line.split_whitespace();
        let (mut game_id, mut cube_sets): Game = (0, Vec::new());
        let mut cube_set = CubeSet::default();

        loop {
            let chunk = line_iter.next_chunk::<2>();
            if let Err(_) = chunk { // End of line
                break;
            }
            let chunk_unwrap = chunk.unwrap();

            if chunk_unwrap[0].eq("Game") {
                game_id = chunk_unwrap[1].parse().expect("Could not parse game_id");
            } else {
                let num = chunk_unwrap[0].parse().expect("Could not parse num");
                let mut color = chunk_unwrap[1];
                let mut eof = false;
                
                if color.contains(";") {
                    eof = true;
                    color = &color[..color.len() - 1];
                }

                cube_set.inc_color_count(&num, &color);

                if eof {
                    cube_sets.push(cube_set);
                    cube_set.clear();
                }
            }
        }

        //println!("game_id: {game_id} {:#?}", cube_sets);
        games.push((game_id, cube_sets));
    }

    let mut sum_of_powers: u32 = 0;

    for (_game_id, cube_sets) in games {
        let (mut min_red, mut min_blue, mut min_green) = (0u32, 0u32, 0u32);
        for cube_set in cube_sets {
            if cube_set.red_count as u32 > min_red {
                min_red = cube_set.red_count.into();
            }
            if cube_set.blue_count as u32 > min_blue {
                min_blue = cube_set.blue_count.into();
            }
            if cube_set.green_count as u32 > min_green {
                min_green = cube_set.green_count.into();
            }
        }
        sum_of_powers += min_red * min_blue * min_green;
    }

    println!("Answer: {sum_of_powers}");

    Ok(())
}
