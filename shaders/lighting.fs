#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;

#define MAX_LIGHTS 225
#define SPECULAR_STRENGTH 0.5
#define LIGHT_DECAY 1.2
#define DST_DECAY 1.075

uniform vec3 lightPositions[MAX_LIGHTS];
uniform vec3 lightColors[MAX_LIGHTS];
uniform vec3 viewPos;
uniform vec3 objectColor;
uniform vec3 fogColor;
uniform vec3 ambientColor;
uniform uint lights;

void main() {
    vec3 norm = normalize(Normal);
    vec3 diffuse_acc = vec3(0.);
    vec3 specular_acc = vec3(0.);

    for (uint i = uint(0); i < lights; ++i) {
        vec3 ray = lightPositions[i] - FragPos;
        float lightStrength = pow(LIGHT_DECAY, -length(ray));

        if (lightStrength < 0.01) {
            continue;
        }

        vec3 lightDir = normalize(ray);
        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * lightColors[i] * lightStrength;

        vec3 viewDir = normalize(viewPos - FragPos);
        vec3 reflectDir = reflect(-lightDir, norm);
        float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.);
        vec3 specular = SPECULAR_STRENGTH * spec * lightColors[i] * lightStrength;

        diffuse_acc += diffuse;
        specular_acc += specular;
    }

    float dst = length(FragPos - viewPos);
    float dst_decay = pow(DST_DECAY, -dst);

    vec3 result = (1.0 - dst_decay) * fogColor
        + dst_decay * (diffuse_acc + specular_acc + ambientColor) * objectColor;
    FragColor = vec4(result, 1.0);
}