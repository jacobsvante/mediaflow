# Changelog

## [0.9.0](https://github.com/jacobsvante/mediaflow/compare/v0.8.0...v0.9.0) (2022-09-26)


### ⚠ BREAKING CHANGES

* Original width/height format wasn't supported

### Bug Fixes

* Original width/height format wasn't supported ([ea2fca9](https://github.com/jacobsvante/mediaflow/commit/ea2fca989e31ad782f82633f2ed989e467c8bd23))

## [0.8.0](https://github.com/jacobsvante/mediaflow/compare/v0.7.0...v0.8.0) (2022-09-26)


### ⚠ BREAKING CHANGES

* Getting file downloads for a folder didn't allow non-recursive mode

### Features

* Retry 429/500 errors for file download requests ([fa69547](https://github.com/jacobsvante/mediaflow/commit/fa69547966a6eb21c27c98fba186174c29139364))


### Bug Fixes

* Getting file downloads for a folder didn't allow non-recursive mode ([93d25e8](https://github.com/jacobsvante/mediaflow/commit/93d25e815b963e49c137bba8710fa9d5f761418a))
* MAX_CONCURRENT_DOWNLOADS env var didn't apply ([299cf39](https://github.com/jacobsvante/mediaflow/commit/299cf3901dd08e4db87a9e9850942cb79736f683))

## [0.7.0](https://github.com/jacobsvante/mediaflow/compare/v0.6.2...v0.7.0) (2022-07-20)


### ⚠ BREAKING CHANGES

* `get_formats` didn't require a specific mediaflow type
* Require download type to derive trait MediaflowFileDownload

### Bug Fixes

* `get_formats` didn't require a specific mediaflow type ([43977fc](https://github.com/jacobsvante/mediaflow/commit/43977fc0269223652a668aca96b23c8c9d68e7a1))
* Rename `entities` to `tokens` ([2c5ac5d](https://github.com/jacobsvante/mediaflow/commit/2c5ac5d6dd0cd1e552644170cb47eae6952715e4))
* Require download type to derive trait MediaflowFileDownload ([2caecd3](https://github.com/jacobsvante/mediaflow/commit/2caecd386603f3fd514d7bf8c66ab0244a8c220a))

## [0.6.2](https://github.com/jacobsvante/mediaflow/compare/v0.6.1...v0.6.2) (2022-07-20)


### Bug Fixes

* Actually make derive macros available outside of crate (try 2) ([22b676a](https://github.com/jacobsvante/mediaflow/commit/22b676ab7333f5feb5914888ba96370c0751c29a))

## [0.6.1](https://github.com/jacobsvante/mediaflow/compare/v0.6.0...v0.6.1) (2022-07-19)


### Bug Fixes

* Actually make derive macros available outside of crate ([02a5a17](https://github.com/jacobsvante/mediaflow/commit/02a5a1766c24f85f2769e69fc19d8f61c1ef33eb))

## [0.6.0](https://github.com/jacobsvante/mediaflow/compare/v0.5.0...v0.6.0) (2022-07-08)


### ⚠ BREAKING CHANGES

* Tie file id to each download

### Features

* Tie file id to each download ([4321ba8](https://github.com/jacobsvante/mediaflow/commit/4321ba83882c4426884339061861eb95475ad012))

## [0.5.0](https://github.com/jacobsvante/mediaflow/compare/v0.4.1...v0.5.0) (2022-07-08)


### Features

* Add FileDownloadUrl type ([c57992c](https://github.com/jacobsvante/mediaflow/commit/c57992c24f127445c7d202fd99272a3a44fbc838))


### Bug Fixes

* Make FileDownloadFull fields public ([d7d1044](https://github.com/jacobsvante/mediaflow/commit/d7d10440fee0c5c7c8f1fa7ab7a259ccdd323420))

## [0.4.1](https://github.com/jacobsvante/mediaflow/compare/v0.4.0...v0.4.1) (2022-07-08)


### Bug Fixes

* Lower default max concurrent downloads ([9d65025](https://github.com/jacobsvante/mediaflow/commit/9d65025a5e2c564c1a4e76203f5a7633f29189ce))

## [0.4.0](https://github.com/jacobsvante/mediaflow/compare/v0.3.0...v0.4.0) (2022-07-08)


### ⚠ BREAKING CHANGES

* env_logger, configparser and dirs are now only installed with `cli` feature

### Features

* Ability to download files ([1aa664b](https://github.com/jacobsvante/mediaflow/commit/1aa664b2c5224f3282216a4adf938f50b7ffc101))


### Bug Fixes

* env_logger, configparser and dirs are now only installed with `cli` feature ([9b6c9ec](https://github.com/jacobsvante/mediaflow/commit/9b6c9ec1fa026a94cb7487d167135f5e17618dc2))
* Return error code 1 when CLI is not installed ([c8571d6](https://github.com/jacobsvante/mediaflow/commit/c8571d6f1086db983c09a9ea0044d35b1dc474d7))

## [0.3.0](https://github.com/jacobsvante/mediaflow/compare/v0.2.0...v0.3.0) (2022-05-15)


### ⚠ BREAKING CHANGES

* Remove need for declaring RestAPI as mutable

### Features

* Remove need for declaring RestAPI as mutable ([2a1948b](https://github.com/jacobsvante/mediaflow/commit/2a1948b875c0b2ab5e5dd6a968e5fbb1ce9a333c))

## [0.2.0](https://github.com/jacobsvante/mediaflow/compare/v0.1.1...v0.2.0) (2022-05-02)


### Features

* Expose RestApi and Config in crate root ([6328fbb](https://github.com/jacobsvante/mediaflow/commit/6328fbb4f423ec14ca4187a4863b7dcc46277fab))
* Make fields public on folder and file entities ([15c3587](https://github.com/jacobsvante/mediaflow/commit/15c3587aa8ae09ba650b685d21c4618e3e79ecf1))

### [0.1.1](https://github.com/jacobsvante/mediaflow/compare/v0.1.0...v0.1.1) (2022-05-02)


### Bug Fixes

* add version to mediaflow_derive ([f32206f](https://github.com/jacobsvante/mediaflow/commit/f32206fed9a69ce41441ade7416bf001a2363767))

## 0.1.0 (2022-04-29)


### Bug Fixes

* Release 0.1 ([3dc3bd2](https://github.com/jacobsvante/mediaflow/commit/3dc3bd2cd8dd45be5a191541ac89d86c72d6fb9c))
