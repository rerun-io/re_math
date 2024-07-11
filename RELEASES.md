# Release Checklist

* [ ] Update `CHANGELOG.md` using `./scripts/generate_changelog.py --commit-range 0.x.y..HEAD`
* [ ] Bump version numbers
* [ ] `git commit -m 'Release 0.x.0 - summary'`
* [ ] `cargo publish --quiet`
* [ ] `git tag -a 0.x.0 -m 'Release 0.x.0 - summary'`
* [ ] `git pull --tags && git tag -d latest && git tag -a latest -m 'Latest release' && git push --tags origin latest --force && git push origin main ; git push --tags`
* [ ] Do a GitHub release: https://github.com/rerun-io/re_math/releases/new
