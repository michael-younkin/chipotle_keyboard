extern crate glium;

use std::collections::HashMap;
use std::rc::Rc;
use std::io;
use std::io::Read;
use std::fmt;
use std::error;
use std::fs;

/// The error type for operations performed by the ResourceManager.
#[derive(Debug)]
pub enum ResourceError {
    /// The operation failed due to an IO related error.
    Io(io::Error),
    /// ResourceError::Shader program creation failed (at the glium/OpenGL level).
    Shader(glium::program::ProgramCreationError)
}

impl error::Error for ResourceError {
    fn description(&self) -> &str {
        match self {
            &ResourceError::Io(ref err) => err.description(),
            &ResourceError::Shader(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &ResourceError::Io(ref err) => Some(err),
            &ResourceError::Shader(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ResourceError::Io(ref err) => write!(f, "IO Error: {}", err),
            &ResourceError::Shader(ref err) => write!(f, "Shader Error: {}", err)
        }
    }
}

impl From<glium::program::ProgramCreationError> for ResourceError {
    fn from(e: glium::program::ProgramCreationError) -> Self {
        ResourceError::Shader(e)
    }
}

impl From<io::Error> for ResourceError {
    fn from(e: io::Error) -> Self {
        ResourceError::Io(e)
    }
}

pub struct ResourceManager<'a> {
    shader_cache: HashMap<(String, String), Rc<glium::Program>>,
    gl_display: &'a glium::Display
}

impl<'a> ResourceManager<'a> {
    pub fn new(gl_display: &'a glium::Display) -> ResourceManager<'a> {
        ResourceManager {
            shader_cache: HashMap::new(),
            gl_display: gl_display
        }
    }

    pub fn load_shader(&mut self, vertex_shader: &str, fragment_shader: &str)
            -> Result<Rc<glium::Program>, ResourceError> {
        let vertex_shader_fname = "assets/shaders/".to_string() + vertex_shader;
        let fragment_shader_fname = "assets/shaders/".to_string() + fragment_shader;

        let cache_key = (vertex_shader_fname.clone(), fragment_shader_fname.clone());
        if let Some(program) = self.shader_cache.get(&cache_key) {
            return Ok(program.clone())
        }

        let vertex_src = try!(read_file(&vertex_shader_fname));
        let fragment_src = try!(read_file(&fragment_shader_fname));
        let program = Rc::new(
            try!(glium::Program::from_source(self.gl_display, &vertex_src, &fragment_src, None)));
        self.shader_cache.insert(cache_key, program.clone());
        Ok(program)
    }
}

impl<'a> fmt::Debug for ResourceManager<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "ResourceManager {{ shader_cache: {:?}, gl_display: glutin_display",
               self.shader_cache)
    }
}

fn read_file(name: &str) -> Result<String, ResourceError> {
    let mut buf = String::new();
    let mut f = try!(fs::File::open(name));
    try!(f.read_to_string(&mut buf));
    Ok(buf)
}
