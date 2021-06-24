# What Color is Your Terminal's Background?

I have been using a light background for my terminal for a bit now. For reasons
that are beyond the scope of this (and that, honestly, I don't fully grok),
sometimes programs either assume a dark background or interpret my terminal's
color palette incorrectly...I don't know. Whatever the reason, every so often I
run some program where the output just isn't readable on my background.

Additionally, sometimes I want a _dark_ background for my terminal, but then I
have to reset all the colors for my prompt, etc., which gives me a sad.

This got me thinking about finding or building a utility that could not only
report a terminal's background color, but also make a judgment about whether the
color is "light" or "dark" that I could use to automatically set things
appropriately.

(I realized after I was way too deep into this that there _are_ other programs
that tell you exactly this--or at least the color--but I wanted to finish it
anyway.)


## Yeah, okay, what does it do?

Glad you asked. Here are some screenshots.

My normal terminal colors:

<img
    alt="A light-colored background, showing the output of 'termbg' and 'bgstyle' commands"
    src="https://github.com/Crosse/termbg/blob/main/assets/light.png"
    width="640">

...and now dark colors (also witness the fail that is my prompt, which simply
believes in its heart of hearts that the terminal is set up for light colors):

<img
    alt="A dark-colored background, showing the output of 'termbg' and 'bgstyle' commands"
    src="https://github.com/Crosse/termbg/blob/main/assets/dark.png"
    width="640">

## Alright, maybe that's neat. where do I get it?

Right now, you have to build the code yourself. It requires Rust. (Sorry. In the
future there will be binaries here to download.)

Clone the repo and run `cargo install --path .`, like so:

    $ cargo install --path .
      Installing termbg v0.1.0 (/Users/seth/code/mine/termbg)
        Updating crates.io index
        Finished release [optimized] target(s) in 1.21s
      Installing /Users/seth/.cargo/bin/bgstyle
      Installing /Users/seth/.cargo/bin/termbg
       Installed package `termbg v0.1.0 (/Users/seth/code/mine/termbg)` (executables `bgstyle`, `termbg`)

You now have two new commands: `bgstyle` and `termbg`. The first one tells you
if your background is "dark", "light", or "unknown". The second gives you the
color code in hex format, à la HTML and a billion other things.


## My dude, couldn't that have been a single binary with command-line switches?

Yes.


## Why Rust?

I like Rust.


## Yeah, but those binaries are, like, half a meg each!

Are we really trying to save a few hundred kilobytes?

_sighs_

Alright, then tweak and use the C version instead:

    $ make
    cc    -c -o main.o main.c
    cc -o "termbg" "main.o"

    $ ./termbg
    rgb:f8f8/f2f2/e5e4
    r: f8, g: f2, b: e5
    r: 248, g: 242, b: 229
    HSP: 242.37
    this seems like a light color

It's only 13KiB (on my machine). Knock yourself out!


## Does this work on macOS? Linux?

Yes, and theoretically? Thus far, I've tested it in the following environments:

| OS    | Application         | local/ssh?                      | tmux?              | tmux in ssh?       |
| ----- | ------------        | ------------------              | ------------------ | ------------------ |
| macOS | Terminal.app        | :heavy_check_mark:              | :heavy_check_mark: | :heavy_check_mark: |
| macOS | iTerm.app           | :heavy_check_mark:              | :heavy_check_mark: | :heavy_check_mark: |
| macOS | xterm (via XQuartz) | :x: (see note 1)                | (untested)         | (untested)         |
| macOS | urxvt (via XQuartz) | :heavy_check_mark: (see note 2) | :heavy_check_mark: | :heavy_check_mark: |
| macOS | Alacritty           | :x: (see note 1)                | (untested)         | (untested)         |
| macOS | Kitty               | :x: (see notes 1, 3)            | (untested)         | (untested)         |

(The SSH session tested was to a Linux machine, so the programs _run_, but I
have not yet had a chance to test the various Linux/*nix terminals directly.)

Notes
1. The C version reports the correct information, so this is a bug in the Rust implementation.
2. `urxvt` on macOS had `$TERM` set to `xterm-color`, not `rxvt-unicode`.
3. Kitty spits out part of the terminal's response to stdout, so there may be a
   timing issue involved.


## Does this work for Windows?

No. Or at least, not for the native Windows command- and PowerShell
prompts. (For PowerShell, there are going to be much easier ways to get this
information, I'm sure.) It could possibly work for Cygwin shells or WSL
terminals, but I haven't tested that.

Pull requests welcome!
