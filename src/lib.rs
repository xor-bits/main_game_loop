#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]
#![feature(box_patterns)]
#![feature(deadline_api)]
#![feature(duration_constants)]

//

use event::{CustomEvent, Event};

//

pub mod engine;
pub mod event;
pub mod report;
pub mod state;
pub mod update;
