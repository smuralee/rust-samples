* Cargo detects if the code has changed and runs only the binary if no change is detected
* `cargo check` - Checks the code to make sure it compiles but does not create the executable
* `cargo build --release` - Compiles the code with optimizations
* Variables are immutable by default
    * `let foo = 5; //immutable`
    * `let mut bar = 5; //mutable` 
* References are also immutable by default
    * `&guess //immutable`
    * `&mut guess //mutable`
* The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, **we can change the type of the value but reuse the same name**.
    * ```
      let spaces = "   ";
      let spaces = spaces.len();
      ```
    * ```
      let spaces = "   ";
      spaces = spaces.len(); //throws an error - we are not allowed to mutate the variable type
      ```
* Signed and Un-signed integers are data types - e.g. i32 and u32...likewise
* The functions with return value must have an expression i.e. a statement with no semi-colon for the returning any value
    ```
    fn fn_with_params(param1: u32, param2: f64) -> f32 {
        println!("The params are: {} and {}", param1, param2);
        (param2 / param1 as f64) as f32
    }
    ```
    
    
    
