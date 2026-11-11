use craft_anything_js::{Analysis, Limits, ParseMode, analyze};

fn parsed(source: &str) -> Analysis {
    let result = analyze("bundle.js", source, &Limits::default());
    assert_eq!(result.facts.mode, ParseMode::Parsed, "{result:?}");
    assert_eq!(result.facts.parse_errors, 0);
    result
}

fn tail() -> String {
    (3..=10).map(|value| format!("function(e){{e.exports={value}}}")).collect::<Vec<_>>().join(",")
}

fn entries(spread: &str) -> String {
    format!("function(e,t,r){{e.exports=r(1)}},function(e){{e.exports=32}},{spread},{}", tail())
}

fn assert_known_prefix(result: &Analysis) {
    assert_eq!(result.bundle.modules.iter().map(|module| module.id.as_str()).collect::<Vec<_>>(), ["0", "1"], "{result:?}");
    assert_eq!(result.bundle.modules[0].deps, ["1"]);
    assert!(result.bundle.truncated, "{result:?}");
}

#[test]
fn an_unknown_spread_stops_static_array_module_indices() {
    let result = parsed(&format!("var __webpack_modules__=[{}];", entries("...more")));
    assert_known_prefix(&result);
}

#[test]
fn literal_spreads_also_leave_unproven_module_indices() {
    for spread in ["...[]", "...[function(e){e.exports=71},function(e){e.exports=72}]", "...'xy'"] {
        let result = parsed(&format!("var __webpack_modules__=[{}];", entries(spread)));
        assert_known_prefix(&result);
    }
}

#[test]
fn chunk_arrays_use_the_same_index_boundary() {
    let result = parsed(&format!("self.webpackChunkx.push([[13],[{}]]);", entries("...more")));
    assert_known_prefix(&result);
    assert_eq!(result.bundle.chunk_ids, ["13"]);
    assert!(result.bundle.modules.iter().all(|module| module.format == "webpack-chunk" && module.chunk_ids == ["13"]));
}

#[test]
fn holes_before_a_spread_keep_their_known_indices() {
    let source = format!("var __webpack_modules__=[function(e,t,r){{r(2)}},,function(e){{e.exports=32}},...more,{}];", tail());
    let result = parsed(&source);
    assert_eq!(result.bundle.modules.iter().map(|module| module.id.as_str()).collect::<Vec<_>>(), ["0", "2"], "{result:?}");
    assert_eq!(result.bundle.modules[0].deps, ["2"]);
    assert!(result.bundle.truncated);
}

#[test]
fn arrays_without_spreads_keep_holes_and_all_modules() {
    let result = parsed("var __webpack_modules__=[function(e,t,r){r(2)},,function(e){e.exports=32}];");
    assert_eq!(result.bundle.modules.iter().map(|module| module.id.as_str()).collect::<Vec<_>>(), ["0", "2"]);
    assert_eq!(result.bundle.modules[0].deps, ["2"]);
    assert!(!result.bundle.truncated);
}

#[test]
fn a_spread_inside_a_module_does_not_shift_the_outer_table() {
    let result = parsed("var __webpack_modules__=[function(e){e.exports=[...more]},function(e){e.exports=32}];");
    assert_eq!(result.bundle.modules.iter().map(|module| module.id.as_str()).collect::<Vec<_>>(), ["0", "1"]);
    assert!(!result.bundle.truncated);
}
