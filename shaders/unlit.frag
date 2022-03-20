#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragColor;
layout(location = 0) out vec4 outColor;

// Per-frame UBO
layout(binding = 0) uniform PerFrame {
    mat4 camera[2];
    float anim;
};

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    float i = fragColor.y;
    float colormix = pow(fragColor.z / 30, 2.0);

    float band = fract(i / 80. - anim * 0.3);
    float cutoff = 0.1;
    if (band > cutoff) discard;
    band /= cutoff;

    vec3 color = hsv2rgb(mix(
        mix(vec3(0.3, 0., band), vec3(0.3, 0.8, band), colormix),
        mix(vec3(0.5, 1., band), vec3(0.5, 1., band), colormix),
        colormix
    ));
    outColor = vec4(color, 1.0);
}
