# Eclipse Heart Changelog

> **Document Location:** `.project/changelog.md`
>
> All notable changes to this project will be documented in this file.
> Format based on [Keep a Changelog](https://keepachangelog.com/).

---

## [Unreleased]

### Added
- `IMPLEMENTATION_PLAN.md` with a Rust architecture and delivery plan based on `raw_notes.md`
- Eclipse Heart-specific `.project/prd.md`
- Eclipse Heart-specific `.project/tech-stack.md`
- Eclipse Heart-specific `.project/build-plan.md`
- Initial `eclipse_heart` Rust crate scaffold with separated `data`, `engine`, `state`, `screens`, and `ui` modules
- Initial deterministic rules-engine shell for Encounter resolution, growth, stage upgrades, and Final Climax defeat checks
- Minimal Macroquad prototype shell wired to the separated rules/state layers
- JSON-backed rules and UI text assets
- Daily Life action flow with pass-to-Encounter transition
- JSON-loaded prototype set for 20 story cards, 5 Magical Girls, and 5 Baddies
- Basic offline AI that uses the same match actions as the player-side rules shell

### Changed
- Replaced generic project templates with game-specific planning documents
- Updated planning docs to align with the `Radiance / Dread` rules model
- Reworked the prototype runtime from hardcoded placeholder units toward data-driven loaded content

### Fixed
- Removed placeholder planning content that did not match the actual game design

### Removed
- [Removed features]

---

## [0.3.0] - YYYY-MM-DD

### Added
- Feature Module A with core business logic
- Input validation for all user inputs
- Service layer architecture

### Changed
- Refactored data models for better type safety

### Fixed
- Build warnings in configuration module

---

## [0.2.0] - YYYY-MM-DD

### Added
- Project scaffolding and directory structure
- Build tooling configuration
- Linting and formatting setup
- Base configuration files
- Environment variable handling

### Changed
- Updated dependencies to latest stable versions

---

## [0.1.0] - YYYY-MM-DD

### Added
- Initial project setup
- `.project/` documentation structure
- Product Requirements Document (prd.md)
- Tech Stack documentation (tech-stack.md)
- Build Plan with task tracking (build-plan.md)
- This changelog

---

## Version Guidelines

### Version Format: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes or significant milestones
- **MINOR**: New features, completed phases
- **PATCH**: Bug fixes, small improvements

### Change Types

| Type | Description |
|------|-------------|
| **Added** | New features or capabilities |
| **Changed** | Changes to existing functionality |
| **Deprecated** | Features marked for removal |
| **Removed** | Features that were removed |
| **Fixed** | Bug fixes |
| **Security** | Security-related changes |

---

## Milestones

| Version | Milestone | Date |
|---------|-----------|------|
| 1.0.0 | Production Release | TBD |
| 0.5.0 | Feature Complete | TBD |
| 0.3.0 | Core Features | YYYY-MM-DD |
| 0.1.0 | Project Setup | YYYY-MM-DD |

---

*Last updated: YYYY-MM-DD*
