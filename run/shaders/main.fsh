#version 430 core

out vec4 colorOut;
in vec2 fUVs;

uniform vec4 uColor;

void main() {
    colorOut = uColor;
}