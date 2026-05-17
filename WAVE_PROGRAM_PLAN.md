# Wave Program — Contributor Work Types & Sprint Scoping Plan

## Overview

The Wave Program operates on fixed-length sprint cycles (typically 2 weeks). During each cycle, maintainers publish a curated set of "Wave Issues" — scoped, bounded pieces of work that contributors can pick up, complete, and earn verifiable points for. Each completed Wave mints a Proof of Contribution badge on-chain and updates the contributor's cumulative reputation score in the WaveRegistry.

This document provides a comprehensive taxonomy of the work types that maintainers would post as Wave Issues, along with the scoping guidelines, point valuation rubric, and quality gates required for each category.

---

## 1. Bug Fixes

Bug fixes are the most common and accessible entry point for new contributors. They range from trivial one-liners to complex multi-file investigations.

### 1.1 Triage & Reproduction (5–15 points)
- **Description**: A bug report has been filed but lacks a minimal reproduction, clear steps, or environment details. The contributor's task is to reproduce the bug, document exact steps, identify the affected versions, and attach a minimal reproduction repository or test case.
- **Scoping**: The issue must link to the original bug report. The maintainer verifies the reproduction before approving the Wave.
- **Deliverables**: A comment on the original issue with reproduction steps + a link to a minimal reproduction repo or a passing/failing test.
- **Point modifiers**: +5 if the contributor also bisects to the introducing commit.

### 1.2 Simple Bug Fix (15–30 points)
- **Description**: A well-understood bug with a narrow scope — typically a single function, component, or module. The fix is small (< 50 lines changed) and the root cause is obvious from the issue description.
- **Examples**: Off-by-one error in pagination, incorrect CSS selector in a UI component, typo in an error message that breaks parsing, missing null check in an API handler.
- **Scoping**: The issue must include a clear "Expected behavior" / "Actual behavior" section, plus a link to the relevant source file(s). The maintainer pre-identifies the likely file location.
- **Deliverables**: PR with the fix + one unit test that would have caught the regression.
- **Quality gate**: Must not introduce new lint warnings. Existing tests must pass.

### 1.3 Complex Bug Fix (30–60 points)
- **Description**: A bug that spans multiple modules, requires understanding of a non-trivial interaction between systems, or involves race conditions / concurrency.
- **Examples**: A deadlock in the database connection pool under high concurrency, incorrect state reconciliation between a local cache and a remote API, a memory leak in a long-running background worker.
- **Scoping**: The issue includes a technical analysis from the maintainer describing the suspected subsystem interactions. The contributor is expected to write a short debugging log (what they investigated, what they ruled out) as part of the deliverable.
- **Deliverables**: PR with fix + regression tests + debugging log write-up.
- **Point modifiers**: +10 if the contributor also adds a stress test reproducing the original race condition.

### 1.4 Security Patch (40–80 points)
- **Description**: A vulnerability has been identified (either reported privately or surfaced by an audit/dependency scan). The contributor patches the vulnerability and backports the fix to supported release branches.
- **Scoping**: Maintainers provide a high-level description of the vulnerability class but deliberately omit exploit details until the patch is merged. The contributor works in a private fork or a security-tracker issue.
- **Deliverables**: PR with patch + regression test proving the exploit is mitigated + backport PRs for each supported release branch.
- **Quality gate**: Must pass a security review by a second maintainer. A CVE advisory acknowledgment is issued upon merge.

---

## 2. New Features

Feature work ranges from small additive changes to large, multi-sprint initiatives. Large features are typically decomposed into multiple Wave Issues across consecutive sprints.

### 2.1 Small Additive Feature (20–50 points)
- **Description**: A self-contained enhancement that adds a new capability without altering existing public APIs or data models. The feature is scoped such that a single developer can complete it within a single sprint.
- **Examples**: Adding a new sorting option to an API endpoint, exposing a previously internal metric via a Prometheus endpoint, adding keyboard shortcut support to a UI component, implementing a new output format for an existing command.
- **Scoping**: Maintainers produce an API sketch (function signature, config keys, or UI mock) before the sprint starts. The contributor implements against this sketch.
- **Deliverables**: PR with implementation + unit tests + integration test (if applicable) + updated API documentation.
- **Point modifiers**: +10 if the contributor also updates the relevant OpenAPI/Swagger spec or TypeScript type definitions.

### 2.2 Medium Feature (50–100 points)
- **Description**: A feature that touches 2–3 modules and may require a small data model migration, a new configuration option, or a new page/view. The feature is fully defined but requires thoughtful integration work.
- **Examples**: Adding a webhook system to notify external services on events, implementing a bulk-import CSV endpoint, building a dashboard widget that aggregates data from multiple sources.
- **Scoping**: Maintainers provide a mini-spec (1–2 pages) covering acceptance criteria, data model changes, error handling strategy, and migration plan. The contributor is expected to ask clarifying questions during the first 48 hours of the sprint.
- **Deliverables**: PR with implementation + migration script (if applicable) + full test coverage (> 80% for new code) + integration test + feature flag toggle.
- **Quality gate**: A maintainer reviews the spec understanding with the contributor before coding begins. Feature must be gated behind a feature flag unless it's additive-only.

### 2.3 Large Feature — Decomposed (point allocation per Wave)
- **Description**: A multi-sprint effort broken into discrete, independently mergeable Wave Issues. Each sub-Wave has its own point value, badge, and deliverable.
- **Example decomposition** for "Multi-tenant Organizations" feature:
  - Wave 1 (40 pts): Data model — create `Organization` and `OrganizationMembership` tables + migration
  - Wave 2 (60 pts): API — CRUD endpoints for organizations with auth checks
  - Wave 3 (50 pts): UI — organization settings page with member management
  - Wave 4 (30 pts): Integration — scope existing resources to organizations
  - Wave 5 (20 pts): Docs — write organization admin guide and update API reference
- **Scoping**: A parent epic issue tracks the overall feature. Each sub-Wave is a separate issue with its own scope, point value, and point of contact maintainer.
- **Deliverables per sub-Wave**: Same as Medium Feature above, plus the PR description must link to the parent epic.
- **Bonus**: +15 points to any contributor who completes all sub-Waves of a feature.

### 2.4 Performance Optimization (30–70 points)
- **Description**: A targeted optimization that improves latency, throughput, memory usage, or binary size without changing observable behavior.
- **Examples**: Adding an index that reduces a query from 5s to 50ms, implementing connection pooling for an external service, lazy-loading a heavy JavaScript dependency, reducing cold-start time of a serverless function.
- **Scoping**: The issue includes a benchmark or profile showing the current baseline. The target improvement is stated concretely (e.g., "reduce P95 latency by 50%").
- **Deliverables**: PR with optimization + before/after benchmark results in the PR description + a performance regression test.
- **Quality gate**: A second maintainer independently verifies the benchmark results before merge.

---

## 3. Documentation

Documentation Waves are critical for project health and are often a great starting point for new contributors who want to understand the codebase deeply.

### 3.1 Code-Level Documentation (10–25 points)
- **Description**: Adding or improving doc comments, module-level docs, and inline explanations for complex logic. The focus is on code that is currently undocumented or has stale/incorrect comments.
- **Examples**: Adding rustdoc comments to all public functions in a module, adding inline explanations for a complex algorithm, fixing incorrect doc comments that no longer match the implementation.
- **Scoping**: The maintainer identifies a specific module or file scope. The contributor reads through the code and adds documentation where missing, ensuring accuracy by tracing the actual behavior.
- **Deliverables**: PR with doc changes + verification that `cargo doc` (or equivalent) builds without warnings. No warnings about missing docs after the PR.
- **Point modifiers**: +5 if the contributor also identifies and fixes a minor bug or code smell discovered while reading the code.

### 3.2 User-Facing Documentation (15–40 points)
- **Description**: Writing or improving guides, tutorials, README sections, configuration references, and API documentation that users interact with.
- **Examples**: Writing a "Getting Started" tutorial for a new SDK, updating the deployment guide to reflect a new infrastructure setup, creating a FAQ page from common support questions, writing a migration guide for a breaking release.
- **Scoping**: The issue specifies the target audience (new users, operators, integrators) and the format (Markdown, hosted docs site, API reference). An outline or bullet list of required sections is provided.
- **Deliverables**: PR with documentation files + a review by a technical writer or maintainer for accuracy and clarity.
- **Quality gate**: All code examples in the docs must be tested (either by CI running the examples or by a maintainer verifying manually).

### 3.3 Translations & Localization (10–30 points)
- **Description**: Translating project documentation, error messages, or UI strings into a specific language.
- **Examples**: Translating the README and contributing guide to Spanish, Japanese, or Mandarin; localizing CLI help text and error messages; translating in-app UI strings via i18n framework.
- **Scoping**: The issue specifies the target language and the scope (e.g., "README + CONTRIBUTING only" or "all user-facing CLI strings"). The contributor must be a fluent speaker of both English and the target language.
- **Deliverables**: PR with translation files + a native-speaker review. For i18n, all strings must pass the project's i18n lint rules.
- **Point modifiers**: +10 if the contributor also maintains the translation for 3 consecutive releases.

### 3.4 API Reference Generation (20–40 points)
- **Description**: Generating, formatting, or improving auto-generated API reference documentation. This includes fixing incorrect type signatures, adding missing parameter descriptions, and ensuring examples compile.
- **Examples**: Auditing and fixing all OpenAPI/Swagger endpoint descriptions, ensuring every public TypeScript export has a valid JSDoc comment, adding runnable code examples to every endpoint in the API reference.
- **Scoping**: The maintainer runs the generator and provides a diff of issues found. The contributor fixes each issue systematically.
- **Deliverables**: PR with fixes + a CI check that enforces that newly added public APIs include documentation.

---

## 4. Testing

Testing Waves improve the project's reliability and are excellent for contributors who want to build deep familiarity with the codebase.

### 4.1 Unit Test Coverage (15–40 points)
- **Description**: Adding unit tests for untested or under-tested functions, modules, or edge cases. The contributor identifies gaps by running code coverage tools.
- **Examples**: Adding tests for error paths in a module that only tests the happy path, testing edge cases for input validation functions, adding property-based tests for parsing/serialization functions, achieving 100% branch coverage for a specific module.
- **Scoping**: The issue targets a specific module or set of files, with a current coverage metric and a target metric (e.g., "raise line coverage in `src/parser/` from 45% to 85%").
- **Deliverables**: PR with tests + a coverage report showing the improvement.
- **Quality gate**: All new tests must pass. The target coverage threshold must be met.

### 4.2 Integration Test Suite (25–60 points)
- **Description**: Writing end-to-end or integration tests that exercise real system interactions (database, network, file system). These tests catch issues that unit tests miss.
- **Examples**: Writing a test that spins up a test database and runs through a complete CRUD flow, writing a test that starts a local HTTP server and sends real requests, writing a test that verifies correct behavior across process restarts.
- **Scoping**: The issue identifies a specific integration scenario or user flow that lacks coverage. The maintainer provides the test infrastructure setup (test containers, fixtures, etc.).
- **Deliverables**: PR with integration tests + documentation on how to run them locally.
- **Point modifiers**: +10 if the contributor also adds a CI job to run the integration tests in CI.

### 4.3 Fuzz & Property-Based Testing (30–70 points)
- **Description**: Writing fuzz tests (using `cargo fuzz`, `jazzer.js`, etc.) or property-based tests (QuickCheck, fast-check) that explore edge cases beyond what manual test cases can cover.
- **Examples**: Writing a fuzz harness that feeds random bytes to a parser and checks for panics/crashes, writing property-based tests that verify "serialize(deserialize(x)) == x" for all inputs within a domain.
- **Scoping**: The issue specifies the target function/module and the testing framework to use. A CI fuzzing harness may need to be configured.
- **Deliverables**: PR with fuzz harness + seed corpus + CI job configuration + documentation.
- **Quality gate**: The fuzz test must run for at least 10 minutes without finding a crash. If a crash is found, the contributor gets bonus points for fixing it.

### 4.4 Test Infrastructure & Tooling (20–50 points)
- **Description**: Improving the test experience — faster test runs, better test isolation, improved fixtures, test flakiness reduction.
- **Examples**: Parallelizing a test suite, replacing a slow integration test dependency with a mock, fixing 5 flaky tests identified by the CI dashboard, adding test retry logic for network-dependent tests.
- **Scoping**: The maintainer identifies the specific pain point and provides metrics (e.g., "this test suite takes 12 minutes, target is < 3 minutes" or "these 7 tests flake more than 20% of the time").
- **Deliverables**: PR with improvements + before/after metrics.

---

## 5. Refactoring & Technical Debt

These Waves improve internal code quality without changing external behavior. They require a strong understanding of the codebase and good engineering judgment.

### 5.1 Code Cleanup (10–30 points)
- **Description**: Removing dead code, deprecated APIs, unused dependencies, or addressing compiler/linter warnings.
- **Examples**: Removing a deprecated function that has no callers, deleting an unused feature flag, upgrading a dependency to remove a cargo-deny advisory, fixing all clippy warnings in a module.
- **Scoping**: The maintainer provides a `ripgrep` query or tool output that identifies the issues. The contributor executes the cleanup and verifies no regressions.
- **Deliverables**: PR with removals/cleanup + verification that CI passes with zero new warnings.
- **Quality gate**: Must not change any public API surface or behavior.

### 5.2 Module Refactor (30–80 points)
- **Description**: Restructuring a module or set of modules to improve cohesion, reduce coupling, or align with a cleaner architectural pattern.
- **Examples**: Extracting a god class into smaller focused classes, splitting a monolith module into a `core/` + `extensions/` pattern, migrating from callbacks to async/await, introducing a repository pattern to abstract database access.
- **Scoping**: The maintainer provides a current architecture diagram, a desired architecture diagram, and a migration path. The change must be done incrementally (no giant refactor PRs).
- **Deliverables**: PR(s) with refactored code + updated module-level documentation + migration guide for downstream consumers if the public API changes.
- **Quality gate**: All existing tests must pass without modification. If public APIs are changed, they must be deprecated first with a migration window.

### 5.3 Dependency Upgrade & Deprecation Management (15–45 points)
- **Description**: Upgrading a project dependency to a new major/minor version, handling breaking changes in the upgrade, and removing or replacing deprecated or unmaintained dependencies.
- **Examples**: Upgrading from React 17 to React 18 with all associated changes, migrating from `webpack` to `esbuild` or `turbopack`, replacing a deprecated logging library with the project's standard one, removing a transitive dependency by inlining a small utility.
- **Scoping**: The maintainer identifies the target dependency and the desired version. A list of known breaking changes and affected project files is provided.
- **Deliverables**: PR with dependency changes + migration of affected code + updated lockfile + verification that the full test suite passes.
- **Point modifiers**: +10 if the contributor also adds a Dependabot/Renovate configuration to automate future minor updates.

---

## 6. DevOps & Infrastructure

Infrastructure Waves improve the development workflow, CI/CD pipeline, and operational tooling.

### 6.1 CI/CD Pipeline Improvement (20–60 points)
- **Description**: Enhancing the CI/CD pipeline — faster builds, better caching, additional checks, or new automation.
- **Examples**: Adding a workflow to automatically label PRs based on changed files, implementing build caching to reduce CI time by 40%, adding a workflow to deploy preview environments for PRs, integrating a code coverage service like Codecov or Coveralls.
- **Scoping**: The maintainer describes the desired outcome and constraints (budget, tooling preferences). The contributor proposes a design before implementing.
- **Deliverables**: PR with CI configuration changes + documentation of the new workflow + migration of existing PRs (if needed) + verification that the new checks pass.

### 6.2 Monitoring & Observability (25–60 points)
- **Description**: Adding or improving logging, metrics, tracing, and alerting for the project's production services.
- **Examples**: Adding structured logging with correlation IDs to all API handlers, instrumenting key code paths with OpenTelemetry spans, creating a Grafana dashboard for the service's key SLOs, adding health check endpoints.
- **Scoping**: The issue specifies the target SLO or observability gap. The maintainer provides access to the observability stack (Grafana, Datadog, etc.) if needed.
- **Deliverables**: PR with instrumentation + dashboard JSON (if applicable) + runbook entry for any new alerts.

### 6.3 Containerization & Deployment (30–70 points)
- **Description**: Improving Dockerfiles, Kubernetes manifests, Helm charts, or deployment scripts.
- **Examples**: Multi-stage Dockerfile optimization to reduce image size by 60%, writing a Helm chart for the service, adding liveness/readiness probes to existing Kubernetes manifests, writing a Terraform module for the project's infrastructure.
- **Scoping**: The maintainer provides the current deployment artifacts and the desired improvements. The contributor tests the changes in a staging environment.
- **Deliverables**: PR with deployment changes + verification that a staging deployment succeeds + rollback instructions.

---

## 7. Review & Quality Assurance

These Waves focus on the human side of quality — code review, design review, and audit.

### 7.1 Focused Code Review Wave (15–30 points)
- **Description**: A dedicated review pass over a specific area of the codebase, looking for correctness, security, performance, and style issues. Unlike ordinary PR review, this is a systematic audit.
- **Examples**: Reviewing all uses of unsafe Rust in the codebase, auditing every place where user input flows into an SQL query, reviewing all error handling paths in the payment module.
- **Scoping**: The maintainer defines the scope (files, patterns, or concern). The contributor produces a review document with findings.
- **Deliverables**: A GitHub issue or PR comment thread with findings, severity ratings, and suggested fixes. The contributor may optionally fix the issues (earning additional points).
- **Point modifiers**: +5 per actionable fix the contributor implements.

### 7.2 Design Review & RFC Response (20–40 points)
- **Description**: Participating in the project's RFC or design review process by providing a structured, well-researched response to a proposed design document.
- **Examples**: Analyzing the performance implications of a proposed new data structure, researching how other projects solved the same problem and summarizing findings, stress-testing a proposed API by writing example usage code.
- **Scoping**: The maintainer links the RFC or design document and specifies the angle of analysis needed. The contributor writes their analysis as a comment or a companion document.
- **Deliverables**: A structured review comment or companion document that the maintainer can reference during the design decision.

---

## 8. Community & Ecosystem

These Waves strengthen the project's community and ecosystem reach.

### 8.1 Issue Triage & Management (10–25 points per sprint)
- **Description**: A contributor is assigned as the sprint's issue triager — they review new issues, apply labels, request missing information, close duplicates, and route bugs to appropriate maintainers.
- **Examples**: Processing the issue backlog, applying `needs-reproduction` / `good-first-issue` / `help-wanted` labels, following up on stale issues that lack maintainer response.
- **Scoping**: The maintainer provides a dashboard. The contributor processes all issues filed during the sprint and reduces the unlabeled queue to zero.
- **Deliverables**: A triage report at sprint end summarizing actions taken. At least 80% of new issues must be triaged within 24 hours.

### 8.2 Maintainer Mentorship (30–50 points)
- **Description**: An experienced contributor serves as a dedicated mentor for one or more new contributors during a sprint. They provide onboarding, code review, and guidance.
- **Examples**: Pairing with a first-time contributor on their first PR, providing daily async check-ins and code review feedback, recording a walkthrough video of the contribution process.
- **Scoping**: The mentor is paired with 1–3 specific newcomers. The maintainers track the newcomers' success rate and time-to-first-merge.
- **Deliverables**: At least one newcomer's PR merged during the sprint + a retrospective comment on what worked well and what could be improved.

### 8.3 Integration & Plugin Development (30–80 points)
- **Description**: Building or improving an integration, plugin, or extension that connects the project with other tools or platforms.
- **Examples**: Writing a GitHub Action that uses the project's API, creating a VS Code extension that provides project-specific language support, building a Slack bot that notifies a channel of important project events, developing a Terraform provider for the project.
- **Scoping**: The maintainer and contributor agree on the integration scope, target platform, and delivery timeline. The integration must be documented and tested.
- **Deliverables**: The integration code + documentation + usage examples + tests.

---

## Point Valuation Rubric

Each Wave Issue's point value is determined by the following factors:

| Factor | Weight | Description |
|--------|--------|-------------|
| Time estimate | Base | 1 point ≈ 1 hour of focused work |
| Complexity | ×1–3 | Trivial (1), Moderate (2), Complex (3) |
| Risk | ×1–2 | Low (1), Medium (1.5), High (2) — risk of breaking things |
| Knowledge required | +0–20 | Familiarity with specific subsystem or language |
| Collaboration overhead | +0–10 | Coordination needed with other contributors/stakeholders |

**Example**: A complex bug fix estimated at 8 hours, with high complexity (×2), low risk (×1), moderate knowledge required (+5), and no collaboration overhead:
`8 × 2 × 1 + 5 = 21 points` (rounded to nearest 5 → **20 points**).

---

## Sprint Cadence & Badge Naming

Waves are tagged with a standardized badge format:
```
<EcosystemName>-Wave-<SprintNumber>
```

For example: `Drips-Core-Wave-12`, `Stellar-Wave-4`, `Arbitrum-Wave-7`.

Each Wave badge is a soulbound "Proof of Contribution" that the contributor earns permanently. The WaveRegistry tracks:

- **Total points earned** — cumulative across all ecosystems
- **Sprints completed** — total number of distinct sprints contributed to
- **Badge list** — every Wave the contributor has completed, providing a tamper-proof work history

---

## Summary

The Wave Program supports 8 major work categories with over 20 specific work types, ensuring that contributors of all skill levels and interests can find meaningful, scoped work. The point system is transparent and formula-driven, and the on-chain WaveRegistry ensures every contribution is verifiable and permanent. By structuring work this way, projects can onboard contributors efficiently, maintain high quality standards, and build a trusted, portable reputation layer for open-source development.
