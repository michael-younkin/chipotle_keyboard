#[macro_use]
extern crate glium;

use glium::DisplayBuild;
use glium::Surface;

mod resources;
mod graphics;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}
implement_vertex!(Vertex, position);

fn main() {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    // Setup a test triangle
    //let vertex_1 = Vertex { position: [-0.5, -0.5, 0.0] };
    //let vertex_2 = Vertex { position: [0.0, 0.5, 0.0] };
    //let vertex_3 = Vertex { position: [0.5, -0.25, 0.0] };
    //let triangle = vec![vertex_1, vertex_2, vertex_3];
    //let vertex_buffer  = glium::VertexBuffer::new(&display, &triangle).unwrap();
    //let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut resource_manager = resources::ResourceManager::new(&display);
    //let program = resource_manager.load_shader(
    //    "assets/shaders/identity_vertex_shader.glsl",
    //    "assets/shaders/color_fragment_shader.glsl").unwrap();
    let graphics = graphics::ImmediateGraphics::new(&display, &mut resource_manager).unwrap();

    loop {
        // Handle window events
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }

        // Update
        // TODO

        // Draw
        let mut target = display.draw();
        target.clear_color(100.0/255.0, 149.0/255.0, 237.0/255.0, 1.0);
        graphics.draw_square(&mut target,
                             graphics::RGBAColor::new(255,255,255,255),
                             graphics::Rect2D::new(-0.5, -0.5, 1.0, 1.0)).unwrap();
        target.finish().unwrap();
    }
}
