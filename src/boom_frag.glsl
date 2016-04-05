#version 150

varying vec2 uv;
uniform float effect_time;

const float GROW = 0.0;
const float THIN = 0.1;
const float DONE = 0.3;

const vec4 COLOR = vec4(1, 1, 0, 1);
const vec4 BLACK = vec4(0, 0, 0, 1);

out vec4 targ_color;

void grow(in float time)
{
    if (length(uv) < time * 0.9) {
        targ_color = vec4(1.0, 1.0, 1.0 - time, 1.0);
    } else {
        discard;
    }
}

void thin(in float time)
{
    float l = length(uv);
    float t = cos(asin(uv.y)) * (1.0 - time * 2.0);
    float s = (1.0 - time);

    if (l < 0.9 && uv.x < t) {
        targ_color = vec4(sqrt(s), s, 0, 1);
    }
    else if (abs(l - 0.9) < s / 10.0) {
        targ_color = vec4(sqrt(s), s, 0, 1);
    }
    else {
        discard;
    }
}

void main()
{
    if (effect_time < THIN) {
        grow((effect_time - GROW) / (THIN - GROW));
    }
    else if (effect_time < DONE) {
        thin((effect_time - THIN) / (DONE - THIN));
    }
    else {
        discard;
    }
}
