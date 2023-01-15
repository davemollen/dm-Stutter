## dm-Reverb

A reverb effect written in Rust.
The effect can be compiled to a [lv2](./lv2) or [vst](./vst) plugin.
This plugin has been written primarily to run on [Mod devices](https://moddevices.com/). And because I mainly use this for guitar it's just mono to stereo for now.

## Table of contents:

- [Mod devices installation](#Mod-devices-installation)
- [LV2 installation](#LV2-installation)
- [VST installation](#VST-installation)

## Mod devices installation

You can find the plugin for the Mod Dwarf [here](./lv2/dm-Reverb.lv2/).

For Mod Duo, follow the [lv2 instructions](#LV2-installation) first. Then finish the instructions below.

- Copy the .lv2 folder into your Mod:

  ```
  scp -rp <path to dm-Reverb.lv2> root@192.168.51.1:/root/.lv2
  ```

- Enter Mod password
- Reboot Mod

## LV2 installation

In order to build the binaries you need to have Docker installed. If so, proceed with the following steps:

- Run `./build-lv2.sh` in the root directory.
- Copy/paste the binary of the target platform from the `./lv2/out` directory into `./lv2/dm-Reverb.lv2`

## VST installation

First go to the [vst scripts folder](./vst/scripts).

Windows:

1. Run `./build.sh`
2. Copy libdm_Reverb.dll in /target/release to your vst plugin folder

Mac

Run `./build-for-mac.sh`
