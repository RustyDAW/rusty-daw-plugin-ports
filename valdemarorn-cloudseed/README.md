# CloudSeed

This is a Rust port of ValdemarOrn's
[CloudSeed](https://github.com/ValdemarOrn/CloudSeed), an algorithmic reverb.

## Structure

This plugin is comprised of a DSP backend and various frontends.

 - [`cloudseed-core`](cloudseed-core/) - The DSP that implements the CloudSeed
   algorithmic reverb.
 - `cloudseed-baseplug` (TODO) - A [baseplug](https://github.com/wrl/baseplug)
   instantiation of `cloudseed-core` that provides a VST2 plugin (and in the
   future, other plugin formats) of this reverb.
 - `cloudseed-device` (far future) - An instantiation of `cloudseed-core` that
   provides a DAW device for the future DAW project.

## License

The original [CloudSeed](https://github.com/ValdemarOrn/CloudSeed) is licensed
under the MIT license. You can view the original license
[here](cloudseed-LICENSE.txt) or
[here](https://github.com/ValdemarOrn/CloudSeed/blob/master/license.txt).

This Rust port is licensed under the MIT license. You can view that license
[here](LICENSE.txt).
