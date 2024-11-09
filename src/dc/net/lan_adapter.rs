use crate::prelude::*;

extern "C" {
   pub fn la_init() -> c_int;
   pub fn la_shutdown() -> c_int;
}
