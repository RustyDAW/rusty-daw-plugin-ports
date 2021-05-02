# TODO

Done:
 - [x] Port `ReverbController.h` and `Parameter.h` (done: `lib.rs` and `parameters.rs`)
 - [x] Port `ValueTables.cpp` and `ValueTables.h` (done: `value_tables.rs`)
 - [x] Port `ReverbChannel.h` (done: `reverb_channel.rs`)

Next:
 - [ ] Port `AudioLib/ShaRandom.cpp` and `AudioLib/ShaRandom.h`

Eventually (DSP):
 - [ ] Port `DelayLine.h`
 - [ ] Port `ModulatedDelay.h`
 - [ ] Port `MultitapDiffuser.h`
 - [ ] Port `AllpassDiffuser.h`
 - [ ] Port `AudioLib/Biquad.cpp` and `AudioLib/Biquad.h`
 - [ ] Port `AudioLib/Hp1.h`
 - [ ] Port `AudioLib/Lp1.h`
 - [ ] Port `AudioLib/MathDefs.h`
 - [ ] Port `FastSin.cpp` and `FastSin.h`
 - [ ] Port `ModulatedAllpass.h`
 - [ ] Port `Utils.h`

Eventually (misc):
 - [ ] Write a test suite that compares inputs/outputs between Rust CloudSeed to C++ CloudSeed
 - [ ] Remove as many `Vec<T>` as possible
 - [ ] See if you can fix the weird parameter thing in `delay_line.rs`
 - [ ] Clean up code (`#[allow(...)]`, etc)
 - [ ] Re-architect and make "rusty", now that you know how everything works
 - [ ] Figure out what the C# code is doing (just GUI?)
 - [ ] Make this work with `baseplug`
 - [ ] Port the presets (`Factory Programs`)
 - [ ] Refactor, make API nice with `baseplug`, etc.
 - [ ] Make a GUI for the baseplug plugin
 - [ ] Benchmark, optimize
 - [ ] Finalize documentation
