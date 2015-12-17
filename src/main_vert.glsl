#version 110

attribute vec2 vertex_pos;
uniform mat4 view_matrix;
uniform mat4 proj_matrix;

void main() {
    gl_Position = vertex_pos * mat;
}
