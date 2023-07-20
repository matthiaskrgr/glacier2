// OK

#![register_tool(tool)]
#![feature(register_attr)]

#![feature(register_attr)]
#![register_attr(attr)]

use tool as renamed_tool; // OK
use register_attr as tool; //~ ERROR cannot use an explicitly registered attribute through an import

#[feature(register_attr)] //~ ERROR cannot use an explicitly registered attribute through an import
#[register_attr(attr)] // OK
                      // edition:2018
fn tool() {}
