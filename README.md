# wink

WARNING: WINK WAS NOT WRITTEN BY ANYONE WITH ANY EXPERTISE IN RUST, WINDOWS, LINUX, OR SYSTEMS PROGRAMMING, AND IS LARGELY UNTESTED, ESPECIALLY ANY PARTS INVOLVING CONFIGURATION FILES. USE AT YOUR OWN RISK, NOTING THAT SOME COMMANDS MAY SHUT DOWN THE COMPUTER.

The wink command line tool lets you access hundreds of Linux and Windows applications and features with just a few keystrokes from both Windows Console and Windows Subsystem for Linux (WSL) shells. For a list, run wink with no command line arguments, which will show usage information, or see https://github.com/deliverystack/wink/blob/main/src/wsl/inv/invocablecategory.rs. You can use a JSON file to add your own categories to this list.

![wink command line usage](https://wslguy.files.wordpress.com/2021/06/image-14.png)

https://rustapopoulos.com/2021/06/24/anatomy-of-a-rust-program-part-i-wink-cargo-toml/

wink is written in rust using cargo and intended to be run from the Windows Console (cmd.exe), PowerShell, and Windows Subsystem for Linux (WSL) bash.exe shell windows.

wink is a constant work in progress as its author(s) learn about rust and discover(s) more Windows features and how to access them and add(s) support for more third-party Windows applications.

wince is a bash shell script that you can use to build and run wink binaries for both Windows and WSL.

```
//TODO: address hard-coding in wince
```

You can compile wink for WSL both Windows and Linux, or you can use a single binary from both cmd.exe and bash.exe shells. Other than testing, there is no reason to do either of the following, but from a bash.exe shell, you can invoke the Windows wink.exe or Linux wink binary directly, and from a cmd.exe shell, you can use bash.exe with the -c argument to invoke the Linux wink binary (or call wink bash /path/to/wink, which does the same thing).

```
bash.exe -c /path/to/wink
```

To get usage information, run wink with the -h (help) argument. In general, pass a command code as the first argument to wink (after optional arguments described below), followed by any arguments that you want to pass to the corresponding command.

To see what wink would do without actually doing it, such as to copy the command line that wink generates, use the -v (verbose) argument with the -d (dry run) argument.

For a JSON representation of wink's internal configurtion, use the -e (export) argument.

If you use the same JSON format in $HOME/.wink.json (WSL) or %USERPROFILE%\\wink.json (where .wink.json can be a symbolic link to wink.json), you can add and possibly override wink commands without updating the source code. BEWARE THAT THIS LOGIC HAS NOT BEEN WELL THOUGHT-THROUGH OR TESTED.

Wink has a few dependencies for regular expression processing, common type derivation, and JSON serialziation/deserialization (see Cargo.toml).

```
[dependencies]
regex = "1"
derive-new = "0.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Bash shell users may want to define this function to launch processes in the background, though any output to stdout or stderr is lost.

```
function win { nohup wink $@ > /dev/null 2>&1 & } 
```