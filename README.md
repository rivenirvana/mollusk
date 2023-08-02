# mollusk

Barebone Rust Shell

## Changes
* implemented `cd` as built-in command
* support for chaining operators `&&` and `||`
* display current directory
* 'stateless' arithmetic via `echo` (e.g. `echo $((6+9))`)
