# OpenModelica Modal Analyzer

Program logic:
- create `linearize.mos` script in temporary directory
  - `loadFile("modelname.mo");`
  - `linearize(modelname, stopTime=0.0);`
  - `getErrorString();` gets the error (optional)

- run `omc linearize.mos` in temporary directory
- parse A Matrix from `linearized_model.mo`
- compute Eigenfrequencies
- Display Results

TODO:
- format eigenfrequencies as table
- Saving of path inputs
- autocompletion for available models inside the modelica file?
- show if model file is valid immidiately on entering
