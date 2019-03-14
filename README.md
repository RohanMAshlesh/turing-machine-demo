# turing-machine-demo
A demo of a universal turing machine in Rust

The input consists of a comma separated list of transitions. The states are inferred from the transitions.

Eg - 001R0,010R0,0BBL1,101L2,200L2,211L2,2BBR5,110L3,310L3,301L2

These are the transitions for a turing machine to get the 2's complement of a binary number.

Instruction format-
[ current state : input : output : 'L' or 'R' : next state ]

Current and next states are unsigned integers; input and output are characters.

