extern crate glium;

use std::collections::HashMap;
use std::rc::Rc;
use std::io;
use std::io::Read;
use std::fmt;
use std::error;
use std::fs;

#[derive(Debug)]
pub enum ResourceErrorKind {
    IoError(io::Error),
    ShaderCreationError(glium::program::ProgramCreationError)
}

impl fmt::Display for ResourceErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &ResourceErrorKind::IoError(ref err) =>
                write!(f, "IO Error: {}", err),
            &ResourceErrorKind::ShaderCreationError(ref err) =>
                write!(f, "Shader creation error: {}", err)
        }
    }
}

#[derive(Debug)]
pub struct ResourceError {
    kind: ResourceErrorKind,
    desc: String
}

impl error::Error for ResourceError {
    fn description(&self) -> &str {
        &self.desc
    }

    fn cause(&self) -> Option<&error::Error> {
        match &self.kind {
            &ResourceErrorKind::IoError(ref err) => Some(err),
            &ResourceErrorKind::ShaderCreationError(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.desc)
    }
}

impl From<io::Error> for ResourceError {
    fn from(err: io::Error) -> Self {
        let kind = ResourceErrorKind::IoError(err);
        let desc = kind.to_string();
        ResourceError {
            kind: kind,
            desc: desc
        }
    }
}

impl From<glium::program::ProgramCreationError> for ResourceError {
    fn from(err: glium::program::ProgramCreationError) -> Self {
        let kind = ResourceErrorKind::ShaderCreationError(err);
        let desc = kind.to_string();
        ResourceError {
            kind: kind,
            desc: desc
        }
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
        let cache_key = (vertex_shader.to_string(), fragment_shader.to_string());

        if let Some(program) = self.shader_cache.get(&cache_key) {
            return Ok(program.clone())
        }

        let vertex_src = try!(read_file(vertex_shader));
        let fragment_src = try!(read_file(fragment_shader));
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
