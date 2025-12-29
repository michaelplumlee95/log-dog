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
