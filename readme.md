# OpenModelica Modal Analyzer

This is a small GUI tool to compute the eigenfrequencies of a given Modelica Model file.

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
- [x] Restore appstate
- [ ] autocompletion for available models inside the modelica file?
- [ ] show if model file is valid immidiately on entering
