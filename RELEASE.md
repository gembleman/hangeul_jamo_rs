# Release Process

This document describes how to create a new release for hangeul_jamo_rs.

## Automated Release Workflow

This project uses automated GitHub releases triggered by Git tags.

### How It Works

1. **Tag Creation**: When you push a tag matching the pattern `v*.*.*` (e.g., `v0.1.0`, `v1.2.3`), the release workflow automatically triggers.

2. **Release Creation**: The workflow automatically:
   - Generates a changelog from commit messages
   - Creates a GitHub release with release notes
   - Triggers the wheel build workflow
   - Builds and uploads Python wheels for all platforms (Linux, Windows, macOS)

3. **Pre-releases**: Tags with suffixes (e.g., `v0.1.0-beta`, `v1.0.0-rc1`) are marked as pre-releases.

### Creating a New Release

You have **three options** to create a release:

#### Option 1: GitHub Web UI (Easiest)

1. Go to the [Actions tab](https://github.com/gembleman/hangeul_jamo_rs/actions/workflows/release.yml)
2. Click "Run workflow" button
3. Enter the version number (e.g., `0.2.0`)
4. Optionally check "Mark as pre-release" for beta/RC versions
5. Click "Run workflow"

The workflow will automatically:
- Update version numbers (if needed)
- Create the tag
- Generate changelog
- Create GitHub release
- Build and upload wheels

#### Option 2: Command Line (Recommended for developers)

##### Step 1: Update Version Numbers

Update the version in both files:

**Cargo.toml**:
```toml
[package]
version = "0.2.0"  # Update this
```

**pyproject.toml**:
```toml
[project]
version = "0.2.0"  # Update this
```

##### Step 2: Commit Version Changes

```bash
git add Cargo.toml pyproject.toml
git commit -m "chore: bump version to 0.2.0"
```

##### Step 3: Create and Push Tag

```bash
# Create an annotated tag
git tag -a v0.2.0 -m "Release version 0.2.0"

# Push the tag to GitHub
git push origin v0.2.0
```

##### Step 4: Wait for Automation

The GitHub Actions workflow will automatically:
1. Create a GitHub release (within ~1 minute)
2. Build wheels for all platforms (within ~10-15 minutes)
3. Upload wheels to the release

### Commit Message Conventions

For better changelog generation, use conventional commit messages:

- `feat: add new feature` - New features
- `fix: resolve bug` - Bug fixes
- `docs: update documentation` - Documentation changes
- `chore: update dependencies` - Maintenance tasks
- `refactor: improve code structure` - Code refactoring
- `test: add tests` - Test additions
- `perf: improve performance` - Performance improvements

### Creating a Pre-release

For beta or release candidate versions:

```bash
git tag -a v0.2.0-beta -m "Beta release 0.2.0-beta"
git push origin v0.2.0-beta
```

This will create a release marked as "pre-release" on GitHub.

#### Option 3: Manual GitHub Release (Not Recommended)

If you create a release directly from the GitHub Releases page:

1. Go to the [GitHub Releases page](https://github.com/gembleman/hangeul_jamo_rs/releases)
2. Click "Draft a new release"
3. Create a new tag (e.g., `v0.2.0`)
4. Manually write the release notes
5. Click "Publish release"

**Note**: This method will:
- ✅ Build and upload wheels automatically
- ❌ NOT generate changelog automatically
- ❌ NOT trigger the release.yml workflow

**Recommendation**: Use Option 1 (GitHub Actions UI) or Option 2 (command line) instead.

### Troubleshooting

**Workflow doesn't trigger:**
- Ensure the tag matches the pattern `v*.*.*`
- Check that you pushed the tag: `git push origin <tag-name>`
- Verify workflow permissions in repository settings

**Build fails:**
- Check the [Actions tab](https://github.com/gembleman/hangeul_jamo_rs/actions) for error details
- Ensure all tests pass before creating a tag
- Verify Cargo.toml and pyproject.toml are valid

**Wheels not uploaded:**
- The wheel build takes 10-15 minutes across all platforms
- Check individual job logs in the build-wheels workflow
- Ensure the release was created successfully first

### Version Numbering

This project follows [Semantic Versioning](https://semver.org/):

- **MAJOR** version (x.0.0): Incompatible API changes
- **MINOR** version (0.x.0): New functionality, backwards compatible
- **PATCH** version (0.0.x): Bug fixes, backwards compatible

### PyPI Release (Optional)

To publish to PyPI, add PyPI credentials to GitHub Secrets and update the build-wheels.yml workflow to include PyPI upload steps.
