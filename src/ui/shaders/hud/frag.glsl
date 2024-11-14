#version 330

#define M_PI 3.1415926535897932384626433832795

precision lowp float;

uniform vec4 _Time;
uniform vec2 canvas_size;
uniform float render_scale;
uniform float frac_remaining;
uniform vec3 fill_color;
uniform float flipped;

varying vec2 frag_uv;

float radial_factor(float x, float y) {
    float r = x * x + y * y;
    if (r > 0.9) {
        return 0.0;
    }
    float border_thickness = 0.05;
    float radial_factor = 1.0 - max(x * x + y * y, (1.0 - border_thickness));
    return radial_factor /= border_thickness;
}

float angular_factor(float x, float y) {
    float angle = M_PI + atan(y, x);
    float f = 1.0; // 2.0 * flipped - 1.0;
    if (frac_remaining < 0.5) {
        return float(f * angle / (4.0 * M_PI) < f * frac_remaining);
    }
    angle = 2 * M_PI - angle;
    return float(f * angle / (4.0 * M_PI) < 1.0 - f * frac_remaining);
}

void main() {
    float x = 2.0 * (frag_uv.x - 0.5);
    float y = 2.0 * (frag_uv.y - 0.5);
    float r_fac = radial_factor(x, y);
    float a_fac = angular_factor(x, y);
    vec3 color = r_fac * a_fac * fill_color;
    gl_FragColor = vec4(color, 1);
}
