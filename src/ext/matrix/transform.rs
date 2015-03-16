use basenum::BaseFloat;
use mat::mat::Matrix4;
use vec::vec::Vector3;

/// Builds a translation 4 * 4 matrix created from a vector of 3 components.
///
/// Uses input matrix `m` and translation vector coordinates `v`.
///
/// # Example
///
/// ```ignore
/// # extern crate num;
/// # extern crate glm;
/// use glm::*;
/// use glm::ext::*;
///
/// let matrix = glm::ext::translate(&num::one(), vec3(1., 2., 3.));
/// assert_eq!(matrix[3], vec4(1., 2., 3., 1.));
/// ```
#[inline]
pub fn translate<P>(
    m: &Matrix4<P>,
    v: Vector3<P>
) -> Matrix4<P>
where
    P : BaseFloat
{
    let mut result = m.clone();
    result[3] = m[0] * v[0] + m[1] * v[1] + m[2] * v[2] + m[3];
    result
}

#[cfg(test)]
mod test {
    use num;
    use vec::vec::{ vec3, vec4 };
    use ext::translate;

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
}
