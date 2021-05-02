# Rizm-Emulator
An emulator that can be used to run AddIns for the Casio Prizm Calculators in a variety of environments

## CLI-Options
Name | Description
--- | ---
-i {path} | The Path to the input file that should be loaded into the Emulator

## CLI-Commands
Name | Description
--- | ---
run | Runs the Program until it reached the end, an exception or breakpoint
b {address} | Sets a breakpoint at the given Address
step | Executes the next Instruction and then stops again
info reg | Prints the current contents of the Registers
info instr | Prints the current and next Instruction
info code | Prints all the Code loaded from the given File
info stack | Prints the current Stack
verbose {true|false} | Turns the Debug information on/off
