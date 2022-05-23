#version 430 core

out vec4 colorOut;
in vec2 fUVs;

void main() {
    colorOut = vec4(1.0, fUVs, 1.0);
}