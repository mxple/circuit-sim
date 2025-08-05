#version 100
precision mediump float;

varying vec2 uv;

uniform vec2 u_view_min;
uniform vec2 u_view_max;
uniform vec2 u_resolution; 
uniform vec4 u_color;

void main() {
    // Convert uv to world space
    vec2 world_pos = mix(u_view_min, u_view_max, uv);

    // Compute pixel size in world units
    vec2 world_size = u_view_max - u_view_min;
    vec2 pixel_size = world_size / u_resolution;

    float pixel_thickness = min(pixel_size.x, pixel_size.y);

    // Distance to nearest grid line
    vec2 grid = min(mod(world_pos, 1.0), 1.0 - mod(world_pos, 1.0));
    float min_dist = min(grid.x, grid.y);

    // 1-pixel line: fade when near center of line
    float alpha = smoothstep(pixel_thickness, 0.0, min_dist);

    gl_FragColor = vec4(u_color.rgb, alpha);
}
