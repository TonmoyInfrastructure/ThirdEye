/* 
* Documentation : https://github.com/samuelsleight/rust-handlers
*/

use crate::handler::{file_path, FileType};
/* 
* This defines a system struct, an object trait, and a handler trait for each 
* defined handler in the system. The system will have each signal as a method, 
* which will call the appropriate slot for each object of that handler type it 
* contains.
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
        /* 
        * Similar to parsed_threads, these lines retrieve boolean values from Lua global variables.
        */   
        if !logging_initialized {
            set_logging_level(debug, logging);
        }
        /* 
        * If logging has not been initialized externally, it sets the logging level based on the 
        * retrieved debug and logging flags.
        */    
        let threads: u8 = if parsed_threads == 0 {
            let total_num_of_threads: usize = available_parallelism()?.get() / 2;
            log::error!(
                "Config Error: The value of `threads` option should be a non-zero positive integer"
            );
            log::error!("Falling back to using {} threads", total_num_of_threads);
            total_num_of_threads as u8
        } else {
            parsed_threads
        };
        /* 
        * Determines the number of threads to use. If parsed_threads is zero, it calculates the total number 
        * of threads available and logs an error message, falling back to using half of them. Otherwise, 
        * it uses the parsed value.
        */
        let rate_limiter: HashMap<String, u8> = globals.get("rate_limiter")?;
        /* 
        * Retrieves a Lua table representing the rate limiter configuration and tries to parse it 
        * into a HashMap<String, u8>.
        */
        let parsed_safe_search: u8 = globals.get("safe_search")?;
        let safe_search: u8 = match parsed_safe_search {
            0..=4 => parsed_safe_search,
            _ => {
                log::error!(
                    "Config Error: The value of `safe_search` option should be a non-zero positive integer from 0 to 4."
                );
                log::error!("Falling back to using the value `1` for the option");
                1
            }
        };
        /* 
        * Parses the safe search value from Lua and ensures it falls within a certain range. If not, it logs an 
        * error and falls back to 1.
        */
        #[cfg(any(feature = "redis-cache", feature = "memory-cache"))]
        let parsed_cet = globals.get("cache_expiry_time")?;
        #[cfg(any(feature = "redis-cache", feature = "memory-cache"))]
        let cache_expiry_time = match parsed_cet {
            0..=59 => {
                log::error!(
                    "Config Error: The value of `cache_expiry_time` must be greater than 60"
                );
                log::error!("Falling back to using the value `60` for the option");
                60
            }
            _ => parsed_cet,
        };
        /* 
        * Conditional compilation based on whether the features "redis-cache" or "memory-cache" are enabled. 
        * If enabled, retrieves and validates the cache expiry time. If it's less than 60, logs an error and 
        * falls back to 60.
        */
        Ok(Config {
            port: globals.get::<_, u16>("port")?,
            binding_ip: globals.get::<_, String>("binding_ip")?,
            style: Style::new(
                globals.get::<_, String>("theme")?,
                globals.get::<_, String>("colorscheme")?,
                globals.get::<_, Option<String>>("animation")?,
            ),
            /* 
            * Retrieves various configuration values and constructs a Style object.
            */
            #[cfg(feature = "redis-cache")]
            redis_url: globals.get::<_, String>("redis_url")?,
            /* 
            * Retrieves the Redis URL if the "redis-cache" feature is enabled.
            */
            aggregator: AggregatorConfig {
                random_delay: globals.get::<_, bool>("production_use")?,
            },
            logging,
            debug,
            adaptive_window,
            upstream_search_engines: globals
                .get::<_, HashMap<String, bool>>("upstream_search_engines")?,
            request_timeout: globals.get::<_, u8>("request_timeout")?,
            tcp_connection_keepalive: globals.get::<_, u8>("tcp_connection_keepalive")?,
            pool_idle_connection_timeout: globals.get::<_, u8>("pool_idle_connection_timeout")?,
            threads,
            rate_limiter: RateLimiter {
                number_of_requests: rate_limiter["number_of_requests"],
                time_limit: rate_limiter["time_limit"],
            },
            safe_search,
            #[cfg(any(feature = "redis-cache", feature = "memory-cache"))]
            cache_expiry_time,
        })
    }
}