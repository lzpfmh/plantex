//! Procedurally generating the game world.
//!

// extern crate noise;

use world::{Chunk, ChunkIndex, ChunkProvider, HeightType, HexPillar};
use world::{GroundMaterial, PillarSection, Prop, PropType};
use rand::Rand;
use gen::PlantGenerator;
use noise::{PermutationTable, open_simplex2};
use gen::seeded_rng;

// pub const LANDSCAPE_FLAT: f32 = 0.01;
pub const LANDSCAPE_MODERATE: f32 = 0.05;
// pub const LANDSCAPE_HILLY: f32 = 0.1125;


/// Main type to generate the game world. Implements the `ChunkProvider` trait
/// (TODO, see #8).
pub struct WorldGenerator {
    seed: u64,
    terrain_table: PermutationTable,
}

impl WorldGenerator {
    /// Creates the generator with the given seed.
    pub fn with_seed(seed: u64) -> Self {
        let mut terrain_rng = seeded_rng(seed, 0, ());

        WorldGenerator {
            seed: seed,
            terrain_table: PermutationTable::rand(&mut terrain_rng),
        }
    }

    /// Returns the seed of this world generator.
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

impl ChunkProvider for WorldGenerator {
    fn load_chunk(&self, index: ChunkIndex) -> Option<Chunk> {
        Some(Chunk::with_pillars(index, |pos| {
            let real_pos = pos.to_real();
            let x = real_pos.x;
            let y = real_pos.y;

            let noise = open_simplex2::<f32>(&self.terrain_table,
                                             &[(x as f32) * LANDSCAPE_MODERATE,
                                               (y as f32) * LANDSCAPE_MODERATE]);
            let height = 50.0 + noise * 50.0;

            let ground_section = PillarSection::new(GroundMaterial::Dirt,
                                                    HeightType::from_units(0),
                                                    HeightType::from_units(height as u16));
            let mut props = Vec::new();

            // Place a test plant every few blocks
            const TREE_SPACING: i32 = 8;
            if pos.q % TREE_SPACING == 0 && pos.r % TREE_SPACING == 0 {
                let mut rng = super::seeded_rng(self.seed, "TREE", (pos.q, pos.r));
                let gen = PlantGenerator::rand(&mut rng);

                props.push(Prop {
                    baseline: HeightType::from_units(height as u16),
                    prop: PropType::Plant(gen.generate(&mut rng)),
                });
            }

            HexPillar::new(vec![ground_section], props)
        }))
    }

    fn is_chunk_loadable(&self, _: ChunkIndex) -> bool {
        // for the moment returns true
        true
    }
}
