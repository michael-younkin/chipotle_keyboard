#version 140

in vec2 position;
in vec2 texcoords;
out vec2 v_texcoords;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_texcoords = texcoords;
}
