#version 110

attribute vec2 vertex_pos;
uniform mat2 shape_trans;

void main() {
    vec2 pos = shape_trans * vertex_pos;
    gl_Position = vec4(pos, 0, 1);
}
