# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies
- Bump `anyhow` from 1.0.89 to 1.0.91 ([#5](https://github.com/vbrandl/sablier-proxy/pull/5), [#6](https://github.com/vbrandl/sablier-proxy/pull/6))
- Bump `tokio` from 1.40.0 to 1.41.0 ([#7](https://github.com/vbrandl/sablier-proxy/pull/7))
- Bump `serde` from 1.0.210 to 1.0.213 ([#8](https://github.com/vbrandl/sablier-proxy/pull/8))

## [0.3.0] 2024-10-11

- Normalize Sablier URL by removing trailing slash
- Log if upstream is unavailable ([#4](https://github.com/vbrandl/sablier-proxy/pull/4))

### Dependencies
- Bump `anyhow` from 1.0.88 to 1.0.89 ([#1](https://github.com/vbrandl/sablier-proxy/pull/1))
- Bump `reqwest` from 0.12.7 to 0.12.8 ([#2](https://github.com/vbrandl/sablier-proxy/pull/2))
- Bump `futures` from 0.3.30 to 0.3.31 ([#3](https://github.com/vbrandl/sablier-proxy/pull/3))

## [0.2.0] 2024-09-13
- Improved logging
- Better tagging for Docker images

## [0.1.0] 2024-09-13
- Initial release
