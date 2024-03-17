# Terminal Color Change Notification

## Abstract
TODO

## Feedback / Signals
This table tracks signals and feedback from relevant parties.

| Project     | Discussion         | Signal | Implementation |
|-------------|--------------------|--------|----------------|
| [Alacritty] |                    |        |                |
| [iTerm2]    |                    |        |                |
| [Konsole]   |                    |        |                |
| [tmux]      |                    |        |                |
| [VTE]       | [issue][vte-issue] | -      | -              |
| [zellij]    |                    |        |                |

*You are warmly invited to open a PR if your project is not on the list.*

## Detailed Description
TODO

## Prior Art and Alternatives
### `SIGWINCH`
This mechanism is implemented by [iTerm][iterm-sigwinch].
Some tools such as [tmux][tmux-sigwinch] and [zellij][zellij-sigwinch] already interpret `SIGWINCH` as a color changed signal.

Using an escape sequence to deliver the change notification
has a couple of advantages over using `SIGWINCH`:
* `SIWGINCH` is fired many times when the terminal is resized.
  Applications that care about the color need to debounce the signal somehow
  to avoid sending `OSC 10` / `OSC 11` too often.
* An escape sequence can deliver the new color value
  directly so applications don't have to send `OSC 10` / `OSC 11`
  themselves.
* An escape sequences is portable to Windows.

## Resources
* [xterm Control Sequences](https://invisible-island.net/xterm/ctlseqs/ctlseqs.pdf)


[VTE]: https://gitlab.gnome.org/GNOME/vte
[vte-issue]: https://gitlab.gnome.org/GNOME/vte/-/issues/2740
[Konsole]: https://invent.kde.org/utilities/konsole
[tmux]: https://github.com/tmux/tmux
[zellij]: https://github.com/zellij-org/zellij
[Alacritty]: https://github.com/alacritty/alacritty
[iTerm2]: https://gitlab.com/gnachman/iterm2/-/issues
[iterm-sigwinch]: https://gitlab.com/gnachman/iterm2/-/issues/9855
[tmux-sigwinch]: https://github.com/tmux/tmux/issues/3582
[zellij-sigwinch]: https://github.com/zellij-org/zellij/pull/1358
