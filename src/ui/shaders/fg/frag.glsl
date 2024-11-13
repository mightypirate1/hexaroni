#version 330
precision lowp float;

uniform sampler2D Texture;

varying vec4 frag_pos;
varying vec3 frag_normal;
varying vec2 frag_uv;
varying vec4 frag_color;
varying float frag_glow;
varying vec3 v_frag_to_light;
varying vec3 v_frag_to_cam;

vec4 raw_color(vec4 color, float height, float center_dist) {
    // lid shade
    float clamp_val = 0.6 + 0.2 * frag_glow;
    float lid_shade = clamp(center_dist, clamp_val, 1.0);
    lid_shade = (lid_shade - 0.4) / (clamp_val);
    // side shade
    float side_shade = height;
    side_shade += max(height, 0.7) - 0.7;
    vec3 shaded = lid_shade * side_shade * color.rgb;
    return vec4(shaded, 1.0);
}

vec4 base_color() {
    float height = frag_uv.y;
    float center_dist = frag_uv.x;
    vec4 texture_color = texture2D(Texture, frag_uv);
    if (texture_color.rgb == vec3(1)) {
        // TODO: do this with 2 shaders instead of this dumb way...
        // explanation:
        //  if no texture is passed, the texture is implicitly white.
        //  thus, we can smuggle in objec colors as textures, and paint
        //  tiles by hand.
        float height = frag_uv.y;
        float center_dist = frag_uv.x;
        return raw_color(frag_color, height, center_dist);
    } else {
        return texture_color;
    }
}

float diffuse_weight(vec3 normal, vec3 to_light) {
    return clamp(dot(normal, to_light), 0.0, 1.0);
}

float specular_weight(vec3 normal, vec3 to_light, vec3 to_cam, float shininess) {
    vec3 reflected_light = reflect(-to_light, normal);
    float spec_angle = max(dot(reflected_light, to_cam), 0.0);
    return pow(spec_angle, shininess);
}

void main() {
    vec3 normal = normalize(frag_normal);
    vec3 to_light = normalize(v_frag_to_light);
    vec3 to_cam = normalize(v_frag_to_cam);

    vec4 color = base_color();
    const float c_ambient = 0.6;
    float c_diffuse = 2.8 * diffuse_weight(normal, to_light);
    float c_specular = 0.2 * specular_weight(normal, to_light, to_cam, 98.0);

    vec3 shaded_color = color.rgb * max(frag_glow, c_ambient + c_diffuse);
    vec3 specular_color = (1 - 0.8 * frag_glow) * c_specular * vec3(1);

    gl_FragColor = vec4(shaded_color + specular_color, 1);
    // gl_FragColor = vec4(normal, 1);
}
