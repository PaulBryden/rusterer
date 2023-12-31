use std::time::Instant;

use rusterer::framebuffer::Framebuffer;
use rusterer::geometry::{Mesh, AnimatedMesh};
use minifb::{Key, Window, WindowOptions};
use rusterer::renderer::Renderer;
use rusterer::texture_helper::get_texture_from_bmp;
const WIDTH: usize = 1600;
const HEIGHT: usize = 900;

fn main() {
    let mut framebuffer: Framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mesh_texture = get_texture_from_bmp(include_bytes!("../demo_objects/floating_islands_demo_texture.bmp"));
    let mut mesh_list: Vec<Mesh> = Vec::new();
    mesh_list.push(Mesh::new(&mesh_texture, include_bytes!("../demo_objects/floating_islands_demo.obj")));

    let mut animated_mesh_list: Vec<AnimatedMesh> = Vec::new();
    let mut animated_mech: Vec<Vec<u8>> = Vec::new();
    let human_frame_1 = include_bytes!("../demo_objects/low_poly_human/low_poly_human1.obj");
    let human_frame_2 = include_bytes!("../demo_objects/low_poly_human/low_poly_human2.obj");
    let human_frame_3 = include_bytes!("../demo_objects/low_poly_human/low_poly_human3.obj");
    let human_frame_4 = include_bytes!("../demo_objects/low_poly_human/low_poly_human4.obj");
    let human_texture = get_texture_from_bmp(include_bytes!("../demo_objects/low_poly_human/low_poly_human_texture.bmp"));
    animated_mech.push(human_frame_1.to_vec());
    animated_mech.push(human_frame_2.to_vec());    
    animated_mech.push(human_frame_3.to_vec());
    animated_mech.push(human_frame_4.to_vec());
    animated_mesh_list.push(AnimatedMesh::new(&human_texture,animated_mech,5.0, true));

    let mut renderer = Renderer::new(mesh_list, animated_mesh_list, WIDTH, HEIGHT, 0x00ace6);

    //Give some perspective for the demo.
    renderer.translate_camera_y(-12.0);
    renderer.translate_camera_backward(75.0,1.0);

    let mut window = Window::new(
        "Render Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut now = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {

        let elapsed_time = now.elapsed();
        now = Instant::now();
        let time_elapsed = elapsed_time.as_secs_f32();

        //Get Control Inputs
        if window.is_key_down(Key::Up)
        {
            renderer.translate_camera_y(8.0*time_elapsed);
        }
        if window.is_key_down(Key::Down)
        {
            renderer.translate_camera_y(-8.0*time_elapsed);
        }
        if window.is_key_down(Key::W)
        {
            renderer.translate_camera_forward(8.0, time_elapsed);
        }
        if window.is_key_down(Key::S)
        {
            renderer.translate_camera_backward(8.0, time_elapsed);
        }
        if window.is_key_down(Key::A)
        {
            renderer.translate_yaw(-2.0*time_elapsed);
        }
        if window.is_key_down(Key::D)
        {
            renderer.translate_yaw(2.0*time_elapsed);
        }

        renderer.render(time_elapsed, &mut framebuffer);

        window
            .update_with_buffer(framebuffer.get_framebuffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
