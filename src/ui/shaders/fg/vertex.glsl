#version 330
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
attribute vec4 normal;

uniform vec4 _Time;
uniform mat4 Model;
uniform mat4 Projection;
uniform vec3 light_pos;
uniform vec3 cam_pos;

varying vec4 frag_pos;
varying vec3 frag_normal;
varying vec2 frag_uv;
varying vec4 frag_color;
varying float frag_glow;
varying vec3 v_frag_to_light;
varying vec3 v_frag_to_cam;

void main() {
    // forward values
    vec4 vtx_world_pos = Model * vec4(position, 1);
    vec4 light_world_pos = Model * vec4(light_pos, 1);
    vec4 cam_world_pos = Model * vec4(cam_pos, 1);

    frag_normal = normal.xyz;
    frag_glow = normal.w;
    frag_uv = texcoord;
    v_frag_to_light = (light_world_pos - vtx_world_pos).xyz;
    v_frag_to_cam = (cam_world_pos - vtx_world_pos).xyz;
    frag_color = vec4(color0.rgb / 255.0, 1);

    // set vertex position
    gl_Position = Projection * vtx_world_pos;
}
