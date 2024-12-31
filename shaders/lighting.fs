#version 330 core
out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;

uniform vec3 lightPos;
uniform vec3 viewPos;
uniform vec3 lightColor;
uniform vec3 objectColor;
uniform vec3 fogColor;
uniform vec3 ambientColor;

void main() {
    float dst = length(FragPos - viewPos);
    float dst_decay = pow(1.1, -dst);

    vec3 norm = normalize(Normal);
    vec3 ray = lightPos - FragPos;
    float rayDistance = length(ray);
    float lightStrength = pow(1.1, -rayDistance);
    vec3 lightDir = normalize(ray);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * lightColor * lightStrength;

    float specularStrength = 0.5;
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 reflectDir = reflect(-lightDir, norm);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.);
    vec3 specular = specularStrength * spec * lightColor * lightStrength;

    vec3 result = (1.0 - dst_decay) * fogColor + dst_decay * (diffuse + specular + ambientColor) * objectColor;
    FragColor = vec4(result, 1.0);
}