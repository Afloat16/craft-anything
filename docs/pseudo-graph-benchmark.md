# Pseudo-C graph lifting benchmark

The block structurer previously recomputed population counts for every immediate-dominator
candidate, built immediate post-dominators twice, and computed immediate dominators that the
emitter never used. This change computes post-dominator sizes once, reuses the completed
post-dominator vector for loop exits, and removes unused results. The dominator algorithm,
limits, tie order and emitter behavior are unchanged.

## Reproduce

Run this example on the baseline and this change, retaining both binaries before switching:

```sh
cargo run --release -p craft-anything-disasm --example pseudo_graph_benchmark -- 5
```

The example constructs real encoded x86-64 jump, condition, loop and return fixtures, analyzes
them with the public API, then times only complete `pseudo::lift` calls. Program analysis,
fixture creation and JSON serialization are outside the measured interval. Its JSON includes
the entire `Pseudo` value, which must match between variants before timings are compared.
The argument controls lifts per sample; each invocation takes nine timing samples per fixture.

## Recorded comparison

Linux x86-64, Rust 1.99.0, release profile with thin LTO and one codegen unit. Baseline production
code is `9574a2f72843663313c968c268374eb0a08d4784`, with only the same benchmark example added.
Five alternating rounds used five lifts per sample, giving 45 samples and 225 lifts per variant
and fixture. The benchmark processes were pinned to the same available CPU. Our compilation
and tests had finished before these rounds; other shared-environment activity is not controlled,
and the hardware model was not available. The table uses the median across each variant's
45 samples, divided by five. Timing data and complete-output SHA-256 values are in
[pseudo-graph-lifting.json](benchmarks/pseudo-graph-lifting.json).

| Fixture | Blocks | Baseline ms | Changed ms | Ratio |
| --- | ---: | ---: | ---: | ---: |
| chain_16 | 16 | 0.012 | 0.011 | 1.07x |
| chain_256 | 256 | 1.196 | 0.970 | 1.23x |
| chain_1000 | 1000 | 8.688 | 3.750 | 2.32x |
| chain_1500 | 1500 | 21.998 | 6.191 | 3.55x |
| linear_fallback_1501 | 1501 | 0.562 | 0.554 | 1.01x |
| reverse_chain_1000 | 1000 | 26.527 | 21.948 | 1.21x |
| diamonds_64 | 193 | 0.733 | 0.660 | 1.11x |
| diamonds_400 | 1201 | 8.688 | 5.709 | 1.52x |
| loops_64 | 192 | 0.826 | 0.691 | 1.20x |
| loops_400 | 1200 | 10.940 | 5.828 | 1.88x |
| many_exits_400 | 1201 | 7.917 | 2.128 | 3.72x |

All eleven complete serialized outputs are identical in every recorded run. The unchanged
1501-block linear fallback is a timing control; its small variation is not claimed as an
improvement. These are synthetic control-flow fixtures, not an end-to-end application or
hardware execution benchmark.

The reverse-ordered chain also exposes a pre-existing limitation: after the dominator
iteration budget is exhausted, the current structurer can invent loops in an acyclic graph.
This performance change preserves that output; a separate pending R3.11 item tracks it.
