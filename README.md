# gui

## Why gui?

To prevent bike shedding, I'll come up with a better name when the project needs one.

## Dependencies

Gui requires a recent rust toolchain to build; it does not (yet) have an
explicit minimum supported rust version, but the latest stable version should
work.

On Linux and BSD, Gui also requires `pkg-config` and `clang`,
and the development packages of `wayland`, `libxkbcommon` and `libxcb`, to be installed.
Some of the examples require `vulkan-loader`.

Most distributions have `pkg-config` installed by default. To install the remaining packages on Fedora, run
```sh
sudo dnf install clang wayland-devel libxkbcommon-x11-devel libxcb-devel vulkan-loader-devel
```
To install them on Debian or Ubuntu, run
```sh
sudo apt-get install pkg-config clang libwayland-dev libxkbcommon-x11-dev libvulkan-dev
```

