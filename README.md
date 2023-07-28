# topdrop

A small tool for managing dropdown/quake-style terminals, similar to [tdrop](https://github.com/noctuid/tdrop)

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
