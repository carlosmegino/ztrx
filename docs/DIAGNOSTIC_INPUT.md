# Interactive diagnostic input

This optional helper is separate from the profiling contribution. Build with
diagnostics and set `VICTORY_KEYS_FILE` to an operator-owned local text file before
launch. The normal release build has no file input. If the variable is unset, there
is no file polling. The existing `VICTORY_KEYS` schedule retains its behavior.

Write one command, for example `1:Enter`, then wait for the observed result before
issuing another with a larger revision, e.g. `2:Down`. Keys are the same letter,
digit and named keys supported by VICTORY_KEYS. Each accepted command holds the
virtual key for 200 ms; the runtime logs its revision. Duplicate/older revisions and
invalid commands do nothing. Leave at least 400 ms between commands, and verify the
log/UI before repeating an uncertain action. There is no queue: only the newest
file contents are read. The first read can accept an existing valid command.
Commands must end in a newline (a partial write without it is ignored). Use ASCII
or UTF-8 without a byte-order mark, and publish each complete command atomically.

Polling is bounded to once per 100 ms and reads at most 65 bytes (commands over
64 bytes are rejected). A pulse ends independently of file availability. Do not
point this at network storage or untrusted writers. Commands affect only this
runtime's existing input readings; they do not inject keyboard events into Windows.

Use the helper for observed menu navigation when host-generated key taps are too
short for polled game input. Keep it disabled in controlled performance comparisons,
or record it identically in every run: file polling has an observer cost. Keep the
command file and derived test data outside Git. Unit tests cover revision ordering,
expiry, malformed input and input-size bounds.
