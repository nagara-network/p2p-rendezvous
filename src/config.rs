#[derive(clap::Parser)]
#[derive(core::fmt::Debug)]
#[clap(author, version)]
pub struct Config {
    /// P2P TCP & WebRTC UDP IPv4 Address
    #[clap(long, short = 'a', default_value = "0.0.0.0")]
    pub listen_address: core::net::Ipv4Addr,
    /// P2P TCP & WebRTC UDP IPv4 Address
    #[clap(long, short = 'p', default_value = "10111")]
    pub listen_port: u16,
    /// Ed25519 Secret Key (32 Bytes) in hexadecimal
    #[clap(
        long,
        short = 's',
        default_value = Config::RANDOM_TRIGGER,
        value_parser = Config::decode_or_generate_secret_seed,
    )]
    pub secret_seed: libp2p::identity::Keypair,
    /// WebRTC certificate (PEM file path)
    #[clap(
        long,
        short = 'w',
        default_value = "webrtc.pem",
        value_parser = Config::decode_or_generate_pem,
    )]
    pub webrtc_certificate: libp2p_webrtc::tokio::Certificate,
}

impl Config {
    const LONG_TIMEOUT: core::time::Duration = core::time::Duration::from_secs(3600);
    const RANDOM_TRIGGER: &'static str = "{RANDOM}";

    fn decode_or_generate_secret_seed(input: &str) -> crate::Result<libp2p::identity::Keypair> {
        let keypair = if input.eq(Self::RANDOM_TRIGGER) {
            libp2p::identity::Keypair::generate_ed25519()
        } else {
            let secret_seed = hex::decode(input).map_err(|_| crate::Error::BadSecretSeed)?;

            libp2p::identity::Keypair::ed25519_from_bytes(secret_seed).unwrap()
        };

        Ok(keypair)
    }

    fn decode_or_generate_pem(input: &str) -> crate::Result<libp2p_webrtc::tokio::Certificate> {
        let path = std::path::Path::new(input);
        let certificate = if path.exists() {
            let pem_string = std::fs::read_to_string(path)?;

            libp2p_webrtc::tokio::Certificate::from_pem(&pem_string)?
        } else {
            let certificate = libp2p_webrtc::tokio::Certificate::generate(&mut rand::thread_rng())?;
            let pem_string = certificate.serialize_pem();
            std::fs::write(path, pem_string)?;

            certificate
        };

        Ok(certificate)
    }

    pub fn load() -> Self {
        <Self as clap::Parser>::parse()
    }

    pub fn get_listen_addresses(&self) -> [libp2p::Multiaddr; 2] {
        let tcp_address = libp2p::Multiaddr::empty()
            .with(libp2p::multiaddr::Protocol::Ip4(self.listen_address))
            .with(libp2p::multiaddr::Protocol::Tcp(self.listen_port));
        let webrtc_address = libp2p::Multiaddr::empty()
            .with(libp2p::multiaddr::Protocol::Ip4(self.listen_address))
            .with(libp2p::multiaddr::Protocol::Udp(self.listen_port))
            .with(libp2p::multiaddr::Protocol::WebRTCDirect);

        [tcp_address, webrtc_address]
    }

    pub fn into_swarm(self) -> crate::Result<libp2p::Swarm<crate::protocols::L2MediatorBehaviour>> {
        let listen_addresses = self.get_listen_addresses();
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(self.secret_seed)
            .with_tokio()
            .with_tcp(
                libp2p::tcp::Config::default().nodelay(true),
                libp2p::noise::Config::new,
                libp2p::yamux::Config::default,
            )?
            .with_other_transport(|keypair_ref| {
                let certificate =
                    libp2p_webrtc::tokio::Certificate::generate(&mut rand::thread_rng())?;
                let webrtc_transport =
                    libp2p_webrtc::tokio::Transport::new(keypair_ref.clone(), certificate);

                Ok(webrtc_transport)
            })
            .map_err(|_| crate::Error::TransportBuilderError)?
            .with_behaviour(crate::protocols::L2MediatorBehaviour::create)
            .map_err(|_| crate::Error::SwarmBehaviourError)?
            .with_swarm_config(|swarm_config| {
                swarm_config.with_idle_connection_timeout(Self::LONG_TIMEOUT)
            })
            .build();

        for listen_address in listen_addresses {
            swarm.listen_on(listen_address)?;
        }

        Ok(swarm)
    }
}
