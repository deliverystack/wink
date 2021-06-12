# wink

The wink command line tool lets you access hundreds of Windows applications and features with just a few keystrokes.

wink is written in rust using cargo and intended to be run from Windows cmd.exe and Windows Subsystem for Linux (WSL) bash.exe shells.

wink is a constant work in progress as its author(s) discover(s) more Windows features and how to access them and adds support for more third-party Windows applications.

wince is a ?nix shell script that you can use to build and run wink. 

You can compile wink for WSL both Windows and Linux, or you can use a single binary from both cmd.exe and bash.exe shells. From a bash.exe shell, you can invoke the Windows wink.exe or Linux wink binary directly. From a cmd.exe shell, you can use bash.exe with the -c argument to invoke the Linux binary.

bash.exe -c /path/to/wink
