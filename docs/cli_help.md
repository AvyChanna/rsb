# Command-Line Help for `rsb`

This document contains the help content for the `rsb` command-line program.

**Command Overview:**

* [`rsb`↴](#rsb)
* [`rsb generate`↴](#rsb-generate)
* [`rsb validate`↴](#rsb-validate)
* [`rsb serve`↴](#rsb-serve)

## `rsb`

Create resumes from JsonResume structured data

**Usage:** `rsb <COMMAND>`

###### **Subcommands:**

* `generate` — generate resume from input
* `validate` — check input for errors
* `serve` — start a server for easy editing



## `rsb generate`

generate resume from input

**Usage:** `rsb generate <INPUT_PATH>`

###### **Arguments:**

* `<INPUT_PATH>` — file path for data



## `rsb validate`

check input for errors

**Usage:** `rsb validate <INPUT_PATH>`

###### **Arguments:**

* `<INPUT_PATH>` — file path for data



## `rsb serve`

start a server for easy editing

**Usage:** `rsb serve [ADDRESS]`

###### **Arguments:**

* `<ADDRESS>` — bind address for the server

  Default value: `127.0.0.1:8080`



