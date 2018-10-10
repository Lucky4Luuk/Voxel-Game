#version 450 core

layout (location = 0) in vec3 Position;

uniform mat4 mvp_mat;

void main()
{
    gl_Position = mvp_mat * vec4(Position, 1.0);
}
