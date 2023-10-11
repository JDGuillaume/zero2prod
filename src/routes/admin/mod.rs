mod dashboard;
mod newsletters;
mod password;

pub use dashboard::admin_dashboard;
pub use newsletters::*;
pub use password::*;

mod logout;
pub use logout::log_out;
