# Measures for reproducible results
* idle machine
* cpufreq performance governor
* disable turbo mode
* [cset shield](https://documentation.suse.com/sle-rt/12-SP4/html/SLE-RT-all/cha-shielding-model.html)
* pin benchmark to one of the shielded cores
* disable hyper-threading (not sure if necessary, my test machine has no hyper threading)
* codegen-units = 1 (most likely not necessary)

# Factors affecting performance
* missed inlining plays a huge role of course, unfortunately one can not use `#[inline(always)] on
* functions with `#[target_feature(enable = "...")]` and even that would only be a hint. What is needed
  is an error on non-inlining. Simulating that using [cargo asm](https://github.com/gnzlbg/cargo-asm) to
  make sure that methods supposed to be inlined do not exist in the rlib.
* alignment plays a huge role
  * up to 20% better performance on long but unaligned slices (which are apparently likely at least on Linux)
  * the added code causes a litte slowdown in compat on shorter inputs

# Lessons learned
* 0-initialized temp buf instead of 0x20-initialized (less instructions) -> ✔️ improved perf.
* single 0-initialized temp buf -> ❌ not faster
* single aligned buffer -> ❌ not faster
* two buffers, aligned -> ✔️ improved perf.
* Aligment on `Utf8CheckingState<T>` and `SimdInput` -> ❌ not faster, but seems like a good idea nevertheless

# Tuning criterion

# Laptops
* Generally more noisy due to power and temperature constraints
* Beware of BD PROCHOT on aged machines, can cause severe throttling

# Test machines
* Intel(R) Xeon(R) CPU E3-1225 v3 @ 3.20GHz (Sandy bridge)
* Intel(R) Core(TM) i7-6700HQ CPU @ 2.60GHz (Skylake)
* Intel(R) Core(TM) i7-10750H CPU @ 2.60GHz (Comet Lake)