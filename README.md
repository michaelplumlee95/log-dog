# Log Dog

Incident Correlations for Operational Logs

## Overview

Log Dog is a command-line tool that ingests operational logs, normalizes events, and automatically groups related events into incidents. Its goal is to reduce large volumes of noisy logs into concise, human-readable summaries that support faster triage and engineering decision making.

This project is inspired by real world operational environments (data centers, power systems, industrial infrastructure), where engineers must identify meaningful patterns across many routine readings and alarms.

![CI](https://github.com/michaelplumlee95/log-dog/actions/workflows/ci.yml/badge.svg)

## Problem Statement

Operational systems generate large numbers of logs:

- routine status readings
- warnings
- error events
- alarms across multiple subsystems

Individually, these log entries are often low-signal.
Taken together, they may represent a single underlying incident.

Manually reviewing logs to determine:

- when an incident started
- which systems were involved
- how severe it was

is time-consuming and error-prone.

*Log Dog automates the first-pass analysis* by clustering related events into incidents and producing structured summaries suitable for engineers or operators.

## What This Tool Does

- Ingests semi-structured log files (JSON)
- Normalizes raw log entries into a consistent internal event model
- Groups events into incidents using a configurable time-window correlation strategy
- Produces concise incident summaries including:
  - start and end time
  - affected systems
  - error and warning counts
  - involved hosts/components
- Exports results in machine-readable and human-readable formats

## What This Tool Does Not Do

- Perform OCR or digitization of handwritten logs
- Use machine learning or anomaly detection
- Guarantee perfect incident detection
- Replace human judgment.

Log Dog is intended to be a *decision-support* tool, not an autonomous monitoring system.

## Log Model

Each input log entry is normalized into an internal `Event` representation:

- timestamp
- system or service
- severity level (INFO / WARN / ERROR)
- message or event description
- host or component identifier
- error or alarm code
- raw payload (preserved for traceability)

This normalization allows logs from different subsystems to be analyzed uniformly.

## Incident Definition

An *incident* is defined as a sequence of events where:

- events occur within a configurable time window of each other
- events are temporally adjacent enough to plausibly share a root cause

This simple correlation strategy reflects how operators often reason about cascading or related failures during initial triage.

## Example Use Case

Instead of reviewing hundreds of individual log lines:

```code
10:01 WARN HVAC temp high
10:02 ERROR HVAC compressor trip
10:03 ERROR POWER breaker alert
10:06 WARN HVAC airflow reduced
```

Log Dog produces a summary such as:

```codeIncident #3
Time: 10:01–10:06
Systems involved: HVAC, Power
Errors: COMP_TRIP, BRKR_ALERT
Hosts affected: dc01, dc02
```

## Project Status

This project is under active development.

Current focus:

- Core parsing and normalization
- Incident correlation logic
- CLI usability
- Clear documentation and test coverage

Future enhancements may include:

- Multiple input formats
- Severity scoring
- Persistent storage
- Visualization support

## Why this project Exists

This project was built to demonstrate:

- backend and systems-oriented problem solving
- handling of messy, real-world operational data
- translating domain knowledge into software logic
- designing internal tools that support engineers, not replace them.

The logs used in this project are *synthetic*, but intentionally modeled after real operational environments to reflect realistic constraints and workflows

## License

MIT License
