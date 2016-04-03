#version 110

varying vec2 uv;
uniform float effect_time;

const float GROW = 0.0;
const float THIN = 0.1;
const float DONE = 0.2;

const vec4 COLOR = vec4(1, 1, 0, 1);
const vec4 BLACK = vec4(0, 0, 0, 1);

void grow(in float time)
{
    if (length(uv) < time) {
        gl_FragColor = vec4(1.0, 1.0, 1.0 - time, 1.0);
    } else {
        discard;
    }
}

void thin(in float time)
{
    float l = length(uv);
    float t = cos(asin(uv.y)) * (1.0 - time * 2.0);
    float s = (1.0 - time);

    if (l < 1.0 && uv.x < t) {
        gl_FragColor = vec4(sqrt(s), s, 0, 1);
    }
    else if (abs(l - 1.0) < s / 10.0) {
        gl_FragColor = vec4(sqrt(s), s, 0, 1);
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
