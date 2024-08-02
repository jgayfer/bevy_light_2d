# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Add `LightOccluder2d` and `LightOccluder2dBundle` for basic light occlusion (hard shadows).

## [0.2.2] - 2024-07-18

### Fixed

- Point lights not despawning (#19).

## [0.2.1] - 2024-07-19

### Fixed

- Ambient light not working when there are no point lights (#17).

## [0.2.0] - 2024-07-04

### Added

- WebGL2 support (#7).

### Changed

- Updated Bevy version from `0.13` to `0.14` (#9).
- Updated `PointLight2d#color` to use the new [`bevy::color`](https://bevyengine.org/learn/migration-guides/0-13-to-0-14/#overhaul-color) module (#9).
- Moved `bevy_sprite`, `png`, and `x11` Bevy features to `dev-dependencies` (#12).

### Fixed

- Crash when HDR was enabled (#11).

## [0.1.3] - 2024-06-02

### Fixed

- Point light position not respecting camera transform (#4).

## [0.1.0] - 2024-05-26

Initial release.
