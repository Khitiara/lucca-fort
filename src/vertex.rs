use glium::Surface;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

#[derive(Copy, Clone)]
pub struct Quad {
    top_left: Vertex,
    bottom_right: Vertex,
}

impl Quad {
    pub fn make_vert_buf(&self, display: &glium::Display) -> glium::VertexBuffer<Vertex> {
        let bottom_left = Vertex { position: [self.top_left.position[0], self.bottom_right.position[1]],
                                   tex_coords: [self.top_left.tex_coords[0], self.bottom_right.tex_coords[1]]};
        let top_right = Vertex { position: [self.bottom_right.position[0], self.top_left.position[1]],
                                   tex_coords: [self.bottom_right.tex_coords[0], self.top_left.tex_coords[1]]};
        let shape = vec![self.top_left, top_right, bottom_left, top_right, bottom_left, self.bottom_right];
        return glium::VertexBuffer::new(display, &shape).unwrap();
    }
    pub fn make_index_buf(&self) -> glium::index::NoIndices {
        return glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    }

    pub fn draw<U>(&self, frame: &mut glium::Frame, display: &glium::Display, program: &glium::Program, uniforms: &U)
        where U: glium::uniforms::Uniforms {
        frame.draw(&self.make_vert_buf(display), &self.make_index_buf(), program, uniforms, &Default::default()).unwrap();
    }
}