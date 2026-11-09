# Changelog

## [0.2.0] - 2026-11-09

WebHID support for wrapped sites on Linux and Windows, working YouTube pre-roll blocking on directly opened videos, and a complete ad scriptlet library.

### Added

- `navigator.hid` for wrapped sites, backed by the system HID stack. Covers `requestDevice`, `getDevices`, `open`, `close`, `forget`, `sendReport`, `sendFeatureReport`, `receiveFeatureReport`, `inputreport`, `connect` and `disconnect`, with parsed `collections`. Firmware updaters that use WebHID work.
- Native device chooser, shown on every `requestDevice()` call. A site can only use the devices picked for that window, until it closes.
- Security keys and keyboard-, keypad-, mouse- or pointer-only devices are never offered.
- Chooser strings in all 12 languages.
- README section on WebHID, including the udev rule Linux usually needs.

### Fixed

- YouTube pre-rolls on a video opened directly from a link, bookmark or the start page. The inline player response now has its ad keys blanked instead of deleted, which is what left the player dead before.
- Scriptlet helpers `arglist-parser`, `jsonpath`, `range-parser` and `urlskip` were missing, so every scriptlet that needed them was dropped. They are fetched with the library now.
- The scriptlet extractor cut helpers short at a `{}` default parameter, lost `async` and `class` helpers, and was thrown off by regex literals containing quotes. All 153 parsed resources are now valid JavaScript.

### Changed

- `checkJs` is back on for the Node scripts.
- Linux packages depend on `libudev`.
- Removed `TODO.md`.

## [0.1.0]

First version.
