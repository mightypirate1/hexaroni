#version 100
precision lowp float;
uniform vec2 canvasSize;
uniform vec4 _Time;
uniform sampler2D Texture;

vec2 internalCoord() {
    // TODO: the 4.0 is `render_scale` from main...
    vec2 rawCoord = gl_FragCoord.xy / (4.0 * canvasSize.xy);
    return vec2(1, -1) * (-1.0 + 2. * rawCoord);
}

vec4 color(vec2 x) {
    return vec4(1.0 - 0.5 * vec3(1) * length(x), 1);
}

void main() {
    float time = _Time.x;
    vec2 coord = internalCoord();
    gl_FragColor = color(coord);
}
