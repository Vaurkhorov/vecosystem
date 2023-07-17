mod maps;
use maps::maps::{Map, Terrain};
use rand::Rng;

fn main() {
    let random_seed: u32 = rand::thread_rng().gen_range(0..10000);
    let test_map = Map::new(30, 60, random_seed);

    for row in test_map.blocks {
        for block in row {
            let symbol = match block.terrain {
                Terrain::Plains => '🟩',
                Terrain::Forest => '🌲',
                Terrain::Mountain => '🗻',
                Terrain::Desert => '🟨',
                Terrain::Tundra => '🧊',
                // _ => ' ', // Ignore water for now
            };
            print!("{}", symbol);
        }
        println!();
    }

    println!("Seed for this map: {:?}", random_seed);

}
