use craft_anything_formats::dotnet::{
    self,
    meta::{Metadata, t},
    sig::{Cur, GenCtx, Names},
};

#[test]
fn complete_method_and_property_signatures_reject_trailing_bytes() {
    let bytes = craft_anything_testkit::managed::managed_assembly(1);
    let assembly = dotnet::open(&bytes).unwrap();
    let names = Names::new(&assembly.md);
    for convention in [0, 8] {
        let mut signature = vec![convention, 1, 8, 14];
        assert!(names.method_sig(&signature, &GenCtx::default()).is_some());
        signature.push(0);
        assert!(names.method_sig(&signature, &GenCtx::default()).is_none());
    }
}

#[test]
fn complete_field_local_and_instantiation_signatures_reject_trailing_bytes() {
    let bytes = craft_anything_testkit::managed::managed_assembly(1);
    let assembly = dotnet::open(&bytes).unwrap();
    let names = Names::new(&assembly.md);
    let g = GenCtx::default();
    assert_eq!(names.field_sig(&[6, 8], &g).as_deref(), Some("int"));
    assert_eq!(names.locals_sig(&[7, 2, 8, 14], &g), Some(vec!["int".into(), "string".into()]));
    assert_eq!(names.method_spec_args(&[10, 1, 8], &g), Some(vec!["int".into()]));
    for tail in [0, 8, 0x41, 0xff] {
        assert!(names.field_sig(&[6, 8, tail], &g).is_none());
        assert!(names.locals_sig(&[7, 2, 8, 14, tail], &g).is_none());
        assert!(names.method_spec_args(&[10, 1, 8, tail], &g).is_none());
    }
    assert_eq!(names.locals_sig(&[7, 0], &g), Some(vec![]));
    assert!(names.locals_sig(&[7, 0, 8], &g).is_none());
}

#[test]
fn nested_function_pointers_leave_the_enclosing_parameters_available() {
    let bytes = craft_anything_testkit::managed::managed_assembly(1);
    let assembly = dotnet::open(&bytes).unwrap();
    let names = Names::new(&assembly.md);
    let g = GenCtx::default();
    let signature = names.method_sig(&[0, 2, 1, 0x1b, 0, 1, 8, 8, 14], &g).unwrap();
    assert_eq!(signature.return_type, "void");
    assert_eq!(signature.params, ["delegate*<int, int>", "string"]);
    let mut cursor = Cur::new(&[8, 14]);
    assert_eq!(names.ty(&mut cursor, &g, 0).as_deref(), Some("int"));
    assert_eq!(cursor.pos, 1);
    assert_eq!(names.ty(&mut cursor, &g, 0).as_deref(), Some("string"));
}

fn type_spec_metadata(signature: &[u8]) -> Vec<u8> {
    let mut tables = vec![0, 0, 0, 0, 2, 0, 0, 1];
    tables.extend_from_slice(&(1u64 << t::TYPE_SPEC).to_le_bytes());
    tables.extend_from_slice(&0u64.to_le_bytes());
    tables.extend_from_slice(&1u32.to_le_bytes());
    tables.extend_from_slice(&1u16.to_le_bytes());
    let mut blobs = vec![0, signature.len() as u8];
    blobs.extend_from_slice(signature);
    let mut bytes = b"BSJB\x01\0\x01\0\0\0\0\0\x04\0\0\0v4\0\0\0\0\x02\0".to_vec();
    bytes.extend_from_slice(&52u32.to_le_bytes());
    bytes.extend_from_slice(&(tables.len() as u32).to_le_bytes());
    bytes.extend_from_slice(b"#~\0\0");
    bytes.extend_from_slice(&(52 + tables.len() as u32).to_le_bytes());
    bytes.extend_from_slice(&(blobs.len() as u32).to_le_bytes());
    bytes.extend_from_slice(b"#Blob\0\0\0");
    bytes.extend_from_slice(&tables);
    bytes.extend_from_slice(&blobs);
    bytes
}

#[test]
fn standalone_type_specifications_must_consume_their_blob() {
    for (signature, expected) in [(&[0x1d, 8][..], "int[]"), (&[0x1d, 8, 14][..], "?")] {
        let bytes = type_spec_metadata(signature);
        let md = Metadata::parse(&bytes).unwrap();
        assert!(md.warnings.is_empty(), "{:?}", md.warnings);
        assert_eq!(Names::new(&md).type_name(t::TYPE_SPEC, 1, &GenCtx::default(), 0), expected);
    }
}
