use bedrock_cracker::crack;
use bedrock_cracker::raw_data::block::Block;
use bedrock_cracker::raw_data::block_type::BlockType;
use bedrock_cracker::raw_data::modes::{BedrockGeneration, OutputMode};

const DATA: &str =
"-1 123 -7 Bedrock
-1 123 -9 Bedrock
-2 123 -11 Bedrock
-3 123 -10 Bedrock
-5 123 -10 Bedrock
-5 123 -9 Bedrock
-5 123 -8 Bedrock
-5 123 -6 Bedrock
-6 123 -6 Bedrock
-6 123 -4 Bedrock
-6 123 -9 Bedrock
-7 123 -12 Bedrock
-7 123 -8 Bedrock
-7 123 -7 Bedrock
-8 123 -7 Bedrock
-8 123 -4 Bedrock
-9 123 -13 Bedrock
-9 123 -8 Bedrock
-9 123 -6 Bedrock
-9 123 -4 Bedrock
-10 123 -5 Bedrock
-11 123 -6 Bedrock
-12 123 -13 Bedrock
-12 123 -12 Bedrock
-12 123 -11 Bedrock
-12 123 -2 Bedrock
-12 123 0 Bedrock
-13 123 -10 Bedrock
-13 123 -9 Bedrock
-13 123 -7 Bedrock
-13 123 -6 Bedrock
-13 123 0 Bedrock
23 4 -92 Bedrock
24 4 -92 Bedrock
25 4 -92 Bedrock
25 4 -93 Bedrock";

fn main() {
    let positions: Vec<Block> = DATA.lines()
        .map(|l| l.split_whitespace()
            .take(3)
            .map(|i| i.parse::<i32>().unwrap())
            .collect())
        .map(|l: Vec<i32>| Block::new(*l.get(0).unwrap(), *l.get(1).unwrap(), *l.get(2).unwrap(), BlockType::BEDROCK))
        .collect();

    let seeds = crack(positions.as_ptr(), positions.len(), 11, BedrockGeneration::Normal, OutputMode::WorldSeed);
    let seeds = unsafe { Vec::from_raw_parts(seeds.ptr as *mut i64, seeds.len, seeds.len) };

    println!("Expecting: {}", 765906787396911863i64);

    for seed in seeds {
        println!("Found: {}", seed);
    }
}
