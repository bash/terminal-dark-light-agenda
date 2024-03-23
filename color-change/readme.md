# Color Change Events

Terminal programs can already query the terminal's foreground, background, etc. colors by sending `OSC 1[1-9]` with `?` instead of a color. Supporting terminals respond with an `OSC 1[1-9]` sequence with the current color.

Long lived applications currently have no way of being notified of changes to these colors.
This is particiularly interesting in terminals that dynamically adapt to the OS's dark-light preference.

The goal of this proposal is to extend `OSC 1[1-9]` with continous reporting.

## Detailed Description
The `OSC 10` ... `OSC 19` sequences are extended to accept the special values `?+` and `?-`.

If `?+` is given, then the terminal will reply with a control sequence of the same form which can be used to set the corresponding dynamic color every time that color is changed.

If `?-` is given, then the terminal will stop continuously reporting the corresponding dynamic color.

The terminal also report if the color is set via the corresponding `OSC` sequence.
To prevent infinite loops between terminal and application, no notification is sent when the dynamic color already has the specified value.

Examples:
```
# Enable events for the VT100 text foreground color
<OSC> 10 ; ?+ <ST>
# Enable events for text cursor color
<OSC> 12 ; ?+ <ST>
# Disable events for the VT100 text foreground color
<OSC> 10 ; ?- <ST>
```

## Implementation
* [VTE]: [issue][vte-issue] open, in discussion
* [Alacritty]: no issue opened yet
* [iTerm2]: no issue opened yet
* [tmux]: no issue opened yet
* [zellij]: no issue opened yet

Not on the list? Feel free to open a PR.

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

## Open Questions
None yet.

## Ⅰ. Dynamic Colors
* `OSC 10`: VT100 text foreground color
* `OSC 11`: VT100 text background color
* `OSC 12`: text cursor color
* `OSC 13`: pointer foreground color
* `OSC 14`: pointer background color
* `OSC 15`: Tektronix foreground color
* `OSC 16`: Tektronix background color
* `OSC 17`: highlight background color
* `OSC 18`: Tektronix cursor color
* `OSC 19`: highlight foreground color

[source][xterm-ctrlseqs]

## Ⅱ. Additional Links
* [xterm's Control Sequences][xterm-ctrlseqs]
* [Kitty's Terminal Protocol Extensions](https://sw.kovidgoyal.net/kitty/protocol-extensions/)
* [WezTerm's Escape Sequences](https://wezfurlong.org/wezterm/escape-sequences.html)
* [iTerm2's Escape Sequences](https://iterm2.com/documentation-escape-codes.html)


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
[xterm-ctrlseqs]: https://invisible-island.net/xterm/ctlseqs/ctlseqs.pdf
