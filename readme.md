# Tau's Terminal Auto Dark/Light Mode Agenda

**Goal: Terminal Applications Should Automatically Adapt To Dark/Light Mode.**

This repository is a place for me to collect links / resources and have a direct
jumping off point to the different PRs / issues.

## Products
* [terminal-colorsaurus]: A Rust library that answers the question "Is this terminal dark or light?".
* [terminal-trx]: A Rust library that provides a readable and writable handle to the current tty.
* [Continuous Color Reporting]: Specification for an extension to OSC 4, 10...19 for continously receiving updates when the color changes.
* [Tau's Exhaustive List of DEC Modes][dec-modes]: An exhaustive list of all DEC (private) modes that are encountered in the wild.  

## Contributions
* (PR) [delta]: [Detect Dark/Light Mode from Terminal][delta-pr]
* (PR) [bat]: [Choose Theme Based on The Terminal's Color Scheme](https://github.com/sharkdp/bat/pull/2896)

## Things to Investigate
* zellij
  * Had issues with termbg: https://github.com/zellij-org/zellij/issues/538
  * Currently does not detect the color scheme
  * In my case sometimes reported white as fg and sometimes black?

[bat]: https://github.com/sharkdp/bat
[delta]: https://github.com/dandavison/delta
[delta-pr]: https://github.com/dandavison/delta/pull/1615
[terminal-colorsaurus]: https://github.com/bash/terminal-colorsaurus
[terminal-trx]: https://github.com/bash/terminal-trx
[Continuous Color Reporting]: https://github.com/bash/continuous-color-reporting
[dec-modes]: https://tau.garden/dec-modes/
