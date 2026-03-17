pub enum ConnectRequest {
    /// this is after verifying with the homeserver
    OnlineVerified {
        homeserver: String,
        uid: u64,
        label: String,
    },
    Offline {
        label: String,
    },
    Local,
}
