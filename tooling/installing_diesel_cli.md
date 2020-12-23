# Installing Diesel CLI
This is just a small note on installing the Diesel CLI on Windows, since it took a bit to figure out and wasn't documented well.

## Relevant Links
- https://github.com/diesel-rs/diesel/issues/487
- https://www.reddit.com/r/rust/comments/65fo0q/installing_diesel_cli_on_windows_10_woes/
- https://gist.github.com/zeljic/d8b542788b225b1bcb5fce169ee28c55

## Building `sqlite3.lib`
Download the SQLite amalgamation source code and binary from [SQLite's download page](https://www.sqlite.org/download.html).

Extract both to a single folder.

Open the Developer Command Prompt for VS 2019 in that directory.

Run `lib /DEF:sqlite3.def /OUT:sqlite3.lib /MACHINE:x64`.

## Creating a PATH Directory For Libraries
Make a new folder somewhere in your system - ie. `C:\Libs`.

Copy `sqlite3.lib` from the folder you ran the command in to your libs directory.

Add your libs directory to your PATH variable, and restart if necessary.

## Install diesel_cli
Using a BASH terminal (ie. [Git BASH](https://gitforwindows.org/)), export the `SQLITE3_LIB_DIR` environment variable as the path to your libs dir.
For example: `export SQLITE3_LIB_DIR="C:\\Libs"`

Then, run the `install` command:

`cargo install diesel_cli --no-default-features --features sqlite`

## Notes
It's entirely possible one or more of these steps are unnecessary - this doc exists just as a note of what I did to get it to work, for future reference.
