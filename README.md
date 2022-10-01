# BaseApp

Basic functions and utilities for productivity and well-being.

## Technologies:

- Tauri
- Rust

## How to run project localy:

Prerequisites:

- tailwindcss CLI
- cargo tauri CLI

**Frontend(optional)**
Run tailwind css file generation on change by runnin below command in `frontend` directory

`$ tailwindcss -o ./styles/output.css --watch`

**Run whole application (also activates above watch command)**
From root directory run

`$ cargo tauri dev`
