use basenum::BaseFloat;
use builtin::{ cross, dot, normalize };
use traits::GenFloat;
use num;
use mat::mat::{ Matrix3, Matrix4 };
use vec::vec::{ Vector3, Vector4 };

/// Builds a translation 4 * 4 matrix created from a vector of 3 components.
///
/// Uses input matrix `m` and translation vector coordinates `v`.
///
/// # Example
///
/// ```rust
/// # extern crate num;
/// # extern crate glm;
/// # fn main() {
/// use glm::*;
/// use glm::ext::*;
///
/// let matrix = translate(&num::one(), vec3(1., 2., 3.));
/// assert_eq!(matrix[3], vec4(1., 2., 3., 1.));
/// # }
/// ```
#[inline]
pub fn translate<P>(
    m: &Matrix4<P>,
    v: Vector3<P>
) -> Matrix4<P>
where
    P : BaseFloat
{
    Matrix4::new(
        m.c0, m.c1, m.c2,
        m.c0 * v.x + m.c1 * v.y + m.c2 * v.z + m.c3)
}

#[inline]
pub fn perspective<P>(
    fov_y: P,
    aspect: P,
    z_near: P,
    z_far: P
) -> Matrix4<P>
where
    P : BaseFloat
{
    let zero = num::zero::<P>();
    let one = num::one::<P>();
    let two = one + one;
    let q = one / (fov_y / two).tan();
    let a = q / aspect;
    let b = (z_near + z_far) / (z_near - z_far);
    let c = (two * z_near * z_far) / (z_near - z_far);

    Matrix4::new(
        Vector4::new(   a, zero, zero, zero),
        Vector4::new(zero,    q, zero, zero),
        Vector4::new(zero, zero,    b, zero - one),
        Vector4::new(zero, zero,    c, zero)
    )
}

#[inline]
pub fn rotate<P>(
    m: Matrix4<P>,
    angle: P,
    v: Vector3<P>
) -> Matrix4<P>
where
    P : BaseFloat + GenFloat<P>
{
    let zero = num::zero::<P>();
    let one = num::one::<P>();

    let a = angle;
    let (s, c) = a.sin_cos();
    let axis = normalize(v);
    let temp = axis * (one - c);

    let rotate = Matrix3::new(
        Vector3::new(
            c + temp.x * axis.x,
            temp.x * axis.y + s * axis.z,
            temp.x * axis.z - s * axis.y),
        Vector3::new(
            temp.y * axis.x - s * axis.z,
            c + temp.y * axis.y,
            temp.y * axis.z + s * axis.x),
        Vector3::new(
            temp.z * axis.x + s * axis.y,
            temp.z * axis.y - s * axis.x,
            c + temp.z * axis.z)
        );

    Matrix4::new(
		m.c0 * rotate.c0.x + m.c1 * rotate.c0.y + m.c2 * rotate.c0.z,
		m.c0 * rotate.c1.x + m.c1 * rotate.c1.y + m.c2 * rotate.c1.z,
		m.c0 * rotate.c2.x + m.c1 * rotate.c2.y + m.c2 * rotate.c2.z,
		m.c3
        )
}

#[inline]
pub fn look_at<P>(
    eye: Vector3<P>,
    center: Vector3<P>,
    up: Vector3<P>
) -> Matrix4<P>
where
    P : BaseFloat + GenFloat<P>
{
    let zero = num::zero::<P>();
    let one = num::one::<P>();
    let f = normalize(center - eye);
    let up_n = normalize(up);
    let s = cross(f, up_n);
    let u = cross(s, f);
    Matrix4::new(
        Vector4::new(s.x, u.x,-f.x, zero),
        Vector4::new(s.y, u.y,-f.y, zero),
        Vector4::new(s.z, u.z,-f.z, zero),
        Vector4::new(-dot(s, eye), -dot(u, eye), dot(f, eye), one)
    )
}

#[cfg(test)]
mod test {
    use num;
    use std::f32;
    use vec::vec::{ vec3, vec4 };
    use ext::{ perspective, translate };

    #[test]
    fn test_translate() {
        let v = vec3(1.0, 3.0, 2.0);
        let m = num::one();
        let t = translate(&m, v);
        assert_eq!(t[0], vec4(1., 0., 0., 0.));
        assert_eq!(t[1], vec4(0., 1., 0., 0.));
        assert_eq!(t[2], vec4(0., 0., 1., 0.));
        assert_eq!(t[3], v.extend(1.));
    }

    #[test]
    fn test_perspective() {
        let p = perspective(f32::consts::PI * 2.0 * 45.0 / 360.0, 1920.0 / 1080.0, 0.1, 100.0);
    }
}
