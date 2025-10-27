---
layout: base.liquid
title: "Technical Overview"
description: "Learn how RustDHT combines Rust, WebAssembly, libp2p, and CRDTs to create a high-performance decentralized database."
---

<div class="content">

# Technical Overview

RustDHT combines proven decentralized patterns with modern implementation technologies to create a high-performance, browser-native P2P graph database.

## Architecture Overview

The system operates as a network of interconnected peer nodes, where each node functions as both a client and a server, directly sharing data and resources. This design eliminates reliance on a single central authority for data storage or processing.

### Core Technologies

**Rust for Performance and Safety**
- Memory safety without garbage collection
- Zero-cost abstractions and predictable performance
- Concurrency safety prevents data races at compile time
- Critical for P2P systems with many simultaneous connections

**WebAssembly for Universal Compatibility**
- Near-native performance in web browsers
- Same Rust code runs natively and in browser
- Enables complex database operations client-side
- Works on desktop, mobile, and embedded devices

## Peer-to-Peer Networking with libp2p

RustDHT uses libp2p, a modular networking framework that provides the foundation for building P2P applications. It's used by major projects including IPFS, Ethereum 2.0, and Polkadot.

### Transport Layer

**Multi-Transport Support:**
- **TCP**: Direct connections for native-to-native communication
- **WebRTC**: Browser-to-native and browser-to-browser connections
- **WebSockets**: Fallback transport for browser compatibility

**Connection Features:**
- Encrypted connections by default (Noise protocol)
- Stream multiplexing (multiple logical streams over single connection)
- NAT traversal via STUN/TURN servers
- Automatic peer discovery and routing

### Current Implementation Status

From the project architecture documentation:

**Native Server (Operational):**
- Multi-transport listener supporting TCP and WebRTC
- Event-driven architecture using Tokio for async I/O
- GossipSub messaging for real-time communication
- Ping protocol for connection health monitoring

**WASM Client (In Development):**
- Browser-compatible P2P node
- WebRTC-only transport layer (browser limitation)
- Resolving libp2p compatibility issues with WebSocket fallback

## Distributed Hash Table (DHT)

The core data storage mechanism uses a Distributed Hash Table for decentralized key-value storage across the network.

### How DHTs Work

**Consistent Hashing:**
- Data distributed evenly among participating nodes
- Each node responsible for specific range of keys
- Uses Kademlia algorithm with XOR-based distance metric
- O(log n) lookup complexity for efficient routing

**Sharding and Replication:**
- Data sharded across network based on key hash
- Multiple copies stored on different nodes for fault tolerance
- Automatic load balancing as nodes join/leave
- Self-healing network topology

### Storage Backend

**Browser Storage:**
- IndexedDB for structured data and large objects
- localStorage for simple key-value pairs
- Modular design allows other backends (SQLite-WASM planned)
- Browser nodes primarily serve as local cache

**Consistency Model:**
The DHT operates under eventual consistency - updates may not be immediately visible everywhere, but all nodes converge to the same state over time. This prioritizes high availability and low latency over strong consistency[^1].

## Graph Data Model

Inspired by GunDB's flexible graph structure, data is organized as interconnected "nodes" with unique identifiers called "souls".

### Node Structure

```javascript
{
  "soul": "user_alice",
  "name": "Alice", 
  "age": 30,
  "posts": { "#": "post_1", "#": "post_2" }  // Links to other nodes
}
```

**Key Features:**
- **Flexible Schema**: No rigid structure required
- **Natural Relationships**: Links between nodes create graph edges
- **Metadata Fields**: Each node contains system information for security and conflict resolution
- **Hypergraph Support**: Complex relationships through multiple links

### Data Classification

The security model supports three data types:

- **Public Data**: Readable by anyone, writable by anyone (unless signed)
- **Protected Data**: Readable by anyone, writable only by owner
- **Private Data**: Encrypted, accessible only to authorized users

## Conflict Resolution with CRDTs

Conflict-Free Replicated Data Types ensure that concurrent updates merge automatically without coordination.

### HAM Algorithm Inspiration

Drawing from GunDB's Hypothetical Amnesia Machine (HAM):
- **Last-Write-Wins** with sophisticated ordering
- **Vector clocks** for causality tracking
- **Machine-relative timestamps** handle clock inconsistencies
- **Deferred updates** prevent malicious timestamp manipulation

### CRDT Implementation

For the MVP, a simple Last-Write-Wins (LWW) Register approach:
- Leverages metadata for ordering operations
- Guarantees eventual consistency across all replicas
- Enables offline-first operation with automatic sync
- Extensible architecture for advanced CRDT types

### Practical Benefits

**For Users:**
- Seamless offline work with automatic synchronization
- No "merge conflict" dialogs to resolve
- Predictable outcomes from concurrent edits

**For Developers:**
- Built-in conflict resolution
- No manual merge code required
- Extensible for custom data types

## Performance Characteristics

### Current Targets

**Latency:**
- Sub-100ms for basic data operations
- Comparable to Holochain benchmarks (50ms write, 30ms read)
- Network conditions affect browser-to-browser connections

**Throughput:**
- 10-20 TPS minimum per node for key-value operations
- Scales proportionally with network participation
- GunDB benchmarks show 0.2-2ms for small node operations

**Resource Usage:**
- Minimal CPU and memory footprint
- Critical for browser-based nodes
- Rust + WASM optimization for efficiency

### Scalability Design

**Horizontal Scaling:**
- Adding nodes increases total capacity
- DHT architecture supports large numbers of nodes
- No single bottleneck as network grows

**Network Effects:**
Unlike centralized systems that degrade under load, P2P networks become MORE resilient as they grow:
- Each new node adds capacity and redundancy
- Geographic distribution improves naturally
- Attack resistance increases with network size

## Security Model

### Cryptographic Primitives

**Digital Signatures:**
- Every data modification cryptographically signed
- Links data back to its origin
- Makes tampering detectable
- Provides verifiable audit trail

**Encryption:**
- Symmetric and asymmetric encryption for private data
- Content remains unreadable to unauthorized parties
- Even with data replicated across untrusted nodes

### Access Control

**Cryptographic Ownership:**
- User's public key identifies them as data owner
- Private key grants write permissions to data space
- No central authority can override ownership

**Validation Rules:**
- Distributed to all peers for collective verification
- Similar to Holochain's DNA rules
- Network identifies and isolates malicious actors
- Enables community-driven moderation

## Current Status and Limitations

### What's Working

**Sprint 2 Complete:**
- Native server fully operational
- TCP and WebRTC transports
- GossipSub messaging working
- Ping protocol functional
- Docker deployment ready

### Current Challenges

**Browser Client Blocked:**
- libp2p version compatibility issues
- WebRTC-websys lagging behind main libp2p
- Being addressed through WebSocket fallback

**Infrastructure Dependencies:**
Despite P2P architecture, still requires:
- Bootstrap nodes for initial discovery
- STUN/TURN servers for NAT traversal
- Signaling server for WebRTC connections

### Honest Assessment

**RustDHT is Currently a Prototype:**
- Not production-ready
- Many core features not yet implemented (full DHT, CRDTs)
- Limited testing and real-world validation
- Browser support incomplete

**Broader Infrastructure Reality:**
Most computing infrastructure remains centralized:
- Cloud providers (AWS, Azure, Google)
- DNS infrastructure and certificate authorities
- Internet backbone corporate/state-owned

## The Path Forward

### Pragmatic Approach

Rather than claiming to solve all problems, RustDHT:
- Starts with small communities and specific use cases
- Focuses on scenarios where P2P makes sense
- Learns from real-world deployment
- Gradually expands capabilities
- Maintains honest assessment of limitations

### Scaling Advantage

The key insight: P2P networks become MORE resilient as they grow, unlike centralized systems that face bottlenecks. Starting small is not a limitation but a strategy for building antifragile systems.

---

## Learn More

- [Use Cases](/use-cases/) - Real-world applications enabled by this architecture
- [Why Decentralized](/why-decentralized/) - The principles behind these technical choices
- [Community](/community/) - How to contribute to development
- [Roadmap](/roadmap/) - Planned features and timeline

---

[^1]: [RustDHT PRD: Eventual Consistency Model](https://github.com/rustdht/rustdht/blob/main/docs/PRD%20for%20Decentralized%20Graph%20Database_.md)

</div>
