use multiarray::*;
use std::vec::*;
use crate::aabb::*;

#[derive(Debug, Clone)]
struct BlockState {
    blk: String,
}

impl BlockState {
    fn default() -> Self {
        BlockState { blk: "air".to_string() }
    }
}

struct World {
    chunks: Vec<Chunk>
}

struct Chunk {
    blocks: Array3D<BlockState>,
    buildings: Vec<Building>,
    // entities: Vec<Entity>,
}

struct Building {
    pos: AABB,
    building_type: String
}

impl Chunk {
    fn new(size: V3i) -> Self {
        let blocks = Array3D::new([size.x, size.y, size.z], BlockState::default());
        let bldgs = Vec::new();
        // let entities = Vec::new();
        Chunk { blocks: blocks, buildings: bldgs /*, entities: entities */ }
    }

    fn block_at(&mut self, pos: V3i) -> &mut BlockState {
        &mut self.blocks[[pos.x, pos.y, pos.z]]
    }

    fn building_at(&self, pos: V3i) -> Option<&Building> {
        self.buildings.iter().find(|&b| b.pos.contains(pos))
    }

    fn layer(&mut self, layer: usize) -> Array2DRefMut<BlockState> {
        self.blocks.eliminated_dim_mut(2, layer)
    }
}