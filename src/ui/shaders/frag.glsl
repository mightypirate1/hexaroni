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

vec2 imul(vec2 a, vec2 b) {
    return vec2(
        a.x * b.x - a.y * b.y,
        a.x * b.y + a.y * b.x
    );
}

vec2 mandel_iter(vec2 z0, vec2 zi) {
    return imul(zi, zi) + z0;
}

vec4 color(int n, vec2 maxima, vec2 minima, vec4 time) {
    float x = 1.0 / sqrt(float(n));
    float d = length(maxima - minima);
    return vec4(0.2 * x, 0.0, 1.0 - d, 1.);
}

vec2 perturbance(vec4 time) {
    return 0.2 * vec2(
            0.6 * time.z * sin(1.41 * time.x),
            6.0 * sin(0.2 * time.x)
        );
}

vec4 mandel(vec2 x, vec4 time) {
    vec2 z0 = vec2(1, 1) * (x - vec2(0.3, 0));
    vec2 zi = z0 + perturbance(time);
    int n;
    int diverged_at = 1;
    vec2 maxima = vec2(-10, -10);
    vec2 minima = vec2(10, 10);
    for (n = 1; n < 25; n++) {
        zi = mandel_iter(z0, zi);
        maxima = vec2(max(zi.x, maxima.x), max(zi.y, maxima.y));
        minima = vec2(min(zi.x, minima.x), min(zi.y, minima.y));
        if (length(zi) >= 2.0) {
            diverged_at = n;
        }
    }
    return color(diverged_at, maxima, minima, time);
}

void main() {
    vec4 time = _Time;
    vec2 coord = internalCoord();
    gl_FragColor = mandel(coord, time);
}
