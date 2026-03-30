# Application Tasks

## Task 1: Write a program to print binary package names from debian/control

The program takes path to debian/control as the first command line argument.
Run with `cargo run --release -- <path to debian/control>`.

## Task 2: Check how many times lintian-brush parses the same files on a sample package

I chose to profile the `file` package. The following describes the steps I took
to figure out the amount of time same packages are parsed (the steps assume all
needed dependencies are installed). Tested on Debian 13.

Get sources of file package and initialize git repository (lintian-brush
requires a git repository with no pending changes, since it automatically makes
commits for the changes it makes).

```
mkdir file
cd file
apt-get source file
cd file-5.46
git init
git add .
git commit -m "First commit"
```

Now it's possible to run lintian-brush on the repository. It is notable that
once lintian-brush has been run once, it has already made the necessary changes,
so profiling subsequent runs might not give the same results.

lintian-brush seems to be a collection of smaller scripts where each scripts
checks and fixes a specific thing. I'm assuming that each script opens the
required file(s), parses the contents and closes the file. Therefore, I'm using
strace to look for the `open` family of syscalls. It's possible that some of the
files are also opened for reasons other than parsing, or that the file contents
are stored in memory and parsed multiple times without re-opening the
corresponding file.

With some testing, it seems that opening files are done specifically with the
`openat` syscall. Running strace and grepping the results for `debian`, shows a
bunch of `openat` calls with arguments to open the debian package files. For
example in this case running the following command gives 18 entries.

`strace --trace=openat lintian-brush &>/dev/stdout | grep debian/control`


## Task 3: Find a few fixers from lintian-brush that operate on the same files

**TODO**
