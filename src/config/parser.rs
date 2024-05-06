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
}