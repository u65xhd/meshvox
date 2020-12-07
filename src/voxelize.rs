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
        AABB {
            min: vector_to_grid_step_floor(&self.aabb.min, step),
            max: vector_to_grid_step_floor(&self.aabb.max, step),
        }
    }
    fn voxelize(&self, step: T) -> Vec<[isize; 3]> {
        let eps = T::epsilon()*((T::one()+T::one())*(T::one()+T::one())*(T::one()+T::one())+(T::one()+T::one()));
        let aabb = self.grid_aabb(step);
        let mut voxels = Vec::new();
        for i in (aabb.min.x)..(aabb.max.x + 2) {
            for j in (aabb.min.y)..(aabb.max.y + 2) {
                for k in (aabb.min.z)..(aabb.max.z + 2) {
                    let center = Vector3::new(
                        T::from(i).unwrap(),
                        T::from(j).unwrap(),
                        T::from(k).unwrap(),
                    ) * step;
                    let aabb = AABB::new(&center, step+eps);
                    if triangle_aabb_intersects(self, &aabb) {
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

#[inline]
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

impl<T: Float> AABB<T> {
    fn new(center: &Vector3<T>, size: T) -> Self {
        let half = size / (T::one() + T::one());
        AABB {
            min: *center - Vector3::new(half, half, half),
            max: *center + Vector3::new(half, half, half),
        }
    }
}

/// A set of voxels.
pub struct Voxels<T: Float> {
    /// A set of center points of voxels on the grid.
    /// That is, the grid position times the step value is the center of voxel.
    pub grid_positions: HashSet<[isize; 3]>,
    /// A width of the grid.
    pub step: T,
}
impl<T: Float> Voxels<T> {
    pub fn new(grid_positions: &HashSet<[isize;3]>, step: T) -> Self{
        Self{
            grid_positions: grid_positions.clone(),
            step,
        }
    }
    pub fn voxelize(vertices: &Vec<[T; 3]>, indices: &Vec<[usize; 3]>, step: T) -> Self {
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
        Voxels {
            grid_positions: voxels.into_iter().collect::<HashSet<_>>(),
            step,
        }
    }
    /// Fills the interior with voxels
    pub fn fill(&mut self) {
        let ((max_x, max_y, max_z), (min_x, min_y, min_z)) = self.grid_positions.iter().fold(
            (
                (isize::min_value(), isize::min_value(), isize::min_value()),
                (isize::max_value(), isize::max_value(), isize::max_value()),
            ),
            |(max, min), p| {
                (
                    (max.0.max(p[0]), max.1.max(p[1]), max.2.max(p[2])),
                    (min.0.min(p[0]), min.1.min(p[1]), min.2.min(p[2])),
                )
            },
        );
        for i in min_x..max_x + 1 {
            for j in min_y..max_y + 1 {
                let mut do_fill = false;
                for k in min_z..max_z + 1 {
                    let contains = self.grid_positions.contains(&[i, j, k]);
                    if contains {
                        do_fill = !do_fill;
                    }
                    if !contains && do_fill {
                        self.grid_positions.insert([i, j, k]);
                    }
                }
            }
        }
    }
    pub fn vertices_indices(&self) -> (Vec<[T;3]>, Vec<usize>) {
        let mut meshes = Vec::new();
        for voxel_pos in self.grid_positions.iter() {
            let x_p =
                !self
                    .grid_positions
                    .contains(&[voxel_pos[0] + 1, voxel_pos[1], voxel_pos[2]]);
            let x_n =
                !self
                    .grid_positions
                    .contains(&[voxel_pos[0] - 1, voxel_pos[1], voxel_pos[2]]);
            let y_p =
                !self
                    .grid_positions
                    .contains(&[voxel_pos[0], voxel_pos[1] + 1, voxel_pos[2]]);
            let y_n =
                !self
                    .grid_positions
                    .contains(&[voxel_pos[0], voxel_pos[1] - 1, voxel_pos[2]]);
            let z_p =
                !self
                    .grid_positions
                    .contains(&[voxel_pos[0], voxel_pos[1], voxel_pos[2] + 1]);
            let z_n =
                !self
                    .grid_positions
                    .contains(&[voxel_pos[0], voxel_pos[1], voxel_pos[2] - 1]);
            let mesh_dir = [x_p, x_n, y_p, y_n, z_p, z_n];
            let mut mesh = voxel_to_mesh(*voxel_pos, self.step, mesh_dir);
            meshes.append(&mut mesh);
        }
        let len = meshes.len();
        (meshes, (0..len).collect())
    }
    /// Gets center points of boxes
    pub fn point_cloud(&self) -> Vec<[T; 3]> {
        self.grid_positions
            .iter()
            .map(|v| {
                [
                    T::from(v[0]).unwrap() * self.step,
                    T::from(v[1]).unwrap() * self.step,
                    T::from(v[2]).unwrap() * self.step,
                ]
            })
            .collect()
    }
}

fn voxel_to_mesh<T: Float>(
    voxel: [isize; 3],
    step: T,
    mesh_direction: [bool; 6],
) -> Vec<[T;3]> {
    let half = step / (T::one() + T::one());
    let x = T::from(voxel[0]).unwrap() * step;
    let y = T::from(voxel[1]).unwrap() * step;
    let z = T::from(voxel[2]).unwrap() * step;
    let p1 = Vector3::new(x + half, y + half, z + half);
    let p2 = Vector3::new(x + half, y + half, z - half);
    let p3 = Vector3::new(x + half, y - half, z + half);
    let p4 = Vector3::new(x + half, y - half, z - half);
    let p5 = Vector3::new(x - half, y + half, z + half);
    let p6 = Vector3::new(x - half, y + half, z - half);
    let p7 = Vector3::new(x - half, y - half, z + half);
    let p8 = Vector3::new(x - half, y - half, z - half);

    let mut mesh = Vec::new();
    // x plus
    if mesh_direction[0] {
        mesh.append(&mut tri_mesh(&p1, &p2, &p3));
        mesh.append(&mut tri_mesh(&p3, &p2, &p4));
    }
    // x minus
    if mesh_direction[1] {
        mesh.append(&mut tri_mesh(&p5, &p7, &p6));
        mesh.append(&mut tri_mesh(&p8, &p6, &p7));
    }
    // y plus
    if mesh_direction[2] {
        mesh.append(&mut tri_mesh(&p1, &p5, &p6));
        mesh.append(&mut tri_mesh(&p1, &p6, &p2));
    }
    // y minus
    if mesh_direction[3] {
        mesh.append(&mut tri_mesh(&p7, &p3, &p8));
        mesh.append(&mut tri_mesh(&p3, &p4, &p8));
    }
    // z plus
    if mesh_direction[4] {
        mesh.append(&mut tri_mesh(&p7, &p5, &p1));
        mesh.append(&mut tri_mesh(&p7, &p1, &p3));
    }
    // z minus
    if mesh_direction[5] {
        mesh.append(&mut tri_mesh(&p6, &p8, &p2));
        mesh.append(&mut tri_mesh(&p8, &p4, &p2));
    }
    mesh
}

#[inline]
fn tri_mesh<T: Float>(p1: &Vector3<T>, p2: &Vector3<T>, p3: &Vector3<T>) -> Vec<[T;3]> {
    vec![[p1.x, p1.y, p1.z], [p2.x, p2.y, p2.z], [p3.x, p3.y, p3.z]]
}
