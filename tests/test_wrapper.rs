extern crate gltf;

use std::{fs, io};
use std::io::Read;

use gltf::mesh::Bounds;

#[test]
fn test_accessor_bounds() {
    // file derived from minimal.gltf with changed min/max values
    let file = fs::File::open("tests/minimal_accessor_min_max.gltf").unwrap();
    let mut reader = io::BufReader::new(file);
    let mut buffer = vec![];
    reader.read_to_end(&mut buffer).unwrap();
    let gltf = gltf::Gltf::from_slice(&buffer).unwrap();
    let mesh = &gltf.meshes().nth(0).unwrap();
    let prim = mesh.primitives().nth(0).unwrap();
    let bounds = prim.bounding_box();
    assert_eq!(bounds, Bounds { min: [-0.03, -0.04, -0.05], max: [1.0, 1.01, 0.02]});
}


