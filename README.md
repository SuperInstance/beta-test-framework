# Beta Test Framework

**A Rust library for managing beta test programs** — tracks testers, cohorts, feedback, and rollout phases for software beta programs with structured data collection.

## Why It Matters

Beta testing is the critical validation phase between internal QA and public release. A well-managed beta program:

- **Segments users** into cohorts (early adopters, power users, enterprise) to get diverse feedback
- **Collects structured feedback** — ratings + freeform comments per phase
- **Gates rollout** — ensures a phase meets quality criteria before expanding
- **Measures satisfaction** — average ratings reveal whether the product is ready for GA

This framework provides the data model and operations for managing these programs programmatically, without relying on spreadsheets or ad-hoc tools.

## How It Works

The `BetaTestFramework` struct maintains three collections:

1. **Testers** (`HashMap<String, BetaTester>`) — Each tester has an ID, email, cohort assignment, and active/inactive flag. Testers can only submit feedback if they're registered.

2. **Phases** (`HashMap<String, BetaPhase>`) — Each phase has a name, description, max participant count, and acceptance criteria (e.g., "no crash on startup", "API latency < 200ms").

3. **Feedback** (`Vec<FeedbackEntry>`) — Each entry links a tester to a phase, includes a 0–10 rating, and a freeform comment. The framework computes average ratings per phase to measure overall satisfaction.

Cohort queries (`cohort_members()`) let you see who's active in a specific group, and `average_rating()` gives a quick health metric for a phase.

## Quick Start

```rust
use beta_test_framework::{BetaTestFramework, BetaTester, BetaPhase, FeedbackEntry};

let mut fw = BetaTestFramework::new();

// Register testers
fw.register_tester(BetaTester {
    id: "t1".into(), email: "alice@example.com".into(),
    cohort: "early".into(), active: true,
}).unwrap();

// Define a phase
fw.define_phase(BetaPhase {
    name: "beta-1".into(),
    description: "Initial beta with core features".into(),
    max_participants: 100,
    criteria: vec!["no_crashes".into(), "api_works".into()],
});

// Collect feedback
fw.submit_feedback(FeedbackEntry {
    tester_id: "t1".into(), phase: "beta-1".into(),
    rating: 8, comment: "Solid, minor UI bugs".into(),
}).unwrap();

println!("Average rating: {:?}", fw.average_rating("beta-1"));
println!("Early cohort: {} testers", fw.cohort_members("early").len());
```

## API

- **`BetaTester`** — id, email, cohort, active status
- **`BetaPhase`** — name, description, max_participants, criteria
- **`FeedbackEntry`** — tester_id, phase, rating (0–10), comment
- **`BetaTestFramework`** — Main manager
  - `register_tester(tester)` — Enroll a tester
  - `define_phase(phase)` — Create a test phase
  - `submit_feedback(entry)` — Record tester feedback
  - `cohort_members(cohort)` → `Vec<&BetaTester>` — Active testers in a cohort
  - `average_rating(phase)` → `Option<f64>` — Mean rating for a phase

## Architecture Notes

Provides the beta-program management layer for SuperInstance release tooling. Designed to be backed by a database in production (currently in-memory). Integrates with `canary-release` for data-driven rollout decisions. See the [architecture overview](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## License

MIT
