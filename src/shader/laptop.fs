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
uniform int camera_changing;

// Output fragment color
out vec4 finalColor;

float mod289(float x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 mod289(vec4 x){return x - floor(x * (1.0 / 289.0)) * 289.0;}
vec4 perm(vec4 x){return mod289(((x * 34.0) + 1.0) * x);}

float noise(vec3 p, float scale){
    p *= scale; // Tweak for allowing scale of noise
    vec3 a = floor(p);
    vec3 d = p - a;
    d = d * d * (3.0 - 2.0 * d);

    vec4 b = a.xxyy + vec4(0.0, 1.0, 0.0, 1.0);
    vec4 k1 = perm(b.xyxy);
    vec4 k2 = perm(k1.xyxy + b.zzww);

    vec4 c = k2 + a.zzzz;
    vec4 k3 = perm(c);
    vec4 k4 = perm(c + 1.0);

    vec4 o1 = fract(k3 * (1.0 / 41.0));
    vec4 o2 = fract(k4 * (1.0 / 41.0));

    vec4 o3 = o2 * d.z + o1 * (1.0 - d.z);
    vec2 o4 = o3.yw * d.x + o3.xz * (1.0 - d.x);

    return o4.y * d.y + o4.x * (1.0 - d.y);
}

void main() {
  vec4 tex = texture2D(texture0, fragTexCoord);

  if (camera_changing == 1) {
    float n = noise(vec3(fragTexCoord.x, fragTexCoord.y, rand / 1000000.0), width / ((width / 1440.0) * 4));
    finalColor.rgb = vec3(n,n,n);
  } else {
    float n = noise(vec3(fragTexCoord.x, fragTexCoord.y, rand / 1000000.0), width / ((width / 1440.0) * 4)) / 8.0;
    finalColor.rgb = tex.rgb - n;
  }
  finalColor.a = 1.0;
}