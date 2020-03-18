# olympus-liveview (PoC?)

`olympus-liveview` is a program that piped realtime video from an Olympus OM-D or Pen digital camera to a virtual webcam on GNU/Linux.

## Build
Simply run `rustc main.rs` and all is done.

## Usage
1. Build and install [v4l2loopback](https://github.com/umlaeute/v4l2loopback).
2. `sudo modprobe v4l2loopback exclusive_caps=1`
3. Set your camera in Wifi mode and connect your computer to it.
4. `./web.sh`

## Tested cameras
* Pen Lite E-PL7
