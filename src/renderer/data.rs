use gl;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct f32_f32_f32 {
    pub d0: f32,
    pub d1: f32,
    pub d2: f32,
}
impl f32_f32_f32 {
    pub fn new(d0: f32, d1: f32, d2: f32) -> f32_f32_f32 {
        f32_f32_f32 {
            d0, d1, d2
        }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}
impl From<(f32, f32, f32)> for f32_f32_f32 {
    fn from(other: (f32, f32, f32)) -> Self {
        f32_f32_f32::new(other.0, other.1, other.2)
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct u2_u10_u10_u10_rev_float {
    data: u32,
}
impl u2_u10_u10_u10_rev_float {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> u2_u10_u10_u10_rev_float {
        let x = (clamp(x) * 1023f32).round() as u32;
        let y = (clamp(y) * 1023f32).round() as u32;
        let z = (clamp(z) * 1023f32).round() as u32;
        let w = (clamp(w) * 3f32).round() as u32;

        let mut c: u32 = 0;
        c |= w << 30;
        c |= z << 20;
        c |= y << 10;
        c |= x << 0;

        u2_u10_u10_u10_rev_float {
            data: c
        }
    }

    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        gl::EnableVertexAttribArray(location as gl::types::GLuint);
        gl::VertexAttribPointer(
            location as gl::types::GLuint,
            4, // the number of components per generic vertex attribute
            gl::UNSIGNED_INT_2_10_10_10_REV, // data type
            gl::TRUE, // normalized (int-to-float conversion)
            stride as gl::types::GLint,
            offset as *const gl::types::GLvoid
        );
    }
}
impl From<(f32, f32, f32, f32)> for u2_u10_u10_u10_rev_float {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        u2_u10_u10_u10_rev_float::new(
            other.0, other.1, other.2, other.3
        )
    }
}

#[inline]
fn clamp(c: f32) -> f32 {
    if c < 0.0 {
        return 0.0;
    }
    if c > 1.0 {
        return 1.0;
    }
    c
}
