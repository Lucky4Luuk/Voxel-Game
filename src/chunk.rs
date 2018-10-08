#![feature(box_syntax)]
use std;
extern crate gl;

pub struct Chunk {
    pub voxels: std::boxed::Box<[[[u8; 128]; 128]; 128]>,
    pub vertices: Vec<f32>,
    pub vbo: gl::types::GLuint,
    pub vao: gl::types::GLuint,
}

impl Chunk {
    pub fn new() -> Chunk {
        let mut c = Chunk{voxels: box [[[0; 128]; 128]; 128], vertices: vec![0.0], vbo: 0, vao: 0};
        c.updateChunk();
        return c;
    }

    pub fn getVoxel(&self, x: usize, y: usize, z: usize) -> u8 {
        return self.voxels[x][y][z];
    }

    pub fn setVoxel(&mut self, x: usize, y: usize, z: usize, kind: u8) {
        self.voxels[x][y][z] = kind;
    }

    pub fn updateChunk(&mut self) {
        self.vertices = vec![];
        for x in 0..128 {
            for y in 0..128 {
                for z in 0..128 {
                    let mut next_to_air = false;
                    if x == 0 || y == 0 || z == 0 || x == 127 || y == 127 || z == 127 {
                        next_to_air = true;
                    }

                    if x > 0 && self.voxels[x as usize - 1][y as usize][z as usize] != 0 {
                        next_to_air = true;
                    }

                    if y > 0 && self.voxels[x as usize][y - 1][z as usize] != 0 {
                        next_to_air = true;
                    }

                    if z > 0 && self.voxels[x as usize][y as usize][z as usize - 1] != 0 {
                        next_to_air = true;
                    }

                    if x < 127 && self.voxels[x as usize + 1][y as usize][z as usize] != 0 {
                        next_to_air = true;
                    }

                    if y < 127 && self.voxels[x as usize][y as usize + 1][z as usize] != 0 {
                        next_to_air = true;
                    }

                    if z < 127 && self.voxels[x as usize][y as usize][z as usize + 1] != 0 {
                        next_to_air = true;
                    }

                    if next_to_air {
                        self.vertices.push(x as f32);
                        self.vertices.push(y as f32);
                        self.vertices.push(z as f32);
                    }
                    //self.vertices.push(self.voxels[x as usize][y as usize][z as usize] as f32);
                }
            }
        }
        //println!("{}", self.vertices.len() / 3);

        unsafe {
            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo); //bind buffer
            gl::BufferData(
                gl::ARRAY_BUFFER, //target
                (self.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, //size of data in bytes
                self.vertices.as_ptr() as *const gl::types::GLvoid, //pointer to data
                gl::STATIC_DRAW, //usage
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); //unbind buffer
        }

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao); //bind vertex array
            gl::EnableVertexAttribArray(0); //this is "layout (location = 0)" in vertex shader
            gl::VertexAttribPointer(
                0, //index of the generic vertex attribute ("layout (location = 0)")
                3, //the number of components per generic vertex attribute
                gl::FLOAT, //data type
                gl::FALSE, //normalized (int-to-float conversion)
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint, //stride (byte offset between consecutive attributes)
                std::ptr::null() //offset of the first component
            );
            gl::BindVertexArray(0);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(
                gl::POINTS, //mode
                0, //starting index in the enabled arrays
                (self.vertices.len() / 3) as i32 //number of indices to be rendered
            );
        }
    }
}
