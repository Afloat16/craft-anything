//! Name-insensitive structural fingerprints: the sequence of syntax node kinds plus literal
//! values, ignoring identifier names. Two builds that only renamed variables fingerprint the same.

use oxc_ast::AstKind;
use oxc_ast_visit::Visit;
use sha2::{Digest, Sha256};

pub(crate) struct Fingerprint {
    hasher: Sha256,
    pub nodes: u64,
}

impl Fingerprint {
    pub fn new() -> Self {
        Fingerprint { hasher: Sha256::new(), nodes: 0 }
    }

    pub fn finish(self) -> String {
        hex::encode(self.hasher.finalize())
    }

    fn literal_bytes(&mut self, bytes: &[u8]) {
        self.hasher.update((bytes.len() as u64).to_le_bytes());
        self.hasher.update(bytes);
    }
}

impl<'a> Visit<'a> for Fingerprint {
    fn enter_node(&mut self, kind: AstKind<'a>) {
        self.nodes += 1;
        self.hasher.update(format!("{:?};", kind.ty()).as_bytes());
        match kind {
            AstKind::StringLiteral(s) => {
                self.hasher.update(s.value.as_bytes());
                self.hasher.update([0]);
            }
            AstKind::BigIntLiteral(n) => self.literal_bytes(n.value.as_bytes()),
            AstKind::RegExpLiteral(r) => {
                self.literal_bytes(r.regex.pattern.text.as_bytes());
                self.hasher.update([r.regex.flags.bits()]);
            }
            // A tag can observe raw escapes even when the cooked text would be the same.
            AstKind::TemplateElement(t) => self.literal_bytes(t.value.raw.as_bytes()),
            AstKind::NumericLiteral(n) => self.hasher.update(n.value.to_le_bytes()),
            AstKind::BooleanLiteral(b) => self.hasher.update([u8::from(b.value)]),
            AstKind::BinaryExpression(b) => self.hasher.update(b.operator.as_str().as_bytes()),
            AstKind::LogicalExpression(l) => self.hasher.update(l.operator.as_str().as_bytes()),
            AstKind::UnaryExpression(u) => self.hasher.update(u.operator.as_str().as_bytes()),
            AstKind::AssignmentExpression(a) => self.hasher.update(a.operator.as_str().as_bytes()),
            AstKind::UpdateExpression(u) => {
                self.hasher.update([u8::from(u.prefix), u8::from(matches!(u.operator, oxc_syntax::operator::UpdateOperator::Increment))])
            }
            _ => {}
        }
    }
}
