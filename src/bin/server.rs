use futures::stream::StreamExt;
use libp2p::{
    core::upgrade,
    gossipsub::{self, MessageAuthenticity, ValidationMode},
    identity,
    noise,
    ping,
    swarm::{SwarmEvent, Swarm},
    tcp,
    yamux,
    PeerId,
    Transport,
};
use libp2p_webrtc as webrtc;
use std::error::Error;
use std::time::Duration;
use tokio::net::TcpListener;

// Server configuration - Signaling server on port 9090
const SIGNALING_PORT: u16 = 9090;
const HTTP_PORT: u16 = 8080;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Generate a keypair
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    // Create TCP transport
    let tcp_transport = tcp::tokio::Transport::default()
        .upgrade(upgrade::Version::V1Lazy)
        .authenticate(noise::Config::new(&local_key)?)
        .multiplex(yamux::Config::default())
        .boxed();

    // Create WebRTC transport with proper upgrade chain
    let webrtc_transport = webrtc::tokio::Transport::new(
        local_key.clone(),
        webrtc::tokio::Certificate::generate(&mut rand::thread_rng())?,
    )
    .map(|(peer_id, conn), _| (peer_id, libp2p::core::muxing::StreamMuxerBox::new(conn)));

    // Combine transports and map to a consistent type
    let transport = tcp_transport
        .or_transport(webrtc_transport)
        .map(|either, _| match either {
            futures::future::Either::Left(output) => output,
            futures::future::Either::Right(output) => output,
        })
        .boxed();

    // Create GossipSub configuration
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(1))
        .validation_mode(ValidationMode::Strict)
        .build()
        .expect("Valid config");

    // Create GossipSub behavior
    let mut gossipsub = gossipsub::Behaviour::new(MessageAuthenticity::Signed(local_key.clone()), gossipsub_config)
        .expect("Valid configuration");

    // Create a topic for chat messages
    let chat_topic = gossipsub::IdentTopic::new("chat");
    gossipsub.subscribe(&chat_topic).expect("Valid subscription");

    // Create ping behavior
    let ping = ping::Behaviour::new(ping::Config::new().with_interval(Duration::from_secs(1)));

    // Combine behaviors
    #[derive(libp2p::swarm::NetworkBehaviour)]
    struct MyBehaviour {
        ping: ping::Behaviour,
        gossipsub: gossipsub::Behaviour,
    }

    let behaviour = MyBehaviour { ping, gossipsub };

    // Create swarm
    let mut swarm = Swarm::new(
        transport,
        behaviour,
        local_peer_id,
        libp2p::swarm::Config::with_tokio_executor().with_idle_connection_timeout(Duration::from_secs(5)),
    );

    // Listen on WebRTC for browser signaling and P2P connections
    // IPv4
    swarm.listen_on(format!("/ip4/0.0.0.0/udp/{}/webrtc-direct", SIGNALING_PORT).parse()?)?;
    // IPv6
    swarm.listen_on(format!("/ip6/::/udp/{}/webrtc-direct", SIGNALING_PORT).parse()?)?;

    // Start HTTP health check server
    let http_listener = TcpListener::bind(format!("0.0.0.0:{}", HTTP_PORT)).await?;
    tokio::spawn(async move {
        loop {
            match http_listener.accept().await {
                Ok((mut stream, _)) => {
                    tokio::spawn(async move {
                        use tokio::io::AsyncWriteExt;
                        let response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK";
                        let _ = stream.write_all(response.as_bytes()).await;
                        let _ = stream.shutdown().await;
                    });
                }
                Err(e) => eprintln!("Failed to accept HTTP connection: {}", e),
            }
        }
    });

    println!("\n=== RustDHT Signaling Server Starting ===");
    println!("🔑 Local peer id: {:?}", swarm.local_peer_id());
    println!("🌐 Signaling port: {}", SIGNALING_PORT);
    println!("🌐 HTTP health check port: {}", HTTP_PORT);
    println!("=========================================\n");

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("🚀 Listening on {address:?}");
            }
            SwarmEvent::IncomingConnection { connection_id, local_addr, send_back_addr } => {
                println!("📥 Incoming connection {} from {} to {}", connection_id, send_back_addr, local_addr);
            }
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                println!("✅ Connected to peer: {} via {}", peer_id, endpoint.get_remote_address());
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                println!("❌ Disconnected from peer: {} (cause: {:?})", peer_id, cause);
            }
            SwarmEvent::IncomingConnectionError { connection_id, local_addr, send_back_addr, error, .. } => {
                println!("❌ Incoming connection error {} from {} to {}: {}", connection_id, send_back_addr, local_addr, error);
            }
            SwarmEvent::OutgoingConnectionError { connection_id, peer_id, error } => {
                println!("❌ Outgoing connection error to {:?} ({}): {}", peer_id, connection_id, error);
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::Ping(event)) => {
                println!("🏓 Ping event: {event:?}");
            }
            SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                propagation_source: peer_id,
                message_id: _,
                message,
            })) => {
                println!("💬 Message from {}: {}", peer_id, String::from_utf8_lossy(&message.data));
            }
            SwarmEvent::Dialing { peer_id, connection_id } => {
                println!("📞 Dialing peer {:?} (connection {})", peer_id, connection_id);
            }
            event => {
                println!("🔍 Other event: {:?}", event);
            }
        }
    }
}
