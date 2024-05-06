/* 
* Documentation : https://github.com/samuelsleight/rust-handlers
*/

use crate::handler::{file_path, FileType};
/* 
* This defines a system struct, an object trait, and a handler trait for each defined handler in the system. 
* The system will have each signal as a method, which will call the appropriate slot for each object of 
* that handler type it contains
*/

/* 
* Documentation: https://docs.rs/model/latest/model/
*/
// Aims to reduce the boiler plate required to write model-based and linearizability tests.
use crate::models::parser_models::{
    AggregatorConfig,
    RateLimiter,
    Style,
};

/* 
* Documentation : https://docs.rs/mlua/0.9.7/mlua/
*/
use mlua::Lua;
/* 
* The mlua crate provides safe high-level bindings to the Lua programming language.
* The main type exported by this library is the Lua struct. In addition to methods 
* for executing Lua chunks or evaluating Lua expressions, it provides methods for 
* creating Lua values and accessing the table of globals.
*/

/* 
* Documentation : https://doc.rust-lang.org/std/
*/
use std::{
    collections::HashMap,
    fs,
    thread::available_parallelism,
};
/* 
* The Rust Standard Library is the foundation of portable Rust software, a set of minimal 
* and battle-tested shared abstractions for the broader Rust ecosystem. It offers core types, 
* like Vec<T> and Option<T>, library-defined operations on language primitives, standard macros, 
* I/O and multithreading, among many other things.
* std is available to all Rust crates by default. Therefore, the standard library can be accessed 
* in use statements through the path std, as in use std::env.
*/

/* 
* Documentaion : https://doc.rust-lang.org/rust-by-example/mod/struct_visibility.html
*/

/* 
* Structs have an extra level of visibility with their fields. The visibility defaults 
* to private, and can be overridden with the pub modifier. This visibility only matters 
* when a struct is accessed from outside the module where it is defined, and has the goal 
* of hiding information (encapsulation).
*/
pub struct Config {
    pub bindings_ip : String,
    pub debug : bool,
    pub logging_initialized : bool,
    pub threads : u8,
}

/* 
* Documentation : https://doc.rust-lang.org/std/keyword.impl.html#
*/

/* 
* The `impl` keyword is primarily used to define implementations on types. Inherent implementations 
* are standalone, while trait implementations are used to implement traits for types, or other traits.
* Functions and consts can both be defined in an implementation. A function defined in an impl block 
* can be standalone, meaning it would be called like Foo::bar(). If the function takes self, &self, 
* or &mut self as its first argument, it can also be called using method-call syntax, a familiar feature 
* to any object oriented programmer, like foo.bar().
*/
impl Config{
    pub fn parse(logging_initialized : bool) -> Result<Self, Box<dyn std::error::Error>>{
        /* 
        * pub fn parse(logging_initialized: bool) -> Result<Self, Box<dyn std::error::Error>>: 
        * This is the function signature for a function named parse. It takes a boolean argument 
        * logging_initialized and returns a Result. The Ok variant contains an instance of Self 
        * (which refers to the type this method belongs to, likely a Config struct), and the Err 
        * variant contains a boxed trait object implementing std::error::Error.
        */
        let lua = Lua::new();
        let globals = lua.globals();
        /* 
        * let lua = Lua::new();: Creates a new instance of the Lua interpreter.
        * let globals = lua.globals();: Retrieves a table representing the global environment in Lua.
        */
        lua.load(&fs::read_to_string(file_path(FileType::Config)?)?).exec()?;
        /* 
        * fs::read_to_string(file_path(FileType::Config)?): Reads the content of the config file 
        * into a string.
        * lua.load(&fs::read_to_string(file_path(FileType::Config)?)?): Loads the Lua code from the 
        * config file into the Lua interpreter.
        * .exec()?: Executes the loaded Lua code.
        */
        let parsed_threads: u8 = globals.get("threads")?;
        /* 
        * Retrieves the value associated with the key "threads" from the Lua global 
        * environment and tries to parse it as a u8. The ? is used to propagate any errors.
        */
        let debug: bool = globals.get("debug")?;
        let logging: bool = globals.get("logging")?;
        let adaptive_window: bool = globals.get("adaptive_window")?;    
    }
}