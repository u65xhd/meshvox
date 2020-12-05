use meshvox::surface_voxelize;
use nalgebra::Vector3;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::path::Path;
use stl_io::{Normal, Triangle, Vertex};
use std::time::Instant;

fn main() {
    let step = 0.05;
    let path = Path::new("objs/teapot.obj");
    let (models, _materials) = tobj::load_obj(&path, true).expect("Failed to load file");
    let mut voxel = Vec::new();
    println!("start");
    let start = Instant::now();
    for model in models.iter() {
        let mesh = &model.mesh;
        let mut indices = Vec::new();
        let len = mesh.indices.len();
        for j in (0..len).step_by(3) {
            let index1 = mesh.indices[j] as usize;
            let index2 = mesh.indices[j + 1] as usize;
            let index3 = mesh.indices[j + 2] as usize;
            indices.push([index1, index2, index3]);
        }
        let mut vertices = Vec::new();
        let len = mesh.positions.len();
        for j in (0..len).step_by(3) {
            let pos = [
                mesh.positions[j] as f32,
                mesh.positions[j + 1] as f32,
                mesh.positions[j + 2] as f32,
            ];
            vertices.push(pos);
        }
        voxel.append(&mut surface_voxelize(&vertices, &indices, step));
    }
    let voxels: HashSet<_> = voxel.into_iter().collect::<HashSet<_>>();
    println!("{}", voxels.len());
    let end = start.elapsed();
        println!(
            "elapsed time: {}.{:06} sec",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
    let mut mesh = Vec::new();
    for voxel in voxels.iter() {
        let x_p = !voxels.contains(&[voxel[0]+1, voxel[1], voxel[2]]);
        let x_n = !voxels.contains(&[voxel[0]-1, voxel[1], voxel[2]]);
        let y_p = !voxels.contains(&[voxel[0], voxel[1]+1, voxel[2]]);
        let y_n = !voxels.contains(&[voxel[0], voxel[1]-1, voxel[2]]);
        let z_p = !voxels.contains(&[voxel[0], voxel[1], voxel[2]+1]);
        let z_n = !voxels.contains(&[voxel[0], voxel[1], voxel[2]-1]);
        let mesh_dir = [x_p, x_n, y_p, y_n, z_p, z_n];
        let mesh_normal = voxel_to_mesh(*voxel, step, mesh_dir);
        for (points, normal) in mesh_normal {
            let triangle_stl = Triangle {
                normal: Normal::new(normal),
                vertices: [
                    Vertex::new(points[0]),
                    Vertex::new(points[1]),
                    Vertex::new(points[2]),
                ],
            };
            mesh.push(triangle_stl);
        }
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("mesh.stl")
        .unwrap();
    stl_io::write_stl(&mut file, mesh.iter()).unwrap();
}

fn voxel_to_mesh(voxel: [isize; 3], step: f32, mesh_direction: [bool;6]) -> Vec<(Vec<[f32; 3]>, [f32; 3])> {
    let half = step / 2.0;
    let x = voxel[0] as f32 * step;
    let y = voxel[1] as f32 * step;
    let z = voxel[2] as f32 * step;
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
    if mesh_direction[0]{
        mesh.push(tri_mesh(&p1, &p2, &p3));
        mesh.push(tri_mesh(&p3, &p2, &p4));
    }
    // x minus
    if mesh_direction[1]{
        mesh.push(tri_mesh(&p5, &p7, &p6));
        mesh.push(tri_mesh(&p8, &p6, &p7));
    }
    // y plus
    if mesh_direction[2]{
        mesh.push(tri_mesh(&p1, &p5, &p6));
        mesh.push(tri_mesh(&p1, &p6, &p2));
    }
    // y minus
    if mesh_direction[3]{
        mesh.push(tri_mesh(&p7, &p3, &p8));
        mesh.push(tri_mesh(&p3, &p4, &p8));
    }
    // z plus
    if mesh_direction[4]{
        mesh.push(tri_mesh(&p7, &p5, &p1));
        mesh.push(tri_mesh(&p7, &p1, &p3));
    }
    // z minus
    if mesh_direction[5]{
        mesh.push(tri_mesh(&p6, &p8, &p2));
        mesh.push(tri_mesh(&p8, &p4, &p2));
    }
    mesh
}

#[inline]
fn tri_mesh(p1: &Vector3<f32>, p2: &Vector3<f32>, p3: &Vector3<f32>) -> (Vec<[f32; 3]>, [f32; 3]) {
    let normal = (p2 - p1).cross(&(p3 - p1)).normalize();
    (
        vec![[p1.x, p1.y, p1.z], [p2.x, p2.y, p2.z], [p3.x, p3.y, p3.z]],
        [normal.x, normal.y, normal.z],
    )
}
