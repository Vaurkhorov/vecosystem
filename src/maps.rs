pub mod maps {
    use noise::{Perlin, NoiseFn};

    const HEIGHT_SCALE: u32 = 5000;
    const RAIN_SCALE: u32 = 100;
    const TEMP_SCALE: i8 = 64;


    #[derive(Debug)]
    pub struct Map {
        // map_name: &'a str,
        seed: u32,
        pub blocks: Vec<Vec<Block>>,
        map_dim: [usize; 2],
        // time_created: Duration,
        // time_modified: Duration,
    }

    impl Map {
        pub fn new(rows: usize, columns: usize, seed: u32) -> Self {
            let blocks = generate_blocks(rows, columns, seed);

            Map {
                // map_name: map_name,
                seed: seed,
                blocks: blocks,
                map_dim: [rows, columns],
                // time_created: SystemTime::now(),
                // time_modified: SystemTime::now(),
            }
        }
    }

    fn generate_blocks(rows: usize, columns: usize, seed: u32) -> Vec<Vec<Block>> {
        let mut blocks = vec![vec![Block::default(); columns]; rows];
        let heightmap_generator = Perlin::new(seed);
        let rainmap_generator = Perlin::new(seed + 1);      // Not sure how I can generate 3 unique maps with
        let heatmap_generator = Perlin:: new(seed + 2);     // one seed, so this is what I did for now.

        for x in 0..rows {
            for y in 0..columns {
                // gen = generated
                let gen_height = heightmap_generator.get([x as f64 / rows as f64, y as f64 / columns as f64]);
                blocks[x][y].height = (gen_height.abs() * HEIGHT_SCALE as f64) as u32;

                let gen_rain = rainmap_generator.get([x as f64 / rows as f64, y as f64 / columns as f64]);
                blocks[x][y].average_rainfall = (gen_rain.abs() * RAIN_SCALE as f64) as u32;

                let gen_temp = heatmap_generator.get([x as f64 / rows as f64, y as f64 / columns as f64]);
                blocks[x][y].average_temperature = (gen_temp * TEMP_SCALE as f64) as i8;

                blocks[x][y].terrain = determine_terrain(blocks[x][y].height, blocks[x][y].average_rainfall, blocks[x][y].average_temperature)
            }
        }
        blocks

    }

    fn determine_terrain(height: u32, rain: u32, temp: i8) -> Terrain {
        if height > 2000 {
            Terrain::Mountain
        } else if rain > 50 {
            Terrain::Forest
        } else if temp > 30 {
            Terrain::Desert
        } else if temp < -40 {
            Terrain::Tundra
        } else{
            Terrain::Plains
        }
    }

    #[derive(Debug, Default, Clone, Copy)]
    pub struct Block {
        // water_feature: WaterFeature,
        pub terrain: Terrain,
        height: u32,
        // fertility: u8,
        // water: u8,                      // water available to drink
        average_rainfall: u32,
        average_temperature: i8,
        // current_weather: Weather,
    }

    // #[derive(Debug)]
    // enum Weather {
    //     Clear,
    //     Raining,
    // }

    #[derive(Debug, Default, Clone, Copy)]
    pub enum Terrain {
        #[default] Plains,
        Forest,
        Mountain,
        Desert,
        Tundra,
    }


    // #[derive(Debug)]
    // enum WaterFeature {
    //     None,
    //     Stream,
    //     River,
    //     Lake,
    // }
}
