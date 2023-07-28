# topdrop

A small tool for managing dropdown/quake-style terminals, similar to [tdrop](https://github.com/noctuid/tdrop).

Supported by any window manager supported by xdotool.

```
Usage: topdrop [OPTIONS] <APPLICATION_NAME> <X1> <Y1> <X2> <Y2>

Arguments:
  <APPLICATION_NAME>  The name of the application to launch
  <X1>                The top left x coordinate of the window
  <Y1>                The top left y coordinate of the window
  <X2>                The bottom right x coordinate of the window
  <Y2>                The bottom right y coordinate of the window

Options:
  -u, --unique-name <UNIQUE_NAME>  Only needed if trying to manage multiple dropdowns from the same application
  -s, --state <STATE>              [possible values: modal, sticky, maximized-vert, maximized-horz, above, below, skip-taskbar, skip-pager, fullscreen, hidden, shaded, demands-attention]
  -h, --help                       Print help
```

## Requirements

- [xdotool](https://github.com/jordansissel/xdotool)
- Linux-based/MacOS operating system

## Installation

### Cargo
- cargo install topdrop

### Pre-built binary
- Can be downloaded via [releases](https://github.com/dob9601/topdrop/releases/latest)
- Available for Apple Darwin and MUSL targets.

## Configuration

### Example Configuration for Gnome with Kitty
![image](https://github.com/dob9601/topdrop/assets/24723950/04d77fa8-0c34-4102-8321-f547ce775766)

[Demo](https://github.com/dob9601/topdrop/assets/24723950/0bf3c1ef-c74e-4b3a-b1d1-7deab263b72b)
