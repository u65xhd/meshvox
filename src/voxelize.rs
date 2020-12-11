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
    #[inline]
    fn new(p1: &Vector3<T>, p2: &Vector3<T>, p3: &Vector3<T>) -> Self {
        let points = [p1.clone(), p2.clone(), p3.clone()];
        let min_x = p1.x.min(p2.x).min(p3.x);
        let min_y = p1.y.min(p2.y).min(p3.y);
        let min_z = p1.z.min(p2.z).min(p3.z);
        let max_x = p1.x.max(p2.x).max(p3.x);
        let max_y = p1.y.max(p2.y).max(p3.y);
        let max_z = p1.z.max(p2.z).max(p3.z);

        let aabb = AABB {
            min: Vector3::new(min_x, min_y, min_z),
            max: Vector3::new(max_x, max_y, max_z),
        };
        Self { points, aabb }
    }
    #[inline]
    fn grid_aabb(&self, step: T) -> AABB<i32> {
        AABB {
            min: vector_to_grid_step_floor(&self.aabb.min, step),
            max: vector_to_grid_step_ceil(&self.aabb.max, step),
        }
    }
    fn voxelize(&self, step: T, eps: T) -> Vec<[i32; 3]> {
        let eps_vec = Vector3::new(eps, eps, eps);
        let step_vec = Vector3::new(step, step, step);
        let tri_aabb = self.grid_aabb(step);
        let mut voxels = Vec::new();
        let mut intersects_pre = false;
        for i in (tri_aabb.min.x)..(tri_aabb.max.x + 1) {
            for j in (tri_aabb.min.y)..(tri_aabb.max.y + 1) {
                for k in (tri_aabb.min.z)..(tri_aabb.max.z + 1) {
                    let min = Vector3::new(
                        T::from(i).unwrap(),
                        T::from(j).unwrap(),
                        T::from(k).unwrap(),
                    ) * step;
                    let max = min + step_vec;
                    let aabb = AABB {
                        min: min - eps_vec,
                        max: max + eps_vec,
                    };
                    let intersects = triangle_aabb_intersects(self, &aabb);
                    if intersects {
                        voxels.push([i, j, k]);
                    }
                    if intersects_pre && !intersects {
                        intersects_pre = false;
                        break;
                    }
                    intersects_pre = intersects;
                }
            }
        }
        voxels
    }
}

#[inline]
fn to_grid_step_floor<T: Float>(value: T, step: T) -> i32 {
    let div = value / step;
    div.floor().to_i32().expect("cannot convert to i32")
}

#[inline]
fn vector_to_grid_step_floor<T: Float>(vector: &Vector3<T>, step: T) -> Vector3<i32> {
    Vector3::new(
        to_grid_step_floor(vector.x, step),
        to_grid_step_floor(vector.y, step),
        to_grid_step_floor(vector.z, step),
    )
}

#[inline]
fn to_grid_step_ceil<T: Float>(value: T, step: T) -> i32 {
    let div = value / step;
    div.ceil().to_i32().expect("cannot convert to i32")
}

#[inline]
fn vector_to_grid_step_ceil<T: Float>(vector: &Vector3<T>, step: T) -> Vector3<i32> {
    Vector3::new(
        to_grid_step_ceil(vector.x, step),
        to_grid_step_ceil(vector.y, step),
        to_grid_step_ceil(vector.z, step),
    )
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AABB<T: Copy> {
    pub min: Vector3<T>,
    pub max: Vector3<T>,
}

/// A set of voxels.
pub struct Voxels<T: Float> {
    /// A set of positions of voxels on the grid.
    /// That is, the grid position times the step value is the voxel position (minimum corner).
    pub grid_positions: HashSet<[i32; 3]>,
    /// A width of the grid.
    pub step: T,
}
impl<T: Float> Voxels<T> {
    #[inline]
    pub fn new(grid_positions: &HashSet<[i32; 3]>, step: T) -> Self {
        Self {
            grid_positions: grid_positions.clone(),
            step,
        }
    }
    pub fn voxelize(vertices: &[[T; 3]], indices: &[usize], step: T) -> Self {
        if step <= T::epsilon() {
            panic!("step should be positive value");
        }
        let mut tris = Vec::new();
        for index in indices.chunks(3) {
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
        let eps = T::epsilon() * T::from(10).unwrap();
        for tri in tris {
            let mut voxel = tri.voxelize(step, eps);
            voxels.append(&mut voxel);
        }
        Voxels {
            grid_positions: voxels.into_iter().collect(),
            step,
        }
    }
    pub fn min_max(&self) -> ([i32; 3], [i32; 3]) {
        let ((max_x, max_y, max_z), (min_x, min_y, min_z)) = self.grid_positions.iter().fold(
            (
                (i32::min_value(), i32::min_value(), i32::min_value()),
                (i32::max_value(), i32::max_value(), i32::max_value()),
            ),
            |(max, min), p| {
                (
                    (max.0.max(p[0]), max.1.max(p[1]), max.2.max(p[2])),
                    (min.0.min(p[0]), min.1.min(p[1]), min.2.min(p[2])),
                )
            },
        );
        ([min_x, min_y, min_z], [max_x, max_y, max_z])
    }
    /// Fills the interior with voxels
    pub fn fill(&mut self) {
        let (min, max) = self.min_max();

        let mut inside_along_z = HashSet::new();
        for x in min[0]..(max[0] + 1) {
            for y in min[1]..(max[1] + 1) {
                let mut inside = true;
                let mut i = 0;
                let mut z_pre = 0;
                for z in min[2]..(max[2] + 1) {
                    if let Some(pos) = self.grid_positions.get(&[x, y, z]) {
                        if i != 0 && pos[2] - z_pre > 1 {
                            if inside {
                                for p in (z_pre + 1)..pos[2] {
                                    inside_along_z.insert([x, y, p]);
                                }
                            }
                            inside = !inside;
                        }
                        i += 1;
                        z_pre = pos[2];
                    }
                }
            }
        }
        let mut inside_along_x = HashSet::new();
        for y in min[1]..(max[1] + 1) {
            for z in min[2]..(max[2] + 1) {
                let mut inside = true;
                let mut i = 0;
                let mut x_pre = 0;
                for x in min[0]..(max[0] + 1) {
                    if let Some(pos) = self.grid_positions.get(&[x, y, z]) {
                        if i != 0 && pos[0] - x_pre > 1 {
                            if inside {
                                for p in (x_pre + 1)..pos[0] {
                                    inside_along_x.insert([p, y, z]);
                                }
                            }
                            inside = !inside;
                        }
                        i += 1;
                        x_pre = pos[0];
                    }
                }
            }
        }
        let mut inside_along_y = HashSet::new();
        for z in min[2]..(max[2] + 1) {
            for x in min[0]..(max[0] + 1) {
                let mut inside = true;
                let mut i = 0;
                let mut y_pre = 0;
                for y in min[1]..(max[1] + 1) {
                    if let Some(pos) = self.grid_positions.get(&[x, y, z]) {
                        if i != 0 && pos[1] - y_pre > 1 {
                            if inside {
                                for p in (y_pre + 1)..pos[1] {
                                    inside_along_y.insert([x, p, z]);
                                }
                            }
                            inside = !inside;
                        }
                        i += 1;
                        y_pre = pos[1];
                    }
                }
            }
        }
        let inside_points = inside_along_x.intersection(&inside_along_y).cloned().collect::<HashSet<_>>();
        let inside_points = inside_points.intersection(&inside_along_z);
        for inside_point in inside_points {
            self.grid_positions.insert(*inside_point);
        }
    }
    pub fn vertices_indices(&self) -> (Vec<[T; 3]>, Vec<usize>) {
        let mut meshes = Vec::new();
        let set: HashSet<_> = self.grid_positions.iter().collect();
        for voxel_pos in self.grid_positions.iter() {
            let x_p = !set.contains(&[voxel_pos[0] + 1, voxel_pos[1], voxel_pos[2]]);
            let x_n = !set.contains(&[voxel_pos[0] - 1, voxel_pos[1], voxel_pos[2]]);
            let y_p = !set.contains(&[voxel_pos[0], voxel_pos[1] + 1, voxel_pos[2]]);
            let y_n = !set.contains(&[voxel_pos[0], voxel_pos[1] - 1, voxel_pos[2]]);
            let z_p = !set.contains(&[voxel_pos[0], voxel_pos[1], voxel_pos[2] + 1]);
            let z_n = !set.contains(&[voxel_pos[0], voxel_pos[1], voxel_pos[2] - 1]);
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

fn voxel_to_mesh<T: Float>(voxel: [i32; 3], step: T, mesh_direction: [bool; 6]) -> Vec<[T; 3]> {
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
fn tri_mesh<T: Float>(p1: &Vector3<T>, p2: &Vector3<T>, p3: &Vector3<T>) -> Vec<[T; 3]> {
    vec![[p1.x, p1.y, p1.z], [p2.x, p2.y, p2.z], [p3.x, p3.y, p3.z]]
}
