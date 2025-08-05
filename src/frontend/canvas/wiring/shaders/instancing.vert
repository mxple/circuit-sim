#version 100
attribute vec2 a_position;
attribute vec2 a_offset;
attribute float a_rotation;
varying vec2 v_uv;
uniform mat4 projection;

void main() {
    vec2 position = a_position + a_offset;

    float x_scale = 1. / pow(2., 10.);

    gl_Position = projection * vec4(position, 0., 1.);
    v_uv = a_position;
    v_uv.x *= x_scale;
    v_uv.x += a_rotation * x_scale;
}
