> This project is under development

# Yet Another Interactive Shell

I tried all kinds of interactive shells available on market.
You know what, it's hard to say.

I'm going to make a state-of-art shell, with intuitive interacting
and powerful general purpose design pattern. 

## Getting Started

To get this piece of art running, 
```shell
cargo run
```

Or for those who don't know rust very well, just download
the latest snapshot build from 
[my CI](https://build.zhufucdev.com/job/yaish/).

## Syntax

Yaish doesn't support bash syntax, in place of which is a
reimagined scripting language called yass (yet another shell script).

To know more about yass, go to [SYNTAX.md](SYNTAX.md)

## Design Pattern
Yaish comes with a very different design and interacting pattern than most
other interactive shells.

### CLI Mode

In CLI mode, the shell is interactive, which means it's got auto-completion,
command history, suggestions, typo fix, stylization and many other things you
could possibly imagine.

Yass, however, is not available in CLI mode. It doesn't mean it's got no marcos,
instead, the shortcuts are well-designed and more powerful, but it's just
I think too complicated and inconvenient to write automation in a line-based
command prompt.

#### Auto-completion

The completion suggestions stay sticky to the screen bottom, each ending with
a number indicator.

It's basically an input method logic, where you choose items to fill in within
number keys row.

#### Suggestions

The first option of auto-completion is sticky to the end of line, which you
press tab or right arrow key to fill in.

### Scripting Mode

Scripting and macros make things easy. However, bash syntax, in my point of view,
is a mess.

Here's an example of the yass scripting language.
```yass
literal('pacman -##') {
    where '##' selects 'D', S', 'Q', 'R'
}

literal('pacman -S##) {
    where '##' contains 's', 'y', 'q', 'w', 'c', 'g' // ...
}

literal('pacman -S ##') {
    where '##' selects cached(cache_control: max_age(10min)) {
        entries cli('pacman -Ssq')
    }
}
```