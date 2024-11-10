#version 330
precision lowp float;
uniform vec2 canvas_size;
uniform vec4 _Time;
uniform float render_scale;

const int n_iters = 25;

vec2 internal_coord() {
    vec2 raw_coord = gl_FragCoord.xy / (render_scale * canvas_size.xy);
    vec2 im_coord = vec2(1, -1) * (-1.0 + 2. * raw_coord);
    return im_coord;
}

vec2 perturbance(float offset) {
    vec4 time = _Time + offset;
    return 0.44 * vec2(
            sin(0.43 * time.x),
            1.6 * cos(0.41 * time.x)
        );
}

vec2 imul(vec2 a, vec2 b) {
    a += perturbance(0.1);
    b -= perturbance(0.3);
    return vec2(
        a.x * b.x - a.y * b.y,
        a.x * b.y + a.y * b.x
    );
}

vec2 mandel_iter(vec2 z0, vec2 zi) {
    return imul(zi, zi) + z0;
}

vec4 color(int n, vec2 maxima, vec2 minima) {
    float x = float(n) / float(n_iters);
    float alpha = 0.01;
    float d1 = length(maxima.x - minima.x);
    float d2 = length(maxima.y - minima.y);
    float d = min(1.0, d1 + d2);
    vec4 fractal = 0.3 * d * x * vec4(1, 0, 0, 1);
    vec4 div1 = alpha / d1 * vec4(255, 222, 77, 255) / 255;
    vec4 div2 = alpha / d2 * vec4(230, 67, 250, 255) / 255;
    return fractal + div1 + div2;
}

vec4 mandel(vec2 x) {
    vec2 z0 = vec2(1, 1) * (x - vec2(0.3, 0));
    vec2 zi = z0 + perturbance(0.0);
    int n;
    int diverged_at = 0;
    vec2 maxima = vec2(-10, -10);
    vec2 minima = vec2(10, 10);
    for (n = 1; n < n_iters; n++) {
        zi = mandel_iter(z0, zi);
        maxima = vec2(max(zi.x, maxima.x), max(zi.y, maxima.y));
        minima = vec2(min(zi.x, minima.x), min(zi.y, minima.y));
        if (length(zi) >= 4.0) {
            diverged_at = n;
        }
    }
    return color(diverged_at, maxima, minima);
}

void main() {
    vec2 coord = internal_coord();
    gl_FragColor = 2.0 * mandel(coord);
}
