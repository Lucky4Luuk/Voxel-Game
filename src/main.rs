#![feature(box_syntax)]

extern crate sdl2;
extern crate gl;
extern crate glm;

use std::os::raw;
use std::ffi::{CStr, CString};

pub mod render_gl;
pub mod chunk;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Voxel game yaes x)", 1280, 720)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut test = chunk::Chunk::new();
    test.setVoxel(0,0,0,1);

    unsafe {
        gl::Viewport(0, 0, 1280, 720); // set viewport
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    //  Create shaders                                                                           //
    ///////////////////////////////////////////////////////////////////////////////////////////////
    //Create default vertex shader
    let default_vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("shaders/default_vertex.glsl")).unwrap()
    ).unwrap();

    //Create default fragment shader
    let default_frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("shaders/default_fragment.glsl")).unwrap()
    ).unwrap();

    //////////////////////////Create default shader program
    let default_shader_program = render_gl::Program::from_shaders(
        &[default_vert_shader, default_frag_shader]
    ).unwrap();

    //Create 3D vertex shader
    let render_vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("shaders/3D/vertex.glsl")).unwrap()
    ).unwrap();

    let render_geom_shader = render_gl::Shader::from_geom_source(
        &CString::new(include_str!("shaders/3D/geometry.glsl")).unwrap()
    ).unwrap();

    //Create 3D fragment shader
    let render_frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("shaders/3D/fragment.glsl")).unwrap()
    ).unwrap();

    //////////////////////////Create 3D shader program
    let render_shader_program = render_gl::Program::from_shaders(
        &[render_vert_shader, render_geom_shader, render_frag_shader]
    ).unwrap();

    let proj_mat = render_gl::perspective(1.0, 720.0 / 1280.0, 0.02, 128.0);
    let proj_mat_loc = gl::GetUniformLocation(render_shader_program.id(), CString::new("proj_mat").unwrap().as_ptr());
    gl::UniformMatrix4fv(proj_mat_loc, 1, gl::FALSE, proj_mat);

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        render_shader_program.set_used();

        test.draw();

        //Render voxels
        //default_shader_program.set_used();

        //println!("reee");

        window.gl_swap_window();
    }
}
