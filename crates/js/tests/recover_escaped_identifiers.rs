use craft_anything_js::recover::{RecoveredModule, RewriteLevel, recover_modules};
use craft_anything_js::{Limits, analyze};

fn recover(body: &str) -> RecoveredModule {
    let source = format!("(self.webpackChunkx=self.webpackChunkx||[]).push([[1],{{7:function(e,t,n){{{body}}}}}]);");
    let limits = Limits::default();
    let analysis = analyze("chunk.js", &source, &limits);
    assert_eq!(analysis.bundle.modules.len(), 1, "{analysis:?}");
    let modules = recover_modules(&source, &analysis.bundle.modules, RewriteLevel::Standard, &limits);
    assert_eq!(modules.len(), 1);
    assert!(modules[0].printed, "{:?}", modules[0]);
    modules.into_iter().next().unwrap()
}

#[test]
fn escaped_global_references_cannot_be_captured_by_new_parameter_names() {
    for (body, old, target) in [
        (r"e.exports = \u006dodule.value;", "e", "module"),
        (r"e.exports = \u{6d}odule.value;", "e", "module"),
        (r"e.exports = \u0065xports.value;", "t", "exports"),
        (r"e.exports = \u005f_webpack_require__.value;", "n", "__webpack_require__"),
    ] {
        let module = recover(body);
        assert!(!module.renamed.contains(&(old.into(), target.into())), "{body}: {module:?}");
    }
}

#[test]
fn escaped_bindings_keep_their_own_identity() {
    for body in [
        r"var \u006dodule = {value: 37}; e.exports = \u006dodule.value;",
        r"function value(\u006dodule) { return e.exports + \u006dodule; } e.exports = value(37);",
        r"try { throw 37; } catch (\u006dodule) { e.exports = \u006dodule; }",
    ] {
        let module = recover(body);
        assert!(!module.renamed.contains(&("e".into(), "module".into())), "{body}: {module:?}");
    }
}

#[test]
fn escaped_shorthand_is_still_an_identifier_reference() {
    let module = recover(r"e.exports = {\u006dodule};");
    assert!(!module.renamed.contains(&("e".into(), "module".into())), "{module:?}");
    assert!(module.text.contains("e.exports"), "{}", module.text);
}

#[test]
fn static_property_names_do_not_prevent_safe_renaming() {
    let module = recover(r"e.exports = obj.module + obj?.exports + obj.__webpack_require__ + obj.\u006dodule + obj['module'];");
    for (old, target) in [("e", "module"), ("t", "exports"), ("n", "__webpack_require__")] {
        assert!(module.renamed.contains(&(old.into(), target.into())), "{module:?}");
    }
    assert!(module.text.contains("module.exports"), "{}", module.text);
}

#[test]
fn unescaped_references_still_block_capture() {
    let module = recover("e.exports = module.value;");
    assert!(!module.renamed.contains(&("e".into(), "module".into())), "{module:?}");
}
