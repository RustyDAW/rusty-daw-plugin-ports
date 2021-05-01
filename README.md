# rusty-daw-plugin-ports

Ports of open source audio plugins to Rust & baseplug for use with the RustyDAW
project.

## Repo structure

This repo is a cargo workspace, comprised of various different crates that make
up the plugin ports.

Generally, each directory within the repo's root is a different plugin, although
we may have some directories for common functionality between the plugins.

## Plugin ports

 - [`valdemarorn-cloudseed`](valdemarorn-cloudseed/) - A Rust port of
   ValdemarOrn's [CloudSeed](https://github.com/ValdemarOrn/CloudSeed), an
   algorithmic reverb.

Each plugin may be licensed differently. Check that plugin's directory for
details.
