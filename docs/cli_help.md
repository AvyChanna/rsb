# Command-Line Help for `rsb-cli`

This document contains the help content for the `rsb-cli` command-line program.

**Command Overview:**

* [`rsb-cli`↴](#rsb-cli)
* [`rsb-cli generate`↴](#rsb-cli-generate)
* [`rsb-cli validate`↴](#rsb-cli-validate)
* [`rsb-cli serve`↴](#rsb-cli-serve)

## `rsb-cli`

Create resumes from JsonResume structured data.
Input type is infered from file extension.
(Supports JSON/JSON5/JSONNET/YAML/RON)

**Usage:** `rsb-cli <COMMAND>`

###### **Subcommands:**

* `generate` — generate resume from input
* `validate` — check input for errors
* `serve` — start a server for easy editing



## `rsb-cli generate`

generate resume from input

**Usage:** `rsb-cli generate <INPUT_PATH>`

###### **Arguments:**

* `<INPUT_PATH>` — file path for data



## `rsb-cli validate`

check input for errors

**Usage:** `rsb-cli validate <INPUT_PATH>`

###### **Arguments:**

* `<INPUT_PATH>` — file path for data



## `rsb-cli serve`

start a server for easy editing

**Usage:** `rsb-cli serve [ADDRESS]`

###### **Arguments:**

* `<ADDRESS>` — bind address for the server

  Default value: `127.0.0.1:8080`



