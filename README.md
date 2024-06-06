# zj-quit 

A [zellij](https://zellij.dev/) plugin that prompts for confirmation before killing the current session.

![image](https://github.com/cristiand391/zj-quit/assets/6853656/0b2537c4-6872-402b-aa5d-f0713c46c32b)

This has been requested multiple times by users:
* https://github.com/zellij-org/zellij/issues/467
* https://github.com/zellij-org/zellij/issues/1229
* https://github.com/zellij-org/zellij/issues/3147

so I decided go the *zellij way* ™️ and made a plugin for this :)

## Usage

Download the last release available at https://github.com/cristiand391/zj-quit/releases/ and set up an alias for it:
```kdl
plugins {
  zj-quit location="file:/path/to/zj-quit.wasm"
}
```

https://zellij.dev/documentation/plugin-aliases

You can also configure the keybindings within the plugin:

```kdl
plugins {
  zj-quit location="file:/path/to/zj-quit.wasm" {
    confirm_key "q"
    cancel_key "Esc"
  }
}
```

Keys are referenced from: [zellij doc](https://docs.rs/zellij-tile/latest/zellij_tile/prelude/enum.Key.html)

Then set a keybind to launch it in a floating window:

```kdl
keybinds clear-defaults=true {
  shared_except "locked" {
    bind "Ctrl q" {
      LaunchOrFocusPlugin "zj-quit" {
        floating true
      }
    }
}
```
