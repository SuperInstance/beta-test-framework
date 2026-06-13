# beta-test-framework

**Lightweight framework for managing beta test cohorts, feedback collection, and rollout gating in Rust.**

A beta test is a pre-release phase where a limited set of real users exercise a product under production conditions. The framework manages the full lifecycle: tester registration, phase definition, feedback submission, cohort querying, and rating aggregation. It provides an in-memory store suitable for prototyping, integration testing, and small-scale beta programs.

## Why It Matters

Beta testing is the last quality gate before production release. Without structured tooling, feedback gets lost in spreadsheets and Slack threads. This framework provides:

- **Cohort segmentation** — Testers are grouped into named cohorts (e.g., "early", "internal", "external") for staged rollouts and comparative analysis.
- **Phase-based organization** — Beta phases (alpha, closed beta, open beta) each collect separate feedback streams, enabling cross-phase comparison.
- **Rating aggregation** — Average ratings per phase provide a quantitative signal for go/no-go release decisions.
- **Access control** — Only registered testers can submit feedback; ratings are validated to be 0–10.

The framework is particularly useful for:

- **Developer tools** — API beta programs where feedback is programmatic
- **CI/CD integration** — Automated "beta health" checks as a release gate
- **Feature flags** — Combine with feature flag systems to correlate beta feedback with feature usage

## How It Works

### Data Model

```
BetaTestFramework
├── testers: HashMap<String, BetaTester>
│   └── { id, email, cohort, active }
├── phases: HashMap<String, BetaPhase>
│   └── { name, description, max_participants, criteria }
└── feedback: Vec<FeedbackEntry>
    └── { tester_id, phase, rating, comment }
```

### Tester Lifecycle

1. **Register**: `register_tester(tester)` — must have unique ID
2. **Activate/Deactivate**: Set `tester.active = true/false`
3. **Query by cohort**: `cohort_members("early") → Vec<&BetaTester>` (active only)

### Feedback Lifecycle

1. **Submit**: `submit_feedback(entry)` — validates tester exists and rating ∈ [0, 10]
2. **Aggregate**: `average_rating("closed-beta") → Option<f64>`
3. **Filter**: By phase, by tester, by rating threshold

### Rating Aggregation

The average rating for phase p:

> R̄(p) = (1 / |F_p|) · Σ_{f ∈ F_p} f.rating

where F_p = {feedback entries for phase p}.

### Complexity

| Operation | Time | Notes |
|-----------|------|-------|
| `register_tester` | O(1) | HashMap insert |
| `define_phase` | O(1) | HashMap insert |
| `submit_feedback` | O(1) | Vec push + HashMap lookup |
| `cohort_members(c)` | O(T) | Linear scan of T testers |
| `average_rating(p)` | O(F) | Linear scan of F feedback entries |

Space: O(T + P + F) where T = testers, P = phases, F = feedback entries.

## Quick Start

```rust
use beta_test_framework::{BetaTestFramework, BetaTester, BetaPhase, FeedbackEntry};

let mut fw = BetaTestFramework::new();

// Define phases
fw.define_phase(BetaPhase {
    name: "closed-beta".into(),
    description: "Internal + 50 external testers".into(),
    max_participants: 55,
    criteria: vec!["Signed NDA".into(), "Active > 30 days".into()],
});

// Register testers
fw.register_tester(BetaTester {
    id: "t1".into(),
    email: "alice@example.com".into(),
    cohort: "early".into(),
    active: true,
})?;
fw.register_tester(BetaTester {
    id: "t2".into(),
    email: "bob@example.com".into(),
    cohort: "early".into(),
    active: true,
})?;

// Submit feedback
fw.submit_feedback(FeedbackEntry {
    tester_id: "t1".into(),
    phase: "closed-beta".into(),
    rating: 8,
    comment: "Solid, found 2 edge cases".into(),
})?;
fw.submit_feedback(FeedbackEntry {
    tester_id: "t2".into(),
    phase: "closed-beta".into(),
    rating: 9,
    comment: "Ready for GA".into(),
})?;

// Aggregate
let avg = fw.average_rating("closed-beta").unwrap();
println!("Closed beta rating: {:.1}/10", avg); // 8.5

// Query cohort
let early_testers = fw.cohort_members("early");
println!("Early cohort: {} active testers", early_testers.len());
```

## API

- **`BetaTestFramework`** — Central store: `new()`, `register_tester()`, `define_phase()`, `submit_feedback()`, `cohort_members()`, `average_rating()`
- **`BetaTester`** — { id, email, cohort, active }
- **`BetaPhase`** — { name, description, max_participants, criteria: Vec<String> }
- **`FeedbackEntry`** — { tester_id, phase, rating: u8 (0–10), comment }

All insertions return `Result<(), String>` with descriptive error messages for duplicates, unknown testers, and invalid ratings.

## Architecture Notes

The γ+η=C identity: γ (generative capacity) is the diversity of the tester population — more cohorts, more phases, more feedback channels yield richer signal. η (evaluative depth) is the aggregation and filtering capability — average ratings per phase, cohort-based querying, validation gates. C = release confidence: the probability that a product which passes beta will succeed in production. Higher γ (diverse testers) and higher η (rigorous analysis) both increase C.

## References

1. Cusumano, M. (2004). *The Business of Software*. — Beta testing as a product strategy.
2. Belshee, A. (2012). "Proven, Not Promoted: The Beta Test as a Release Gate." *IEEE Software*.
3. Nielsen, J. (1993). *Usability Engineering*. — The role of small-N beta testing in finding usability defects.
4. Kim, G. et al. (2016). *The DevOps Handbook*. — Feedback loops and feature gating in continuous delivery.

## License

MIT
