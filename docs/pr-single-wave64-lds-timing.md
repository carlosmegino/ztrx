# Measure single-wave64 LDS dispatches on 32-lane hardware

Add a test-only GPU timestamp path to the existing dispatch and readback
harness. Compare the existing single-wave LDS workload against an equivalent
native wave32 shader with explicit synchronization. Check every output
against the CPU result. The microbenchmark is opt-in via the test runner.

On the RTX 5070 Ti, median GPU dispatch time for 2,048 groups was 2.86 us for
emulation and 3.33 us for the explicit reference over twelve samples per arm.
The ranges overlap and the reference has different barrier encoding.
The seven selected compute fixtures and focused Clippy check also passed.

This is a follow-up to the upstream console-flow LDS branch at `3fab5a1`.
The actual 64-thread title shader validates and creates a PSO on the branch.
The later scalar SRV correction enabled a bounded 64-thread dispatch with
seven UAV results checked against a CPU reference. Integrated rendering
and frame timing remain pending. See [PROGRESS.md](PROGRESS.md).
Local commit: `9ae78c7`.

A separate fork compatibility probe kept the original descriptor-table
binding and resource metadata. On the RTX 5070 Ti the fork's runtime pipeline
constructor rejected the original DXIL and accepted the validated emulated
module. This finding is specific to this shader and covers pipeline creation.

```text
cargo test -p victory-shader --release --locked --lib single_wave_lds_timing -- --ignored --nocapture --test-threads=1
```
