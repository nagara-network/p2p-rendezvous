#[derive(libp2p::swarm::NetworkBehaviour)]
pub struct L2MediatorBehaviour {
    identify: libp2p::identify::Behaviour,
    rendezvous: libp2p::rendezvous::server::Behaviour,
    relay: libp2p::relay::Behaviour,
    kademlia: libp2p::kad::Behaviour<libp2p::kad::store::MemoryStore>,
    autonat: libp2p::autonat::Behaviour,
    ping: libp2p::ping::Behaviour,
}

impl L2MediatorBehaviour {
    pub const PING_INTERVAL: core::time::Duration = core::time::Duration::from_secs(5);
    pub const PROTOCOL_VERSION: &'static str = "/nagara-l2/1.0.24";
    pub const TIMEOUT: core::time::Duration = core::time::Duration::from_secs(5);

    pub fn create(keypair: &libp2p::identity::Keypair) -> Self {
        let public_key = keypair.public();
        let peer_id = public_key.to_peer_id();
        let identify = {
            let config =
                libp2p::identify::Config::new(Self::PROTOCOL_VERSION.to_owned(), public_key);

            libp2p::identify::Behaviour::new(config)
        };
        let rendezvous = {
            let config = libp2p::rendezvous::server::Config::default();

            libp2p::rendezvous::server::Behaviour::new(config)
        };
        let relay = {
            let config = libp2p::relay::Config::default();

            libp2p::relay::Behaviour::new(peer_id, config)
        };
        let kademlia = {
            let config = libp2p::kad::Config::default();
            let store_config = libp2p::kad::store::MemoryStoreConfig::default();
            let store = libp2p::kad::store::MemoryStore::with_config(peer_id, store_config);

            libp2p::kad::Behaviour::with_config(peer_id, store, config)
        };
        let autonat = {
            let config = libp2p::autonat::Config {
                only_global_ips: false,
                timeout: Self::TIMEOUT,
                ..Default::default()
            };

            libp2p::autonat::Behaviour::new(peer_id, config)
        };
        let ping = {
            let config = libp2p::ping::Config::default()
                .with_interval(Self::PING_INTERVAL)
                .with_timeout(Self::TIMEOUT);

            libp2p::ping::Behaviour::new(config)
        };

        Self {
            identify,
            rendezvous,
            relay,
            kademlia,
            autonat,
            ping,
        }
    }
}
