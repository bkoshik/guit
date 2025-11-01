# CHANGELOG

## [Unreleased]

### Added

- Add `FileStatusKind` enum
- Add `FileStatusEntry` struct
- Add getter methods to `FileStatusEntry`
- Add `FileStatusEntry::new()` method
- Add `TryFrom<git2::Repository>` implementation for `RepositoryInfo`

### Changed

- `RepositoryInfo::new()` takes `Path` as argument

### Fixed

- Fix dates in `CHANGELOG.md`

## [0.1.0-alpha.5] - 2025-10-31

### Added

- Add `AuthorInfo::to_signature()` method
- Add `Display` implementation for `Message`
- Add `RepositoryInfo::add_files()` method
- Add `RepositoryInfo::remove_files()` method
- Add `RepositoryInfo::commit_changes()` method
- Add `CHANGELOG.md`

### Changed

- Separate commit info retrieval logic from `CommitInfo::new()`
- Rename `Message::description` field to `message`
- Rename `lib::utils::log` module to `commands`

### Fixed

- Make `CommitInfo` fields private (were incorrectly `pub`)

### Removed

- Remove `consts.rs`

## [0.1.0-alpha.4] - 2025/10/29

### Added

- Add getter methods to `AuthorInfo`
- Add getter methods to `Branches`
- Add getter methods to `CommitInfo`
- Add getter methods to `Message`
- Add getter methods to `RepositoryInfo`
- Add `From<git2::Config>` implementation for `AuthorInfo`
- Add `Display` implementation for `AuthorInfo`

### Changed

- Change getter methods to return references instead of clones

## [0.1.0-alpha.3] - 2025/10/29

### Added

- Add `RepositoryInfo::log()` method
- Add `consts.rs`
    - Add color consts
    - Add `NO_` message consts
    - Add helper functions
        - Add `pub fn find_branches(...) -> Result<Vec<String>>`
        - Add `Result<T, Box<dyn std::error::Error>`
    - Add `utils::log` functions
        - Add `braches.rs`
        - Add `date.rs`
        - Add `head.rs`
        - Add `tags.rs`

## [0.1.0-alpha] - 2025/10/28

### Added

- Add `AuthorInfo::new()` method
- Add `Branches::new()` method
- Add `Message::new()` method
- Add `RepositoryInfo::new()` method
- Add `AuthorInfo` struct
- Add `Branches` struct
- Add `Message` struct
- Add `RepositoryInfo` struct
