# Preserve structured SRVs in scalar buffer loads

## Summary

Lower scalar reads of structured table SRVs using the descriptor's
declared stride. Compute the element index and offset for each dword,
including loads that cross an element boundary.

## Problem and root cause

The scalar load path annotated every SRV as a raw buffer. A shader could
therefore create a valid compute PSO and read the wrong data from a
structured view. A 64-thread hardware replay produced a nonzero value
for a lane that the CPU reference rejected.

## Validation

- Base `3fab5a1`, candidate `d44159f`, RTX 5070 Ti.
- A synthetic release regression dispatches a 64-thread group and reads
  a dword from the second element of a 12-byte structured SRV.
- With the same synthetic inputs and upstream descriptor ABI, the
  original module disagreed with the CPU reference. The candidate
  matched all seven UAV readbacks and preserved guard elements with
  D3D12 GPU-based validation enabled.
- Focused release Clippy and `git diff --check` passed. The workspace
  formatting check reports existing differences outside the patch.

## Compatibility and limitations

The change applies to scalar reads of table SRVs whose reported stride
is dword-aligned. Existing CBV and raw views keep their established
bindings. Runtime integration with an older CBV ABI and emulation of
two logical waves per group are separate changes. Scene and frame-time
measurements are pending.
