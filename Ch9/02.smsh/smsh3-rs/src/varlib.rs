//! varlib.rs:  a simple storage system to store `name=value` pairs with
//!             facility to mark items as part of the environment
//!             
//! `VLenviron2table` and `VLtable2environ` are currently unavailable

#![allow(unused)]

/// Var: type to represent a shell variable
#[derive(Default)]
struct Var {
    name: String,
    value: String,
    global: bool,
}

/// VarTable: type to represent a shell variable table
pub struct VarTable {
    table: Vec<Var>,
}

impl VarTable {
    /// purpose:  find `name` in table
    ///
    /// action: iterate over the table to see if `name` exists
    ///
    /// arguments:
    ///     * `name`: shell variable name
    ///
    /// return: if `name` exists in the table, return mutable reference to that item.
    ///         Otherwise, return None
    fn find_item<'a, 'b>(&'a mut self, name: &'b str) -> Option<&'a mut Var> {
        for item in self.table.iter_mut() {
            if item.name == name {
                return Some(item);
            }
        }
        None
    }

    /// purpose: shell variable table initialization
    ///
    /// action: initialize an empty table(Vector)
    ///
    /// return: an empty table
    pub fn new() -> Self {
        Self { table: Vec::new() }
    }

    /// purpose: perform shell's set command
    ///
    /// action: iterate over the table, printing every item to stdout
    ///         if the variable is global, prefix it with `*`
    pub fn list(&self) {
        for item in self.table.iter() {
            if item.global {
                println!("* {}={}", item.name, item.value);
            } else {
                println!("  {}={}", item.name, item.value);
            }
        }
    }

    /// purpose: make a variable global
    ///
    /// action: make a variable for export, add it if not there
    ///
    /// arguments:
    ///     * `name`: variable name
    pub fn export(&mut self, name: &str) {
        if let Some(item) = self.find_item(name) {
            item.global = true;
        } else {
            self.table.push(Var {
                name: name.into(),
                value: String::new(),
                global: true,
            });
        }
    }

    /// purpose: return the value of name
    ///
    /// action: if found, return Some(value); otherwise, return None
    ///
    /// arguments:
    ///     *`name`: variable name
    ///     
    /// return: if found, return Some(value); otherwise, return None
    pub fn lookup(&self, name: &str) -> Option<String> {
        for item in self.table.iter() {
            if item.name == name {
                return Some(item.value.clone());
            }
        }

        None
    }

    /// purpose: store or update a variable
    ///
    /// action: if `name` already existed, update it; Otherwise, store it to the table
    ///
    /// arguments:
    ///     * `name`: variable name
    ///     * `value`: variable value
    pub fn store(&mut self, name: &str, value: &str) {
        if let Some(item) = self.find_item(name) {
            item.value = value.into();
        } else {
            self.table.push(Var {
                name: name.into(),
                value: value.into(),
                global: false,
            });
        }
    }
}
