/// How the user identify to the server upon initial connection
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum UserIdent {
    /// online mode user
    Online {
        /// the homeserver domain
        homeserver: String,
        /// the user id on the homeserver
        uid: u64,
    },
    /// offline mode user
    Offline { label: String },
    /// local user (singleplayer)
    Local,
}

impl UserIdent {
    pub fn normalised(&self) -> UserIdentNormalised {
        let mut copied = self.clone();

        match &mut copied {
            Self::Online { homeserver, uid: _ } => *homeserver = homeserver.to_lowercase(),
            Self::Offline { label } => *label = label.to_lowercase(),
            Self::Local => {}
        }

        UserIdentNormalised(copied)
    }
}

/// How the user identify to the server upon initial connection
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct UserIdentNormalised(UserIdent);

/// display name for the user
#[derive(PartialEq, Eq)]
pub enum UserDisplay {
    Named {
        /// may contain upper/lower case characters
        label: String,
    },
    Unnamed,
}
