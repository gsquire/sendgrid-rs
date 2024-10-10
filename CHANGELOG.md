# Changelog

## 0.23.0 - 2024-10-10

### Added

- The message field now supports `mail_settings`. See #118.

### Changed

- Update to data-encoding 2.6.

## 0.22.1 - 2024-09-04

### Added

- `Personalization::new_many` was added to simplify the API.

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
