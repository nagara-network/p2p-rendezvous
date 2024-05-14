pub type Result<T> = core::result::Result<T, self::Error>;

#[derive(Debug)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Protocol secure upgrade failure")]
    ProtocolSecureUpgradeFailure(#[from] libp2p::noise::Error),
    #[error("General transport error")]
    GeneralTransportError(#[from] libp2p::TransportError<std::io::Error>),
    #[error("WebRTC transport error")]
    WebRTCTransportError(#[from] libp2p_webrtc::tokio::Error),
    #[error("Invalid PEM certificate for WebRTC")]
    InvalidPEMCertificateWebRTC(#[from] libp2p_webrtc::tokio::certificate::Error),
    #[error("Transport builder error")]
    TransportBuilderError,
    #[error("Swarm behaviour error")]
    SwarmBehaviourError,
    #[error("Bad secret seed")]
    BadSecretSeed,
}
