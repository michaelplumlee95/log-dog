# Log Dog 🐕

**A CLI tool for correlating operational logs into incidents**

![CI](https://github.com/michaelplumlee95/log-dog/actions/workflows/ci.yml/badge.svg)

## Overview

Log Dog is a command-line tool that ingests operational log data and groups related events into **incidents** using a configurable time-window correlation strategy.

It is designed to reduce noisy, high-volume logs into concise summaries that support **initial incident triage** in infrastructure and backend environments.

The project is inspired by real-world operational workflows in data centers and industrial systems, where engineers reason about incidents by correlating warnings and errors across multiple subsystems over time.

---

## Problem

Operational systems generate large numbers of logs:

- routine status messages (`INFO`)
- warnings (`WARN`)
- errors and alarms (`ERROR`)

Individually, these entries are often low signal.  
Taken together, they may represent a single underlying incident.

Manually reviewing logs to determine:

- when an incident started
- which systems were involved
- how severe it was

is time-consuming and error-prone.

**Log Dog automates this first-pass correlation.**

---

## What Log Dog Does

- Ingests newline-delimited JSON (`.jsonl`) log files
- Normalizes entries into a consistent internal event model
- Filters noise (`INFO`) and correlates `WARN` / `ERROR` events
- Groups temporally related events into incidents
- Produces concise, human-readable summaries
- Supports deterministic log generation for testing

---

## What Log Dog Does *Not* Do

- Perform machine learning or anomaly detection
- Guarantee perfect incident detection
- Replace human judgment or monitoring systems

Log Dog is a **decision-support tool**, not an autonomous monitoring platform.

---

## Example

### **Input logs (excerpt)**

```text
{"ts":"2025-12-29T18:44:22.693610Z","system":"power","level":"WARN","msg":"Voltage dip detected","host":"dc03","code":"VOLT_DIP"}
{"ts":"2025-12-29T18:44:49.693610Z","system":"hvac","level":"WARN","msg":"Supply air temperature high","host":"dc01","code":"TEMP_HIGH"}
{"ts":"2025-12-29T18:45:00.693610Z","system":"cooling","level":"INFO","msg":"No active alarms","host":"dc03","code":"CLEAR"}
{"ts":"2025-12-29T18:45:04.693610Z","system":"hvac","level":"INFO","msg":"Routine status check","host":"dc01","code":"STATUS_OK"}
{"ts":"2025-12-29T18:45:39.693610Z","system":"hvac","level":"ERROR","msg":"Compressor trip detected","host":"dc03","code":"COMP_TRIP"}
{"ts":"2025-12-29T18:46:15.693610Z","system":"power","level":"ERROR","msg":"UPS on battery","host":"dc01","code":"UPS_BATT"}
{"ts":"2025-12-29T18:46:21.693610Z","system":"power","level":"ERROR","msg":"Critical load shed","host":"dc03","code":"LOAD_SHED"}
{"ts":"2025-12-29T18:46:28.693610Z","system":"hvac","level":"ERROR","msg":"Compressor trip detected","host":"dc01","code":"COMP_TRIP"}
{"ts":"2025-12-29T18:46:37.693610Z","system":"hvac","level":"INFO","msg":"Minor fluctuation detected","host":"dc02","code":"FLUX"}
{"ts":"2025-12-29T18:46:45.693610Z","system":"hvac","level":"INFO","msg":"No active alarms","host":"dc01","code":"CLEAR"}
{"ts":"2025-12-29T18:46:46.693610Z","system":"power","level":"ERROR","msg":"Critical load shed","host":"dc03","code":"LOAD_SHED"}
{"ts":"2025-12-29T18:47:06.693610Z","system":"power","level":"INFO","msg":"No active alarms","host":"dc03","code":"CLEAR"}
```

### **Log Dog output**

```text
Incidents found: 4
Incident #1 2025-12-29T18:44:22.693610+00:00 → 2025-12-29T18:46:46.693610+00:00 (2m24s)
   events: 7  |   ERROR: 5   WARN: 2    INFO 0
   systems: hvac, power
   hosts: dc01, dc03
  sample messages:
      - WARN power: Voltage dip detected
      - WARN hvac: Supply air temperature high
      - ERROR hvac: Compressor trip detected
  top codes:
      - COMP_TRIP (2)
      - LOAD_SHED (2)
      - TEMP_HIGH (1)
      - UPS_BATT (1)
      - VOLT_DIP (1)
```

---

## Incident Model

An **incident** is defined as a sequence of non-INFO events where:

- consecutive events occur within a configurable time window (default: 5 minutes)
- events are temporally close enough to plausibly share a root cause

This mirrors how operators reason during early incident triage.

---

## Usage

### Generate synthetic logs

    log-dog gen --seed 42 --incident-count 3 --out sample.jsonl

### Correlate incidents

    log-dog incidents --input sample.jsonl --window 300

---

## Design Highlights

- **Deterministic log generation** via seeded RNG
- **RFC3339 timestamps** for reliable ordering and parsing
- **Separation of concerns** (parsing, correlation, reporting)
- **Unit + integration tests**
- **CI-enforced formatting, linting, and tests**

---

## Project Goals

This project exists to demonstrate:

- backend and systems-oriented problem solving
- translating operational domain knowledge into code
- building reliable internal tools
- testable, maintainable Rust CLI design

All logs are **synthetic**, but modeled after real operational environments.

---

## License

MIT License
