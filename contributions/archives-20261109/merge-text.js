// Temporary integration helper; conflicts require explicit review.
export default function mergeText(base, ours, theirs) {
  const lines = text => text.match(/[^\n]*\n|[^\n]+$/g) || [];
  function diff(a, b) {
    let start = 0, ae = a.length, be = b.length;
    while (start < ae && start < be && a[start] === b[start]) start++;
    while (ae > start && be > start && a[ae - 1] === b[be - 1]) { ae--; be--; }
    const n = ae - start, m = be - start;
    if (n * m > 20000000) throw Error("Diff exceeds bounded work allowance");
    const width = m + 1, table = new Uint32Array((n + 1) * width);
    for (let i = n - 1; i >= 0; i--) for (let j = m - 1; j >= 0; j--) {
      table[i * width + j] = a[start + i] === b[start + j]
        ? table[(i + 1) * width + j + 1] + 1
        : Math.max(table[(i + 1) * width + j], table[i * width + j + 1]);
    }
    let i = 0, j = 0, h = null;
    const changes = [];
    const flush = () => { if (h) changes.push(h); h = null; };
    while (i < n || j < m) {
      if (i < n && j < m && a[start + i] === b[start + j]) {
        flush(); i++; j++; continue;
      }
      if (!h) h = { start: start + i, end: start + i, lines: [] };
      if (j < m && (i === n || table[i * width + j + 1] >= table[(i + 1) * width + j])) {
        h.lines.push(b[start + j++]);
      } else { i++; h.end = start + i; }
    }
    flush();
    return changes;
  }
  const a = lines(base), o = diff(a, lines(ours)), t = diff(a, lines(theirs));
  const apply = hs => {
    let pos = 0; const result = [];
    for (const h of hs) { if (h.start < pos) throw Error("Overlapping changes"); result.push(...a.slice(pos, h.start), ...h.lines); pos = h.end; }
    result.push(...a.slice(pos)); return result.join("");
  };
  if (apply(o) !== ours || apply(t) !== theirs) throw Error("Diff did not reproduce its input");
  const all = [...t.map(h => ({...h,side:"theirs"})), ...o.map(h => ({...h,side:"ours"}))]
    .sort((x,y) => x.start-y.start || x.end-y.end || (x.side==="theirs"?-1:1));
  const merged = [], conflicts = [];
  for (const h of all) {
    const prev = merged[merged.length - 1];
    if (prev && prev.start === h.start && prev.end === h.end) {
      if (prev.lines.join("") === h.lines.join("")) continue;
      if (h.start === h.end) { prev.lines.push(...h.lines); continue; }
      conflicts.push({left:prev,right:h}); continue;
    }
    if (prev && h.start < prev.end) { conflicts.push({left:prev,right:h}); continue; }
    merged.push({...h, lines:[...h.lines]});
  }
  return { content: conflicts.length ? null : apply(merged), conflicts, ours: o, theirs: t };
}
