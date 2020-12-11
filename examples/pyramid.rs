use meshvox::Voxels;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p0 = [0.0, 0.0, 1.0];
    let p1 = [1.0, 0.0, 0.0];
    let p2 = [0.0, 1.0, 0.0];
    let p3 = [-1.0, 0.0, 0.0];
    let p4 = [0.0, -1.0, 0.0];

    let i0 = [0, 2, 1];
    let i1 = [0, 1, 4];
    let i2 = [0, 4, 3];
    let i3 = [0, 3, 2];
    let i4 = [2, 3, 4];
    let i5 = [2, 4, 1];

    let vertices = vec![p0, p1, p2, p3, p4];
    let indices = vec![i0, i1, i2, i3, i4, i5]
        .iter()
        .flatten()
        .map(|i| *i)
        .collect::<Vec<_>>();

    let box_size = 0.05;
    let pyramid = Voxels::voxelize(&vertices, &indices, box_size);
    let (v, i) = pyramid.vertices_indices();

    let file = File::create("examples/pyramid.obj")?;
    let mut file = BufWriter::new(file);
    for vertex in v {
        writeln!(
            file,
            "v {:0.6} {:0.6} {:0.6}",
            vertex[0], vertex[1], vertex[2]
        );
    }
    for index in i.iter().step_by(3) {
        writeln!(file, "f {} {} {}", index + 1, index + 2, index + 3);
    }
    file.flush()?;

    Ok(())
}
