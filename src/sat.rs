// Reference: Tomas Akenine-Möller, "Fast 3D Triangle-Box Overlap Testing"

use super::vector::Vector3;
use super::voxelize::{Triangle, AABB};
use num_traits::Float;

pub(crate) fn triangle_aabb_intersects<T: Float>(triangle: &Triangle<T>, aabb: &AABB<T>) -> bool {
    // 3 axes tests
    //let aabb_aabb_sat = aabb_aabb_intersects(triangle, aabb);
    //if !aabb_aabb_sat {
    //    return aabb_aabb_sat;
    //}

    // 1 axis test
    let plane_aabb_sat = plane_aabb_intersects(triangle, aabb);
    if !plane_aabb_sat {
        return plane_aabb_sat;
    }

    // 9 axes tests
    tri_edge_aabb_intersects(triangle, aabb)
}

//#[inline]
//fn aabb_aabb_intersects<T: Float>(triangle: &Triangle<T>, aabb: &AABB<T>) -> bool {
//    let two = T::one() + T::one();
//    let a_c = (aabb.max + aabb.min) / two;
//    let a_h = (aabb.max - aabb.min) / two;
//    let b_c = (triangle.aabb.max + triangle.aabb.min) / two;
//    let b_h = (triangle.aabb.max - triangle.aabb.min) / two;
//    (a_c.x - a_h.x <= b_c.x + b_h.x)
//        && (b_c.x - b_h.x <= a_c.x + a_h.x)
//        && (a_c.y - a_h.y <= b_c.y + b_h.y)
//        && (b_c.y - b_h.y <= a_c.y + a_h.y)
//        && (a_c.z - a_h.z <= b_c.z + b_h.z)
//        && (b_c.z - b_h.z <= a_c.z + a_h.z)
//}

#[inline]
fn plane_aabb_intersects<T: Float>(triangle: &Triangle<T>, aabb: &AABB<T>) -> bool {
    let normal =
        (triangle.points[1] - triangle.points[0]).cross(&(triangle.points[2] - triangle.points[0]));
    let plane_point = triangle.points[0];

    let d = -(normal.x * plane_point.x + normal.y * plane_point.y + normal.z * plane_point.z);
    let two = T::one() + T::one();
    let c = (aabb.max + aabb.min) / two;
    let h = (aabb.max - aabb.min) / two;
    let e = h.x * normal.x.abs() + h.y * normal.y.abs() + h.z * normal.z.abs();
    let s = c.dot(&normal) + d;
    !((s - e) > T::zero() || (s + e) < T::zero())
}

#[inline]
fn tri_edge_aabb_intersects<T: Float>(triangle: &Triangle<T>, aabb: &AABB<T>) -> bool {
    let two = T::one() + T::one();
    let c = (aabb.max + aabb.min) / two;
    let h = (aabb.max - aabb.min) / two;
    let v = vec![
        triangle.points[0] - c,
        triangle.points[1] - c,
        triangle.points[2] - c,
    ];
    let e = vec![Vector3::x_axis(), Vector3::y_axis(), Vector3::z_axis()];
    let f = vec![v[1] - v[0], v[2] - v[1], v[0] - v[2]];
    for i in 0..3 {
        for j in 0..3 {
            let a = e[i].cross(&f[j]);
            let p0 = a.dot(&(v[0]));
            let p1 = a.dot(&(v[1]));
            let p2 = a.dot(&(v[2]));
            let r = h.x * a.x.abs() + h.y * a.y.abs() + h.z * a.z.abs();
            if p0.min(p1).min(p2) > r || p0.max(p1).max(p2) < -r {
                return false;
            }
        }
    }
    true
}
