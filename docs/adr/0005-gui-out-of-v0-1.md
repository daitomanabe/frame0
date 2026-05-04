# ADR 0005: GUI Is Out Of v0.1

Status: accepted

FRAME0 v0.1 is CLI-first. GUI editor, node editor, marketplace, and Processing-style creative API are deferred.

Rationale:

- AI and developers need identical CLI commands and JSON inspection.
- Runtime, device, graph, and plugin contracts must harden before visual authoring layers are added.
- GUI-only debug paths would violate the inspectable runtime requirement.

