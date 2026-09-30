//! beta-test-framework
//! A lightweight framework for managing beta test cohorts, feedback collection, and rollout gating.

use std::collections::HashMap;

/// Represents a single beta tester.
#[derive(Debug, Clone)]
pub struct BetaTester {
    pub id: String,
    pub email: String,
    pub cohort: String,
    pub active: bool,
}

/// Configuration for a beta test phase.
#[derive(Debug, Clone)]
pub struct BetaPhase {
    pub name: String,
    pub description: String,
    pub max_participants: usize,
    pub criteria: Vec<String>,
}

/// The main framework for managing beta tests.
pub struct BetaTestFramework {
    testers: HashMap<String, BetaTester>,
    phases: HashMap<String, BetaPhase>,
    feedback: Vec<FeedbackEntry>,
}

/// A single feedback entry from a tester.
#[derive(Debug, Clone)]
pub struct FeedbackEntry {
    pub tester_id: String,
    pub phase: String,
    pub rating: u8,
    pub comment: String,
}

impl BetaTestFramework {
    pub fn new() -> Self {
        Self {
            testers: HashMap::new(),
            phases: HashMap::new(),
            feedback: Vec::new(),
        }
    }

    /// Register a new beta tester.
    pub fn register_tester(&mut self, tester: BetaTester) -> Result<(), String> {
        if self.testers.contains_key(&tester.id) {
            return Err(format!("Tester {} already registered", tester.id));
        }
        self.testers.insert(tester.id.clone(), tester);
        Ok(())
    }

    /// Define a new beta phase.
    pub fn define_phase(&mut self, phase: BetaPhase) {
        self.phases.insert(phase.name.clone(), phase);
    }

    /// Submit feedback for a phase.
    pub fn submit_feedback(&mut self, entry: FeedbackEntry) -> Result<(), String> {
        if !self.testers.contains_key(&entry.tester_id) {
            return Err(format!("Unknown tester: {}", entry.tester_id));
        }
        if entry.rating > 10 {
            return Err("Rating must be 0-10".into());
        }
        self.feedback.push(entry);
        Ok(())
    }

    /// List active testers in a given cohort.
    pub fn cohort_members(&self, cohort: &str) -> Vec<&BetaTester> {
        self.testers
            .values()
            .filter(|t| t.cohort == cohort && t.active)
            .collect()
    }

    /// Compute the average rating for a phase.
    pub fn average_rating(&self, phase: &str) -> Option<f64> {
        let ratings: Vec<u8> = self
            .feedback
            .iter()
            .filter(|f| f.phase == phase)
            .map(|f| f.rating)
            .collect();
        if ratings.is_empty() {
            return None;
        }
        Some(ratings.iter().map(|r| *r as f64).sum::<f64>() / ratings.len() as f64)
    }
}

impl Default for BetaTestFramework {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_query() {
        let mut fw = BetaTestFramework::new();
        fw.register_tester(BetaTester {
            id: "t1".into(),
            email: "a@b.c".into(),
            cohort: "early".into(),
            active: true,
        });
        assert_eq!(fw.cohort_members("early").len(), 1);
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
