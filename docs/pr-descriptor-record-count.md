# GPU: read indexed buffer record counts from SRV and UAV descriptions

PR #127 exposes buffer descriptor words through an upload table. Its record
count came from descriptor word 2, which holds component mapping for an SRV
and part of FirstElement for a UAV. Read NumElements through the existing
SRV and UAV decoders and keep empty table entries zeroed.

On an RTX 5070 Ti two synthetic views with 37 elements previously published
5768 and 5. Both now publish 37, and clearing the SRV slot clears its entry.
The Windows release build, descriptor tests, Clippy and asset policy pass.

This is a one-file follow-up to #127 at `e3ad2a2`. Its offline pipeline admission
results are separate from the table test. Shader readback, rendered shadows and
frame timing still require a matched runtime comparison.
