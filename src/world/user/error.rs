use crate::{
    common::{self, world::ComponentId},
    world::user::ident::{UserIdent, UserIdentNormalised},
};

pub enum Error {
    Common(Box<common::Error>),
    PlayerAlreadyOnline { ident: UserIdentNormalised },
    InvalidPlayerId { id: ComponentId },
}
