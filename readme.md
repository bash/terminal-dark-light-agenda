## Goals
* [ ] Survey Terminals if they support my latency measuring trick (`printf '\e[c'`)
* [ ] Survey Terminals if they support reading the foreground and background color
      and what format they return (`printf '\e]10;?\a'`, `printf '\e]11;?\a'`)

### Side quests
* [ ] Survey Terminals to see what `TERM`/`TERM_PROGRAM`/etc. they set.
* [ ] Measure Latency

## Results
| Terminal          | `CSI c` | fg                   | bg                   | mean latency | `TERM`           | `TERM_PROGRAM`    | `TERM_PROGRAM_VERSION` | Version Tested             |
|-------------------|---------|----------------------|----------------------|--------------|------------------|-------------------|------------------------|----------------------------|
| Jetbrains Fleet   | yes     | no                   | no                   | 82.924µs     | `xterm-256color` | `Jetbrains.Fleet` | yes                    | build 1.29.213 (macOS)     |
| macOS Terminal    | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 67.267µs     | `xterm-256color` | `Apple_Terminal`  | yes                    | Version 2.13 (447)         |
| iTerm2            | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 39.944317ms  | `xterm-256color` | `iTerm.app`       | yes                    | Build 3.5.0beta18          |
| Alacritty         | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 121.323µs    | `xterm-256color` | no                | no                     | Version 0.13.1 (1) (macOS) |
| VSCode (xterm.js) | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 7.96848ms    | `xterm-256color` | `vscode`          | yes                    | 1.85.1 (macOS)             |
| iSH (hterm)       | yes     | no                   | no                   | -            | `xterm-256color` | no                | no                     | 1.3.2 (Build 494) (iOS)    |
| IntelliJ IDEA     | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 53.284µs     | `xterm-256color` | no [^1]           | no                     | PyCharm 2023.3.2 (macOS)   |


[^1]: But it set's `TERMINAL_EMULATOR=JetBrains-JediTerm` instead.

## Terminals to Test
* [x] macOS Terminal
* [x] iTerm2
* [x] Alacritty
* GNOME Terminal, Console, etc.
* [x] VSCode (xterm.js)
* IntelliJ IDEA
* [x] JetBrains Fleet
* [x] iSH (hterm)
* Windows Terminal
* Old Windows conhost.exe
* PuTTY
* `screen`
* `tmux`

## Supported Terminal Sequences
* [xterm.js](https://xtermjs.org/docs/api/vtfeatures)
* [VT100](https://vt100.net/docs/vt100-ug/chapter3.html)
* [hterm](https://chromium.googlesource.com/apps/libapps/+/HEAD/hterm/docs/ControlSequences.md)

## Various Resources
* [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code)
* [ANSI/VT100 Terminal Control Escape Sequences](http://web.archive.org/web/20190624214929/http://www.termsys.demon.co.uk/vtansi.htm)
* [Serverfault answer for measuring latency](https://serverfault.com/a/977082)
* [iterm2 Proprietary Escape Codes](https://iterm2.com/documentation-escape-codes.html)
* [Awesome Terminal](https://github.com/cdleon/awesome-terminals)
