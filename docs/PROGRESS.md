# Native performance work log

## Reference and scope

2026-11-25: original reference `90b529f878a470fec9bf07b7be316bc769a12232`.
The existing fork is carlosmegino/ztrx with Zettrox/ztrx as upstream. Both main
references matched after fetch. The worktree was initially clean on the existing
`perf/frame-profiling` branch. Remote writes were initially withheld. The owner
subsequently authorized publication and continued work on 2026-11-25.

## Completed

- Inventoried the workspace and inspected execution, graphics, memory, texture,
  shader, cache, profiling, scripts, policy and CI paths. Recorded current community
  work in UPSTREAM_ANALYSIS; inspected both relevant upstream project heads.
- Built the exact reference in release and diagnostic profiles; preserved its
  diagnostic executable outside the repo with SHA-256
  `DE6AE8E2A5C1ECD553384C32912A2BD8CA8D4544ED49586396F9CCAD8115089A`.
- Implemented opt-in diagnostics CSV/JSONL cadence and existing-meter export, bounded
  queue/storage, loss warnings, successful-Present counting and honest fence labeling.
- Added benchmark/process/device sampling, percentile summary and synthetic checks.
  Candidate diagnostic executable SHA-256:
  `4264C79E287658AA1BCBA785A8D45322183BBB2181069FEA41A8DDC1199FF2C8`.
- Added incremental plans and permanent contribution rules. Prepared a local PR draft
  in pr-frame-profiling.md. A gameplay speedup has not been claimed.

## Verified validation

| Check | Result |
|---|---|
| Reference release + diag | passed |
| Reference normal tests | 496 passed, 0 failed, 7 ignored |
| Reference Clippy | passed in an isolated exact-commit source archive |
| Candidate release + diag | passed |
| Candidate normal tests | 496 passed, 0 failed, 7 ignored |
| Candidate all-feature tests, serial | 507 passed, 0 failed, 7 ignored |
| New exporter unit tests | 3 passed |
| Candidate GPU diagnostics, serial and parallel retry | 120 passed, 1 ignored each |
| Both CI Clippy configurations, -D warnings | passed; later incremental-cache hard-link warnings are environmental |
| Synthetic PowerShell summary checks | passed: distribution/lows/percentiles, loss, gaps, partial rows |
| CSV/JSONL runtime equivalence | all 5,438 records matched field by field in the 180-second menu capture |
| Source asset policy / diff whitespace | passed before final commit staging; rerun at commit |
| Global cargo fmt --check | fails on pre-existing reference formatting; new module passes edition-2024 rustfmt |

Hardware verified: Intel i7-8700K (12 logical processors), RTX 5070 Ti (16,303 MiB
reported), 32 GB RAM, Windows 11 build 26300, NVIDIA 621.01. Rust 1.98.1, Cargo,
rustup, VS 18 Community/MSVC (14.29 and 14.52), Windows SDK 10.0.26100 and DXC 1.9
are installed. PIX, Nsight Graphics and PresentMon were not found in the checked
locations. About 174 GiB were initially free on C:; no large offline cache was built.

## Runtime observations and experiments

Evidence and logs stay outside all source trees; no title-derived
files or screenshots are added to Git. Validation logs also stay outside the repo.

1. Original 120-second run: process survived until the scripted stop. Inherited
   RUST_LOG=warn hid info metrics and GPU tests overlapped part of the run. This was
   a smoke test, not a usable performance baseline. The wrapper now explicitly
   selects info logging and records/restores its changes.
2. Candidate 180-second cold menu capture: 5,265 successful Present records,
   173 interval records, zero reported loss, source 3840x2160. Visual inspection
   identified the legal/onboarding screen. Over profiler seconds 60-150: 2,699
   intervals, 33.332989 ms mean, 30.000310 Present/s, tail-mean 1% low 27.064305,
   0.1% low 25.938565, nearest-rank p99 35.5252 ms and p99.9 38.3964 ms.
   These are menu cadence observations, not gameplay results or a speedup.
3. Native window keyboard pulses did not advance the runtime's polled stand-in pad.
   Existing diagnostic VICTORY_KEYS (200 ms pulses) did advance it. Repeated Enter
   alone opened a legal article. An observed Down/Down/Enter sequence passed that
   screen and reached navigation setup. The user explicitly requested autonomous
   advancement because the physical controller is unavailable.
4. Fixed schedules reached the end of configuration but repeatedly entered a legal
   article instead of completing the notice sequence. Closing the window did not
   terminate two test processes; they were identified by their exact owned executable
   path and stopped before rebuilding. No unrelated process was stopped.
5. Added a separate diagnostics-only, bounded `VICTORY_KEYS_FILE` helper on
   `test/runtime-input`. Nine targeted input tests passed (two new), release/diag
   builds and all-target/all-feature HLE Clippy passed. Fourteen observed commands
   completed profile selection, initial defaults and required startup screens.
   The game entered loading and the introduction without a physical controller.
6. `candidate-live` uses a fresh cache and helper binary SHA-256
   `F33DDD16991378902BC10759D1643AB0F899E3AEF34C6E38E86DA1CA74370758`.
   No builds/GPU tests accompany the 3D loading phase. At 07:48:12 UTC the cold
   trace showed 705.8 ms/frame, 205.1 ms preparation, 150.9 ms of it texture work,
   38.3 ms CPU fence waiting, 44.1 ms copy-back, and 1,327 pipelines being made.
   At 07:49:56: 3744.6 ms/frame, 2886.2 ms preparation (2180.0 ms textures),
   219.3 ms fence waiting, 530.0 ms copy-back, 2,042 draws/frame, 806 dropped without
   a pipeline and 922 pipelines being made. These are individual cold meter windows,
   not sustained gameplay or GPU execution measurements. Visual completeness is
   not established; known shader translation/pipeline refusals remain visible.
7. The bounded run stopped at its 900-second deadline (908.38 s including shutdown;
   exit -1 is the requested forced stop). It produced 8,969 Present and 589 interval
   records: all 9,558 CSV/JSONL pairs matched field by field, reported loss zero,
   combined profile size 4,660,823 bytes. No unhandled-exception/device-loss event
   was found in the captured log. This is not proof of long-session stability.
   Source presentation dimensions remained 3840x2160; internal render resolution
   was not established. Intro graphics/credits appeared with defects, followed by
   black images; a correct playable world was not visually verified.
8. Profiler seconds 300-420 (cold introduction, 115 complete intervals): mean
   1032.4336 ms / 0.968585 Present/s. Tail-mean lows were 0.194142 (1%) and 0.184757
   (0.1%); the sample is too small for robust tail estimates. Meter-window totals
   over the same complete window averaged 488.219 ms preparation, including 350.647
   ms texture work, 85.672 ms CPU fence waiting and 65.834 ms copy-back per frame.
   Dropped draws averaged 1,619.226/frame. Do not sum overlapping CPU counters or
   treat this as a complete rendering benchmark. Process seconds 300.461-419.645
   averaged 73.51% of the 12 logical processors from sampled CPU-time deltas; this
   process-clock window is distinct from the profiler's clock origin.
9. Whole-run peaks: working set 15.7269 GiB, private bytes 23.2817 GiB, device-wide
   NVIDIA VRAM use 10,588 MiB. Private bytes are not physical RAM, and device VRAM
   includes other applications. No process-specific VRAM or real GPU execution
   measurement was collected. All summaries and raw data remain outside Git.

## Local contributions

- `perf/frame-profiling`: `a8e7689fcb0b0842ae97a1644fb748f715683601`, independent
  profiling/benchmark contribution directly on the original reference. Its PR draft
  is pr-frame-profiling.md; it does not include the input helper or broad audit docs.
- `test/runtime-input`: `f95e768251f01607727600886291ec93ce8c5bcf`, separate input
  diagnostic helper; PR draft pr-runtime-input.md. It is not a physical-input fix.
- Profiling was pushed and submitted as [upstream PR #82](https://github.com/Zettrox/ztrx/pull/82)
  at `a8e7689fcb0b0842ae97a1644fb748f715683601`. The input helper remains local.
  Main remains at the reference. Runtime captures
  started with dirty Git metadata; executable hashes identify the tested builds.
- Broad audit/continuity documents are on `docs/native-performance-audit`, separate
  from the two proposed source contributions. The worktree is intended to be clean
  after recording each experiment; it is not part of PR #82.

## Publication and interpolation validation

2026-11-25: refreshed origin/upstream; both main references still equal the exact
reference above. Reviewed the newer PR inventory before publishing. The GitHub
integration returned 403 for PR creation; the configured Git credential
successfully created PR #82.
Only the independent profiling branch was published. No PR was merged.

The next experiment is `fix/rendering-regressions`, starting at local audit commit
`3b1b3c53c3b5c23aacc1270abbbfb00570abdaf1`. It retains the local input helper and
profiling, so it is not a new independent upstream source submission.

- Reviewed [community PR #56](https://github.com/Zettrox/ztrx/pull/56), source
  `9fe511eb77fa1220bacf05c20e1854fe1dc8861c`, and preserved its author with
  `git cherry-pick -x`. Local source commit:
  `00a780d7d9ae0fdf9897c35024a5a2abe2d4884f`.
- Hypothesis: reusing a dead interpolation intermediate as a mixed-precision
  half-register destination incorrectly propagates `Val::Missing`. Reuse the
  existing half-write semantics, as proposed by the contributor, rather than
  implementing another fix. Relevant sites: `emit/lower.rs:7044`, `:7425`,
  `:7467`, `:8817`; synthetic regression `emit/mod.rs:1748`.
- First applied only the contributor's synthetic regression to the unmodified
  lowering. It failed with `Unsupported: the first half of an interpolation`.
  Restored that test-only edit before cherry-picking the original source commit.
- Candidate all-feature shader tests: 211 passed, 0 failed, 2 ignored, serially.
  Full workspace normal tests also pass serially: 497 passed, 0 failed, 7 ignored.
  Release and diagnostic builds pass. Both workspace CI Clippy commands pass;
  OneDrive incremental-cache hard-link fallback warnings remain environmental.
  Asset policy passes for all 189 tracked files and `git diff --check` passes.
  Global `cargo fmt --all -- --check` still fails at existing reference formatting;
  no unrelated source formatting is changed in the experiment.
- Used the existing `translate --validate --pipelines --list --jobs 2` tool,
  `VICTORY_HOST_MAX_WAVE=32`, the eight hashes identified by the contributor and
  the user's local shader pack. The exact preserved reference emitted 0/8 DXIL
  shaders (all eight failed at the same interpolation check). The candidate emitted
  8/8, validated 8/8 with SDK 10.0.26100.0 `dxil.dll` version 1.9.2602.17, and
  created 8/8 test pipelines on the default hardware D3D12 device. `VICTORY_WARP`
  was unset. These test pipelines use generated companion stages/default state;
  they are not proof that every actual title PSO or rendered output is correct.
- Diagnostic candidate SHA-256:
  `A46B96BC562F4E173622A03B442DF58C81F1E7F2EBCB572703D61129CE8F648B`.
  Before/after logs remain outside Git as `ztrx-pr56-offline-{before,after}.log`.
  No shader bytes, dumps or derived captures are uploaded. The translation CLI
  exits zero even when individual shaders fail: inspect its counters and failures.
- Offline validation does not populate runtime shader/PSO caches, demonstrate
  visual correctness, or measure FPS. No gameplay speedup is claimed.
- Published an [independent validation comment on PR #56](https://github.com/Zettrox/ztrx/pull/56#issuecomment-7159543649)
  with aggregate counts, methods, hardware and limits; no raw title logs or assets.
  The GitHub integration again returned 403 for comments, so publication used the
  existing Git authentication. No duplicate PR was submitted for this source patch.
- PR #82's upstream [CI run](https://github.com/Zettrox/ztrx/actions/runs/45583229379)
  reports `action_required`, with no commit checks/status results yet. This is not
  a passing upstream CI result; local validation is recorded separately.
- Reviewed PR #36's two source commits separately: integer-target blend handling
  `6615c72` and LDS-flag wave lowering `1402bf7`. Neither is applied yet. The
  latter changes inter-wave reasoning and needs independent correctness review;
  keep it separate from the bounded interpolation experiment.

## Integer-target blend experiment

2026-11-25: refreshed origin/upstream; main is still `90b529f`. Reviewed the
current PR inventory (through #90), including the unchanged two-commit PR #36.
PR #86 offers a narrower alternative to #56 for unavailable old half-register
values; it needs separate review rather than being stacked as a duplicate fix.

- Applied only PR #36's first commit `6615c720c41da5855aa2744c063e56df2617a4d7`,
  preserving its author/source with `cherry-pick -x`; local commit `459cd43`.
  No LDS-flag or inter-wave synchronization change is included.
- Added an independent synthetic hardware regression in
  `pipeline/host.rs::tests::integer_mesh_target_accepts_redundant_blending`.
  It compiles the existing synthetic mesh shader and a synthetic integer pixel
  shader with SDK DXC. Before the fix, an R8_UINT pipeline creates with blending
  disabled and returns `0x80070057` with blending enabled. Afterward all four
  independent-blend/blend-enable combinations create. This does not execute a draw
  or establish pixel equivalence. Existing pure-state tests cover other MRTs.
- Test commit on the combined local experiment: `b9bb2f3`.
  Separately tested and published the minimal branch `test/integer-target-blending`,
  whose original parent is `6615c72` and whose only added commit is
  [`6f6e2a824296c76bc3051c97139faec2aad7dc50`](https://github.com/carlosmegino/ztrx/commit/6f6e2a824296c76bc3051c97139faec2aad7dc50).
  All four pipeline-host tests pass on that exact branch; no hardware/DXC skip.
  Shared the regression and validation in [PR #36](https://github.com/Zettrox/ztrx/pull/36#issuecomment-7160332365)
  instead of opening a duplicate source PR.
- Combined branch: 113 normal GPU tests passed/1 ignored; all-feature serial
  workspace: 513 passed/0 failed/7 ignored. Release and diagnostic builds pass;
  both CI Clippy configurations pass. Policy: 189 tracked files pass. Global fmt
  still reports inherited formatting; the new test's own formatting was corrected.
- Runtime binary SHA-256:
  `2E07619E595F4B6CC2B60B53CBCFDA838FBD72A4216C9F6B4899B94A4A6FFA25`.
  `candidate-blend` reused the existing texture cache/save directory but has a new
  executable-keyed shader cache. Cache state is explicitly unknown/partially warm.
  Git metadata is clean at `b9bb2f3`. No builds or GPU tests accompany the capture.
- The 600-second bounded capture stopped as requested (614.285 s including stop;
  exit -1). The runtime now creates the mesh/pixel pipeline previously refused
  for integer-target blending, and creates a pipeline using the pixel shader
  previously blocked by #56. Other compute refusals remain, including the wave64
  case covered by PR #36's second commit. These are observed pipeline creations,
  not proof of correct pixels or a playable world. UI inspection verified profile
  selection, startup and loading art; the 3D scene was not captured before shutdown.
- Profiler seconds 300-540: 104 complete intervals, mean 2246.2481 ms,
  0.445187 successful Present/s. The scene was not visually classified and caches
  are not matched to earlier runs. This is neither a gameplay FPS result nor
  evidence of a before/after speedup or regression. Tail estimates are undersampled.
- First capture integrity: 3,397 Present + 281 interval records; all 3,678
  CSV/JSONL pairs match field by field, zero reported loss, 1,795,473 bytes.
  In the complete 300-540 second window, raw totals divided by 104 frames give
  1587.475 ms preparation (including 1244.296 ms texture work), 194.343 ms CPU
  fence waiting, 247.667 ms copy-back and 524.683 dropped draws/frame. These CPU
  counters can overlap; do not add them as a serial frame-time decomposition.
  Whole-run peaks: 15.8563 GiB working set, 22.5313 GiB private bytes (not RAM).
- Repeated the same binary/cache for a bounded 300-second compatibility run,
  `candidate-blend-repeat` (306.604 s including stop, exit -1 at deadline).
  UI inspection now verifies changing 3D introduction images and characters,
  with very dark/incomplete scene visibility; it does not establish a correct
  playable world. Only one virtual Enter pulse selected the profile; no graphics
  operation or title feature was disabled. Cache counters demonstrate reuse
  (e.g. 2,661 shader hits/2 misses and 2,313 pipelines loaded in an early window).
  2,713 Present + 228 interval records: all 2,941 CSV/JSONL pairs match, zero
  reported loss, 1,435,642 bytes. Profiler 150-270 seconds: 111 intervals,
  1072.2742 ms mean / 0.932597 Present/s, undersampled tails. This is a changing
  introduction with partial rendering, not a controlled gameplay benchmark.
- Started the next investigation without modifying wave lowering: the existing
  offline translator emits and validates the affected wave64 compute shader but
  hardware pipeline creation fails. Baseline is the same diagnostic binary above;
  `VICTORY_HOST_MAX_WAVE=32`, `--validate --pipelines --list --jobs 1` against the
  local authorized pack. Full output is private in `ztrx-lds-before.log`.

## LDS-flag validation branch

Kept the blend checkpoint on `fix/rendering-regressions` at `ba53c01` and started
`fix/lds-flag-validation` from it. The community LDS-flag commit
`1402bf7b821fc4ca9c28073debed28e8ee65fb0d` is preserved with attribution as local
`a980e85`. This is an experiment on top of the compatibility stack, not a claim
that all wave64 programs or ray tracing now work.

- The synthetic flag pipeline fails on the original lowering with the same
  invalid-parameter rejection as the observed title compute. The original test
  helpers are synthetic and remain authorized public inputs.
- Added two independent regressions (local `ff1881b`): a live region read must keep
  64 lanes; a hardware test validates DXIL and checks per-lane output in 192
  dispatches. Its four program variants write once/twice and select lanes above/
  below a root-constant threshold. Eight thresholds cover empty/full masks and
  boundaries 31/32/33, two scalar paths run/skip the internal read, each repeated
  three times. All 64 lanes have distinct output dwords; a single shared output
  would hide disagreement between lanes. Hardware test and all nine wave64 tests
  pass without a hardware/validator skip on this machine.
- Offline real-input check: before, the affected compute emitted/validated but
  could not create a hardware pipeline; afterward it emits, validates and creates
  one. No shader contents or raw logs are public. Candidate diagnostic SHA-256:
  `B9AEE57B592948B14B4DC5C67D39BB422FF4CEFDF4EA49464FDD604E48CBB2BE`.
- Release and diagnostic builds and both CI Clippy commands pass; source policy
  passes for 189 tracked files. Global fmt still fails on existing/imported source
  formatting; our new test block is formatted without reformatting other code.
- This coverage does not prove every divergent loop, side-effect combination,
  adapter or real title dispatch correct. Full runtime visual validation of this
  LDS candidate has not been performed. Keep the previous binary/hash/caches for
  controlled comparisons; do not infer an FPS or lighting improvement from PSO
  creation. The unchanged failing LDS-atomic/cross-half shaders remain out of scope.
- The same offline candidate with `VICTORY_LDS_FLAGS=0` validates the compute but
  cannot create its pipeline again. This isolates the lowering change with the
  same executable; the CLI uses no runtime shader cache. The nine wave64 tests
  also pass in the normal feature configuration and on the minimal branch whose
  parent is the contributor's exact `1402bf7` (without #56 or local helpers).

### Cache-mode correctness follow-up

Found a missing dependency in `shader_cache.rs::build`: the imported lowering
reads `VICTORY_LDS_FLAGS`, but this switch was absent from the persistent build
fingerprint. Reusing a runtime cache across enabled/disabled runs could serve the
opposite translation and invalidate an A/B comparison. Offline translate does
not use that cache, so the offline control above is unaffected.

Added a subprocess regression which exercises the real build fingerprint and
OnceLock with inherited settings, changing only LDS mode. Before the fix it
observes identical keys and fails; afterward modes differ and each mode remains
stable across processes. The parent process's environment is never mutated.
The fix adds this switch to the existing settings list: local commit `8e82bfd`.
All three cache tests pass, and shader_cache.rs passes rustfmt independently.

Prepared minimal `test/lds-flag-results`, based on the original PR #36 head, with
only our hardware/live-value tests (`4ec17b6`) and cache-key correction (`4c3d260`).
The exact branch passes all nine wave64 tests and all three cache tests; no GPU or
validator case was skipped locally. It has no profiling/input/audit dependencies.

Published both follow-ups on that branch and shared their reproducible evidence
in [PR #36](https://github.com/Zettrox/ztrx/pull/36#issuecomment-7161744657).
No duplicate source PR or upstream merge was made. Final combined validation after
the cache fix: release + diagnostic builds pass; both CI Clippy configurations
pass; serial all-feature workspace tests: **518 passed, 0 failed, 7 ignored**;
normal-feature GPU tests: **114 passed, 0 failed, 1 ignored**. Asset policy and
diff whitespace pass; the global fmt limitation above remains explicit.

Final clean source checkpoint is `8e82bfd28c9821e1cdc529929debd2a1cf7b9471`
(subsequent changes are documentation only). Preserved diagnostic executable and
build metadata outside Git. SHA-256:
`B6581079FF5C3049314B07869094E65ACCC688BC7D376F8394AC69528150872A`.
The earlier pre-cache-fix LDS binary is separately preserved as well. No runtime
capture with this final candidate is claimed yet.

## LDS runtime failure — unresolved

The final executable above was then exercised in `candidate-lds-world`, requested
for 900 seconds. The run selected the profile with one virtual Enter pulse and
advanced through changing introduction/credit/third-person images. Both affected
mesh and compute pipelines create in the real runtime. The scene still has missing
surfaces and lighting; the BVH builder remains behind the original default gate.
No graphics setting or required operation was newly disabled for this capture.

The process exited on its own after 759.348 seconds with `0x80000003` (a guest
breakpoint/unhandled exception), not the requested deadline. The private title
report records about 744 seconds of title running time. Its allocation/streaming
and queue state provide investigation leads, not an established root cause.
No device-removal event was identified in the captured log. Unit tests/PSO
acceptance therefore do not justify calling the LDS candidate runtime-stable.
Keep it isolated on the validation branch; no upstream merge was performed.

The exported prefix ends at profiler second 743.188: 3,856 Present + 311 interval
records. All 4,167 CSV/JSONL pairs match, zero reported loss, 2,032,964 bytes.
Profiler seconds 300-540 contain 61 complete intervals, mean 3926.1028 ms /
0.254706 Present/s. This is an incomplete changing renderer, with undersampled
tails, not a playable-world benchmark or a matched speedup/regression comparison.

WPR CPU recording was attempted at 12:19:37 UTC and failed before recording with
`0xc5585011`; its unique recorder instance was verified inactive. No ETL exists.
A later W movement pulse was written after the process had already exited and
has no acknowledgement: no movement response is claimed. The last valid UI
observation is insufficient to establish interactive world entry.

Next isolate the failure with the same preserved executable and LDS mode disabled,
using a recorded save/settings snapshot, independently conditioned shader keys and
no ETW attempt. If needed repeat enabled mode from that exact snapshot. Do not
raise guest allocation limits or remove barriers merely to pass this failure.

## LDS-disabled runtime control

Recorded a private eleven-file, ~2.1 MB save/settings snapshot after the failed
enabled run, with SHA-256 manifest (`data-lds-ab-seed`), then copied it to a separate
control directory. The original data directory was not overwritten. The control
uses the exact same preserved executable SHA-256, `VICTORY_LDS_FLAGS=0`, the same
hardware/default graphics settings, and its separately keyed shader-cache mode.
Only one virtual Enter pulse selects the profile. No ETW attempt occurs in it.
The original enabled run predates this snapshot: this is not a matched save/scene
pair for performance comparisons or time-to-crash claims.

`candidate-lds-off-control` also exits naturally with `0x80000003`, at the same
guest-relative fault location as the enabled run. Recorded process duration is
905.314 s, `stopped_at_deadline=false`; the title report gives ~891 s running time.
The affected compute pipeline is again refused as expected with LDS disabled.
Both private title reports identify the same unsignaled cross-queue barrier and
prestream state. One allocation pool near its limit in the enabled report is well
below its limit in the disabled report; that observation alone cannot explain
both crashes. The failure can occur without LDS lowering, which does not prove
LDS is regression-free or establish the shared failure's root cause.

The disabled capture's complete exported prefix has 3,783 Present + 304 interval
records: all 4,087 CSV/JSONL pairs match, zero reported loss, 1,994,038 bytes,
last Present at profiler second 890.017. Seconds 300-540: 65 complete intervals,
3541.7267 ms mean / 0.282348 Present/s, with undersampled tails. Rendering remains
partial and changing. No speedup or playable-world result is inferred from this.

An attempted five-point CIM memory sample completed after the process had already
exited (13:31:49-54 UTC, versus a run starting at 12:39:01 UTC). It measures the idle
system, not the renderer. Marked it invalid for runtime-pressure inference in an
adjacent validity JSON; it must not support a RAM/exhaustion conclusion. This was
an unsuccessful measurement attempt, retained explicitly rather than discarded.

Reviewed [existing issue #28](https://github.com/Zettrox/ztrx/issues/28): an original
main run reports the same exception class and unsignaled-barrier/prestream symptoms,
but a different guest fault location. It is a relevant lead, not confirmation of
an identical root cause. Inspect queue memory-wait progress, missing pipeline
outputs and streaming state before changing scheduling, fences or allocation limits.

Updated our existing [PR #36 validation comment](https://github.com/Zettrox/ztrx/pull/36#issuecomment-7161744657)
with the runtime failures and their limits, and contributed the aggregate
[reproduction summary to issue #28](https://github.com/Zettrox/ztrx/issues/28#issuecomment-7164309859).
No raw title reports, telemetry identifiers, captures or shader data were uploaded.
The next reference experiment should replay the preserved exact-original diagnostic
executable from the same recorded data snapshot before attributing the shared
failure to a specific compatibility patch. Queue waits/cuts already exist and are
enabled by default; do not implement a duplicate scheduler or bypass their waits.

## Windows texture no-wait trial

Refreshed the PR inventory and issue #28 before the experiment, during it and
before exact-branch validation. Main remains `90b529f878a470fec9bf07b7be316bc769a12232`;
PR #137 remains `55fb82c90a056aae1767afb795ad010bb3c9af5b`. The inventory now
includes #142, which replaces closed #138/#140/#141. See UPSTREAM_ANALYSIS.md.

Tested the existing `VICTORY_TEXTURE_NO_WAIT=1` path, without importing #137's
default change. Preserved binary SHA-256:
`B6581079FF5C3049314B07869094E65ACCC688BC7D376F8394AC69528150872A`, built from
`8e82bfd28c9821e1cdc529929debd2a1cf7b9471`; clean checkout at launch was
documentation checkpoint `f00daf70c6930c78c1370ec70d167b9250b77130`.
LDS was disabled, as in the previous control. Used an independent copy of the
same frozen data seed, reused caches with unknown conditioning, and one scheduled
Enter pulse at process second 15. No further live input was sent. No concurrent
builds or GPU tests ran during the capture. Outputs and seed manifest stay private,
outside source trees.

The requested 1,200-second smoke run reached its deadline: 1,205.7987021 seconds
including forced shutdown, `stopped_at_deadline=true`, exit -1 from the script's
intentional termination. No unhandled-exception/device-removal/panic message was
found in the captured runtime log. This is one right-censored observation, not a
crash-rate estimate. The two earlier waiting-mode runs ended spontaneously;
different cache conditioning and input timing prevent causal or performance claims.

All 3,998 complete CSV/JSONL records match (3,233 Present + 765 meter intervals),
zero reported losses, 2,015,139 bytes. Last exported Present: profiler second
1197.4425372. In seconds 300-540, 232 complete intervals average 1027.9464 ms
(0.972813 Present/s); tails are undersampled. These are incomplete/changing images,
not a playable-world FPS result or a speedup comparison.

The existing detailed texture meter reported zero title-memory-lock wait in all
717 texture windows; 494 windows reported the kernel-read/deferred-refresh path.
The preceding disabled-LDS waiting control had positive waits in 226 of its 234
texture windows. This confirms the existing path is exercised and removes that
specific measured wait; it does not measure GPU execution or exclude waits elsewhere.
Graphics alternated between nearly blank images and partial 3D output. Interactive
world entry, complete lighting/geometry and visual equivalence remain unverified.

Process sampling: 1,188 samples, peak working set 17.5729 GiB, peak private memory
25.0337 GiB. NVIDIA sampling: 1,194 device-wide samples, peak allocated VRAM
12,246 MiB. Device-wide use is not per-process residency or a budget measurement.
No CPU stack/GPU timestamp attribution was obtained.

Created isolated `test/texture-no-wait-windows` at exact #137 head, with no local
source edits. Windows GPU suite: 111 passed, zero failed, one ignored,
including the new switch-parsing test. Complete serial workspace checks: 497
passed/zero failed/seven ignored normally, 505/zero/seven with all features.
Both workspace/all-targets Clippy configurations pass with `-D warnings`;
asset policy passes (170 files). The existing mesh pipeline/library test also
passes with `--nocapture` and no skip message. Global `cargo fmt --all -- --check`
fails with repository-wide differences, as it already did on the original reference;
no formatting changes were applied. The opt-in runtime trial above used the preserved
combined compatibility binary, not the exact #137 executable or its changed default.
The exact #137 release/diagnostic builds also pass and are preserved with a build
manifest outside the repository. Shared target directories now contain #137
outputs: a checkout alone does not establish an executable's source. Select an
explicit preserved executable and verify its build manifest for subsequent captures.

The first documentation policy invocation reused the shared-target `xtask` built
in the PR worktree and checked its 170 files instead of this checkout's 189.
`xtask::workspace_root` embeds `CARGO_MANIFEST_DIR`; the workspace path reported by
Cargo metadata was correct, but the reused helper still targeted the other tree.
Rejected that invocation as validation of these documents. Rebuilt the helper in
`target/xtask-root-verification` and reran: all 189 files pass. Added an AGENTS rule
to isolate checkout-dependent helper outputs and verify their scope. No runtime
source or upstream tooling change was made for this local verification workaround.

Published the scoped checks and opt-in observation in the existing
[#137 discussion](https://github.com/Zettrox/ztrx/pull/137#issuecomment-7166157749),
after rechecking its head and current comments. No duplicate PR, default change,
raw title artifact or unvalidated performance claim was published.

## Changed-file inventory

- Profiling commit: Cargo.lock; crates/victory-gpu/Cargo.toml;
  crates/victory-gpu/src/{host,lib,pipeline,profiling}.rs;
  scripts/{benchmark,summarize-profile,test-profile-summary}.ps1;
  docs/{BENCHMARKING,pr-frame-profiling}.md.
- Input helper: crates/victory-hle/src/modules/gameinput.rs and gameinput/live.rs;
  docs/{DIAGNOSTIC_INPUT,pr-runtime-input}.md.
- Compatibility follow-ups: community changes in GPU pipeline/host.rs and shader
  emit/lower.rs/emit/wave64_tests.rs; our hardware blend regression in pipeline/host.rs,
  per-lane/negative regressions in emit/wave64_tests.rs, and mode-key correction/
  subprocess regression in GPU shader_cache.rs. These are separate from PR #82.
- Audit: AGENTS.md and docs/{NATIVE_ARCHITECTURE,PERFORMANCE_AUDIT,UPSTREAM_ANALYSIS,
  TEXTURE_AOT_PLAN,SHADER_AOT_PLAN,MEMORY_REDESIGN,DXR_ROADMAP,DEVELOPMENT_ROADMAP,PROGRESS}.md.

## Failures and limitations

- Restricted sandbox networking blocked missing Cargo dependencies and loopback
  tests. Repeating with host permissions succeeded. This is not a product fix.
- An initial parallel all-feature candidate run ended with 0xc0000005 in GPU tests.
  Serial and isolated parallel retries passed. Root cause remains unproven.
- The clean reference GPU diagnostic suite independently failed its timing-sensitive
  `frame.rs:241` assertion in parallel (116 passed/1 failed), then passed (117/0).
  Both source versions pass serially. Do not conflate this with the access violation.
- OneDrive incremental compilation sometimes falls back from hard links to copies.
- No repeated controlled before/after gameplay comparison, instrumented observer-cost
  estimate, long stability test, DRED/PIX trace, or multi-adapter validation yet.
  CPU fence waiting is not GPU execution time; device-wide NVIDIA utilization is
  not per-process GPU work. Dynamic resource correctness is unchanged.
- Input pulses are acknowledged by the helper, not by the title: always observe
  the UI response. Two Menu pulses during the cold introduction did not reveal a
  usable menu; no further control outcome or completed world entry is claimed.

## Decisions and hypotheses

The confirmed small issue is misleading/lossy telemetry. Fix measurement first,
reuse existing counters, and retain graphics work, fences and default settings.
Community reports and source support investigating preparation/copy-back visibility
as a candidate critical path. Local cold windows now show substantial preparation/
texture costs while shader/PSO workers remain active. This does not prove the steady
scene bottleneck. `draw.rs::heap_buffers` (1270, timing at 1404-1405) also calls
`heap_textures`: its reported pass duration is inclusive, not pure descriptor-scan
cost and not an independent number to add to preparation/texture totals.
Active shader/texture/depth-bound PRs and upstream selective-page work should be
reviewed before developing overlapping fixes. Texture/shader AOT stays incremental
and shares existing libraries/caches; no new runtime architecture is introduced.

## Compute triage and descriptor record counts

Grouped the existing 1,200-second no-wait capture. Counts below are log entries,
not accumulated dispatches. Compute emission refused three f64 conversions and
one each for descriptor address/format, divergent lane-mask exchange and permlane.
D3D12 refused five compute PSOs with `0x80070057`. Four recurring PSOs appeared in
188, 187, 187 and 135 drop windows (up to two per frame each); divergent lane-mask
exchange appeared in 130 windows. The separate opt-in BVH builder was inactive
(`pipeline.rs::builder_runs`, `draw.rs::dispatch`): 187 windows, up to 3,033
reported drops per frame. The capture has no matching UAV/barrier failure or
device-removal report. Earlier synchronization evidence remains a separate lead.

The preserved runtime binary reproduced four remaining PSO refusals offline
after DXIL validation. The D3D12 debug layer identified a required wave size of
64 on 32-lane hardware for the two recurring cases retested. Investigate the
wave-size choice in `emit/lower.rs` and existing #30/#36 before changing barriers
or relaxing wave requirements. Existing #52/#142, #49/#124 and #58/#118 cover
overlapping lowering areas. #58 specifically targets the observed address/format
compute rejection. None was stacked here.

Exact #127 (`e3ad2a2d48906a2256e70669932fff4e10e55160`) emitted, validated and
created all five selected test pipelines on the RTX 5070 Ti. The reference
(`90b529f`) emitted zero. The full-resolution compute case also created its
test pipeline on a single-worker repeat. `descriptor/host.rs::note_buffer`,
`draw.rs::heap_words` and `emit/lower.rs::heap_view_word` are the relevant table
producer, binder and consumer. A mapped-table probe showed that the producer
returned 5768/5 for synthetic SRV/UAVs with 37 elements. Local follow-up
`fc334165bcc83e3fdf93de8e150464832d3d808c` reads NumElements via the existing view
decoders. Both regressions now return 37. Release, focused Clippy, the 17-test
descriptor group and asset policy pass. The correction depends on #127's table;
descriptor format encoding, GPU readback and real-game dispatch effects remain
unverified. No matched visual, drop-rate or frame-time comparison is available.
The previous 300–540 s window remains 1027.95 ms per successful Present.

The initial offline reference command left three diagnostic dumps in the source
directory because a null .NET environment setter created empty variables.
Moved the untracked files outside Git immediately. The repeat ran from the
private evidence directory after removing diagnostic environment keys and
produced no dumps. Only aggregate results are recorded here.

## Isolated compute PR checks

The original Git remote and PR API returned 404. Recovered #30, #49 and #54
from their public author branches at the recorded heads. Used the preserved
#36 head. Offline tests used the RTX 5070 Ti, a 32-lane host setting and the
Windows SDK DXIL validator. #127 and the independent `fc33416` follow-up
stay in their own worktree.

| PR and tested head | Relevant hardware result |
| --- | --- |
| #30 `52e06e5c18b78d37bf6b2e559b3c234e7211a105` | Its divergent-barrier fixture completed with 1, 10, 32, 33 and 64 live lanes. The three real wave64 compute candidates validated but created zero PSOs. |
| #36 `1402bf7b821fc4ca9c28073debed28e8ee65fb0d` | Three validated, one PSO created for the prior LDS flag case. Both remaining targets still declare wave64 and are refused. |
| #49 `e06eef85826bb942178fb2e8cabd3065996df158` | The real permlane compute went from zero emitted to one validated and one PSO created. The new full-EXEC fixture checked four dispatches with 64 distinct lane outputs each. |
| #54 `0bd72dac09e8df1ca90ae8834043e4eaeec0101d` | Three prior f64 emission failures advanced. Two validate but their wave64 PSOs are refused. The third stops on a lane mask read as data. Arithmetic and cross-half reductions pass the synthetic tests. |

#36 retains the previous evidence for LDS regions that only raise a flag (192
hardware dispatches and a negative live-read fixture). The two remaining cases
include divergent LDS reads and writes, 64/128-thread groups and lane-dependent
mask or lane-read operations. Upstream [#189](https://gf.openbay.app/greyfinch/VIctory/issues/189)
and [#439](https://gf.openbay.app/greyfinch/VIctory/issues/439) cover related
emulation work with different group-size limits. A safe lane-equivalent lowering
for these two programs remains open. The #49 test is bounded to full EXEC
and leaves inactive-source semantics for a separate review.

Local test-only follow-ups: `2db173cb597f1126b33a7b2dbb8369283693c0d4` for
#49 and `dd73e115c775efd2618a6096b0dece0feaf07174` for #54. The latter checks
all 64 lane values at eight EXEC cutoffs instead of reading a single shared
slot. Both ran on the physical GPU. Shared Cargo target output initially reused
the #30 binary for #49 and #54. Discarded those outputs and rebuilt #49 and #54
in separate target directories, confirming distinct executable hashes.

The paired intro replay uses `8e82bfd` and `5338222` (the same source plus #49).
Its input schedule, runtime flags and frozen data are equal. The two run roots
start with 11 byte-matched save files and seven matched image/PSO cache files.
Both shader caches start empty since their keys depend on executable content.
Both completed the scripted 600-second intro replay at a 3840×2160 source,
with no exporter loss. The target PSO changed from zero creations and ten
drop windows (peak two per frame) to one creation, an observed first bind and
zero target drop windows. Compute PSO refusals remained five on each arm.

In nominal profiler seconds 390–540, mean Present interval was 870.35 ms
on the reference (172 intervals) and 924.16 ms on the candidate (161).
Medians were 675.29 and 600.82 ms. The visual checks revealed different shots
and stages at the same elapsed time. Near the deadline the reference was
outdoors with incomplete surfaces while the candidate still showed credits.
Some candidate views also had foreground occlusion. This pair measures
runtime admission and named drop windows. A fixed scene, fixed camera and
counterbalanced repeat are required for a frame-time or visual-quality result.

## Single-wave64 compute on the RTX 5070 Ti

Isolated the 64-thread compute case `369cbe906ea72764`. The other refused
case is a 128-thread group and remains separate. The preserved #36 build
validates DXIL but D3D12 rejects its requested 64-lane wave. Upstream
[`shader/console-flow-lds`](https://github.com/r4tedx/VIctory/tree/shader/console-flow-lds)
at `3fab5a19f586c690fedf4578c1fb7b4c2de298dd` emits and validates the same
input and creates one compute PSO on the 32-lane RTX 5070 Ti. The isolated
release binary SHA-256 is
`5FEF28B9C9827C66F9ACCAC681209B37B849C7F21A3776F3E4A8EEF85CF30BCF`.

Seven selected upstream tests passed on that adapter with the DXIL validator.
They exercised logical masks, cross-half lane reads and reduction, masked
permutation, ordered LDS, divergent read/write blocks, explicit barriers and
visibility atomics. The nested EXEC fixture matched 1,024 lane results across
eight masks in two separate 64-thread groups. Private input and logs remain
outside source trees. Real title validation stops at PSO creation. The
dispatch/readback evidence comes from independent synthetic inputs.

Test-only follow-up `9ae78c75684d4915367a3bd20dc86e82470dc179` adds GPU
timestamps around a synthetic dispatch of 2,048 groups, with three warm-up
rounds and twelve interleaved samples per arm. Every output matches the CPU
reference. Median dispatch time was 2.86 us (2.75–4.19) for emulation, versus
3.33 us (3.14–4.42) for a separately authored native wave32 reference with
explicit barriers. The ranges overlap and the emitted barrier forms differ.
This is a workload-specific GPU cost. Release, focused tests and Clippy
passed. The benchmark is ignored by default.

No extra lowering patch was required on the upstream branch. Its shared
ancestor with the ztrx fork is `ffe310e` and the implementation depends on the
console-flow stack. The test and the emulation remain isolated from ztrx
runtime, `fc33416`, `2db173c` and `dd73e11`. A matched visual comparison
requires a compatible integration and a reproducible scene or a title
dispatch replay with captured bindings. No game launch was performed.

### Fork pipeline boundary

Verified the public `console-flow-lds` head still matches `3fab5a1`. The
standalone LDS change `7388024` fails a patch applicability check against
the fork. It uses the upstream console-flow control translator and
capability model. Importing its LDS barriers alone would leave the fork
without that execution model.

Compared the 64-thread shader's emitted binding contracts. Both versions
fill user-data dword 4 from descriptor table slot 0, dword 0. The emulated
module requires zero extra root descriptors, samplers, title-memory tables,
or write-mark resources. The title table itself has five SRVs, seven UAVs
and one CBV.

An isolated release test at `5338222` used the fork's runtime
`HostPipeline::new` in both arms on the RTX 5070 Ti. The original DXIL was
refused. With validated upstream DXIL and the same fork resource and
binding metadata, the same constructor created the PSO. One test passed in
0.62 s. The temporary test and logs are preserved outside the source trees
and the worktree is clean. This verifies pipeline creation for this exact
table-only shader. Populated descriptors, resource states and dispatch
output have not been verified on this path.

No game launch or runtime translator integration was needed for this test.
A general bridge would need to reject mismatched contracts and separate
cache identity by emitter and target. Keep the 128-thread case separate.

### Single-wave64 dispatch and descriptor validation

The upstream emitter at `3fab5a1` annotated scalar reads of a structured
table SRV as raw reads. With a matching structured view and the upstream
CBV representation, a bounded replay returned a nonzero value for a lane
the CPU reference rejected. A native D3D12 CBV exposed a separate ABI
difference through GPU validation message 939. Upstream uses a raw SRV
for that guest table CBV, while the older fork uses a native host CBV.

Local commit `d44159f` on `fix/structured-scalar-loads` keeps the
structured SRV and calculates an element index and byte offset per dword.
Its synthetic regression crossed an element boundary on the RTX 5070 Ti.
The local 64-thread replay used five synthetic SRVs, seven UAVs and one
guest CBV at a nonzero table base. Seven UAV readbacks matched a CPU
reference in both ABI configurations, with canaries intact and zero
D3D12 GPU validation warnings or errors. A private emitter prototype
retained the native host CBV for the older fork. That variant also created
a PSO through the fork's existing constructor. The prototype is isolated
from the production fork and uses no shader-specific dispatch rules.

An independent check of the exact 128-thread case on `3fab5a1` failed
before DXIL emission. The emulation uses group-wide barriers and two
exchange slots, so the emitter refuses cross-half wave64 communication
when a group contains two logical waves. Integrated rendering and frame
timing remain pending until both cases have a working runtime path.

## Next priority

The isolated #137 Windows checks have been shared on the recorded head. The
existing opt-in no-wait path completed one bounded trial and removes the measured
texture lock wait, but its default must not change here on that basis alone.
Repeat with matched frozen cache/data copies and identical input timing before
claiming a stability or frame-time improvement. For compute, keep the
scalar SRV correction separate and scope any older-fork integration to a
verified descriptor ABI. For the 128-thread case, start with an independent
fixture for cross-half exchanges in two logical waves under different
control flow. Compare a matched scene once both paths are validated.

The integer-target blend fix passes a hardware regression and its actual pipeline
now creates. LDS lowering passes bounded hardware/negative/offline checks and now
keys its mode in the cache, but runtime failure occurs in both modes. Next isolate
the shared guest breakpoint/prestream failure, using issue #28 as a lead and
existing queue/list/copy diagnostics. Keep the LDS experiment separate and do not
integrate it as runtime-stable. Obtain a reliable CPU stack trace when host profiling
permissions permit. Do not treat accepted PSOs or intro images as a complete world.
Reach and verify a stable 3D scene, record a repeatable save/camera/window, then collect
matched reference/candidate and profiler-off/on runs with conditioned caches.
Rank CPU preparation, fence waits, copy-back, GPU execution and residency separately.
Only then select one small synchronization/transfer optimization. Keep gameplay
compatibility work and broader audit documentation independently reviewable.
