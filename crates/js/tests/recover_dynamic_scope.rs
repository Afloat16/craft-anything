use craft_anything_js::recover::{RecoveredModule, RewriteLevel, pretty_print, recover_modules};
use craft_anything_js::{Limits, analyze};

fn print(source: &str) -> (String, usize) {
    pretty_print("dynamic.cjs", source, RewriteLevel::Standard, &Limits::default()).unwrap()
}

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
fn with_objects_cannot_replace_the_value_of_void_zero() {
    let (text, rewrites) = print("function value(){with({undefined:37}){return [void 0,!0];}}");
    assert!(text.contains("void 0"), "{text}");
    assert!(text.contains("true"), "{text}");
    assert_eq!(rewrites, 1);
}

#[test]
fn direct_eval_cannot_inject_a_binding_for_rewritten_void_zero() {
    for callee in ["eval", "(eval)", r"\u0065val"] {
        let (text, rewrites) = print(&format!("function value(){{{callee}(\"var undefined=37\");return [void 0,!0];}}"));
        assert!(text.contains("void 0"), "{text}");
        assert!(text.contains("true"), "{text}");
        assert_eq!(rewrites, 1, "{text}");
    }
}

#[test]
fn indirect_calls_do_not_disable_safe_void_rewrites() {
    for call in ["obj.eval('1')", "(0,eval)('1')", "eval?.('1')", "ordinary('1')"] {
        let (text, rewrites) = print(&format!("function value(){{{call};return void 0;}}"));
        assert!(!text.contains("void 0"), "{text}");
        assert!(text.contains("undefined"), "{text}");
        assert_eq!(rewrites, 1, "{text}");
    }
}

#[test]
fn dynamic_lookup_keeps_original_wrapper_parameter_names() {
    for body in [
        "eval('e.exports=n(1)');",
        "(eval)('e.exports=n(1)');",
        r"\u0065val('e.exports=n(1)');",
        "function run(){eval('e.exports=n(1)');}run();",
        "with({e:{exports:37}}){t.answer=e.exports;}",
    ] {
        let module = recover(body);
        assert!(module.renamed.is_empty(), "{body}: {module:?}");
    }
}

#[test]
fn indirect_calls_keep_ordinary_parameter_recovery() {
    for call in ["obj.eval('1')", "(0,eval)('1')", "eval?.('1')", "ordinary('1')"] {
        let module = recover(&format!("{call};e.exports=n(1);"));
        assert_eq!(module.renamed, [("e".into(), "module".into()), ("t".into(), "exports".into()), ("n".into(), "__webpack_require__".into())], "{module:?}");
    }
}
