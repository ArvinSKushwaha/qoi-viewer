# QOI-Viewer

This rust application is just a simple QOI-file viewer. Unfortunately, the QOI format isn't too widely supported, so to make testing rendering results easier,
this application can render it for you.

## Installation

Installing this application is very easie (if you have `cargo` already installed).

```bash
$ git clone https://github.com/ArvinSKushwaha/qoi-viewer.git && cd qoi-viewer
$ cargo install --path .
```

This will install the application to `~/.cargo/bin/`. Eventually, I'll publish the crate
so it can be more easily installed, but this will do for now.

Note: This viewer is extremely rudimentary. Zooming, panning, and whatnot are not implemented yet.

## Usage

```bash
USAGE:
qoi-viewer <IMAGE>

ARGS:
<IMAGE>    The path to the image to render

OPTIONS:
-h, --help       Print help information
-V, --version    Print version information
```

TODOs:
- [ ] Add user interactivity (zooming, panning, etc.)
- [ ] Fix logical display size muckery.
