#version 330
attribute vec3 position;
attribute vec2 texcoord;

varying vec2 frag_uv;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    frag_uv = texcoord;
    gl_Position = Projection * Model * vec4(position, 1);
}
