//! Lightwalletd endpoint failover.
//!
//! `pirate-wallet-service` only tracks one active lightwalletd endpoint per
//! wallet (`set_lightd_endpoint`) with no built-in failover between servers.
//! This tracks an ordered priority list on top of that single-endpoint API:
//! on repeated sync failure it switches to the next endpoint, and it
//! periodically re-probes higher-priority endpoints so it switches back once
//! one of them recovers, rather than staying on a lower-priority endpoint
//! forever.

/// Consecutive sync failures against the active endpoint before failing over.
pub(crate) const FAILURE_THRESHOLD: u32 = 3;

pub struct FailoverState {
    /// Priority order: index 0 is the most preferred endpoint.
    endpoints: Vec<String>,
    active_index: usize,
    consecutive_failures: u32,
}

impl FailoverState {
    pub fn new(endpoints: Vec<String>) -> Self {
        assert!(!endpoints.is_empty(), "failover requires at least one endpoint");
        FailoverState {
            endpoints,
            active_index: 0,
            consecutive_failures: 0,
        }
    }

    pub fn active_url(&self) -> &str {
        &self.endpoints[self.active_index]
    }

    /// Record whether the last sync attempt against the active endpoint
    /// succeeded. Returns `Some(url)` if the caller should switch the active
    /// lightd endpoint to `url` (already reflected in `active_url()`).
    pub fn record_result(&mut self, ok: bool) -> Option<String> {
        if ok {
            self.consecutive_failures = 0;
            return None;
        }
        self.consecutive_failures += 1;
        if self.consecutive_failures < FAILURE_THRESHOLD {
            return None;
        }
        self.consecutive_failures = 0;
        self.active_index = (self.active_index + 1) % self.endpoints.len();
        Some(self.endpoints[self.active_index].clone())
    }

    /// Endpoints with strictly higher priority than the currently active
    /// one, in priority order. Empty when already on the primary endpoint.
    pub fn higher_priority_endpoints(&self) -> impl Iterator<Item = (usize, &str)> {
        self.endpoints[..self.active_index]
            .iter()
            .enumerate()
            .map(|(i, url)| (i, url.as_str()))
    }

    /// Switch the active endpoint to `index` (e.g. because it was found to
    /// have recovered) and reset the failure counter.
    pub fn switch_to(&mut self, index: usize) -> String {
        self.active_index = index;
        self.consecutive_failures = 0;
        self.endpoints[index].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stays_on_primary_while_healthy() {
        let mut state = FailoverState::new(vec!["a".into(), "b".into()]);
        assert_eq!(state.record_result(true), None);
        assert_eq!(state.active_url(), "a");
    }

    #[test]
    fn fails_over_after_threshold_and_wraps_around() {
        let mut state = FailoverState::new(vec!["a".into(), "b".into()]);
        assert_eq!(state.record_result(false), None);
        assert_eq!(state.record_result(false), None);
        assert_eq!(state.record_result(false), Some("b".to_string()));
        assert_eq!(state.active_url(), "b");

        assert_eq!(state.record_result(false), None);
        assert_eq!(state.record_result(false), None);
        assert_eq!(state.record_result(false), Some("a".to_string()));
        assert_eq!(state.active_url(), "a");
    }

    #[test]
    fn success_resets_failure_counter() {
        let mut state = FailoverState::new(vec!["a".into(), "b".into()]);
        state.record_result(false);
        state.record_result(false);
        assert_eq!(state.record_result(true), None);
        assert_eq!(state.record_result(false), None);
        assert_eq!(state.record_result(false), None);
        assert_eq!(state.active_url(), "a");
    }

    #[test]
    fn higher_priority_endpoints_empty_on_primary() {
        let state = FailoverState::new(vec!["a".into(), "b".into(), "c".into()]);
        assert_eq!(state.higher_priority_endpoints().count(), 0);
    }

    #[test]
    fn higher_priority_endpoints_after_failover() {
        let mut state = FailoverState::new(vec!["a".into(), "b".into(), "c".into()]);
        for _ in 0..FAILURE_THRESHOLD {
            state.record_result(false);
        }
        assert_eq!(state.active_url(), "b");
        let higher: Vec<_> = state.higher_priority_endpoints().collect();
        assert_eq!(higher, vec![(0, "a")]);
    }

    #[test]
    fn switch_to_resets_counter() {
        let mut state = FailoverState::new(vec!["a".into(), "b".into()]);
        for _ in 0..FAILURE_THRESHOLD {
            state.record_result(false);
        }
        assert_eq!(state.active_url(), "b");
        assert_eq!(state.switch_to(0), "a");
        assert_eq!(state.active_url(), "a");
        assert_eq!(state.record_result(false), None);
    }
}
