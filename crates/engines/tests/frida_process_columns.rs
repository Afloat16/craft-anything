use craft_anything_engines::frida::{MAX_LINE, parse_processes};

fn application_table(rows: &[(u32, &str, &str)]) -> String {
    let mut text = format!("{:>4}  {:<20}  Identifier\n", "PID", "Name");
    for (pid, name, identifier) in rows {
        text.push_str(&format!("{pid:>4}  {name:<20}  {identifier}\n"));
    }
    text
}

#[test]
fn process_names_keep_dotted_suffixes_without_identifiers() {
    let rows = parse_processes(" PID  Name\n----  ----------------\n  12  craft app.exe\n  13  café app.exe\n");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "craft app.exe");
    assert_eq!(rows[1].name, "café app.exe");
    assert!(rows.iter().all(|row| row.identifier.is_none()));
}

#[test]
fn process_names_keep_repeated_spaces_and_tabs() {
    let rows = parse_processes(" PID  Name\n  12  two  spaces\n  13  tab\tinside\n");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "two  spaces");
    assert_eq!(rows[1].name, "tab\tinside");
    assert!(rows.iter().all(|row| row.identifier.is_none()));
}

#[test]
fn application_identifiers_come_from_the_header_even_without_dots() {
    let text = application_table(&[(12, "Example App", "simple"), (13, "two  spaces", "com.example.app")]);
    let rows = parse_processes(&text);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "Example App");
    assert_eq!(rows[0].identifier.as_deref(), Some("simple"));
    assert_eq!(rows[1].name, "two  spaces");
    assert_eq!(rows[1].identifier.as_deref(), Some("com.example.app"));
}

#[test]
fn a_short_application_row_does_not_invent_an_identifier() {
    let mut text = application_table(&[(12, "Complete", "com.example.complete")]);
    text.push_str("  13  worker app.exe\n");
    let rows = parse_processes(&text);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[1].name, "worker app.exe");
    assert!(rows[1].identifier.is_none());
}

#[test]
fn a_later_process_header_clears_the_application_column() {
    let mut text = application_table(&[(12, "Complete", "com.example.complete")]);
    text.push_str(" PID  Name\n  13  worker app.exe\n");
    let rows = parse_processes(&text);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].identifier.as_deref(), Some("com.example.complete"));
    assert_eq!(rows[1].name, "worker app.exe");
    assert!(rows[1].identifier.is_none());
}

#[test]
fn headerless_rows_keep_names_without_guessing_application_identifiers() {
    let rows = parse_processes("12 worker app.exe\n13 two  spaces\n");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "worker app.exe");
    assert_eq!(rows[1].name, "two  spaces");
    assert!(rows.iter().all(|row| row.identifier.is_none()));
}

#[test]
fn application_column_offsets_count_characters_in_unicode_names() {
    let text = application_table(&[(12, "工具 Example", "org.example.one"), (13, "🌍 café", "org.example.two")]);
    let rows = parse_processes(&text);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "工具 Example");
    assert_eq!(rows[0].identifier.as_deref(), Some("org.example.one"));
    assert_eq!(rows[1].name, "🌍 café");
    assert_eq!(rows[1].identifier.as_deref(), Some("org.example.two"));
}

#[test]
fn ordinary_processes_and_dotted_application_ids_remain_compatible() {
    let processes = parse_processes("  PID  Name\n    1  init\n12345  worker.main\n");
    assert_eq!(processes.len(), 2);
    assert_eq!(processes[0].pid, 1);
    assert_eq!(processes[0].name, "init");
    assert_eq!(processes[1].pid, 12345);
    assert_eq!(processes[1].name, "worker.main");
    assert!(processes.iter().all(|row| row.identifier.is_none()));
    let apps = parse_processes(&application_table(&[(12, "Example", "com.example.app")]));
    assert_eq!(apps[0].name, "Example");
    assert_eq!(apps[0].identifier.as_deref(), Some("com.example.app"));
}

#[test]
fn malformed_rows_and_nonrunning_applications_keep_their_existing_policy() {
    let mut text = application_table(&[(12, "Example", "com.example.app")]);
    text.push_str("----  --------------------  ----------------\n-     Not running           com.example.idle\nbad   Invalid               com.example.bad\n");
    text.push_str("4294967296  Overflow\n 56  init\n");
    let rows = parse_processes(&text);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].pid, 12);
    assert_eq!(rows[1].pid, 56);
    assert_eq!(rows[1].name, "init");
    assert!(rows[1].identifier.is_none());
}

#[test]
fn headers_and_unicode_body_lines_keep_the_existing_byte_cap() {
    let header = format!("PID  Name{}Identifier\n", " ".repeat(MAX_LINE));
    let text = format!("{header}12 plain\n13 {}\n", "é".repeat(MAX_LINE));
    let rows = parse_processes(&text);
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].name, "plain");
    assert!(rows[0].identifier.is_none());
    assert!(rows[1].name.len() <= MAX_LINE - 3);
    assert!(rows[1].name.chars().all(|c| c == 'é'));
    assert!(rows[1].identifier.is_none());
}

#[test]
fn separator_widths_handle_headers_wider_than_their_data_columns() {
    for text in [
        "PID  Name  Identifier\n--  -  -\n12  A  x\n",
        "PID  Name  Identifier\n-  -  -\n1  A  x\n",
    ] {
        let rows = parse_processes(text);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].name, "A");
        assert_eq!(rows[0].identifier.as_deref(), Some("x"));
    }
}
