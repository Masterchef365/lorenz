#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragColor;
layout(location = 0) out vec4 outColor;

// Per-frame UBO
layout(binding = 0) uniform PerFrame {
    mat4 camera[2];
    float anim;
};

void main() {
    float i = fragColor.y;
    vec3 color = vec3(fract(i / 80. + anim));
    outColor = vec4(color, 1.0);
}
