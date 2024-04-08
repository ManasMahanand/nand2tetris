@R0
D=M

@R1
D=D-M

@sum
M=0

@MULT_R0
D;JGE

// R1>R0
(MULT_R1)
  @R0
  D=M
  @EXIT
  D;JEQ

  @R1
  D=M
  @sum
  M=D+M
  @R0
  D=M
  M=D-1

  @MULT_R1
  0;JEQ

// R0>R1
(MULT_R0)
  @R1
  D=M
  @EXIT
  D;JEQ

  @R0
  D=M
  @sum
  M=D+M
  @R1
  D=M
  M=D-1
  
  @MULT_R0
  0;JMP

(EXIT)
  @sum
  D=M
  @R2
  M=D

(END)
  @END
  0;JMP
