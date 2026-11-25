# Diagnostics: accept bounded, revisioned virtual-key pulses from a local file

## Summary

Add optional `VICTORY_KEYS_FILE` input to the existing diagnostic virtual-pad path.
This is a separate test-harness change from frame profiling; it injects no OS input.

## Problem

Interactive test runs without an accessible controller could not advance reliably
using brief host-generated key taps. The existing 200 ms diagnostic key schedule
worked, but its immutable launch-time list required restarting to change navigation.

## Root cause

The existing schedule is a LazyLock initialized once. It cannot accept a newly
observed menu action during a run. The precise host key-tap failure remains unproven;
this change does not claim to fix physical keyboard or controller compatibility.

## Implementation

In diagnostics only, an unset-by-default file source emits the same virtual keys
as VICTORY_KEYS. Commands are newline-terminated `revision:key` records, at most
64 bytes. Duplicate/older revisions and malformed/partial records are ignored.
Reads are bounded and attempted no more than once per 100 ms. Pulses last 200 ms
with a release gap; errors cannot leave a key held. Existing scheduled keys retain
their parser and behavior through the shared key-name decoder. See DIAGNOSTIC_INPUT.

## Performance impact / benchmark methodology

No speedup claimed. No polling when the variable is unset; plain release contains
no file-input helper. When enabled, local file polling and a mutex add observer
cost. Use this for setup/diagnostics, and disable or match it in controlled runs.

## Before and after

Before: changing an input schedule required a new process. After: a logged command
advanced the live runtime from the profile screen to its startup animation without
restarting. Further scene progress is recorded separately in PROGRESS.md.

## Tests

Release/diagnostic builds passed. Targeted GameInput suite: 9 passed, including
two new revision/expiry/malformed/size checks. All-target, all-feature HLE Clippy
passed with -D warnings (incremental hard-link fallback warnings are environmental).
The new module passes edition-2024 rustfmt; asset policy and whitespace checks are
required before committing. Full workspace results for the parent profiling commit
are recorded in pr-frame-profiling.md, not claimed as a full rerun of this helper.

## Compatibility / known limitations / potential regressions

No new dependencies or default input changes. Operator must publish complete commands
atomically to a trusted local file and wait for observation before the next command.
There is no event queue; fast overwritten commands can be missed. Avoid network
storage. File I/O runs on the diagnostic input polling path and may stall on slow
storage. This is not a controller driver, remapping feature or deterministic replay.
No PR has been published.
