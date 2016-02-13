#version 110

attribute vec2 vertex_pos;
uniform mat2 view_matrix;

void main() {
    gl_Position = vec4(vertex_pos * view_matrix, 0, 0);
}
