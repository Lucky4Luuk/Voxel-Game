extern crate glm;

pub fn get_mat4_array(m: glm::Matrix4<f32>) -> [f32; 16] {
    let mut r: [f32; 16] = [0.0; 16];
    r[0] = m.c0.x;
    r[1] = m.c0.y;
    r[2] = m.c0.z;
    r[3] = m.c0.w;

    r[4] = m.c1.x;
    r[5] = m.c1.y;
    r[6] = m.c1.z;
    r[7] = m.c1.w;

    r[8] = m.c2.x;
    r[9] = m.c2.y;
    r[10] = m.c2.z;
    r[11] = m.c2.w;

    r[12] = m.c3.x;
    r[13] = m.c3.y;
    r[14] = m.c3.z;
    r[15] = m.c3.w;

    return r;
}

pub fn length(v: glm::Vector3<f32>) -> f32 {
    return (v.x*v.x + v.y*v.y + v.z*v.z).sqrt();
}

pub fn normalize(v: glm::Vector3<f32>) -> glm::Vector3<f32> {
    let l = length(v);
    return v / l;
}

pub fn cross(a: glm::Vector3<f32>, b: glm::Vector3<f32>) -> glm::Vector3<f32> {
    return glm::Vector3::new(a.y * b.z - b.y * a.z, a.z * b.x - b.z * a.x, a.x * b.y - b.x * a.y);
}
