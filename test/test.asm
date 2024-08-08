// Push value from constant at 10 to the stack.
@10
D=A
@SP
A=M
M=D
@SP
M=M+1
// Push value from constant at 9 to the stack.
@9
D=A
@SP
A=M
M=D
@SP
M=M+1
// Arithmetic instruction: gt.
@SP

        M=M-1
        
D=M
        
@R6
        
M=D
        
@SP
        
M=M-1
        
D=M
        
@R5
        
M=D
        
@R5
        
A=M
        
D=M
        
@R6
        
A=M
        
D=D-M
        
@RESULT_TRUE_1
        
D;JGT
        
(RESULT_FALSE_1)
        
@R7
        
M=0
        
@RESULT_1
        
0;JMP
        
(RESULT_TRUE_1)
        
@R7
        
M=-1
        
(RESULT_1)
        
@R7
        
D=M
        
@SP
        
A=M
        
M=D
        
@SP
        
M=M+1
