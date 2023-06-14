use crate::state::State;

pub trait Exec {
    fn exec(&self, state: &mut State);
}
