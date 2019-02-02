use multiarray::*;
use std::vec::*;
use crate::aabb::*;

#[derive(Debug, Clone)]
pub struct BlockState {
    pub blk: String,
}

impl BlockState {
    pub fn default() -> Self {
        BlockState { blk: "air".to_string() }
    }
}

pub struct World {
    pub chunks: Vec<Chunk>
}

pub struct Chunk {
    pub blocks: Array3D<BlockState>,
    // pub buildings: Vec<Building>,
    // pub entities: Vec<Entity>,
}

// pub struct Building {
//     pub pos: AABB,
//     pub building_type: String
// }

impl Chunk {
    pub fn new(size: V3i) -> Self {
        let blocks = Array3D::new([size.x, size.y, size.z], BlockState::default());
        // let bldgs = Vec::new();
        // let entities = Vec::new();
        Chunk { blocks: blocks /* , buildings: bldgs */ /*, entities: entities */ }
    }

    pub fn block_at(&mut self, pos: V3i) -> &mut BlockState {
        &mut self.blocks[[pos.x, pos.y, pos.z]]
    }

    // pub fn building_at(&self, pos: V3i) -> Option<&Building> {
    //     self.buildings.iter().find(|&b| b.pos.contains(pos))
    // }

    pub fn layer(&mut self, layer: usize) -> Array2DRefMut<BlockState> {
        self.blocks.eliminated_dim_mut(2, layer)
    }
}