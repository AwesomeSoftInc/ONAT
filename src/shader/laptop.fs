#version 330

// Input vertex attributes (from vertex shader)
in vec3 fragPosition;
in vec2 fragTexCoord;
in vec4 fragColor;
in vec3 fragNormal;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;
uniform int rand;

uniform int width;
uniform int height;

// Output fragment color
out vec4 finalColor;

float noise(vec2 co){
    return fract(sin(dot(co, vec2(float(rand / 1000000.0), 78.233))) * 43758.5453);
}

void main() {
  float scale = width / 1440;
  vec4 tex = texture2D(texture0, fragTexCoord);
  float n = noise(fragTexCoord.xy);
  finalColor.rgb = tex.rgb * n;
  finalColor.a = 1.0;
}