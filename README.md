# Hello World

## Instructions

Refactor the hello world application to a better architecture. This means that the code is set up in a way that you can add routes easily without them all being in the same file/function.

This means that the main file should only be used to call the server. The router, and routing functions should be defined elsewhere. It's okay to have the main function create a config object and pass that into wherever you run the server.

Make sure that everything is working by running the tests with the command `check.out`. The output for the checks will be put into the `check.out` file. Please note that even if this check is passing it's still possible to not pass the assessment.

## Grading Rubric

### Check script passing

- Server still returns "Hello world"
- Code is formatted
- Code passes Clippy lint

### Manual check

- Axum app is defined in a file other than main.rs
