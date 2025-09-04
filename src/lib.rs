#[cfg(target_arch = "wasm32")]
use {
    futures::stream::StreamExt,
    futures::channel::mpsc,
    libp2p::{
        gossipsub::{self, MessageAuthenticity, ValidationMode},
        identity,
        ping,
        swarm::SwarmEvent,
        PeerId,
        Multiaddr,
        Transport,
    },
    libp2p_webrtc_websys as webrtc_websys,
    wasm_bindgen::prelude::*,
    wasm_bindgen_futures::spawn_local,
    web_sys::console,
    once_cell::sync::OnceCell,
    std::time::Duration,
};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
}

#[cfg(target_arch = "wasm32")]
static COMMAND_SENDER: OnceCell<mpsc::UnboundedSender<SwarmCommand>> = OnceCell::new();

#[cfg(target_arch = "wasm32")]
enum SwarmCommand {
    Dial(Multiaddr),
    SendMessage(String),
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start_client() {
    spawn_local(async move {
        console::log_1(&"Starting libp2p WebRTC client...".into());

        // Create keypair and peer ID
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        console::log_1(&format!("Local peer id: {}", local_peer_id).into());
        
        // Create WebRTC transport
        let transport = webrtc_websys::Transport::new(
            webrtc_websys::Config::new(&local_key)
        )
        .map(|(peer_id, conn), _| (peer_id, libp2p::core::muxing::StreamMuxerBox::new(conn)))
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
        let ping = ping::Behaviour::new(ping::Config::new());

        // Combine behaviors
        #[derive(libp2p::swarm::NetworkBehaviour)]
        struct MyBehaviour {
            ping: ping::Behaviour,
            gossipsub: gossipsub::Behaviour,
        }

        let behaviour = MyBehaviour { ping, gossipsub };
        
        // Create swarm
        let mut swarm = libp2p::Swarm::new(
            transport,
            behaviour,
            local_peer_id,
            libp2p::swarm::Config::with_wasm_executor(),
        );

        console::log_1(&"Swarm created successfully".into());

        // Create command channel
        let (command_sender, mut command_receiver) = mpsc::unbounded();
        
        // Store command sender globally
        COMMAND_SENDER.set(command_sender).unwrap_or_else(|_| {
            console::log_1(&"Failed to set global command sender".into());
        });

        console::log_1(&"Waiting for events... (ready to connect to server)".into());

        // Main event loop
        loop {
            futures::select! {
                // Handle swarm events
                event = swarm.select_next_some() => {
                    match event {
                        SwarmEvent::Behaviour(MyBehaviourEvent::Ping(ping::Event { peer, connection: _, result })) => {
                            match result {
                                Ok(rtt) => {
                                    console::log_1(&format!("Ping to {} succeeded: RTT = {:?}", peer, rtt).into());
                                }
                                Err(ping::Failure::Timeout) => {
                                    console::log_1(&format!("Ping to {} timed out", peer).into());
                                }
                                Err(ping::Failure::Unsupported) => {
                                    console::log_1(&format!("Ping to {} failed: unsupported", peer).into());
                                }
                                Err(ping::Failure::Other { error }) => {
                                    console::log_1(&format!("Ping to {} failed: {}", peer, error).into());
                                }
                            }
                        }
                        SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                            propagation_source: peer_id,
                            message_id: _,
                            message,
                        })) => {
                            let msg = String::from_utf8_lossy(&message.data);
                            console::log_1(&format!("💬 Message from {}: {}", peer_id, msg).into());
                            
                            // Dispatch message to JavaScript using a simple custom event
                            let window = web_sys::window().expect("Should have window");
                            let event = web_sys::Event::new("p2p-message").expect("Can create event");
                            // Store message data in window for retrieval
                            js_sys::Reflect::set(
                                &window,
                                &JsValue::from_str("lastP2PMessage"),
                                &JsValue::from_str(&format!("{}:{}", peer_id, msg))
                            ).expect("Can set property");
                            window.dispatch_event(&event).expect("Can dispatch event");
                        }
                        SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                            console::log_1(&format!("✅ Connected to: {} via {}", peer_id, endpoint.get_remote_address()).into());
                        }
                        SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                            console::log_1(&format!("❌ Disconnected from: {} (cause: {:?})", peer_id, cause).into());
                        }
                        SwarmEvent::IncomingConnection { connection_id, local_addr, send_back_addr } => {
                            console::log_1(&format!("📥 Incoming connection {} from {} to {}", connection_id, send_back_addr, local_addr).into());
                        }
                        SwarmEvent::OutgoingConnectionError { connection_id, peer_id, error } => {
                            console::log_1(&format!("❌ Outgoing connection error to {:?} ({}): {}", peer_id, connection_id, error).into());
                        }
                        SwarmEvent::IncomingConnectionError { connection_id, local_addr, send_back_addr, error, .. } => {
                            console::log_1(&format!("❌ Incoming connection error {} from {} to {}: {}", connection_id, send_back_addr, local_addr, error).into());
                        }
                        _ => {}
                    }
                }
                // Handle commands from JavaScript
                command = command_receiver.select_next_some() => {
                    match command {
                        SwarmCommand::Dial(addr) => {
                            console::log_1(&format!("🔗 Dialing {}", addr).into());
                            match swarm.dial(addr.clone()) {
                                Ok(_) => {
                                    console::log_1(&format!("📡 Successfully initiated dial to {}", addr).into());
                                }
                                Err(e) => {
                                    console::log_1(&format!("❌ Failed to dial {}: {}", addr, e).into());
                                }
                            }
                        }
                        SwarmCommand::SendMessage(message) => {
                            console::log_1(&format!("📤 Publishing message: {}", message).into());
                            match swarm.behaviour_mut().gossipsub.publish(chat_topic.clone(), message.as_bytes()) {
                                Ok(_) => {
                                    console::log_1(&"✅ Message published successfully".into());
                                }
                                Err(e) => {
                                    console::log_1(&format!("❌ Failed to publish message: {}", e).into());
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn send_message(message: String) {
    console::log_1(&format!("Sending message: {}", message).into());
    
    if let Some(sender) = COMMAND_SENDER.get() {
        match sender.unbounded_send(SwarmCommand::SendMessage(message.clone())) {
            Ok(_) => {
                console::log_1(&"📤 Message sent to swarm".into());
            }
            Err(e) => {
                console::log_1(&format!("❌ Failed to send message: {}", e).into());
            }
        }
    } else {
        console::log_1(&"❌ Client not initialized. Please start the client first.".into());
    }
}

// Function to connect to a server (to be called from JavaScript)
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn connect_to_server(multiaddr: String) {
    console::log_1(&format!("Attempting to connect to server: {}", multiaddr).into());
    
    // Parse the multiaddr
    match multiaddr.parse::<Multiaddr>() {
        Ok(addr) => {
            console::log_1(&format!("✅ Parsed multiaddr: {}", addr).into());
            
            // Send dial command through the channel
            if let Some(sender) = COMMAND_SENDER.get() {
                match sender.unbounded_send(SwarmCommand::Dial(addr.clone())) {
                    Ok(_) => {
                        console::log_1(&format!("📡 Connection command sent for {}", addr).into());
                    }
                    Err(e) => {
                        console::log_1(&format!("❌ Failed to send dial command: {}", e).into());
                    }
                }
            } else {
                console::log_1(&"❌ Client not initialized. Please start the client first.".into());
            }
        }
        Err(e) => {
            console::log_1(&format!("❌ Failed to parse multiaddr '{}': {}", multiaddr, e).into());
        }
    }
}

