# Log Dog – TODO

This file tracks planned work and next milestones for the Log Dog project.
Items are ordered roughly by priority and learning value.

---

## ✅ Completed

- [x] Initialize Rust project scaffold
- [x] Add README with problem statement and scope
- [x] Define `Event` data model
- [x] Parse JSON Lines (`.jsonl`) logs into structured events
- [x] Implement time-window incident correlation (5-minute window)
- [x] Validate incident grouping using synthetic operational logs

---

## 🔜 Next Steps (Core Functionality)

### Incident Logic Refinement

- [x] Decide when an incident should *start*
  - Options to explore:
    - First `WARN` or `ERROR`
    - Severity threshold
    - Ignore INFO-only clusters
- [x] Merge or suppress singleton INFO-only incidents
- [ ] Add basic incident severity scoring

### Output Improvements

- [x] Print incident summaries instead of raw debug output
- [x] Include:
  - time range
  - systems involved
  - error/warn counts
- [ ] Optional: JSON output format for downstream tools

---

## 🧪 Testing

- [x] Add unit tests for `group_incidents`
  - Events within window cluster correctly
  - Events outside window split into new incidents
- [ ] Add test case for mixed INFO/WARN/ERROR logs
- [ ] Ensure deterministic ordering of incidents

---

## 🧰 CLI & UX

- [ ] Add CLI argument parsing (file path, window size)
- [ ] Allow configurable incident window (seconds)
- [ ] Improve error messages for malformed logs

---

## 🧠 Design / Learning Enhancements

- [ ] Convert `level` from `String` to enum (`INFO | WARN | ERROR`)
- [ ] Document assumptions and limitations in README
- [ ] Explore alternative correlation strategies (time + system)

---

## 🌱 Stretch Goals (Optional)

- [ ] Support multiple input formats
- [ ] Add simple visualization (CSV / Markdown output)
- [ ] Persist incidents to disk
- [ ] Compare time-only vs severity-aware correlation

---

## 📝 Notes

- Logs are intentionally synthetic but modeled after real operational systems
- Focus is on backend logic and decision-support tooling
- Accuracy is less important than clarity, structure, and reasoning
