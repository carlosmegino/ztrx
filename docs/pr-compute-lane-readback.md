# Compute lane readback follow-ups

## PR #49: Check permlane results in both halves of a console wave

The existing test checks that the instruction lowers. Add a synthetic
64-thread dispatch with a distinct value per source lane. Check every
destination for both permlane forms and two selector patterns. All four
dispatches validated and matched on an RTX 5070 Ti. This covers full EXEC.

The selected real compute program now validates and creates a D3D12 PSO.
It also creates and binds in the scripted introduction. The paired captures
recorded ten drop windows for that program on the reference and zero with
#49. Overall compute PSO refusals remained five. Different visible checkpoints
prevent a matched frame-time or visual-quality comparison.
The readback fixture is independent of title inputs and depends on #49.
Local commit: `2db173c`.

## PR #54: Check cross-half f64 reduction at each destination lane

Give each lane a separate output slot to detect a result reaching only
one 32-lane half. Check the sum against the CPU reference at eight EXEC
cutoffs, including 31, 32 and 33. All 512 lane results matched on an RTX
5070 Ti. The test reuses the existing synthetic reduction and depends on
#54. Local commit: `dd73e11`.

Two of the three selected title programs now validate and still require
wave64. The third reaches a lane-mask lowering error.
