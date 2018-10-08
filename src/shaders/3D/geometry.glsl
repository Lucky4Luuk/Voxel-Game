#version 450 core

layout(points) in;
layout(triangle_strip, max_vertices=3) out;

void main()
{
  vec4 pos_in = gl_in[0].gl_Position;
  gl_Position = pos_in + vec4(-1.0, -1.0, 0.0, 0.0);
  EmitVertex();
  gl_Position = pos_in + vec4(1.0, -1.0, 0.0, 0.0);
  EmitVertex();
  gl_Position = pos_in + vec4(0.0, 1.0, 0.0, 0.0);
  EmitVertex();
  EndPrimitive();
}
