# CircleCI Optimization Notes

## ✅ Implemented Optimizations

### 1. Enhanced Cargo Cache Strategy (v2)

**Problem**: Dependencies were being re-downloaded and re-compiled on every run.

**Solution**:
- Use more granular caching with `Cargo.lock` checksum
- Cache specific registry paths separately:
  - `~/.cargo/registry/index` - Package index
  - `~/.cargo/registry/cache` - Downloaded crates
  - `~/.cargo/git/db` - Git dependencies
  - `target` - Build artifacts

**Cache Keys**:
```yaml
v2-cargo-cache-{{ arch }}-{{ checksum "Cargo.toml" }}-{{ checksum "Cargo.lock" }}
```

**Benefits**:
- Precise cache invalidation (only when dependencies change)
- Cross-job cache sharing (all jobs use same cache)
- Reduces dependency download/compile time from 10s to near-zero on cache hit

### 2. Workspace Sharing for Build Artifacts

**Problem**: `doc` job was rebuilding the project even though `build_and_test` already built it.

**Solution**:
- Use `persist_to_workspace` in `build_and_test`
- Use `attach_workspace` in `doc` job
- Reuse compiled artifacts

**Benefits**:
- `doc` job now just generates documentation without rebuilding
- Saves ~10-20 seconds per run

### 3. cargo-binstall + Binary Caching

**Problem**: `cargo install` was compiling tools from source every time (3-7 minutes).

**Solution**:
- Use `cargo-binstall` to download precompiled binaries
- Cache the installed binaries
- Check if tool exists before installing

**Benefits**:
- First run: 5-7 minutes → 20-30 seconds
- Cached run: ~2-5 seconds
- 95%+ time reduction

### 4. Job Consolidation

**Problem**: Each job requires ~15s for container spin-up.

**Solution**:
- Merge `check_format` + `lint` → `fast_checks`
- Merge `build` + `test` → `build_and_test`
- Reduce from 7 jobs to 5 jobs

**Benefits**:
- Save ~30 seconds per CI run
- Faster failure feedback

## ⚠️ Known Limitations

### Docker Image Download Time (~12-16 seconds per job)

**Problem**:
CircleCI downloads the `cimg/rust:1.85` image (218-226 MiB) for each job on each run, because:
1. Jobs may run on different CircleCI hosts
2. Docker layer cache is not persisted between runs on free/standard plans
3. The "image cache not found on this host" message indicates the image is not available locally

**Why This Happens**:
- CircleCI uses ephemeral compute instances
- Each instance starts with a fresh Docker daemon
- No shared image registry between instances

**Potential Solutions** (with trade-offs):

#### Option A: Docker Layer Caching (DLC) - **Paid Feature**
```yaml
jobs:
  build_and_test:
    docker:
      - image: cimg/rust:1.85
    docker_layer_caching: true  # Requires paid CircleCI plan
```
- **Cost**: ~$15-30/month additional
- **Benefit**: Complete elimination of image download time
- **Best for**: Teams with frequent CI runs

#### Option B: Use Smaller Base Image
```yaml
jobs:
  build_and_test:
    docker:
      - image: rust:1.85-slim  # ~150MB vs 273MB
```
- **Benefit**: ~30-40% faster download
- **Trade-off**: May need to install additional tools
- **Complexity**: Need to maintain list of required packages

#### Option C: Self-Hosted Runner
- Use your own infrastructure with persistent Docker cache
- **Benefit**: One-time image download, permanent cache
- **Trade-off**: Infrastructure maintenance overhead
- **Cost**: Server costs + maintenance time

#### Option D: Accept the Trade-off (Current Approach)
- Accept 12-16 seconds per job for image download
- **Benefit**: Free, no additional complexity
- **Total Impact**: ~60-80 seconds for 5 jobs
- **Recommendation**: This is reasonable for most projects

### Current Performance Profile

With all free-tier optimizations applied:

| Stage | Time (First Run) | Time (Cached) |
|-------|-----------------|---------------|
| Container spin-up (×5) | ~60-80s | ~60-80s |
| cargo-binstall install | ~20-30s | ~2-5s |
| Cargo dependencies | ~10s | ~1-2s |
| Actual work (build/test/lint) | ~2-3 min | ~2-3 min |
| **Total** | ~4-5 min | ~3-4 min |

**Without optimizations**: ~12-15 minutes
**With optimizations**: ~3-5 minutes
**Improvement**: **60-75% faster** ✨

## 🎯 Recommendations

### For Free/Standard Plans
1. ✅ Keep current configuration (already optimized)
2. ✅ Monitor cache hit rates
3. ✅ Consider merging more jobs if build times increase
4. ⚠️ Accept container spin-up time as baseline overhead

### For Performance Plans
1. Enable Docker Layer Caching (DLC)
2. Consider self-hosted runners for very frequent builds
3. Use CircleCI's performance insights to identify bottlenecks

### For Future Optimization
1. **If adding more jobs**: Consider if they can be merged
2. **If dependencies grow**: Consider using `cargo-chef` for better layer caching
3. **If build time increases**: Consider splitting tests into parallel jobs with `--test-threads`

## 📊 Metrics to Track

Monitor these metrics in CircleCI dashboard:
- Cache hit rate (should be >80%)
- Average build time
- Container spin-up time
- Time to first failure (should be <1 minute)

## 🔗 References

- [CircleCI Caching Strategies](https://circleci.com/docs/caching/)
- [Docker Layer Caching](https://circleci.com/docs/docker-layer-caching/)
- [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)
- [Rust CI Best Practices](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)

