# Basic Example

Some things that would normally be in .gitignore are intentionally left in to show you an idea for the project structure.

# Steps to create

## Install

```bash
cargo install esetres
```

## Initialize

```bash
esetres init
```

## Create batch file for launch (Optional)

You can create the `startup.bat` file if you need to launch the server from outside of the terminal.

## Start

```bash
esetres start
```

## Test

Test if the server is working by running

```bash
curl http://localhost:3000/health
```
