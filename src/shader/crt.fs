#version 330

// Input vertex attributes (from vertex shader)
in vec3 fragPosition;
in vec2 fragTexCoord;
in vec4 fragColor;
in vec3 fragNormal;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;

// Output fragment color
out vec4 finalColor;

vec2 bulge(vec2 uv, vec2 center) {
  vec2 oguv = uv;
  uv -= center;
  
  uv *= pow(length(uv), 0.1); 
  
  uv += center;

  return uv;
}

void main() {
  vec2 center = vec2(0.5, 0.5);
  vec2 bulgeUV = bulge(fragTexCoord, center);
vec4 tex = texture2D(texture0, bulgeUV);
    finalColor.rgb = tex.rgb;
    finalColor.a = 1.0;
}