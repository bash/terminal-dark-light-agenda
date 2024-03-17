# X11 Colors

## Questions
* [Terminology]: [Responses to Queries for the Foreground and Background Color Use Different Format][terminology-issue]

[terminology]: https://git.enlightenment.org/enlightenment/terminology
[terminology-issue]: https://git.enlightenment.org/enlightenment/terminology/issues/14

## Resources
* [Color Strings](https://www.x.org/releases/current/doc/libX11/libX11/libX11.html#Color_Strings)
* [X11 Built-in Color Names](https://gitlab.freedesktop.org/xorg/xserver/blob/master/os/oscolor.c)
* [rgb.txt](https://gitlab.freedesktop.org/xorg/app/rgb/-/blob/master/rgb.txt) (no longer used by the X11 server as of this [commit](https://gitlab.freedesktop.org/xorg/xserver/-/commit/dda10c9066a660b647384179f82e1da8e063264f))
* Wikipedia: [X11 Color Names](https://en.wikipedia.org/wiki/X11_color_names)
* tmux's Implementation of [parsing an X11 color](https://github.com/tmux/tmux/blob/b79e28b2c30e7ef9b1f7ec6233eeb70a1a177231/colour.c#L965)

## Accepted Formats
Survey of terminal support (parsing) for various color formats for use with `OSC 10` and friends.

| Terminal                              | `rgb:<r>/<g>/<b>` | `#<r><g><b>` | named    | `rgbi:<r>/<g>/<b>` | additional                                              |
|---------------------------------------|-------------------|--------------|----------|--------------------|---------------------------------------------------------|
| [hterm][hterm-src]                    | yes               | yes          | yes      | no                 |                                                         |
| [xterm.js][xterm.js-src]              | yes               | yes          | no       | no                 |                                                         |
| [iTerm2][iterm2-src]                  | yes               | yes          | no       | no                 |                                                         |
| [Alacritty][alacritty-src]            | yes               | yes          | no       | no                 |                                                         |
| [Contour][contour-src]                | yes [^1]          | yes [^2]     | no       | no                 |                                                         |
| vte                                   | yes               | TODO         | yes [^3] | TODO               |                                                         |
| [Konsole][konsole-src] using [QColor] | no [^4]           | yes          | yes [^3] | no                 |                                                         |
| QTerminal                             | no                | no           | no       | no                 |                                                         |
| [foot][foot-src]                      | yes [^5]          | yes [^6]     | no       | no                 |                                                         |
| xterm                                 | yes               | yes          | yes      | yes                |                                                         |
| [WezTerm][wezterm-src]                | yes [^5]          | yes          | yes      | no                 | `hsl:...`, [css colors], `transparent`, `none`, `clear` |
| [kitty][kitty-src]                    | yes               | yes          | yes      | no                 |                                                         |
| [Rio][rio-src]                        | yes               | yes          | no       | no                 |                                                         |
| [rxvt-unicode][rxvt-src]              | yes               | yes [^6]     | yes      | yes                |                                                         |
| mrxvt                                 | no                | no           | no       | no                 |                                                         |
| Eterm                                 | no                | no           | no       | no                 |                                                         |
| anyterm                               | no                | no           | no       | no                 |                                                         |
| [Terminology][terminology-src]        | yes               | yes          | no       | yes                |                                                         |
| [tmux][tmux-src]                      | yes               | yes          | yes      | no                 | `cmyk:..`, `cmy:...`                                    |

## Emitted Formats
| Terminal                 | `rgb:<r>/<g>/<b>` | `rgba:<r>/<g>/<b>/<a>` |
|--------------------------|-------------------|------------------------|
| [rxvt-unicode][rxvt-src] | yes               | yes                    |

[^1]: Only 8-bit (i.e. two hex digits) per channel supported (e.g. `rgb:fe/fe/fe` but not `rgb:f/f/f` or `rgb:fee/fee/fee`)
[^2]: Only one or two hex digits per channel supported (e.g. `#fff` or `#fefefe` but not `#feefeefee`)
[^3]: Refers to the [SVG color keyword names], not the X11 list.
[^4]: Note that it still reports the color in the `rgb:<r>/<g>/<b>` format when queried.
[^5]: In addition, colors with alpha are supported i.e `rgba:<r>/<g>/<b>/<a>`.
[^6]: In addition, colors with alpha are supported i.e. `[aa]#<r><g><b>`.

[hterm-src]: https://chromium.googlesource.com/apps/libapps/+/HEAD/libdot/js/lib_colors.js#175
[xterm.js-src]: https://github.com/xtermjs/xterm.js/blob/9ec9dca5f8ca8e1f107f7cf4c8a545672e8f69c4/src/common/input/XParseColor.ts#L23
[iterm2-src]: https://github.com/gnachman/iTerm2/blob/691fd5dd8c7dd7606becee320ece1648152af6c0/sources/VT100Terminal.m#L3729
[alacritty-src]: https://github.com/alacritty/vte/blob/ed51aa19b7ad060f62a75ec55ebb802ced850b1a/src/ansi.rs#L184
[contour-src]: https://github.com/contour-terminal/contour/blob/521b1408600951b63b285ff459f6fc6e9fbf6806/src/vtbackend/Color.cpp#L132
[konsole-src]: https://invent.kde.org/utilities/konsole/-/blob/0880a2137be8907ec06ba96918753735790c02fc/src/session/Session.cpp#L617
[QColor]: https://github.com/qt/qtbase/blob/e146d835a69d57748bf2978cf5134ac5d86d81cf/src/gui/painting/qcolor.cpp#L980
[SVG color keyword names]: https://www.w3.org/TR/SVG11/types.html#ColorKeywords
[foot-src]: https://codeberg.org/dnkl/foot/src/commit/5f41eb798b639774d5cb2a7656fbaf4c61a16352/osc.c#L711
[wezterm-src]: https://github.com/wez/wezterm/blob/889f8a9cd71a2b3552f28f6d1864aa3cd9461fdf/color-types/src/lib.rs#L657
[css colors]: https://docs.rs/csscolorparser/latest/csscolorparser/
[kitty-src]: https://github.com/kovidgoyal/kitty/blob/3c19b6f734349249c014c97324011217eae63867/kitty/rgb.py#L60
[rio-src]: https://github.com/raphamorim/rio/blob/be139e9e847d4c967086a88dde951a32c2464aed/rio-backend/src/performer/handler.rs#L39
[rxvt-src]: http://cvs.schmorp.de/rxvt-unicode/src/command.C?view=markup#l3440
<!-- rxvt source code hint: look at process_color_seq -->
[tmux-src]: https://github.com/tmux/tmux/blob/b79e28b2c30e7ef9b1f7ec6233eeb70a1a177231/colour.c#L965
[terminology-src]: https://git.enlightenment.org/enlightenment/terminology/src/commit/3c967f3379b71e6c563e917784afe96470b75259/src/bin/termptyesc.c#L4022
