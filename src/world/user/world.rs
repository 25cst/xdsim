use std::collections::{HashMap, HashSet};

use crate::{
    common::world::{ComponentId, ComponentIdType},
    world::{
        layout,
        user::{
            self,
            ident::{UserDisplay, UserIdent, UserIdentNormalised},
            requests::ConnectRequest,
        },
    },
};

pub struct WorldState {
    /// internal layout state
    layout_state: layout::WorldState,
    /// list of all online users and their display names
    online_user_displays: HashMap<ComponentId, UserDisplay>,
    /// list of all users and ident every connected
    all_user_ids: HashMap<UserIdentNormalised, ComponentId>,
}

impl WorldState {
    /// send a sanitised user connect request to the world
    pub fn connect(&mut self, request: ConnectRequest) -> Result<ComponentId, Box<user::Error>> {
        let (ident, display) = match request {
            ConnectRequest::OnlineVerified {
                homeserver,
                uid,
                label,
            } => (
                UserIdent::Online { homeserver, uid },
                UserDisplay::Named { label },
            ),
            ConnectRequest::Offline { label } => (
                UserIdent::Offline {
                    label: label.clone(),
                },
                UserDisplay::Named { label },
            ),
            ConnectRequest::Local => (UserIdent::Local, UserDisplay::Unnamed),
        };

        let ident = ident.normalised();

        let user_id = self
            .all_user_ids
            .entry(ident.clone())
            .or_insert(self.layout_state.counter_mut().get(ComponentIdType::Player));

        match self.online_user_displays.get(user_id) {
            Some(_) => Err(user::Error::PlayerAlreadyOnline { ident }.into()),
            None => {
                self.online_user_displays.insert(*user_id, display);
                Ok(*user_id)
            }
        }
    }

    /// disconnect a user from world
    pub fn disconnect(&mut self, id: &ComponentId) -> Result<(), Box<user::Error>> {
        if self.online_user_displays.remove(id).is_some() {
            Ok(())
        } else {
            Err(user::Error::InvalidPlayerId { id: *id }.into())
        }
    }
}
