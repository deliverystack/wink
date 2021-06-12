# wink

The wink command line tool lets you access hundreds of Windows applications and features with just a few keystrokes.

wink is written in rust using cargo and intended to be run from Windows cmd.exe and Windows Subsystem for Linux (WSL) bash.exe shells.

wink is a constant work in progress as its author(s) discover(s) more Windows features and how to access them and adds support for more third-party Windows applications.

wince is a bash shell script that you can use to build and run wink. 

You can compile wink for WSL both Windows and Linux, or you can use a single binary from both cmd.exe and bash.exe shells. From a bash.exe shell, you can invoke the Windows wink.exe or Linux wink binary directly. From a cmd.exe shell, you can use bash.exe with the -c argument to invoke the Linux binary.

`bash.exe -c /path/to/wink`

To get usage information, run wink with the -h (help) argument. In general, pass a command code as the first argument to wink (after optional arguments described below), followed by any arguments that you want to pass to the corresponding command.

To see what wink would do without actually doing it, such as to copy the command line that wink generates, use the -v (verbose) argument with the -d (dry run) argument.

For a JSON representation of wink's internal configurtion, use the -e (export) argument.

If you use the same JSON format in $HOME/.wink.json (WSL) or %USERPROFILE%\\wink.json, you can add and possibly override wink commands without updating the source code. 

`//TODO: try making .wink.json a symbolic link to wink.json`

Wink has a few dependencies for regular expression processing, common type derivation, and JSON serialziation/deserialization.

```
[dependencies]
regex = "1"
derive-new = "0.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

