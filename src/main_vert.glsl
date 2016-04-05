#version 150

in vec3 vertex_pos;
uniform mat4 shape_trans;


void main() {
    gl_Position = shape_trans * vec4(vertex_pos, 1);
}
