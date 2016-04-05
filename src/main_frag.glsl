#version 150

uniform vec4 shape_color;

out vec4 targ_color;

void main() {
    targ_color = shape_color;
}
