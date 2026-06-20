# Sasha
Somtimes I get lost where all my windows are due to the infinite-nature using the [niri tiling manager](https://github.com/niri-wm/niri)... This daemon attempts to enhance the workflow by providing QOL features, such as marking and focusing windows (like [harpoon nvim](https://github.com/theprimeagen/harpoon))

## Configuration
Sasha makes use of its own cargo command, `cargo sasha`, to send and interpret the user's requested intent. To use the functionality, include the following keybindings in the `binds` section of your niri's configuration file:

```
binds {
    // === SASHA CONFIG ===
    Super+Shift+1 { spawn "cargo-sasha" "mark-window" "1"; }
    Super+Shift+2 { spawn "cargo-sasha" "mark-window" "2"; }
    Super+Shift+3 { spawn "cargo-sasha" "mark-window" "3"; }
    Super+Shift+4 { spawn "cargo-sasha" "mark-window" "4"; }
    Super+Shift+5 { spawn "cargo-sasha" "mark-window" "5"; }
    Super+Shift+6 { spawn "cargo-sasha" "mark-window" "6"; }
    Super+Shift+7 { spawn "cargo-sasha" "mark-window" "7"; }
    Super+Shift+8 { spawn "cargo-sasha" "mark-window" "8"; }
    Super+Shift+9 { spawn "cargo-sasha" "mark-window" "9"; }

    Super+1 { spawn "cargo-sasha" "focus-window" "1"; }
    Super+2 { spawn "cargo-sasha" "focus-window" "2"; }
    Super+3 { spawn "cargo-sasha" "focus-window" "3"; }
    Super+4 { spawn "cargo-sasha" "focus-window" "4"; }
    Super+5 { spawn "cargo-sasha" "focus-window" "5"; }
    Super+6 { spawn "cargo-sasha" "focus-window" "6"; }
    Super+7 { spawn "cargo-sasha" "focus-window" "7"; }
    Super+8 { spawn "cargo-sasha" "focus-window" "8"; }
    Super+9 { spawn "cargo-sasha" "focus-window" "9"; }
}
```

## Roadmap
WIP
