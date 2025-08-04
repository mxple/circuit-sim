#version 100
precision mediump float;
varying vec2 v_uv;

void main() {
    // gl_FragColor = texture2D(u_texture, v_uv);
    // gl_FragColor = vec4(v_uv, 0., 1.);
    // gl_FragColor = vec4(0.2);
    gl_FragColor = vec4(1., 0., 0., 1.);
}
