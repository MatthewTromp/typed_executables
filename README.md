# Typed executables
This is a (currently very early prototype) for typed executables.
It currently consists of a basic shell and two programs: `ls` and `cat`.

## What are typed executables?
The idea behind typed executables is that, when starting an executable, string arguments are not a very good interface for passing information.
This project is an effort to develop better interfaces for transmitting information:
using file descriptors instead of paths to pass file arguments,
using structured binary data instead of strings for everything,
clearly enumerating arguments and flags in a machine-readable way to enable smarter autocompleting and input-validating shells,
etc.

Some of this work has already been explored or completed. Examples that I know about so far include
- Principle of least authority shell
- Powershell (passing dotnet objects between executables)
- Capsicum (enabling capabilty passing for linux)
If you are aware of other examples of similar or relevant work, please tell me about it!

## Shell
So far, the shell is a very basic shell.
By default, programs are run as though they are typed.
Currently, that means passing all file arguments as file descriptors.

When using a typed executable, prefix any files arguments with a single quote (').
These arguments will be opened as file descriptors and passed down to the execuable that way,
while all other arguments are passed as normal string arguments.

For instance, to list all files in the current working directory with the typed version of `ls` included in this repository, you would write
```
$ typed_executables/bin/ls '.
```

To execute a non-typed executable,
prefix its name with a single quote and pass all its arguments as normal.

For instance, to run the normal version of ls on the directory `/home/me/Documents`, write
```
$ 'ls /home/me/Documents
```

## File descriptor passing
When running a typed executable, all file arguments (those prefixed with ') are isolated and opened by the shell itself.
The first argument to the executable is the number of file arguments,
then the next n arguments are the file descriptors of the file arguments,
and the remaining non-file arguments are passed after that.

The number of file descriptors and the file descriptors themselves are passed as hex because I am opinionated.

For instance, if we run the command
```
$ my_typed_program 'path/to/file1.txt a_string_argument 'another/path/file2.rs --some_flag
```
the shell first opens the two files (say they recieve file descriptors `4` and `27 = 0x1b`) and then calls the program like so:
```
my_typed_program 2 4 1b a_string_argument --some_flag
```
Note that if you pass no files a typed executable, the first argument to the executable will be 0 (the number of file descriptor arguments).
This is why you need to prefix non-typed executables with ', so that the shell knows to not do this.


