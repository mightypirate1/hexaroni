#version 330
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

uniform vec4 _Time;
uniform mat4 Model;
uniform mat4 Projection;

varying vec2 uv;
varying vec4 frag_color;
varying vec4 frag_pos;

void main() {
    vec4 x = Projection * Model * vec4(position, 1);
    gl_Position = x;
    frag_color = color0 / 255.0;
    frag_pos = x;
    uv = texcoord;
}
