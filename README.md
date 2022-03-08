## Compiler
A compiler for simple toy functional language that allows to define functions, conditionals, and do math.
I kept things simple and hence all types are 32-bit floating point and everything is expression ending with a semicolon.
This compiler converts the source code into WebAssembly Text (`wat`) and binary WebAssembly (`wasm`) files.
Lexer, parser, AST, and generating code are hand-written.

## Requirement
I use `wat2wasm` to generate WebAssembly binary from WebAssembly Text, so `wat2wasm` should be installed.

## Examples
```
# Sum of first `n` integers
def sum(x) 
 if x == 1 
   then 1
   else sum(x-1) + x;
```
```
# Solution of a second
# order equation with coeffients a,b,c

def root1(a b c)
  if discr(a, b, c) < 0
  then 0 
  else (-b + sqrt(discr(a, b, c)))/(2*a);

def root2(a b c)
  if discr(a, b, c) < 0
  then 0 
  else (-b - sqrt(discr(a, b, c)))/(2*a);
```

## Formal definition

### Comments
Comments follows the symbol `#`

### Keywords
`def`, `if`, `then`, `else`

### Lexer
*Identifier* ::= *[a-zA-Z][a-zA-Z0-9]\**<br>
*Number* ::= [0-9]?(.?[0-9])

### Parser
*Program* ::= **def** *Prototype Expression* ; | **def** *Prototype Expression* ; *Program*<br>
*Expression* ::= *Exp* | *IfExp*<br>
*Exp* ::= *SubExp* | *Exp* **<** *SubExp* | *Exp* **>** *SubExp* | *Exp* **<>** *SubExp* | *Exp* **==** *SubExp*<br> 
*SubExp* ::= *Term* | *SubExp* **+** *Term* | *SubExp* **-** *Term* | *SubExp* **\|** *Term*<br>
*Term* ::= *Factor* | *Term* **\*** *Factor* | *Term* **/** *Factor* | *Term* **&** *Factor*<br>
*Factor* ::= -**Exp** | ( *Exp* ) | *Identifier* |  *Number* | *FuncionCall*<br>
*FuncionCall* ::= *Identifier*(*Args*) | *Identifier*()<br>
*Args* ::= *Exp* | *Comma* *Args*<br>
*IfExp* ::= **if** *Exp* **then** *Exp* **else** *Exp*<br>
*Prototype* ::= *Identifier*(*Params*) | *Identifier*()<br>
*Params* ::= *Identifier* *Params* | *Identifier*

### How to Run
`cargo run source.txt target.wat`
It'll generate two files, `target.wat` and `target.wasm`.
To load `target.wasm` and call exported functions we could use javascript and `node.js`.


For example, create a source file computing the n-th Fibonacci number
```
# Fibbonaci
def fib(x)
  if (x == 1) | (x == 2) 
    then 1 
    else fib(x-1) + fib(x-2);
```

Generate WebAssembly files `cargo run source.txt target.wat`. 
Next create a javascript file 
```
const { readFileSync } = require("fs");

const run = async () => {
  const buffer = readFileSync("./target.wasm");
  const module = await WebAssembly.compile(buffer);
  const instance = await WebAssembly.instantiate(module);
  console.log(instance.exports.fib(5));
};

run();
```
and then call `node run.js`.
