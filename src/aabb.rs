#[derive(Debug, Clone, Copy)]
pub struct V2i {
    pub x: usize,
    pub y: usize
}

impl V2i {
    pub fn to_v3i(&self) -> V3i {
        V3i{ x: self.x, y: self.y, z: 0 }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct V3i {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

pub struct AABB {
    pub pos: V3i,
    pub size: V3i
}

impl AABB {
    pub fn contains(&self, pt: V3i) -> bool {
        if pt.x < self.pos.x { return false; }
        else if pt.y < self.pos.y { return false; }
        else if pt.z < self.pos.z { return false; }
        else if pt.x > self.pos.x + self.size.x { return false; }
        else if pt.y > self.pos.y + self.size.y { return false; }
        else if pt.z > self.pos.z + self.size.z { return false; }
        else { return true; }
    }
}