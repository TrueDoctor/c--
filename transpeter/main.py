import sys
import lexer

if __name__ == "__main__":
    if len(sys.argv) < 2:
        exit('Error: no input file specified')
    with open(sys.argv[1], 'r') as file:
        lex = lexer.Lexer(file.read())
    tokens = lex.tokenize()
    for token in tokens:
        size = len(str(token))
        print(str(token) + " " * (20 - size) + 'in line ' + str(token.line))
