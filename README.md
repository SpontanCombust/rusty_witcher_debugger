# Rusty Witcher 3 Debugger

A standalone Command Line Interface debugging tool for The Witcher 3 written in Rust.

This tool is intented for Witcher 3 modders who make mainly script based mods.
The main feature of it is to easily recompile game scripts at run time, which can greatly save time during development.

Parts of this code are based off of `Wolvenkit modding tool` by Traderain, rfuzzo and others
https://github.com/WolvenKit/WolvenKit


## Usage examples
Tool help
```ps1
rw3d_cli.exe -h
```

Recompile game scripts
```ps1
rw3d_cli.exe reload
```

Remotely call an exec function from the game
```ps1
rw3d_cli.exe exec additem('Aerondight', 1)
```

Remotely call an exec function from the game without waiting for tool messages or any game response
```ps1
rw3d_cli.exe --no-info-wait --no-listen exec gotoProlog()
```

Recompile game scripts and automatically exit the program after the tool doesn't get any responses from the game in the span of 5 seconds after the last response
```ps1
rw3d_cli.exe --response-timeout=5000 reload
```
