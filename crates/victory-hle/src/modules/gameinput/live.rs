//! Optional local input pulses for interactive diagnostic runs. No OS input is
//! injected: the same virtual-pad path as VICTORY_KEYS consumes these keys.

use std::io::Read;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

const POLL: Duration = Duration::from_millis(100);
const HOLD: Duration = Duration::from_millis(200);

#[derive(Default)]
struct State {
    revision: u64,
    checked: Option<Instant>,
    pulse: Option<(Instant, u8)>,
}

impl State {
    fn accept(&mut self, text: &str, now: Instant) {
        if text.len() > 64 || !text.ends_with('\n') {
            return;
        }
        let Some((revision, name)) = text.trim().split_once(':') else {
            return;
        };
        let Ok(revision) = revision.parse::<u64>() else {
            return;
        };
        let Some(key) = virtual_key(name) else { return };
        if revision <= self.revision {
            return;
        }
        self.revision = revision;
        self.pulse = Some((now + HOLD, key));
        tracing::info!(revision, key = name.trim(), "diagnostic input pulse");
    }

    fn active(&self, now: Instant) -> Option<u8> {
        self.pulse
            .filter(|(until, _)| now < *until)
            .map(|(_, key)| key)
    }
}

pub(super) fn virtual_key(name: &str) -> Option<u8> {
    Some(match name.trim() {
        "Enter" => 0x0D,
        "Esc" => 0x1B,
        "Tab" => 0x09,
        "Back" => 0x08,
        "Up" => 0x26,
        "Down" => 0x28,
        "Left" => 0x25,
        "Right" => 0x27,
        k if k.len() == 1 && k.as_bytes()[0].is_ascii_alphanumeric() => {
            k.as_bytes()[0].to_ascii_uppercase()
        }
        _ => return None,
    })
}

pub(super) fn key() -> Option<u8> {
    static INPUT: LazyLock<Option<(PathBuf, Mutex<State>)>> = LazyLock::new(|| {
        std::env::var_os("VICTORY_KEYS_FILE")
            .filter(|v| !v.is_empty())
            .map(|path| (path.into(), Mutex::new(State::default())))
    });
    let (path, state) = INPUT.as_ref()?;
    let now = Instant::now();
    let mut state = state.lock().unwrap_or_else(|e| e.into_inner());
    // Finish the pulse and leave at least a poll's gap before accepting another.
    let ready = state.pulse.is_none_or(|(until, _)| now >= until + POLL);
    if ready
        && state
            .checked
            .is_none_or(|at| now.duration_since(at) >= POLL)
    {
        state.checked = Some(now);
        if let Ok(file) = std::fs::File::open(path) {
            let mut text = String::new();
            if file.take(65).read_to_string(&mut text).is_ok() {
                state.accept(&text, now);
            }
        }
    }
    state.active(now)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revisions_are_idempotent_and_pulses_expire() {
        let mut state = State::default();
        let now = Instant::now();
        state.accept("1:Enter\n", now);
        assert_eq!(state.active(now), Some(0x0D));
        state.accept("1:Down\n", now + HOLD);
        assert_eq!(state.active(now + HOLD), None);
        state.accept("0:Down\n", now + HOLD);
        assert_eq!(state.revision, 1);
        state.accept("2:Down\n", now + HOLD + POLL);
        assert_eq!(state.active(now + HOLD + POLL), Some(0x28));
    }

    #[test]
    fn malformed_or_oversized_commands_do_not_press_keys() {
        for text in [
            "1:Unknown\n",
            "bad:Enter\n",
            "0:Enter\n",
            "1:\n",
            "Enter\n",
            "1:E",
        ] {
            let mut state = State::default();
            state.accept(text, Instant::now());
            assert!(state.pulse.is_none());
        }
        let mut state = State::default();
        state.accept(&format!("1:Enter{}", " ".repeat(65)), Instant::now());
        assert!(state.pulse.is_none());
        assert_eq!(virtual_key("a"), Some(b'A'));
    }
}
