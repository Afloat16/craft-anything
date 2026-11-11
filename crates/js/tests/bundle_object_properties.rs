use craft_anything_js::{Analysis, Limits, ParseMode, analyze};

fn parsed(source: &str) -> Analysis {
    let result = analyze("bundle.js", source, &Limits::default());
    assert_eq!(result.facts.mode, ParseMode::Parsed, "{result:?}");
    assert_eq!(result.facts.parse_errors, 0);
    result
}

fn chunk(table: &str) -> Analysis {
    parsed(&format!("(self.webpackChunkx=self.webpackChunkx||[]).push([[1],{table}]);"))
}

#[test]
fn repeated_module_keys_keep_only_the_final_function() {
    for table in [
        "{1:function(e,t,n){n(2)},1:function(e,t,n){n(3)}}",
        "{0x10:function(e,t,n){n(2)},'16':function(e,t,n){n(3)}}",
        "{'item':function(e,t,n){n(2)},['item']:function(e,t,n){n(3)}}",
        "{1:function(e,t,n){n(2)},[1]:function(e,t,n){n(3)}}",
        "{item(e,t,n){n(2)},item(e,t,n){n(3)}}",
    ] {
        let result = chunk(table);
        assert_eq!(result.bundle.modules.len(), 1, "{result:?}");
        assert_eq!(result.bundle.modules[0].deps, ["3"], "{result:?}");
    }
}

#[test]
fn a_later_nonfunction_or_accessor_replaces_an_earlier_module() {
    let functions = (1..=10).map(|id| format!("{id}:function(e){{e.exports={id}}}")).collect::<Vec<_>>().join(",");
    for last in ["1:37", "get 1(){return 37}", "set 1(value){}"] {
        let result = chunk(&format!("{{{functions},{last}}}"));
        assert_eq!(result.bundle.modules.len(), 9, "{result:?}");
        assert!(result.bundle.modules.iter().all(|module| module.id != "1"), "{result:?}");
    }
}

#[test]
fn accessors_are_not_recovered_as_module_bodies() {
    for table in ["{get 7(){return function(e){e.exports=37}}}", "{set 7(e){consume(e)}}"] {
        assert!(chunk(table).bundle.modules.is_empty(), "{table}");
    }
}

#[test]
fn prototype_setters_do_not_hide_or_create_own_module_entries() {
    for table in ["{__proto__:function(e){e.exports=37}}", "{'__proto__':function(e){e.exports=37}}"] {
        assert!(chunk(table).bundle.modules.is_empty(), "{table}");
    }
    for table in [
        "{['__proto__']:function(e){e.exports=37}}",
        "{__proto__(e){e.exports=37}}",
        "{['__proto__']:function(e){e.exports=37},__proto__:function(e){e.exports=99}}",
        "{__proto__:function(e){e.exports=99},['__proto__']:function(e){e.exports=37}}",
    ] {
        let result = chunk(table);
        assert_eq!(result.bundle.modules.len(), 1, "{result:?}");
        assert_eq!(result.bundle.modules[0].id, "__proto__");
    }
}

#[test]
fn unknown_property_writes_leave_only_later_known_entries() {
    let functions = (1..=9).map(|id| format!("{id}:function(e){{e.exports={id}}}")).collect::<Vec<_>>().join(",");
    for unknown in ["...more", "[dynamic]:function(e){e.exports=37}"] {
        let result = chunk(&format!("{{{functions},{unknown},10:function(e){{e.exports=10}}}}"));
        assert_eq!(result.bundle.modules.iter().map(|module| module.id.as_str()).collect::<Vec<_>>(), ["10"], "{result:?}");
    }
}

#[test]
fn browserify_keeps_final_module_and_dependency_definitions() {
    let source = "var modules={1:[function(r,m,e){m.exports=r('item')},{item:2,item:3}],1:[function(r,m,e){m.exports=r('item')},{item:4,item:5}],5:[function(r,m,e){m.exports=37},{}]};";
    let result = parsed(source);
    assert_eq!(result.bundle.modules.len(), 2, "{result:?}");
    let module = result.bundle.modules.iter().find(|module| module.id == "1").unwrap();
    assert_eq!(module.deps, ["5"], "{result:?}");
}

#[test]
fn esbuild_accessors_are_not_module_initializers() {
    for source in [
        "var require_item=__commonJS({get 'src/item.js'(){return function(e,m){m.exports=37}}});",
        "var require_item=__commonJS({set 'src/item.js'(e){consume(e)}});",
    ] {
        let result = parsed(source);
        assert!(result.bundle.modules.is_empty(), "{result:?}");
    }
}

#[test]
fn distinct_data_properties_keep_their_modules() {
    let result = chunk("{1:function(e,t,n){n(2)},2:function(e,t,n){e.exports=37}}");
    assert_eq!(result.bundle.modules.len(), 2);
    assert_eq!(result.bundle.modules[0].id, "1");
    assert_eq!(result.bundle.modules[0].deps, ["2"]);
    assert_eq!(result.bundle.modules[1].id, "2");
}

#[test]
fn browserify_nonmodule_replacements_remove_older_functions() {
    let modules = (1..=10).map(|id| format!("{id}:[function(r,m,e){{m.exports={id}}},{{}}]")).collect::<Vec<_>>().join(",");
    for last in ["1:37", "get 1(){return 37}", "set 1(value){}"] {
        let result = parsed(&format!("var modules={{{modules},{last}}};"));
        assert_eq!(result.bundle.modules.len(), 9, "{result:?}");
        assert!(result.bundle.modules.iter().all(|module| module.id != "1"), "{result:?}");
    }
}

#[test]
fn browserify_dependency_maps_keep_only_final_known_data_values() {
    for mapping in [
        "{item:2,item:5}",
        "{item:2,get item(){return 3},last:5}",
        "{item:2,set item(value){},last:5}",
        "{item:2,...more,last:5}",
        "{item:2,[dynamic]:3,last:5}",
        "{['__proto__']:5,__proto__:37}",
    ] {
        let result = parsed(&format!("var modules={{1:[function(r,m,e){{m.exports=r('item')}},{mapping}],5:[function(r,m,e){{m.exports=37}},{{}}]}};"));
        let module = result.bundle.modules.iter().find(|module| module.id == "1").unwrap();
        assert_eq!(module.deps, ["5"], "{result:?}");
    }
}

#[test]
fn a_shorthand_definition_can_replace_a_previous_module() {
    let modules = (1..=9).map(|id| format!("{id}:function(e){{e.exports={id}}}")).collect::<Vec<_>>().join(",");
    let result = chunk(&format!("{{{modules},item:function(e){{e.exports=37}},item}}"));
    assert_eq!(result.bundle.modules.len(), 9, "{result:?}");
    assert!(result.bundle.modules.iter().all(|module| module.id != "item"));
}
