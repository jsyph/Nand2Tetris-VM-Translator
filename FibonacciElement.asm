// Translation of "../../8/FunctionCalls/FibonacciElement/"
// Generated by vm_translator (1.0.1)
// bootstrap code
@256
D=A
@SP
M=D
@Sys.init_RETURN
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@5
D=D-A
@ARG
M=D
@Sys.init
0;JMP
(Sys.init_RETURN)

// function Main.fibonacci 0
(Main.fibonacci)


// push ARG 0
@0
D=A                 
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1

// push constant 2
@2
D=A
@SP
A=M
M=D
@SP
M=M+1

// lt
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
D=M-D
@LESSTHAN_00B841BE8830478FB0CC48917D1D0977_TRUE
D;JLT
@LESSTHAN_00B841BE8830478FB0CC48917D1D0977
D=0
0;JMP
(LESSTHAN_00B841BE8830478FB0CC48917D1D0977_TRUE)
D=-1
(LESSTHAN_00B841BE8830478FB0CC48917D1D0977)
@SP
A=M
M=D
@SP
M=M+1

// if-goto N_LT_2
@SP
M=M-1
A=M
D=M
@N_LT_2
D;JNE

// goto N_GE_2
@N_GE_2
0;JMP

// label N_LT_2
(N_LT_2)

// push ARG 0
@0
D=A                 
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1

// return
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@ARG
D=M
@R13
M=D
@LCL
D=M
@SP
M=D
@SP
M=M-1
A=M
D=M
@THAT
M=D
@SP
M=M-1
A=M
D=M
@THIS
M=D
@SP
M=M-1
A=M
D=M
@ARG
M=D
@SP
M=M-1
A=M
D=M
@LCL
M=D
@SP
M=M-1
A=M
D=M
@R14
M=D
@R13
D=M+1
@SP
M=D
@R14
A=M
0;JMP

// label N_GE_2
(N_GE_2)

// push ARG 0
@0
D=A                 
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1

// push constant 2
@2
D=A
@SP
A=M
M=D
@SP
M=M+1

// sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1

// call Main.fibonacci 1
@Main.fibonacci_0D8FDF5C4F4F47748562A83EB8D945E5_RETURN
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@6
D=D-A
@ARG
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci_0D8FDF5C4F4F47748562A83EB8D945E5_RETURN)

// push ARG 0
@0
D=A                 
@ARG
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1

// push constant 1
@1
D=A
@SP
A=M
M=D
@SP
M=M+1

// sub
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=M-D
@SP
M=M+1

// call Main.fibonacci 1
@Main.fibonacci_77E7FF1739B245B4A7BF28280971D9C6_RETURN
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@6
D=D-A
@ARG
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci_77E7FF1739B245B4A7BF28280971D9C6_RETURN)

// add
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1

// return
@SP
M=M-1
A=M
D=M
@ARG
A=M
M=D
@ARG
D=M
@R13
M=D
@LCL
D=M
@SP
M=D
@SP
M=M-1
A=M
D=M
@THAT
M=D
@SP
M=M-1
A=M
D=M
@THIS
M=D
@SP
M=M-1
A=M
D=M
@ARG
M=D
@SP
M=M-1
A=M
D=M
@LCL
M=D
@SP
M=M-1
A=M
D=M
@R14
M=D
@R13
D=M+1
@SP
M=D
@R14
A=M
0;JMP

// function Sys.init 0
(Sys.init)


// push constant 4
@4
D=A
@SP
A=M
M=D
@SP
M=M+1

// call Main.fibonacci 1
@Main.fibonacci_A848C34432824B64BB07A0260961AEFC_RETURN
D=A
@SP
A=M
M=D
@SP
M=M+1
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
@SP
D=M
@6
D=D-A
@ARG
M=D
@Main.fibonacci
0;JMP
(Main.fibonacci_A848C34432824B64BB07A0260961AEFC_RETURN)

// label END
(END)

// goto END
@END
0;JMP

