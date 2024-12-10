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

#define screenScale (640.0 * 480.0) / ((1440.0 * 1080.0) / 1.8)
#define CURVATURE_X 0.10
#define CURVATURE_Y 0.25
vec2 CURVATURE_DISTORTION = vec2(CURVATURE_X, CURVATURE_Y);
vec2 barrelScale = 1.0 - (0.23 * CURVATURE_DISTORTION);

vec2 bulge(vec2 uv) {
  vec2 oguv = uv;

  uv *= screenScale;
  uv -= vec2(0.5);
  float rsq = uv.x * uv.x + uv.y * uv.y;
  uv += uv * (CURVATURE_DISTORTION * rsq);
  uv *= barrelScale;
  uv += vec2(0.5);
  uv /= screenScale;

  return uv;
}

void main() {
  vec2 bulgeUV = bulge(fragTexCoord);
  vec4 tex = texture2D(texture0, bulgeUV);
  finalColor.rgb = tex.rgb;
  if (int(fragTexCoord.y * 480) % 2 == 1) {
    finalColor.rgb *= 0.7;
  }
  finalColor.a = 1.0;
}