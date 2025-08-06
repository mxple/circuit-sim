#version 100
precision lowp float;

attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color;

varying vec2 uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    // gl_Position = vec4(position, 1);
    uv = texcoord;
}
