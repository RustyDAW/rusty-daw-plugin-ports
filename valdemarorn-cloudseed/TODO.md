# TODO

Done:
 - [x] Port `ReverbController.h` and `Parameter.h` (done: `lib.rs` and `parameters.rs`)
 - [x] Port `ValueTables.cpp` and `ValueTables.h` (done: `value_tables.rs`)

Next:
 - [ ] Port `ReverbChannel.h`

Eventually (DSP):
 - [ ] Port `AllpassDiffuser.h`
 - [ ] Port `MultitapDiffuser.h`
 - [ ] Port `AudioLib/Biquad.cpp` and `AudioLib/Biquad.h`
 - [ ] Port `AudioLib/Hp1.h`
 - [ ] Port `AudioLib/Lp1.h`
 - [ ] Port `AudioLib/MathDefs.h`
 - [ ] Port `AudioLib/ShaRandom.cpp` and `AudioLib/ShaRandom.h`
 - [ ] Port `Utils/Sha256.cpp` and `Utils/Sha256.h`
 - [ ] Port `DelayLine.h`
 - [ ] Port `FastSin.cpp` and `FastSin.h`
 - [ ] Port `ModulatedAllpass.h`
 - [ ] Port `ModulatedDelay.h`
 - [ ] Port `Utils.h`

Eventually (misc):
 - [ ] Port the tests? (`CoudSeed.Tests`)
 - [ ] Re-architect and make "rusty", now that you know how everything works
 - [ ] Port the presets (`Factory Programs`)
 - [ ] Figure out what the C# code is doing (just GUI?)
 - [ ] Make this work with `baseplug`
 - [ ] Make a GUI for the baseplug plugin
 - [ ] Clean up code (`#[allow(...)]`, etc), refactor, make API nice with `baseplug`, etc.
 - [ ] Benchmark, optimize
 - [ ] Finalize documentation
