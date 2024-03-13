# Ralph

Ralph is an experimental SAPI for PHP implemented in Rust. It uses our own internal [libphp](./libphp) bindings to create the SAPI and can be used in the terminal to execute PHP code.

> Ralph is only concerned with executing PHP code in the terminal for now. This will likely change in the future, allowing you to run full PHP web applications with Ralph and optionally transpile PXP code at runtime.

## Usage

After compiling Ralph from source, you can use the following commands to execute and inspect PHP code.

```sh
ralph run ./example.php
```