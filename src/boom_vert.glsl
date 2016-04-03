#version 110

attribute vec3 vertex_pos;
attribute vec2 texture_pos;
uniform mat3 shape_trans;
varying vec2 uv;

void main() {
    gl_Position = vec4(shape_trans * vertex_pos, 1);
    uv = texture_pos;
}
