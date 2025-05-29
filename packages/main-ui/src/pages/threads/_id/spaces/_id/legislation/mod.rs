mod controller;
mod layout;
mod page;
mod summary;

pub(self) use controller::Controller as LegislationController;

pub use layout::*;
pub use page::*;
pub use summary::*;
