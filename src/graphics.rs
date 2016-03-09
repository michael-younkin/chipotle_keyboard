extern crate glium;

use std::error;
use std::fmt;
use std::rc::Rc;

use resources;

/// The error type for operations performed by a graphics API.
#[derive(Debug)]
pub enum GraphicsError {
    /// When the ResourceManager is unable to load the required resources (probably shaders).
    UnableToLoadResources(resources::ResourceError),
    /// When we couldn't make a vertex buffer of the required type.
    VertexBufferCreation(glium::vertex::BufferCreationError),
    /// When we couldn't make an index buffer of the required type.
    IndexBufferCreation(glium::index::BufferCreationError),
    /// When an error occurs when we try to draw something onto a surface.
    UnableToDraw(glium::DrawError)
}

impl error::Error for GraphicsError {
    fn description(&self) -> &str {
        match self {
            &GraphicsError::UnableToLoadResources(ref err) => err.description(),
            &GraphicsError::VertexBufferCreation(ref err) => err.description(),
            &GraphicsError::IndexBufferCreation(ref err) => err.description(),
            &GraphicsError::UnableToDraw(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &GraphicsError::UnableToLoadResources(ref err) => Some(err),
            &GraphicsError::VertexBufferCreation(ref err) => Some(err),
            &GraphicsError::IndexBufferCreation(ref err) => Some(err),
            &GraphicsError::UnableToDraw(ref err) => Some(err)
        }
    }
}

impl fmt::Display for GraphicsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &GraphicsError::UnableToLoadResources(ref err) =>
                write!(f, "Unable to load resources: {}", err),
            &GraphicsError::VertexBufferCreation(ref err) =>
                write!(f, "Vertex buffer creation failed: {}", err),
            &GraphicsError::IndexBufferCreation(ref err) =>
                write!(f, "Index buffer creation failed: {}", err),
            &GraphicsError::UnableToDraw(ref err) =>
                write!(f, "Drawing to a surface failed: {}", err)
        }
    }
}

impl From<resources::ResourceError> for GraphicsError {
    fn from(e: resources::ResourceError) -> Self {
        GraphicsError::UnableToLoadResources(e)
    }
}

impl From<glium::vertex::BufferCreationError> for GraphicsError {
    fn from(e: glium::vertex::BufferCreationError) -> Self {
        GraphicsError::VertexBufferCreation(e)
    }
}

impl From<glium::index::BufferCreationError> for GraphicsError {
    fn from(e: glium::index::BufferCreationError) -> Self {
        GraphicsError::IndexBufferCreation(e)
    }
}

impl From<glium::DrawError> for GraphicsError {
    fn from(e: glium::DrawError) -> Self {
        GraphicsError::UnableToDraw(e)
    }
}

/// Represents an RGBA color.
#[derive(Copy, Clone, Debug)]
pub struct RGBAColor {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8
}

impl RGBAColor {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> RGBAColor {
        RGBAColor {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha
        }
    }
}

impl Into<[f32; 4]> for RGBAColor {
    fn into(self) -> [f32; 4] {
        [
            self.red as f32 / 255.0,
            self.green as f32 / 255.0,
            self.blue as f32 / 255.0,
            self.alpha as f32 / 255.0
        ]
    }
}

/// Represents a 2D rectangle.
#[derive(Copy, Clone, Debug)]
pub struct Rect2D {
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

impl Rect2D {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect2D {
        Rect2D {
            x: x,
            y: y,
            width: width,
            height: height
        }
    }

    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn top(&self) -> f32 {
        self.y + self.height
    }

    pub fn bottom(&self) -> f32 {
        self.y
    }
}

#[derive(Copy, Clone, Debug)]
struct ImmediateVertex {
    position: [f32; 2],
    texcoords: [f32; 2]
}
implement_vertex!(ImmediateVertex, position, texcoords);

/// Immediate mode graphics API.
pub struct ImmediateGraphics {
    vertex_buffer: glium::VertexBuffer<ImmediateVertex>,
    index_buffer: glium::IndexBuffer<u8>,
    color_shader: Rc<glium::Program>,
    circle_color_shader: Rc<glium::Program>
}

impl ImmediateGraphics {
    pub fn new(
            gl_display: &glium::Display,
            resource_manager: &mut resources::ResourceManager) 
            -> Result<ImmediateGraphics, GraphicsError> {
        Ok(ImmediateGraphics {
            vertex_buffer: try!(glium::VertexBuffer::empty_persistent(gl_display, 4)),
            index_buffer: try!(glium::IndexBuffer::empty_persistent(
                    gl_display, glium::index::PrimitiveType::TrianglesList, 6)),
            color_shader: try!(resource_manager.load_shader(
                    "identity_vertex_shader.glsl",
                    "color_fragment_shader.glsl")),
            circle_color_shader: try!(resource_manager.load_shader(
                    "identity_vertex_shader.glsl",
                    "color_circle_fragment_shader.glsl")),
        })
    }

    fn setup_square_vertex_buffer(&self, dest: Rect2D) {
        let buffer_contents = [
            ImmediateVertex { position: [dest.left(), dest.top()], texcoords: [0.0, 1.0] },
            ImmediateVertex { position: [dest.left(), dest.bottom()], texcoords: [0.0, 0.0] },
            ImmediateVertex { position: [dest.right(), dest.bottom()], texcoords: [1.0, 0.0] },
            ImmediateVertex { position: [dest.right(), dest.top()], texcoords: [1.0, 1.0] },
        ];
        self.vertex_buffer.write(&buffer_contents);
    }

    fn setup_square_index_buffer(&self) {
        let buffer_contents: [u8; 6] = [0, 1, 2, 2, 3, 0];
        self.index_buffer.write(&buffer_contents);
    }

    pub fn draw_square<T>(&self, target: &mut T, color: RGBAColor, dest: Rect2D)
            -> Result<(), GraphicsError>
            where T : glium::Surface {
        self.setup_square_vertex_buffer(dest);
        self.setup_square_index_buffer();
        let color_array: [f32; 4] = color.into();
        let uniforms = uniform! { tint: color_array };
        try!(target.draw(&self.vertex_buffer,
                         &self.index_buffer,
                         &self.color_shader,
                         &uniforms,
                         &Default::default()));
        Ok(())
    }

    pub fn draw_circle<T>(&self, target: &mut T, color: RGBAColor, dest: Rect2D)
            -> Result<(), GraphicsError>
            where T : glium::Surface {
        self.setup_square_vertex_buffer(dest);
        self.setup_square_index_buffer();
        let color_array: [f32; 4] = color.into();
        let uniforms = uniform! { tint: color_array };
        try!(target.draw(&self.vertex_buffer,
                         &self.index_buffer,
                         &self.circle_color_shader,
                         &uniforms,
                         &Default::default()));
        Ok(())
    }
}
