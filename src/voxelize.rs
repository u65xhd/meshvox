use super::sat::triangle_aabb_intersects;
use super::vector::Vector3;
use num_traits::Float;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Triangle<T: Copy> {
    pub(crate) points: [Vector3<T>; 3],
    pub(crate) aabb: AABB<T>,
}

impl<T: Float> Triangle<T> {
    fn new(p1: &Vector3<T>, p2: &Vector3<T>, p3: &Vector3<T>) -> Self {
        let points = [p1.clone(), p2.clone(), p3.clone()];
        let mut max = points[0];
        let mut min = points[0];
        for point in points.iter().skip(1) {
            max.x = max.x.max(point.x);
            max.y = max.y.max(point.y);
            max.z = max.z.max(point.z);
            min.x = min.x.min(point.x);
            min.y = min.y.min(point.y);
            min.z = min.z.min(point.z);
        }
        let aabb = AABB { min, max };
        Self { points, aabb }
    }
    fn grid_aabb(&self, step: T) -> AABB<isize> {
        AABB{
            min: vector_to_grid_step_floor(&self.aabb.min, step),
            max: vector_to_grid_step_floor(&self.aabb.max, step),
        }
    }
    fn voxelize(&self, step: T) -> Vec<[isize; 3]> {
        let aabb = self.grid_aabb(step);
        let mut voxels = Vec::new();
        for i in (aabb.min.x)..(aabb.max.x + 2) {
            for j in (aabb.min.y)..(aabb.max.y + 2) {
                for k in (aabb.min.z)..(aabb.max.z + 2) {
                    let centor = Vector3::new(i, j, k);
                    let voxel = Voxel::new(&centor, step);
                    if triangle_aabb_intersects(self, &voxel.aabb()) {
                        voxels.push([i, j, k]);
                    }
                }
            }
        }
        voxels
    }
}

#[inline]
fn to_grid_step_floor<T: Float>(value: T, step: T) -> isize {
    let div = value / step;
    div.floor().to_isize().expect("cannot convert to isize")
}

fn vector_to_grid_step_floor<T: Float>(vector: &Vector3<T>, step: T) -> Vector3<isize> {
    Vector3::new(
        to_grid_step_floor(vector.x, step),
        to_grid_step_floor(vector.y, step),
        to_grid_step_floor(vector.z, step),
    )
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AABB<T: Copy> {
    pub min: Vector3<T>,
    pub max: Vector3<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Voxel<T: Copy> {
    centor: Vector3<isize>,
    centor_scaled: Vector3<T>,
    step: T,
}

impl<T: Float> Voxel<T> {
    #[inline]
    fn new(centor: &Vector3<isize>, step: T) -> Self {
        let centor_scaled = Vector3::new(
            T::from(centor.x).expect("cannot convert from isize") * step,
            T::from(centor.y).expect("cannot convert from isize") * step,
            T::from(centor.z).expect("cannot convert from isize") * step,
        );
        Self {
            centor: centor.clone(),
            centor_scaled,
            step,
        }
    }
    pub(crate) fn aabb(&self) -> AABB<T> {
        let two = T::one() + T::one();
        let eps = T::epsilon() * (two + two + two + two + two);
        let half = self.step / two + eps;
        let min = Vector3::new(
            self.centor_scaled.x - half,
            self.centor_scaled.y - half,
            self.centor_scaled.z - half,
        );
        let max = Vector3::new(
            self.centor_scaled.x + half,
            self.centor_scaled.y + half,
            self.centor_scaled.z + half,
        );
        AABB { min, max }
    }
}

pub fn surface_voxelize<T: Float>(
    vertices: &Vec<[T; 3]>,
    indices: &Vec<[usize; 3]>,
    step: T,
) -> Vec<[isize; 3]> {
    let mut tris = Vec::new();
    for index in indices {
        let p1 = Vector3::new(
            vertices[index[0]][0],
            vertices[index[0]][1],
            vertices[index[0]][2],
        );
        let p2 = Vector3::new(
            vertices[index[1]][0],
            vertices[index[1]][1],
            vertices[index[1]][2],
        );
        let p3 = Vector3::new(
            vertices[index[2]][0],
            vertices[index[2]][1],
            vertices[index[2]][2],
        );
        tris.push(Triangle::new(&p1, &p2, &p3));
    }
    let mut voxels = Vec::new();
    for tri in tris {
        voxels.append(&mut tri.voxelize(step));
    }
    voxels
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}
