#version 100
attribute vec2 a_position;
attribute vec2 a_offset;
attribute float a_rotation;
varying vec2 v_uv;
uniform mat4 projection;

void main() {
    vec2 position = a_position + a_offset;
    
    vec2 center = a_offset + 0.5;
    vec2 pos = position - center;

    float angle = radians(90.0) * a_rotation;
    float cos_a = cos(angle);
    float sin_a = sin(angle);

    mat2 rot = mat2(
        cos_a, -sin_a,
        sin_a,  cos_a
    );

    vec2 rotated = rot * pos + center;

    gl_Position = projection * vec4(rotated, 0., 1.);
    v_uv = a_position;
}
