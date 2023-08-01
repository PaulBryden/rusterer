use std::time::Instant;

use rusterizer::framebuffer::Framebuffer;
use rusterizer::geometry::Mesh;
use minifb::{Key, Window, WindowOptions};
use rusterizer::renderer::Renderer;
use rusterizer::texture_helper::get_texture_from_bmp;
const WIDTH: usize = 480;
const HEIGHT: usize = 480;

fn main() {
    let mut framebuffer: Framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    let mesh_texture = get_texture_from_bmp(include_bytes!("../demo_objects/floating_islands_demo_texture.bmp"));
    let mut mesh_list: Vec<Mesh> = Vec::new();
    mesh_list.push(Mesh::new(&mesh_texture, include_bytes!("../demo_objects/floating_islands_demo.obj")));

    let mut renderer = Renderer::new(mesh_list, WIDTH, HEIGHT, 0x00ace6);

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
        if(window.is_key_down(Key::Up))
        {
            renderer.translate_camera_y(8.0*time_elapsed);
        }
        if(window.is_key_down(Key::Down))
        {
            renderer.translate_camera_y(-8.0*time_elapsed);
        }
        if(window.is_key_down(Key::W))
        {
            renderer.translate_camera_forward(8.0, time_elapsed);
        }
        if(window.is_key_down(Key::S))
        {
            renderer.translate_camera_backward(8.0, time_elapsed);
        }
        if(window.is_key_down(Key::A))
        {
            renderer.translate_yaw(-2.0*time_elapsed);
        }
        if(window.is_key_down(Key::D))
        {
            renderer.translate_yaw(2.0*time_elapsed);
        }

        renderer.render(time_elapsed, &mut framebuffer);

        window
            .update_with_buffer(framebuffer.get_framebuffer(), WIDTH, HEIGHT)
            .unwrap();
    }
}
