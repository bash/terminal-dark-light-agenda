> [!NOTE]
> These documents are from my original exploration into this topic.
> Much more complete and thorough results can be found in the terminal-colorsaurus repository:
> * [Terminal Survey](https://github.com/bash/terminal-colorsaurus/blob/main/doc/terminal-survey.md)
> * [Latency Measurements](https://github.com/bash/terminal-colorsaurus/blob/main/doc/latency.md)

## Goals
* [ ] Survey Terminals if they support my latency measuring trick (`printf '\e[c'`)
* [ ] Survey Terminals if they support reading the foreground and background color
      and what format they return (`printf '\e]10;?\a'`, `printf '\e]11;?\a'`)

### Side quests
* [ ] Survey Terminals to see what `TERM`/`TERM_PROGRAM`/etc. they set.
* [ ] Measure Latency

## Results
| Terminal              | `CSI c` | fg                   | bg                   | mean latency | `TERM`           | `TERM_PROGRAM`    | `TERM_PROGRAM_VERSION` | Version Tested             |
|-----------------------|---------|----------------------|----------------------|--------------|------------------|-------------------|------------------------|----------------------------|
| Jetbrains Fleet       | yes     | no                   | no                   | 82.924µs     | `xterm-256color` | `Jetbrains.Fleet` | yes                    | build 1.29.213 (macOS)     |
| macOS Terminal        | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 67.267µs     | `xterm-256color` | `Apple_Terminal`  | yes                    | Version 2.13 (447)         |
| iTerm2                | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 39.944317ms  | `xterm-256color` | `iTerm.app`       | yes                    | Build 3.5.0beta18          |
| Alacritty             | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 121.323µs    | `xterm-256color` | no                | no                     | Version 0.13.1 (1) (macOS) |
| VSCode (xterm.js)     | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 7.96848ms    | `xterm-256color` | `vscode`          | yes                    | 1.85.1 (macOS)             |
| iSH (hterm)           | yes     | no                   | no                   | -            | `xterm-256color` | no                | no                     | 1.3.2 (Build 494) (iOS)    |
| IntelliJ IDEA         | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 53.284µs     | `xterm-256color` | no [^1]           | no                     | PyCharm 2023.3.2 (macOS)   |
| [Contour]             | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 25.833µs     | `contour` [^2]   | no [^3]           | no [^4]                | 0.4.1.6292 (macOS)         |
| GNOME Terminal (vte)  | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 10.539126ms  | `xterm-256color` | no                | no                     | 3.50.1                     |
| (GNOME) Console (vte) | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | -            | `xterm-256color` | `kgx`             | yes                    | 45.0                       |
| Konsole               | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 26.593µs     | `xterm-256color` | no                | no                     | 23.08.4                    |
| [QTerminal]           | yes     | no                   | no                   | 27.85µs      | `xterm-256color` | no                | no                     | 1.3.0                      |
| [foot]                | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 15.025µs     | `foot`           | no                | no                     | 1.16.1                     |
| xterm                 | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | 18.9µs       | `xterm-256color` | no                | no                     | 385                        |
| Linux console         | yes     | no                   | no                   | 4.073µs      | `linux`          | no                | no                     | -                          |
| Windows Terminal      | yes     | no                   | no                   | -            | no [^5]          | no                | no                     | 1.18.3181.0                |
| Windows Console Host  | yes     | no                   | no                   | -            | no               | no                | no                     | Windows 10.0.22631.2428    |
| PuTTY                 | yes     | no                   | no                   | -            | -                | -                 | -                      | 0.80                       |
| Hyper                 | yes     | yes                  | yes                  | 18.883401ms  | `xterm-256color` | `Hyper`           | yes                    | 3.4.1 (macOS)              |
| ConEmu / Cmder        | yes     | no                   | no                   | -            | -                | -                 | -                      | 230724 stable              |
| Mintty                | yes     | `rgb:ffff/ffff/ffff` | `rgb:ffff/ffff/ffff` | -            | `xterm`          | `mintty`          | yes                    | 3.6.1                      |


[^1]: But it sets `TERMINAL_EMULATOR=JetBrains-JediTerm` instead.
[^2]: But it provides a terminfo entry by adding `TERMINFO_DIRS`.
[^3]: But it sets `TERMINAL_NAME=contour` instead.
[^4]: But it sets `TERMINAL_VERSION_STRING` and `TERMINAL_VERSION_TRIPLE` instead.
[^5]: But it can be recognized by `WT_SESSION` instead.

## Terminals to Test
* [x] macOS Terminal
* [x] iTerm2
* [x] Alacritty
* [x] GNOME Terminal, (GNOME) Console, MATE Terminal, XFCE Terminal, (GNOME) Builder, (elementary) Terminal, LXTerminal etc. all use the `vte` library.
* [x] VSCode (xterm.js)
* [x] IntelliJ IDEA
* [x] JetBrains Fleet
* [x] iSH (hterm)
* [x] Windows Terminal
* [x] Old Windows conhost.exe
* [x] PuTTY
* [x] [Mintty](https://mintty.github.io/)
* `screen`
* `tmux`
* Others from https://github.com/dalance/termbg

## Supported Terminal Sequences
* [xterm.js](https://xtermjs.org/docs/api/vtfeatures)
* [VT100](https://vt100.net/docs/vt100-ug/chapter3.html)
* [hterm](https://chromium.googlesource.com/apps/libapps/+/HEAD/hterm/docs/ControlSequences.md)
* [ConEmu](https://conemu.github.io/en/AnsiEscapeCodes.html#List_of_supported_codes)

## Various Resources
* [ANSI Escape Code](https://en.wikipedia.org/wiki/ANSI_escape_code)
* [ANSI/VT100 Terminal Control Escape Sequences](http://web.archive.org/web/20190624214929/http://www.termsys.demon.co.uk/vtansi.htm)
* [Serverfault answer for measuring latency](https://serverfault.com/a/977082)
* [iterm2 Proprietary Escape Codes](https://iterm2.com/documentation-escape-codes.html)
* [Awesome Terminal](https://github.com/cdleon/awesome-terminals)
* [Support querying the colors via OSC ](https://github.com/microsoft/terminal/issues/3718) has some interesting insights.
* [Feature reporting revamp](https://gitlab.freedesktop.org/terminal-wg/specifications/-/issues/8) has interesting insights on the problems with asynchronous responses.

## Impl Resources
* [Example impl on how to acquire TTY](https://gist.github.com/bash/51f95bf86781564ff6b8cee3422e217d)
* [Console Screen Buffers (Windows)](https://learn.microsoft.com/en-us/windows/console/console-screen-buffers#character-attributes)
* [Console Handles (Windows)](https://learn.microsoft.com/en-us/windows/console/console-handles)

## Delta
* [Previous PR](https://github.com/dandavison/delta/pull/1493)
* [Issue](https://github.com/dandavison/delta/issues/447)

[Contour]: https://contour-terminal.org/
[QTerminal]: https://github.com/lxqt/qterminal
[foot]: https://codeberg.org/dnkl/foot
