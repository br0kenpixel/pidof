# Simple and fast `pidof` implementation
`pidof` is a simple command that list the PID/s of a process. It looks up processes by their name.

This implementation does not have any arguments, it's extremely minimal. It's useful on constrained systems, or systems that don't have a `pidof` command, such as macOS and Windows.

## Compatibility
- [x] macOS Monterey
- [ ] Windows
    - Theoretically should work, but hasn't been tested.
- [x] Linux

## Notes
This project contains optimizations for size.