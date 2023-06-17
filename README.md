## dm-Reverb

A reverb effect written in Rust.
The effect can be compiled to a [lv2](./lv2), [vst](./vst) or [vst3/CLAP](./vst3) plugin.
This plugin has been written primarily to run on [Mod devices](https://moddevices.com/).

## Table of contents:

- [Mod devices installation](#Mod-devices-installation)
- [VST installation](#VST-installation)
- [VST3 installation](#VST3-installation)

## Mod devices installation

You can find the plugin for the Mod Dwarf [here](./lv2/dm-Reverb.lv2/).

To modify the code and build the plugin see these [instructions](https://github.com/moddevices/mod-plugin-builder).

## VST installation

Windows:

1. Run `./scripts/build-vst.sh`
2. Copy dll file in /target/release to your vst plugin folder

Intel Mac:

- Run `./scripts/build-vst-for-mac.sh`.

M1 Mac:

- Run `./scripts/build-vst-for-mac-m1.sh`.

## VST3 installation

Mac:

- Run: `./scripts/build-vst3-for-mac.sh`
