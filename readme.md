# OpenModelica Modal Analyzer

This is a small GUI tool to compute the eigenfrequencies of a given Modelica Model file.

## Installation
At the moment I'm not providing prebuilt releases. You can easily compile your own version using [rustup](https://rust-lang.org/tools/install/) on all major operating systems.

```bash
cargo install --git "https://github.com/KwentiN-ui/om_modal"
```
This makes the application available as `om_modal` from your terminal.

## What the program does
- create `linearize.mos` script in temporary directory
  - `loadFile("modelname.mo");`
  - `linearize(modelname, stopTime=0.0);`
  - `getErrorString();` gets the error (optional)

- run `omc linearize.mos` in temporary directory
- parse A Matrix from `linearized_model.mo`
- compute Eigenfrequencies
- Display Results

## TODO
- [x] format eigenfrequencies as table
- [x] restore appstate
- [ ] autocompletion for available models inside the modelica file?
- [ ] show if model file is valid immidiately on entering
