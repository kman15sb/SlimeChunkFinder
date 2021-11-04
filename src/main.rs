mod slime;
mod spiral;
// use indicatif;
use std::time::Instant;
use slime_seed_finder::chunk::Chunk;

mod constants {
    pub const GRID_CHECK_SIZE: i32 = 3;
    pub const SEED: u64 = 611464175010909465;
    pub const MAX_SEARCH: i32 = 1000000000;
    pub const MAX_NONSLIME_TILES: i32 = 1;
}

pub fn main() {
    let now = Instant::now();
    let mut chunks: Vec<Vec<i32>> = Vec::new();

    // let bar = indicatif::ProgressBar::new(constants::MAX_SEARCH.try_into().unwrap());
    // bar.set_style(
    //     indicatif::ProgressStyle::default_bar()
    //         .template("[{elapsed_precise}] {bar:40.green} {pos:>7}/{len:7} {msg}")
    //         .progress_chars("xx-"),
    // );

    for i in 1..constants::MAX_SEARCH {
        // bar.inc(1);

        let pos = &spiral::next_spiral(i);
        if check_square(pos.to_vec()) == true {
            chunks.push(pos.to_vec());
        }
    }

    for i in (0..chunks.len()).rev() {
        println!(
            "Found {}x{} chunk at {:?}",
            constants::GRID_CHECK_SIZE,
            constants::GRID_CHECK_SIZE,
            chunks[i]
        );
        viz_chunks(&chunks[i]);
    }
    let elapsed = now.elapsed();
    println!("finished in {:?} seconds", elapsed);
}

fn check_square(pos: Vec<i32>) -> bool {
    let mut count: i32 = 0;
    'outer: for j in 0..constants::GRID_CHECK_SIZE {
        for k in 0..constants::GRID_CHECK_SIZE {
            if slime::is_slime_chunk(
                constants::SEED,
                &Chunk {
                    x: (pos[0] - j) as i32,
                    z: (pos[1] - k) as i32,
                },
            ) == false
            {
                count += 1;
                if count > constants::MAX_NONSLIME_TILES {
                    break 'outer;
                }
            }
        }
    }
    return count <= constants::MAX_NONSLIME_TILES;
}

pub fn viz_chunks(pos: &Vec<i32>) {
    for i in 0..constants::GRID_CHECK_SIZE {
        let mut print_grid = String::from("");
        for j in 0..constants::GRID_CHECK_SIZE {
            if slime::is_slime_chunk(
                constants::SEED,
                &Chunk {
                    x: (pos[0] - i) as i32,
                    z: (pos[1] - j) as i32,
                },
            ) == true
            {
                print_grid.push('X');
            } else {
                print_grid.push('O');
            }
        }
        println! {"{}", print_grid}
    }
}
