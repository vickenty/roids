#version 110

attribute vec3 vertex_pos;
uniform mat4 shape_trans;
varying vec2 uv;

void main() {
    gl_Position = shape_trans * vec4(vertex_pos, 1);
    uv = vertex_pos.xy;
}
