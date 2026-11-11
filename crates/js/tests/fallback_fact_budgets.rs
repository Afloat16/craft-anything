use craft_anything_js::{Limits, ModuleFacts, ParseMode, analyze};

const FACT_LIMIT: usize = 5_000;

fn scan(source: &str) -> ModuleFacts {
    let facts = analyze("fallback.js", source, &Limits { max_parse_bytes: 0, ..Limits::default() }).facts;
    assert_eq!(facts.mode, ParseMode::Lexer);
    facts
}

fn notices(facts: &ModuleFacts, list: &str) -> usize {
    facts.truncated.iter().filter(|name| name.as_str() == list).count()
}

#[test]
fn fetch_facts_keep_the_first_rows_within_the_list_budget() {
    let source: String = (0..FACT_LIMIT + 2).map(|i| format!("fetch('/item/{i}');\n")).collect();
    let facts = scan(&source);
    assert_eq!(facts.endpoints.len(), FACT_LIMIT);
    assert_eq!(facts.endpoints.first().unwrap().url, "/item/0");
    let last = facts.endpoints.last().unwrap();
    assert_eq!(last.url, format!("/item/{}", FACT_LIMIT - 1));
    assert_eq!(last.line, FACT_LIMIT as u32);
    assert_eq!(last.offset as usize, source.find(&format!("fetch('/item/{}')", FACT_LIMIT - 1)).unwrap());
    assert_eq!(notices(&facts, "endpoints"), 1);
}

#[test]
fn literal_urls_cannot_extend_an_already_full_call_budget() {
    let source = format!("{}const later = 'https://later.example.test/path';", "fetch('/same');\n".repeat(FACT_LIMIT));
    let facts = scan(&source);
    assert_eq!(facts.endpoints.len(), FACT_LIMIT);
    assert!(facts.endpoints.iter().all(|endpoint| endpoint.api == "fetch" && endpoint.url == "/same"));
    assert!(facts.strings.iter().any(|value| value == "https://later.example.test/path"));
    assert_eq!(notices(&facts, "endpoints"), 1);
}

#[test]
fn earlier_literal_urls_spend_the_same_endpoint_budget_as_calls() {
    let source = format!("const first = 'https://first.example.test/path';\n{}", "fetch('/same');\n".repeat(FACT_LIMIT));
    let facts = scan(&source);
    assert_eq!(facts.endpoints.len(), FACT_LIMIT);
    let first = facts.endpoints.first().unwrap();
    assert_eq!(first.api, "literal");
    assert_eq!(first.url, "https://first.example.test/path");
    assert_eq!(facts.endpoints.iter().filter(|endpoint| endpoint.api == "fetch").count(), FACT_LIMIT - 1);
    assert_eq!(notices(&facts, "endpoints"), 1);
}

#[test]
fn exposed_bridges_keep_the_first_keys_within_the_list_budget() {
    let source: String = (0..FACT_LIMIT + 2).map(|i| format!("contextBridge.exposeInMainWorld('bridge{i}', {{}});\n")).collect();
    let facts = scan(&source);
    assert_eq!(facts.exposed.len(), FACT_LIMIT);
    assert_eq!(facts.exposed.first().unwrap().key.as_deref(), Some("bridge0"));
    let last = facts.exposed.last().unwrap();
    assert_eq!(last.key, Some(format!("bridge{}", FACT_LIMIT - 1)));
    assert_eq!(last.line, FACT_LIMIT as u32);
    assert_eq!(last.offset as usize, source.find(&format!("exposeInMainWorld('bridge{}'", FACT_LIMIT - 1)).unwrap());
    assert_eq!(notices(&facts, "exposed"), 1);
}

#[test]
fn a_full_endpoint_list_does_not_stop_other_facts() {
    let source = format!(
        "{}require('./later.node'); ipcRenderer.send('later-channel'); contextBridge.exposeInMainWorld('later-bridge', {{}});",
        "fetch('/same');\n".repeat(FACT_LIMIT + 1)
    );
    let facts = scan(&source);
    assert!(facts.imports.iter().any(|import| import.specifier == "./later.node"));
    assert!(facts.native_modules.iter().any(|name| name == "./later.node"));
    assert!(facts.ipc.iter().any(|ipc| ipc.channel.as_deref() == Some("later-channel")));
    assert_eq!(facts.exposed.len(), 1);
    assert_eq!(facts.exposed[0].key.as_deref(), Some("later-bridge"));
    assert_eq!(facts.endpoints.len(), FACT_LIMIT);
    assert_eq!(facts.truncated, ["endpoints"]);
}

#[test]
fn endpoint_and_bridge_budgets_are_independent() {
    let source = format!("{}{}", "fetch('/same');\n".repeat(FACT_LIMIT + 2), "contextBridge.exposeInMainWorld('bridge', {});\n".repeat(FACT_LIMIT + 2));
    let facts = scan(&source);
    assert_eq!(facts.endpoints.len(), FACT_LIMIT);
    assert_eq!(facts.exposed.len(), FACT_LIMIT);
    assert_eq!(notices(&facts, "endpoints"), 1);
    assert_eq!(notices(&facts, "exposed"), 1);
    assert_eq!(facts.truncated.len(), 2);
}

#[test]
fn exactly_full_lists_are_not_reported_as_truncated() {
    let source = format!("{}{}", "fetch('/same');\n".repeat(FACT_LIMIT), "contextBridge.exposeInMainWorld('bridge', {});\n".repeat(FACT_LIMIT));
    let facts = scan(&source);
    assert_eq!(facts.endpoints.len(), FACT_LIMIT);
    assert_eq!(facts.exposed.len(), FACT_LIMIT);
    assert!(facts.truncated.is_empty());
}

#[test]
fn ordinary_fallback_facts_keep_their_metadata() {
    let source = "const url = 'https://api.example.test/path';\nfetch('/relative');\ncontextBridge.exposeInMainWorld('bridge', {});";
    let facts = scan(source);
    assert_eq!(facts.endpoints.len(), 2);
    assert_eq!(facts.endpoints[0].api, "literal");
    assert_eq!(facts.endpoints[0].url, "https://api.example.test/path");
    assert_eq!(facts.endpoints[0].line, 1);
    assert_eq!(facts.endpoints[0].offset as usize, source.find("'https:").unwrap());
    assert_eq!(facts.endpoints[1].api, "fetch");
    assert_eq!(facts.endpoints[1].url, "/relative");
    assert_eq!(facts.endpoints[1].line, 2);
    assert_eq!(facts.endpoints[1].offset as usize, source.find("fetch").unwrap());
    assert_eq!(facts.exposed.len(), 1);
    assert_eq!(facts.exposed[0].key.as_deref(), Some("bridge"));
    assert_eq!(facts.exposed[0].line, 3);
    assert!(facts.truncated.is_empty());
}
