use std::collections::HashSet;

pub fn is_torrent_client_allowed(
    peer_id: &[u8; 20],
    allowed_torrent_clients: &HashSet<Vec<u8>>,
) -> bool {
    for prefix in allowed_torrent_clients.iter() {
        if peer_id.starts_with(prefix) {
            return true;
        }
    }
    false
}
