# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added `Light2d` "marker" component to indicate which cameras should process
  lighting effects (#49).
- Added "minimap" example to showcase camera with lighting disabled (#49).
- Add reflection for Component and Default on `PointLight2D` and `AmbientLight` (#50).

### Changed

- Lighting is no longer processed by default; the `Light2d` marker component
  must be added to the `Camera2d` (#49).

### Fixed

- Fixed occluders not despawning correctly (#47).
- Improved WebGL2 performance (#47).

### Migration guide

- Add a `Light2d` marker component to cameras that should process lighting
  effects.
- Remove `AmbientLight2d` components from cameras. Instead, include them as a
  field on the camera's `Light2d` marker component.

## [0.6.0] - 2025-04-29

### Changed

- Updated Bevy version from `0.15` to `0.16` (#45).

## [0.5.0] - 2024-12-04

### Added

- Added required component support for `PointLight2d` (#39).
- Added required component support for `LightOccluder2d` (#39).

### Changed

- Updated Bevy version from `0.14` to `0.15` (#39).
- Deprecated `PointLight2dBundle` in favour of `PointLight2d` (#39).
- Deprecated `LightOccluder2dBundle` in favour of `LightOccluder2d` (#39).

### Migration guide

- Replace all uses of `PointLight2dBundle` with `PointLight2d`.
- Replace all uses of `LightOccluder2dBundle` with `LightOccluder2d`.

## [0.4.2] - 2024-10-25

### Fixed

- Lighting occasionally not rendering and/or affecting elements in unintended order (#37).

## [0.4.1] - 2024-10-22

### Fixed

- Crash on WebGL2 when no occluders are present (#36).

## [0.4.0] - 2024-09-17

### Changed

- Point lights colours are now added to ambient light, instead of multiplied by it (#24).

### Fixed

- Point lights rendering despite being despawned (#25).
- Shadow sometimes appearing when no occluders were present (#27).

### Migration guide

- Point light intensity needs to be adjusted to account for changes to ambient light. Generally this means point light intensity values need to be lowered. See the relevant changes to the `dungeon` example.

## [0.3.0] - 2024-08-05

### Added

- Added `LightOccluder2d` component and `LightOccluder2dBundle` (#20).

### Changed

- Modified `PointLight2d` to include a `cast_shadows` attribute (defaults to false) (#20).

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
