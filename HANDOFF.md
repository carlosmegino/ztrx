# ZTRX community handoff

The tests below use synthetic inputs and can run without title assets. The
observations from local shader and runtime replays are summarized in
[PROGRESS.md](docs/PROGRESS.md). Title binaries, shaders, dumps, logs and
captures stay outside the public repository. Project code follows the
repository license and upstream attribution is preserved in Git.

## Public discussion

- [Scalar SRV correction against the console-flow branch](https://github.com/r4tedx/VIctory/pull/18).
- [128-thread limitation and single-wave64 microbenchmark](https://github.com/Marvisak/VIctory/issues/2688).
- [NumElements and PR #30, #36, #49 and #54 validation with the original authors mentioned](https://github.com/carlosmegino/ztrx/commit/9a259b03879fab4f6ae96c3c91fbb46db9fdd528#commitcomment-211847345).

## Independent contributions

Each commit adds one original contribution. Its parent identifies the exact
dependency. The inherited implementation belongs to its original authors.

| Contribution | Exact parent | Validation |
| --- | --- | --- |
| [`fc334165bcc83e3fdf93de8e150464832d3d808c`](https://github.com/carlosmegino/ztrx/commit/fc334165bcc83e3fdf93de8e150464832d3d808c) reads `NumElements` from the decoded SRV/UAV and tests clearing an entry | PR #127, `e3ad2a2d48906a2256e70669932fff4e10e55160`, @starnotes-xj | The old results were 5768 and 5 for two 37-element views. Both became 37. |
| [`9ae78c75684d4915367a3bd20dc86e82470dc179`](https://github.com/carlosmegino/ztrx/commit/9ae78c75684d4915367a3bd20dc86e82470dc179) adds GPU timestamp and readback tests | [`shader/console-flow-lds`](https://github.com/r4tedx/VIctory/tree/shader/console-flow-lds), `3fab5a19f586c690fedf4578c1fb7b4c2de298dd`, r4. and contributors | Over twelve samples per arm at 2,048 groups, medians were 2.86 µs emulated and 3.33 µs for the explicit wave32 reference. Ranges overlapped and barrier encodings differed. |
| [`d44159fdf378a293759aee12423c976bfcc78cf3`](https://github.com/carlosmegino/ztrx/commit/d44159fdf378a293759aee12423c976bfcc78cf3) preserves structured SRVs in scalar reads and adds a cross-element regression | Same `3fab5a19f586c690fedf4578c1fb7b4c2de298dd` console-flow branch | One incorrectly accepted lane went from 17 to 0. Seven UAV readbacks matched the CPU reference; the canaries stayed intact and D3D12 GPU-based validation reported zero warnings or errors. |
| [`2db173cb597f1126b33a7b2dbb8369283693c0d4`](https://github.com/carlosmegino/ztrx/commit/2db173cb597f1126b33a7b2dbb8369283693c0d4) adds full-EXEC permlane readback | PR #49, `e06eef85826bb942178fb2e8cabd3065996df158`, @CUBE-Hugo-Viaud | Four dispatches matched their 64-lane references. One selected compute program advanced from no DXIL to a PSO. |
| [`dd73e115c775efd2618a6096b0dece0feaf07174`](https://github.com/carlosmegino/ztrx/commit/dd73e115c775efd2618a6096b0dece0feaf07174) checks f64 reduction at every destination lane | PR #54, `0bd72dac09e8df1ca90ae8834043e4eaeec0101d`, @IAmTheSween | All 512 lane results matched. Two of three selected title programs validate and still require wave64; the third stops at a lane mask read as data. |

PR #30 at [`52e06e5`](https://github.com/carlosmegino/ztrx/commit/52e06e5)
is @rivaldodev's original implementation. Its divergent-barrier fixture ran
with 1, 10, 32, 33 and 64 live lanes. Three selected wave64 programs
validated and created zero PSOs. PR #36 at
[`1402bf7`](https://github.com/carlosmegino/ztrx/commit/1402bf7) is
@FrancescoGelosoEterea's original implementation. It created one of those
three PSOs for the LDS flag case. Two still required wave64. The
[independent LDS tests](https://github.com/carlosmegino/ztrx/tree/test/lds-flag-results)
and the [integer target regression](https://github.com/carlosmegino/ztrx/tree/test/integer-target-blending)
were published previously.

## Reproduction

The checks used an RTX 5070 Ti with driver 621.01, Windows 11 x64 and the
Windows SDK DXIL validator. Use the Rust toolchain specified by each checkout.
Run each command from its matching branch. Set the adapter for a multi-GPU
system when the test supports adapter selection.

| Branch | Focused release test |
| --- | --- |
| `test/descriptor-words-windows` | `cargo test -p victory-gpu --release --locked record_count -- --nocapture` |
| `test/single-wave64-upstream` | `cargo test -p victory-shader --release --locked --lib single_wave_lds_timing -- --ignored --nocapture --test-threads=1` |
| `fix/structured-scalar-loads` | `cargo test -p victory-shader --release --locked --lib scalar_buffer_load_crosses_structured_elements -- --nocapture` |
| `test/permlane-pr49` | `cargo test -p victory-shader --release --locked --lib permlane_wave64_reads_each_row_with_full_exec -- --nocapture` |
| `test/f64-pr54` | `cargo test -p victory-shader --release --locked --lib f64_wave_reduction_reaches_every_lane_on_gpu -- --nocapture` |

The [diagnostics-only input helper](https://github.com/carlosmegino/ztrx/tree/test/runtime-input)
is separate from the compute changes. Its branch includes the focused input
tests and usage notes. The [frame profiling branch](https://github.com/carlosmegino/ztrx/tree/perf/frame-profiling)
contains the existing cadence tests and benchmark tooling.

## Remaining work

The isolated 64-thread dispatch used five SRVs, seven UAVs and one guest CBV
at a nonzero table base. The newer emitter uses a raw SRV for that guest CBV.
The older fork binds a native host CBV. A local prototype validated the
native-CBV variant in a bounded dispatch and through the fork's PSO
constructor. The production fork still needs a scoped ABI integration.

The exact 128-thread case stops before DXIL emission at `3fab5a1`. Four
physical waves represent two logical wave64s. The existing exchange has
two LDS slots and uses group-wide barriers. Start with a synthetic group
whose two logical waves have different masks, LDS values and loop counts.
Check each lane against a CPU reference and track both logical waves'
barrier participation. Once both runtime paths pass, compare the same
scene, inputs and cache state. Visual and frame-time effects remain unmeasured.
