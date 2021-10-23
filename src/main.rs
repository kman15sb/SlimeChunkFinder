mod slime;
mod spiral;
use slime_seed_finder::chunk::Chunk;
use std::time::Instant;

mod constants {
    pub const GRID_CHECK_SIZE: i32 = 3;
    pub const SEED: u64 = 611464175010909465;
    pub const MAX_SEARCH: i32 = 1000000;
}

pub fn main() {
    let start = Instant::now();
    let mut count: i32 = 0;
    for i in 1..constants::MAX_SEARCH {
        let pos = &spiral::next_spiral(i);
        if check_square(pos.to_vec()) == true {
            count += 1;
            println!("Found {}x{} at chunk {:?}", constants::GRID_CHECK_SIZE, constants::GRID_CHECK_SIZE, pos)
        }
    }
    let duration = start.elapsed();
    println!("Checked {} chunks in {:?} and found {} matches", constants::MAX_SEARCH, duration, count);
}

fn check_square(pos: Vec<i32>) -> bool {
    let mut count: i32 = 0;
    'outer: for j in 0..constants::GRID_CHECK_SIZE {
        for k in 0..constants::GRID_CHECK_SIZE {
            if slime::is_slime_chunk(constants::SEED, &Chunk{x: (pos[0]-j) as i32, z: (pos[1]-k) as i32}) == false {
                count += 1;
                break 'outer;
            }
        }
    }
    return count < 1;
}