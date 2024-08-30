# Changelog

## 0.22.0 - 2024-08-30

### Changed

- The V3 sender now accepts an optional reqwest `Client` as a parameter in `new`.

## 0.21.0 - 2024-03-21

### Changed

- `reqwest` was updated to version 0.12.

## 0.20.1 - 2024-02-27

### Fixed

- The main documentation was updated to reflect the current feature flags.

## 0.20.0 - 2024-01-16

### Added

- `blocking_send` function. This allows for asynchronous and blocking code to coexist. (#105)

### Removed

- The `async` feature is now considered default. (#105)
