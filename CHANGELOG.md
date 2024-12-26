# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies
- Bump `anyhow` from 1.0.89 to 1.0.95 ([#5](https://github.com/vbrandl/sablier-proxy/pull/5), [#6](https://github.com/vbrandl/sablier-proxy/pull/6), [#11](https://github.com/vbrandl/sablier-proxy/pull/11), [#12](https://github.com/vbrandl/sablier-proxy/pull/12), [#17](https://github.com/vbrandl/sablier-proxy/pull/17), [#21](https://github.com/vbrandl/sablier-proxy/pull/21))
- Bump `tokio` from 1.40.0 to 1.42.0 ([#7](https://github.com/vbrandl/sablier-proxy/pull/7), [#13](https://github.com/vbrandl/sablier-proxy/pull/13), [#18](https://github.com/vbrandl/sablier-proxy/pull/18))
- Bump `serde` from 1.0.210 to 1.0.216 ([#8](https://github.com/vbrandl/sablier-proxy/pull/8), [#9](https://github.com/vbrandl/sablier-proxy/pull/9), [#14](https://github.com/vbrandl/sablier-proxy/pull/14), [#20](https://github.com/vbrandl/sablier-proxy/pull/20))
- Bump `reqwest` from 0.12.8 to 0.12.9 ([#10](https://github.com/vbrandl/sablier-proxy/pull/10))
- Bump `tracing` from 0.1.40 to 0.1.41 ([#15](https://github.com/vbrandl/sablier-proxy/pull/15))
- Bump `tracing-subscriber` from 0.3.18 to 0.3.19 ([#16](https://github.com/vbrandl/sablier-proxy/pull/16))
- Bump `actions/attest-build-provenance` from 1 to 2 ([#19](https://github.com/vbrandl/sablier-proxy/pull/19))

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
