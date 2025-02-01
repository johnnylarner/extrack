mod cmd;
mod track;

pub use cmd::Extrack;

use cmd::Action;
use track::run_track;

pub fn run(cmd: Extrack) {
    match cmd.action() {
        Action::Track => run_track(cmd.action()),
    }
}
