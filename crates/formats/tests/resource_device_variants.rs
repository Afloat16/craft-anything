use craft_anything_formats::{arsc::config_qualifiers, bytes::Reader};

fn config(f: impl FnOnce(&mut [u8])) -> (String, String) {
    let mut cfg = [0; 64];
    cfg[0] = 64;
    f(&mut cfg);
    config_qualifiers(&Reader::new(&cfg), 0)
}

#[test]
fn locale_groups_keep_explicit_bcp47_subtags() {
    let (q, locale) = config(|c| {
        c[8..12].copy_from_slice(b"srRS");
        c[36..40].copy_from_slice(b"Latn");
        c[40..45].copy_from_slice(b"POSIX");
        c[53..57].copy_from_slice(b"latn");
    });
    assert_eq!(q, "b+sr+Latn+RS+POSIX+u+nu+latn");
    assert_eq!(q, locale);
    assert_eq!(
        config(|c| {
            c[8..12].copy_from_slice(b"srRS");
            c[36..40].copy_from_slice(b"Cyrl");
            c[52] = 1;
        })
        .1,
        "sr-rRS"
    );
}

#[test]
fn carrier_zero_is_distinct_from_unspecified() {
    assert_eq!(config(|c| c[6..8].fill(255)).0, "mnc0");
    assert_eq!(config(|_| {}).0, "default");
}

#[test]
fn round_screen_variants_are_distinct() {
    assert_eq!(config(|c| c[48] = 2).0, "round");
    assert_eq!(config(|c| c[48] = 1).0, "notround");
}

#[test]
fn color_capabilities_precede_orientation() {
    assert_eq!(
        config(|c| {
            c[12] = 2;
            c[49] = 10;
        })
        .0,
        "widecg-highdr-land"
    );
    assert_eq!(config(|c| c[49] = 5).0, "nowidecg-lowdr");
}

#[test]
fn keyboard_and_navigation_availability_keep_selector_order() {
    assert_eq!(
        config(|c| {
            c[16] = 2;
            c[17] = 2;
            c[18] = 11;
        })
        .0,
        "keyssoft-qwerty-navhidden-dpad"
    );
}

#[test]
fn inflection_changes_the_resource_config_but_not_its_locale() {
    assert_eq!(
        config(|c| {
            c[8..10].copy_from_slice(b"fr");
            c[19] = 2;
        }),
        ("fr-feminine".into(), "fr".into())
    );
}
