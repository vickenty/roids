#version 110

attribute vec3 vertex_pos;
uniform mat3 shape_trans;

void main() {
    gl_Position = vec4(shape_trans * vertex_pos, 1);
}
