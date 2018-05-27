use std::{fs, io};

extern crate gltf_json;
use gltf_json::validation::{Validate, Error};
use gltf_json::Path;

fn import_json(filename: &str) -> gltf_json::Root {
    let file = fs::File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    gltf_json::Root::from_reader(reader).unwrap()
}

#[test]
fn test_accessor_bounds_validate_minimally() {
    // file with missing min/max values
    let json = import_json("tests/minimal_accessor_invalid.gltf");
    let mut errs = vec![];
    json.validate_minimally(
        &json,
        gltf_json::Path::new,
        &mut |path, err| errs.push((path(), err)),
    );
    assert_eq!(errs,
        [(Path("meshes[0].primitives[0].attributes[\"POSITION\"].min".into()), Error::Missing),
         (Path("meshes[0].primitives[0].attributes[\"POSITION\"].max".into()), Error::Invalid)]);
}
