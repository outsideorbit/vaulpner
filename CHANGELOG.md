# CHANGELOG


## v0.1.9 (2026-05-24)

### ⚠️ BREAKING CHANGE — Secret Format

The Kubernetes secret used to store Vault initialization data has changed.
**Existing deployments must migrate before upgrading** (see migration guide below).

| | Previous (`≤ v0.1.0`) | Current (`v0.1.9+`) |
|---|---|---|
| Secret name | `vault-root-token` | `vault-init` |
| Keys | `root` (unseal key, mislabelled) | `unseal-key-0` … `unseal-key-N`, `root-token` |
| Encoding | Double-base64 (manually encoded + k8s base64) | Raw bytes (k8s-standard base64 only) |

**Migration:** Delete the old secret and allow vaulpner to re-initialize Vault on next start:
```bash
kubectl delete secret vault-root-token -n <namespace>
# Vault will be re-initialized on next vaulpner start
```

### Features / Refactoring

* refactor: rewrite to hexagonal architecture (Ports & Adapters)
  - `src/core/` — domain model, port traits, vault lifecycle service (no external crate imports)
  - `src/adapters/` — `VaultAdapter` (vaultrs) and `K8sAdapter` (kube) implement ports
  - `src/main.rs` — wiring only
* fix: persist both unseal keys **and** root token to Kubernetes secret on initialization (previously root token was silently discarded)
* fix: use unseal key (not root token) for vault unsealing on restart
* chore: enforce coding standards — RPITIT boxing, no `#[async_trait]`, structured tracing, `PartialEq`/`Eq` on all domain types
* ci: add `version-and-tag` workflow job to anchor semver tags on each release
* ci: add `github-release` workflow job to publish GitHub releases with container image table



### Bug Fixes

* fix: correct where files are copied in build stages ([`8bf1d2b`](https://github.com/outsideorbit/vaulpner/commit/8bf1d2b81f8ee3376940359d2d1511ddefb9c80a))

* fix: modify containerfile to align through stages ([`9d0776e`](https://github.com/outsideorbit/vaulpner/commit/9d0776e2e0c24917567adff2921b5223dbbe35da))

* fix: update deprecated input, remove target for release image ([`3f8d825`](https://github.com/outsideorbit/vaulpner/commit/3f8d825bc93cda620edebd8ec26cbeb2b9bd3ca8))

* fix: more cleanup on workflow steps calling wrong job outputs ([`7891e67`](https://github.com/outsideorbit/vaulpner/commit/7891e6740f0b35def1ebb0d20cb9fdf47b2a0c3d))

* fix: cleanup workflow to use appropriate variables ([`4c44533`](https://github.com/outsideorbit/vaulpner/commit/4c44533950c580d491715b8c04b77795fa97a315))

* fix: adjust workflows to send proper inputs and use proper outputs ([`a1c0f0a`](https://github.com/outsideorbit/vaulpner/commit/a1c0f0ac9336f58cef3c4e3fcf10a3e8633f5748))

* fix: adjust tags to CSV

The action claims to support a sequence for the value, however,
the failure it leads to indicates otherwise. ([`4f7442a`](https://github.com/outsideorbit/vaulpner/commit/4f7442aa900194e04f85e7aadbc58bcbb77d5614))

* fix: make tags a list form ([`ca3285c`](https://github.com/outsideorbit/vaulpner/commit/ca3285cf312a227432ae56e139ff6c58aad38e5a))

* fix: update job strategy with matrix ([`5ab21f6`](https://github.com/outsideorbit/vaulpner/commit/5ab21f623d70eaafca322fa3da2e92acc96d4feb))

* fix: remove container image caching ([`3b0d58a`](https://github.com/outsideorbit/vaulpner/commit/3b0d58a4f32bfc0d6f3251c048ca7e7150e5a188))

* fix: modern rust toolchain ([`a8c809f`](https://github.com/outsideorbit/vaulpner/commit/a8c809f8c7d46e036ecb73288be43cb1d1322af9))

* fix: ai slop ([`291491f`](https://github.com/outsideorbit/vaulpner/commit/291491f858a0900eff3e299d63258f439f34f051))

* fix: update workflows ([`2ac0e7a`](https://github.com/outsideorbit/vaulpner/commit/2ac0e7ab388e674139b3242238163370d648947a))

* fix: semantic versioning ([`66a529c`](https://github.com/outsideorbit/vaulpner/commit/66a529c92abcce0a1e0a453f9c4be022e111d587))

* fix: correct ai slop and update ai knowledge ([`331803c`](https://github.com/outsideorbit/vaulpner/commit/331803cc3e595ae8a3d1eedfba4d81df65e70e56))

* fix: correct ai slop ([`5de8f76`](https://github.com/outsideorbit/vaulpner/commit/5de8f7666fc4a0c0212e3f7783aa278b643e2e9e))

* fix: order workflow steps so login happens earlier ([`7df0335`](https://github.com/outsideorbit/vaulpner/commit/7df03353759adcdfc5caf864a04c84b861ac5ed6))

* fix: update docker actions to appropriate versions ([`62c96ba`](https://github.com/outsideorbit/vaulpner/commit/62c96ba2a1cb5a6bab58f4348436e9bdcab8811e))

* fix: linting and test correction ([`f781e29`](https://github.com/outsideorbit/vaulpner/commit/f781e29d5cda8e7ec62ad8dc67baa8ac4752ac11))

* fix(vault): abstract calls out a bit further ([`032470e`](https://github.com/outsideorbit/vaulpner/commit/032470e1214bd80608f7e55a043888b47d744842))

* fix(vault): some cleanup and abstractions ([`a5e6f9b`](https://github.com/outsideorbit/vaulpner/commit/a5e6f9b049a8d554d10adb17024a92a162691c16))

* fix(vault): abstract implementation code ([`8ecb028`](https://github.com/outsideorbit/vaulpner/commit/8ecb028762cf38c7dc7fdb8dc24c005c09fc01b4))

* fix(vault): add functionality to initialize database ([`be6fab6`](https://github.com/outsideorbit/vaulpner/commit/be6fab65c49069dfc661f789d8a3a1aa0974cd96))

* fix(init): add kick off point ([`46eccbe`](https://github.com/outsideorbit/vaulpner/commit/46eccbe200cfdd6f5c3475a61b818addd3cb2a75))

### Chores

* chore: fighting w/ ai some more ([`daa39d3`](https://github.com/outsideorbit/vaulpner/commit/daa39d303000ed27efd111fae999c3fb54eecbae))

* chore: more understanding of how this package works ([`1b8683a`](https://github.com/outsideorbit/vaulpner/commit/1b8683a70fa0d18aa8eb810e560cd2068cbc387f))

* chore: pulling back more to understand outputs of the command ([`ede2fa5`](https://github.com/outsideorbit/vaulpner/commit/ede2fa5af5bddcf9ce0d54080c3828fd2e14e703))

* chore: pulling back harder ([`730e403`](https://github.com/outsideorbit/vaulpner/commit/730e403a11349b36fb591ec6bbfcb7c26cc02982))

* chore: fighting with ai again ([`a570ca1`](https://github.com/outsideorbit/vaulpner/commit/a570ca10df6a6769850db62296804f713d2e5312))

* chore: use correct env output var ([`48e9cb1`](https://github.com/outsideorbit/vaulpner/commit/48e9cb1f7ec0fe0747cd2a3e82329c53a3bb6e86))

* chore: understand how to make the semantic-release product reusable ([`94813f4`](https://github.com/outsideorbit/vaulpner/commit/94813f4285182e1de10f9df2370551e5aacfe3bc))

* chore: testing semrelease ([`896a208`](https://github.com/outsideorbit/vaulpner/commit/896a208831ed552b9ea86d5b3c396c51c8e32bc2))

* chore: testing sem release ([`cb23777`](https://github.com/outsideorbit/vaulpner/commit/cb2377796511ffca5762a0b365fe4d733e8eaa62))

* chore: correct where the debug container workdir is ([`a6a8ac1`](https://github.com/outsideorbit/vaulpner/commit/a6a8ac1ac11957854d75c5ac332febfaa6c96ebd))

* chore: test using rust as the base image ([`bb72b70`](https://github.com/outsideorbit/vaulpner/commit/bb72b705ddd65c75e4a10fcb9fb1e96173aa650e))

* chore: change rust target to musl ([`7d5cc49`](https://github.com/outsideorbit/vaulpner/commit/7d5cc4957890e465965ca8996688af0c54a55ef2))

* chore: fix workflow container image tagging ([`cf9692f`](https://github.com/outsideorbit/vaulpner/commit/cf9692f6783f76a6c4e77675d25c77be32664736))

* chore: readd username to container registry login ([`65a4fb4`](https://github.com/outsideorbit/vaulpner/commit/65a4fb40da643a12d2b9884f17b42a1528bbc58c))

* chore: change to dispatched workflow for testing ([`64f398c`](https://github.com/outsideorbit/vaulpner/commit/64f398c0918f4a2643372592c04c1ab18e146d7f))

* chore: update workflow to run on push ([`e884f95`](https://github.com/outsideorbit/vaulpner/commit/e884f95de2e4cbf764790472cca0392fbdbe770f))

* chore: test adding glibc as static ([`fabc998`](https://github.com/outsideorbit/vaulpner/commit/fabc99806feba2e48b22f63c060dbd67b897c247))

* chore: ai slop cleanup ([`c36b658`](https://github.com/outsideorbit/vaulpner/commit/c36b658cbfb8974d662daba70341ca86ec37cb1e))

* chore: ai slop cleanup duty ([`77fefb4`](https://github.com/outsideorbit/vaulpner/commit/77fefb478284d29f9484fe8ea3cfa94b7edf872d))

* chore: add debugging container image artifact ([`b534f4b`](https://github.com/outsideorbit/vaulpner/commit/b534f4b8249621cebde29296de3561216dfc4bb8))

* chore: cleanup ai slop...maybe ([`cb78608`](https://github.com/outsideorbit/vaulpner/commit/cb7860851d00c2ee316dbf9c64e69b4d3b20ac09))

* chore: ai slopping its ass off again ([`ea05b45`](https://github.com/outsideorbit/vaulpner/commit/ea05b45ab3407a2d09fe6f2073b41fddf56debac))

* chore: update container image tags ([`f445b79`](https://github.com/outsideorbit/vaulpner/commit/f445b791565e65db937c3dc3fe7b3600d8ff9817))

* chore: bump version to 0.0.1 [skip ci] ([`0a0e8bf`](https://github.com/outsideorbit/vaulpner/commit/0a0e8bfa243071d8d40df750a99b8324bd625606))

* chore: adding ai slop ([`50c7d02`](https://github.com/outsideorbit/vaulpner/commit/50c7d02370d02d0f8ab2f7be1bf7877b29f69a25))

* chore: add supporting operational files ([`3816938`](https://github.com/outsideorbit/vaulpner/commit/3816938582f3493db3b0bb837246ccc1add426c1))

* chore(deps): bump tracing-subscriber from 0.3.19 to 0.3.20

Bumps [tracing-subscriber](https://github.com/tokio-rs/tracing) from 0.3.19 to 0.3.20.
- [Release notes](https://github.com/tokio-rs/tracing/releases)
- [Commits](https://github.com/tokio-rs/tracing/compare/tracing-subscriber-0.3.19...tracing-subscriber-0.3.20)

---
updated-dependencies:
- dependency-name: tracing-subscriber
  dependency-version: 0.3.20
  dependency-type: direct:production
...

Signed-off-by: dependabot[bot] <support@github.com> ([`2d67b62`](https://github.com/outsideorbit/vaulpner/commit/2d67b62b376c5aab367f66ff33d0b3fceca1dd1b))

* chore(ci): add workflows for building container image ([`723686e`](https://github.com/outsideorbit/vaulpner/commit/723686e3659f07e1ce61c5542f76644d622198dc))

* chore(deps): bump tokio from 1.43.0 to 1.43.1

Bumps [tokio](https://github.com/tokio-rs/tokio) from 1.43.0 to 1.43.1.
- [Release notes](https://github.com/tokio-rs/tokio/releases)
- [Commits](https://github.com/tokio-rs/tokio/compare/tokio-1.43.0...tokio-1.43.1)

---
updated-dependencies:
- dependency-name: tokio
  dependency-version: 1.43.1
  dependency-type: direct:production
...

Signed-off-by: dependabot[bot] <support@github.com> ([`5c351d7`](https://github.com/outsideorbit/vaulpner/commit/5c351d7d9b25475bdbd118ca10ba42e8cf8dc99e))

* chore(deps): bump openssl from 0.10.70 to 0.10.72

Bumps [openssl](https://github.com/sfackler/rust-openssl) from 0.10.70 to 0.10.72.
- [Release notes](https://github.com/sfackler/rust-openssl/releases)
- [Commits](https://github.com/sfackler/rust-openssl/compare/openssl-v0.10.70...openssl-v0.10.72)

---
updated-dependencies:
- dependency-name: openssl
  dependency-version: 0.10.72
  dependency-type: indirect
...

Signed-off-by: dependabot[bot] <support@github.com> ([`3537aed`](https://github.com/outsideorbit/vaulpner/commit/3537aed5336bbf416a124efafb9690c600e3cc20))

* chore(deps): bump ring from 0.17.8 to 0.17.13

Bumps [ring](https://github.com/briansmith/ring) from 0.17.8 to 0.17.13.
- [Changelog](https://github.com/briansmith/ring/blob/main/RELEASES.md)
- [Commits](https://github.com/briansmith/ring/commits)

---
updated-dependencies:
- dependency-name: ring
  dependency-type: indirect
...

Signed-off-by: dependabot[bot] <support@github.com> ([`93502e9`](https://github.com/outsideorbit/vaulpner/commit/93502e9296aa6384cde2d6b3dbee4e1e7224c365))

* chore: investigate ways to simplify implementation ([`7938d54`](https://github.com/outsideorbit/vaulpner/commit/7938d54023e91e45499fb85c034370934f7cb63b))

### Features

* feat(vault): unseal with key stored in k8s secret ([`1422080`](https://github.com/outsideorbit/vaulpner/commit/14220801f9f4f94e468b14c3726016cf1575389d))

* feat: add ability to retrieve and create secrets ([`83c2ff6`](https://github.com/outsideorbit/vaulpner/commit/83c2ff6291a75fd2369388e0d01119583a2e3bf3))

### Unknown

* Merge pull request #1 from outsideorbit/dependabot/cargo/ring-0.17.13

chore(deps): bump ring from 0.17.8 to 0.17.13 ([`29735ba`](https://github.com/outsideorbit/vaulpner/commit/29735ba6d52f30603a636e72fbdfcb36f570ae14))

* why is nano deafult editor again ([`f73a872`](https://github.com/outsideorbit/vaulpner/commit/f73a872e533ea457f1c148e4a08abcafcadbf2c1))
