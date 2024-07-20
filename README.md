# Custom_Language_Interpreter


## Interpreter.rs

1. **Imports and Dependencies**

- `use crate::parser::{Expr, Op}` : imports the `Expr` and `Op` types from the parser module.
- `use std::collections::HashMap;` : imports the `HashMap` type from the standard collection type i.e used to store variable name and associated values.

2. **Interpreter Struct**

- Defines a strucct, which is a custom datatypes that comtains multiple fields to store variable and associated values.
- `variables: HashMap<String,i64>`: stores variables in the form of a `HashMap`, where the key is a `String` (the variable name) and the value is an `i64` (the variable's value).

3. **Implementation of Interpreter Struct**
- `new` function is a constructor method which creates a new `Interpreter` instance with an empty `HashMap`.

4. **Interpret Method**
-  The `interpret` function method evaluates an expression. It returns an `Option<i64>` where `Some(value)` is result of expression or `None` if there is error.
- `match` statement is used to match Numbers, variables and operations.
- `assign` method assigns a value i.e it inserts or updates the variable  in `HashMap`.

## Parser.rs
- Parser processes token and constructs an abstract syntax tree of expressions.

1. **Imports**
- `use crate::lexer::Token;` : imports the `Token` type from the lexer.

2. **Enums for Expressions and Operators**
- `Expr` enum represents different types of expression i.e `Varibale`, `Number`, `BinaryOp` and `Assign`
- `Op` enum represents different types of operators.

3. **Parser Struct**
- `pub struct Parser<'a>`: Defines a Parser struct that holds the lexer and the current token being processed.
- It has `lexer: Lexer<'a>` which  provides token for the parser and `cuurent_token` is the token currently being examined by the parser.

4. **Assignment Parsing**
- `fn assignment(&mut self) -> Option<Expr>` handles assignment expression.
- It ccheck if the current token is `Assume` if true it moves past and ensure the next tokenn is an identifier i.e a variable name and then moves past the identifier then check if next token is `eq` and then creates `Expr::Assign` node.

5. **Addition and Subtraction Parsing**
- It parses addition and subtraction.
- Starts by parsing a multiplication or division expression if present
- Then it checks if the next token is `plus` or `minus` to handle additional operations.
- Constructs `Expr::BinaryOp` nodes for each operation and updates `left` with the result.

6. **Multiplication and Division parsing**
- parses multiplication and division.
- Starts by parsing a factor with `self.factor()`
- Checks for  `Token::Into` or `Token::By`
- Includes a check for division by zero and prints an error message.
- Constructs `Expr::BinaryOp` nodes for each multiplication and division operation.

7. **Factor Parsing**
- Parses factor which can be `Token::Number`,`Token::Identifier`.

8. **Advance to Next Tokenn**
- `fn next_token(&mut self)` advances the parser to next token by calling `next_token()` on the lexer.

## Lexer.rs

1. **Token Enum**
- Represents different types of tokens that a lexer can produce.
- `#[derive(Debug, PartialEq, Clone)]` allows tokens to be printed for debugging, compared for equality and cloned.

2. **Lexer Struct**
- holds the state of the lexer.
- `input` reference to the input string to be tokenized.
- `pos` current position of input string.

3. **Lexer Implementation**
- To implement the custom operators.

4. **new Method**
- `pub fn new(input: &'a str) -> Self`: Creates a new lexer instance with the given input string and initializes the pos to 0.

5. **Helper Method**
- `skip_whitespace` method: Skips over any whitespace characters by advancing the position until a non-whitespace character is encountered.
- `current_char` method: Returns the current character at the position pos in the input string.
- `advance` method: Moves the position pos forward by the number of bytes in the current character.
- `matches` method: Checks if the input starting at the current position matches a given keyword. If it matches, the position is advanced by the length of the keyword.
- `number` method: Parses a sequence of digits and returns a Token::Number with the parsed integer value
- `identifier` method: Parses a sequence of alphanumeric characters and underscores to form a variable name, returning a `Token::Identifier`.

## Main.rs

1. **Get the Filename from Command Line Arguments**
   ```rust
   let args: Vec<String> = env::args().collect();
   if args.len() != 2 {
       eprintln!("Usage: {} <file.kr>", args[0]);
       process::exit(1);
   }
   let filename = &args[1];
   ```
   - `env::args().collect()`: Collects command-line arguments into a `Vec<String>`.
   - Checks if exactly two arguments are provided (the program name and the filename).
   - If not, prints a usage message and exits with a status code of `1` to indicate an error.
   - Otherwise, stores the filename in the variable `filename`.

2. **Read the File Contents**
   ```rust
   let input = match fs::read_to_string(filename) {
       Ok(content) => content,
       Err(err) => {
           eprintln!("Error reading file: {}", err);
           process::exit(1);
       }
   };
   ```
   - `fs::read_to_string(filename)`: Attempts to read the file specified by `filename` into a string.
   - If successful (`Ok(content)`), assigns the file contents to `input`.
   - If an error occurs (`Err(err)`), prints an error message and exits with status code `1`.

3. **Initialize the Lexer, Parser, and Interpreter**
   ```rust
   let lexer = Lexer::new(&input);
   let mut parser = Parser::new(lexer);
   let expr = match parser.parse() {
       Some(expression) => expression,
       None => {
           eprintln!("Failed to parse the input");
           process::exit(1);
       }
   };
   ```
   - Creates a new `Lexer` instance with the input string.
   - Creates a new `Parser` instance, initialized with the lexer.
   - Calls `parser.parse()` to parse the input into an `Expr`.
   - If parsing fails (`None`), prints an error message and exits with status code `1`.

4. **Interpret the Expression**
   ```rust
   let mut interpreter = Interpreter::new();
   if let Some(result) = interpreter.interpret(expr) {
       println!("Result: {}", result);
   } else {
       eprintln!("Failed to interpret the expression");
   }
   ```
   - Creates a new `Interpreter` instance.
   - Calls `interpreter.interpret(expr)` to evaluate the parsed expression.
   - If interpretation is successful (`Some(result)`), prints the result.
   - If interpretation fails (`None`), prints an error message.
