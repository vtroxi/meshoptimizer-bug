use meshopt::VertexDataAdapter;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

fn bytes_slice<T: Copy>(t: &[T]) -> &[u8] {
    if std::mem::size_of::<T>() == 0 {
        panic!("size of T is 0")
    }
    unsafe { core::slice::from_raw_parts(t.as_ptr() as *const u8, std::mem::size_of_val(t)) }
}

fn save_obj<P>(file_name: P, indices: &[u32], positions: &[f32]) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let mut file = BufWriter::new(File::create(file_name)?);
    for i in 0..positions.len() / 3 {
        writeln!(
            &mut file,
            "v {} {} {}",
            positions[i * 3],
            positions[i * 3 + 1],
            positions[i * 3 + 2]
        )?;
    }
    for i in 0..indices.len() / 3 {
        writeln!(
            &mut file,
            "f {} {} {}",
            indices[i * 3] + 1,
            indices[i * 3 + 1] + 1,
            indices[i * 3 + 2] + 1,
        )?;
    }
    Ok(())
}

fn main() {
    let model = tobj::load_obj("input.obj", &tobj::LoadOptions::default())
        .expect("failed to load OBJ file")
        .0
        .remove(0);

    let new_indices = meshopt::simplify(
        &model.mesh.indices[..],
        &VertexDataAdapter::new(
            bytes_slice(&model.mesh.positions[..]),
            3 * std::mem::size_of::<f32>(),
            0,
        )
        .unwrap(),
        model.mesh.indices.len() / 4,
        0.05,
        true,
    );

    save_obj(
        "simplified.obj",
        &new_indices[..],
        &model.mesh.positions[..],
    )
    .expect("failed to save OBJ file");
}
