# Rusty Witcher 3 Debugger

A standalone Command Line Interface debugging tool for The Witcher 3 written in Rust.

This tool is intented for Witcher 3 modders who make mainly script based mods.
The main feature of it is to easily recompile game scripts at run time, which can greatly save time during development.

Parts of this code are based off of `Wolvenkit modding tool` by Traderain, rfuzzo and others
https://github.com/WolvenKit/WolvenKit

---


## Usage examples
Tool help.
```ps1
rw3d_cli.exe -h
```

Recompile game scripts.
```ps1
rw3d_cli.exe reload
```

Recompile game scripts and automatically exit the program after the tool doesn't get any responses from the game in the span of 10 seconds after the last response.
```ps1
rw3d_cli.exe --response-timeout=10000 reload
```

Remotely call an exec function from the game. Remember to use quotation marks when passing the argument if it has any spaces in it.
```ps1
rw3d_cli.exe exec "additem('Aerondight', 1)"
```

Remotely call an exec function from the game without waiting for tool messages or any game response.
```ps1
rw3d_cli.exe --no-wait --no-listen exec "gotoProlog()"
```

Get the list of mods installed.
```ps1
rw3d_cli.exe modlist
```

Monitor game's scripts log and highlight specific lines. You can set multiple key words to be highlighted with the same color.
```ps1
rw3d_cli.exe scriptslog --yellow="[My mod]" --yellow="[Also my mod]"
```