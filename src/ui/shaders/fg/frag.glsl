#version 330
precision lowp float;

uniform sampler2D Texture;

uniform vec2 canvas_size;
varying vec2 uv;
varying vec4 frag_color;
varying vec4 frag_pos;

vec4 color(vec4 color, float height, float center_dist) {
    float lid_shade = clamp(center_dist, 0.8, 1.0);
    lid_shade = (lid_shade - 0.4) / 0.4;
    float side_shade = height;
    vec3 shaded = lid_shade * side_shade * color.rgb;
    return vec4(shaded, 1.0);
}

void main() {
    vec4 texture_color = texture2D(Texture, uv);
    if (texture_color.rgb == vec3(1)) {
        // TODO: do this with 2 shaders instead of this dumb way...
        // explanation:
        //  if no texture is passed, the texture is implicitly white.
        //  thus, we can smuggle in objec colors as textures, and paint
        //  tiles by hand.
        float height = uv.y;
        float center_dist = uv.x;
        gl_FragColor = color(frag_color, height, center_dist);
    } else {
        gl_FragColor = texture_color;
    }
}
