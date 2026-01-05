# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


### Dependencies
- Bump `reqwest` from 0.12.14 to 0.13.1 ([#43](https://github.com/vbrandl/sablier-proxy/pull/43), [#48](https://github.com/vbrandl/sablier-proxy/pull/48), [#49](https://github.com/vbrandl/sablier-proxy/pull/49), [#50](https://github.com/vbrandl/sablier-proxy/pull/50), [#51](https://github.com/vbrandl/sablier-proxy/pull/51), [#54](https://github.com/vbrandl/sablier-proxy/pull/54), [#61](https://github.com/vbrandl/sablier-proxy/pull/61), [#72](https://github.com/vbrandl/sablier-proxy/pull/72), [#77](https://github.com/vbrandl/sablier-proxy/pull/77), [#79](https://github.com/vbrandl/sablier-proxy/pull/79), [#81](https://github.com/vbrandl/sablier-proxy/pull/81), [#82](https://github.com/vbrandl/sablier-proxy/pull/82))
- Bump `tokio` from 1.44.1 to 1.49.0 ([#44](https://github.com/vbrandl/sablier-proxy/pull/44), [#46](https://github.com/vbrandl/sablier-proxy/pull/46), [#47](https://github.com/vbrandl/sablier-proxy/pull/47), [#55](https://github.com/vbrandl/sablier-proxy/pull/55), [#56](https://github.com/vbrandl/sablier-proxy/pull/56), [#57](https://github.com/vbrandl/sablier-proxy/pull/57), [#58](https://github.com/vbrandl/sablier-proxy/pull/58), [#73](https://github.com/vbrandl/sablier-proxy/pull/73), [#83](https://github.com/vbrandl/sablier-proxy/pull/83))
- Bump `reqwest` from 0.12.14 to 0.12.15 ([#43](https://github.com/vbrandl/sablier-proxy/pull/43))
- Bump `anyhow` from 1.0.97 to 1.0.100 ([#45](https://github.com/vbrandl/sablier-proxy/pull/45), [#59](https://github.com/vbrandl/sablier-proxy/pull/59), [#67](https://github.com/vbrandl/sablier-proxy/pull/67))
- Bump `stefanzweifel/git-auto-commit-action` from 5 to 7 ([#52](https://github.com/vbrandl/sablier-proxy/pull/52), [#71](https://github.com/vbrandl/sablier-proxy/pull/71))
- Bump `actions/checkout` from 4 to 6 ([#60](https://github.com/vbrandl/sablier-proxy/pull/60), [#74](https://github.com/vbrandl/sablier-proxy/pull/74))
- Bump `actions/attest-build-provenance` from 2 to 3 ([#62](https://github.com/vbrandl/sablier-proxy/pull/62))
- Bump `tracing-subscriber` from 0.3.19 to 0.3.22 ([#63](https://github.com/vbrandl/sablier-proxy/pull/63), [#75](https://github.com/vbrandl/sablier-proxy/pull/75))
- Bump `serde` from 1.0.219 to 1.0.228 ([#64](https://github.com/vbrandl/sablier-proxy/pull/64), [#65](https://github.com/vbrandl/sablier-proxy/pull/65), [#66](https://github.com/vbrandl/sablier-proxy/pull/66), [#69](https://github.com/vbrandl/sablier-proxy/pull/69), [#70](https://github.com/vbrandl/sablier-proxy/pull/70))
- Bump `slab` from 0.4.10 to 0.4.11 ([#68](https://github.com/vbrandl/sablier-proxy/pull/68))
- Bump `tracing` from 0.1.41 to 0.1.44 ([#76](https://github.com/vbrandl/sablier-proxy/pull/76), [#80](https://github.com/vbrandl/sablier-proxy/pull/80))
- Bump `actions/cache` from 4 to 5 ([#78](https://github.com/vbrandl/sablier-proxy/pull/78))

## [0.4.0] 2025-03-15

- Make `group` Parameter Optional ([#38](https://github.com/vbrandl/sablier-proxy/pull/38)) thanks to @BUR4KBEY
- Use Official Traefik Plugin Syntax For `names` Parameter ([#40](https://github.com/vbrandl/sablier-proxy/pull/40)) thanks to @BUR4KBEY
- Add `ARM64`/`aarch64` Support ([#41](https://github.com/vbrandl/sablier-proxy/pull/41)) thanks to @BUR4KBEY

### Dependencies
- Bump `anyhow` from 1.0.89 to 1.0.97 ([#5](https://github.com/vbrandl/sablier-proxy/pull/5), [#6](https://github.com/vbrandl/sablier-proxy/pull/6), [#11](https://github.com/vbrandl/sablier-proxy/pull/11), [#12](https://github.com/vbrandl/sablier-proxy/pull/12), [#17](https://github.com/vbrandl/sablier-proxy/pull/17), [#21](https://github.com/vbrandl/sablier-proxy/pull/21), [#27](https://github.com/vbrandl/sablier-proxy/pull/27), [#32](https://github.com/vbrandl/sablier-proxy/pull/32))
- Bump `tokio` from 1.40.0 to 1.44.1 ([#7](https://github.com/vbrandl/sablier-proxy/pull/7), [#13](https://github.com/vbrandl/sablier-proxy/pull/13), [#18](https://github.com/vbrandl/sablier-proxy/pull/18), [#26](https://github.com/vbrandl/sablier-proxy/pull/26), [#34](https://github.com/vbrandl/sablier-proxy/pull/34), [#37](https://github.com/vbrandl/sablier-proxy/pull/37))
- Bump `serde` from 1.0.210 to 1.0.219 ([#8](https://github.com/vbrandl/sablier-proxy/pull/8), [#9](https://github.com/vbrandl/sablier-proxy/pull/9), [#14](https://github.com/vbrandl/sablier-proxy/pull/14), [#20](https://github.com/vbrandl/sablier-proxy/pull/20), [#23](https://github.com/vbrandl/sablier-proxy/pull/23), [#28](https://github.com/vbrandl/sablier-proxy/pull/28), [#33](https://github.com/vbrandl/sablier-proxy/pull/33))
- Bump `reqwest` from 0.12.8 to 0.12.14 ([#10](https://github.com/vbrandl/sablier-proxy/pull/10), [#24](https://github.com/vbrandl/sablier-proxy/pull/24), [#25](https://github.com/vbrandl/sablier-proxy/pull/25), [#35](https://github.com/vbrandl/sablier-proxy/pull/35), [#36](https://github.com/vbrandl/sablier-proxy/pull/36))
- Bump `tracing` from 0.1.40 to 0.1.41 ([#15](https://github.com/vbrandl/sablier-proxy/pull/15))
- Bump `tracing-subscriber` from 0.3.18 to 0.3.19 ([#16](https://github.com/vbrandl/sablier-proxy/pull/16))
- Bump `actions/attest-build-provenance` from 1 to 2 ([#19](https://github.com/vbrandl/sablier-proxy/pull/19))
- Bump `dangoslen/dependabot-changelog-helper` from 3 to 4 ([#31](https://github.com/vbrandl/sablier-proxy/pull/31))

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
