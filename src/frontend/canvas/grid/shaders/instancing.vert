#version 100

// Vertex attributes
attribute vec2 a_position;    // Line vertex position (0.0, 0.0) or (1.0, 0.0)

// Instance attributes
attribute vec2 a_start_pos;   // Start position of the line in world space
attribute vec2 a_end_pos;   // End position of the line in world space

uniform mat4 projection;

void main() {
    // Interpolate between start and end position based on vertex position
    vec2 world_pos = mix(a_start_pos, a_end_pos, a_position.x);
    
    gl_Position = projection * vec4(world_pos, 0.0, 1.0);
}
