# Module: Snapshot Testing (`crates/rupa-test/src/snapshot.rs`) 📸

This module allows developers to assert that the UI structure hasn't changed unexpectedly.

---

## 🏗️ Logic
- **Comparison**: Diffs the current VNode JSON against a stored baseline.
- **Regression Detection**: Flags stylistic or structural changes as test failures.
- **Auto-Update**: Provides an option to update snapshots if the change was intentional.
