#version 140

uniform vec4 tint;
in vec2 v_texcoords;
out vec4 color;

void main() {
    float u = v_texcoords[0] - 0.5;
    float v = v_texcoords[1] - 0.5;
    if (sqrt(pow(u, 2) + pow(v, 2)) > .5) {
        discard;
    }
    color = tint;
}
