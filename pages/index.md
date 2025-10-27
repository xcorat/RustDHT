---
layout: base.liquid
title: "Help Build User-Owned Data Infrastructure"
description: "RustDHT is a decentralized P2P graph database built with Rust and WebAssembly. Join the movement to put users back in control of their data."
---

<section class="hero">
    <div class="container">
        <h1 class="hero-title">{{ site.tagline }}</h1>
        <p class="hero-subtitle">{{ site.subtitle }}</p>
        
        <div class="hero-actions">
            <a href="/why-decentralized/" class="btn btn-primary">Learn Why</a>
            <a href="{{ site.github }}" class="btn btn-secondary" target="_blank" rel="noopener">
                View on GitHub
                <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                    <path d="M6 6H10V10M10 6L6 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </a>
        </div>
    </div>
</section>

<section class="section">
    <div class="container">
        <h2 class="text-center mb-12">Core Principles</h2>
        
        <div class="cards-grid">
            {% include "card.liquid", 
                title: "Decentralized P2P", 
                description: "No central servers, peer-to-peer architecture",
                preview: "280,000+ IPFS nodes strengthen the network. Each RustDHT peer adds capacity and resilience. Direct connections eliminate intermediaries and single points of failure.",
                url: "/why-decentralized/",
                icon: '<svg width="48" height="48" viewBox="0 0 48 48" fill="none"><circle cx="12" cy="12" r="4" stroke="currentColor" stroke-width="2"/><circle cx="36" cy="12" r="4" stroke="currentColor" stroke-width="2"/><circle cx="12" cy="36" r="4" stroke="currentColor" stroke-width="2"/><circle cx="36" cy="36" r="4" stroke="currentColor" stroke-width="2"/><circle cx="24" cy="24" r="4" stroke="currentColor" stroke-width="2"/><path d="M16 12L20 24M32 12L28 24M16 36L20 24M32 36L28 24" stroke="currentColor" stroke-width="2"/></svg>'
            %}
            
            {% include "card.liquid", 
                title: "Data Sovereignty", 
                description: "You own your data, cryptographically",
                preview: "Cryptographic ownership means no platform can lock or delete your content. Your private key is your identity and authority over your data space.",
                url: "/technical/#security-model",
                icon: '<svg width="48" height="48" viewBox="0 0 48 48" fill="none"><rect x="8" y="20" width="32" height="20" rx="4" stroke="currentColor" stroke-width="2"/><path d="M16 20V12C16 8.68629 18.6863 6 22 6H26C29.3137 6 32 8.68629 32 12V20" stroke="currentColor" stroke-width="2"/><circle cx="24" cy="30" r="3" stroke="currentColor" stroke-width="2"/></svg>'
            %}
            
            {% include "card.liquid", 
                title: "Offline-First", 
                description: "Work without connectivity, sync when available",
                preview: "CRDTs enable seamless offline editing with automatic conflict resolution. Continue working during network outages, sync automatically when reconnected.",
                url: "/use-cases/#offline-productivity",
                icon: '<svg width="48" height="48" viewBox="0 0 48 48" fill="none"><path d="M8 24C8 15.1634 15.1634 8 24 8C32.8366 8 40 15.1634 40 24C40 32.8366 32.8366 40 24 40" stroke="currentColor" stroke-width="2"/><path d="M8 24C8 32.8366 15.1634 40 24 40" stroke="currentColor" stroke-width="2" stroke-dasharray="4 4"/><circle cx="24" cy="24" r="4" fill="currentColor"/></svg>'
            %}
            
            {% include "card.liquid", 
                title: "High Performance", 
                description: "Rust + WebAssembly for near-native speed",
                preview: "Sub-100ms latency, minimal footprint, runs in browser or natively. Memory safety without garbage collection enables predictable performance.",
                url: "/technical/#performance",
                icon: '<svg width="48" height="48" viewBox="0 0 48 48" fill="none"><path d="M24 8L32 16L24 24L16 16L24 8Z" stroke="currentColor" stroke-width="2"/><path d="M8 24L16 32L24 24L16 16L8 24Z" stroke="currentColor" stroke-width="2"/><path d="M40 24L32 32L24 24L32 16L40 24Z" stroke="currentColor" stroke-width="2"/><path d="M24 40L32 32L24 24L16 32L24 40Z" stroke="currentColor" stroke-width="2"/></svg>'
            %}
        </div>
    </div>
</section>

<section class="section" style="background-color: var(--color-bg-secondary);">
    <div class="container">
        <h2 class="text-center mb-8">Existing Projects in the Ecosystem</h2>
        <p class="text-center mb-12 text-lg" style="color: var(--color-text-secondary); max-width: 600px; margin-left: auto; margin-right: auto;">
            RustDHT builds on lessons from proven decentralized technologies. Each project contributes unique insights to the broader ecosystem.
        </p>
        
        <div class="projects-scroll">
            {% for project in projects %}
            <div class="project-card">
                <h4>{{ project.name }}</h4>
                <p class="mb-4">{{ project.description }}</p>
                <p style="font-size: var(--text-xs); color: var(--color-text-muted); margin-bottom: var(--space-4);">{{ project.summary }}</p>
                <a href="{{ project.url }}" target="_blank" rel="noopener" class="btn btn-secondary" style="font-size: var(--text-sm); padding: var(--space-2) var(--space-4);">
                    Learn More
                    <svg width="12" height="12" viewBox="0 0 12 12" fill="none">
                        <path d="M3.5 3H8.5V8M8.5 3L3.5 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                    </svg>
                </a>
            </div>
            {% endfor %}
        </div>
    </div>
</section>

<section class="section">
    <div class="container">
        <div style="display: grid; gap: var(--space-12); align-items: center;">
            <div style="grid-column: 1; text-align: center;">
                <h2 class="mb-6">About RustDHT</h2>
                <p class="text-lg mb-8" style="color: var(--color-text-secondary); max-width: 700px; margin: 0 auto;">
                    A decentralized P2P graph database inspired by GunDB, built with Rust for performance and WebAssembly for universal compatibility. 
                    Combines proven patterns from IPFS, Holochain, and modern CRDT research.
                </p>
                
                <div style="display: grid; gap: var(--space-6); margin: var(--space-8) 0;">
                    <div style="grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); display: grid; gap: var(--space-6);">
                        <div style="text-align: center;">
                            <div style="font-size: var(--text-2xl); font-weight: 700; color: var(--color-accent); margin-bottom: var(--space-2);">Sub-100ms</div>
                            <div style="color: var(--color-text-secondary);">Target Latency</div>
                        </div>
                        <div style="text-align: center;">
                            <div style="font-size: var(--text-2xl); font-weight: 700; color: var(--color-accent); margin-bottom: var(--space-2);">Browser Native</div>
                            <div style="color: var(--color-text-secondary);">WebAssembly</div>
                        </div>
                        <div style="text-align: center;">
                            <div style="font-size: var(--text-2xl); font-weight: 700; color: var(--color-accent); margin-bottom: var(--space-2);">Graph Model</div>
                            <div style="color: var(--color-text-secondary);">Flexible Schema</div>
                        </div>
                    </div>
                </div>
                
                <div style="display: flex; gap: var(--space-4); justify-content: center; flex-wrap: wrap;">
                    <a href="/technical/" class="btn btn-primary">Technical Overview</a>
                    <a href="/use-cases/" class="btn btn-secondary">Use Cases</a>
                </div>
            </div>
        </div>
    </div>
</section>

<section class="section" style="background-color: var(--color-bg-secondary);">
    <div class="container">
        <h2 class="text-center mb-8">Development Roadmap</h2>
        
        <div style="display: grid; gap: var(--space-6);">
            {% for milestone in roadmap %}
            <div class="card" style="cursor: default;">
                <div style="display: flex; align-items: center; gap: var(--space-4); margin-bottom: var(--space-4);">
                    <div style="
                        width: 12px; 
                        height: 12px; 
                        border-radius: 50%; 
                        background-color: {% if milestone.status == 'completed' %}#22c55e{% elsif milestone.status == 'active' %}var(--color-accent){% else %}var(--color-border){% endif %};
                    "></div>
                    <h3 style="margin: 0; color: var(--color-text);">{{ milestone.phase }}</h3>
                    <span style="
                        font-size: var(--text-xs); 
                        padding: var(--space-1) var(--space-2); 
                        border-radius: 4px; 
                        background-color: {% if milestone.status == 'completed' %}rgba(34, 197, 94, 0.1){% elsif milestone.status == 'active' %}rgba(255, 107, 53, 0.1){% else %}var(--color-bg-tertiary){% endif %};
                        color: {% if milestone.status == 'completed' %}#22c55e{% elsif milestone.status == 'active' %}var(--color-accent){% else %}var(--color-text-muted){% endif %};
                        font-weight: 600;
                        text-transform: uppercase;
                    ">{{ milestone.status }}</span>
                </div>
                <ul style="margin: 0; padding-left: var(--space-6); color: var(--color-text-secondary);">
                    {% for item in milestone.items %}
                    <li style="margin-bottom: var(--space-2);">{{ item }}</li>
                    {% endfor %}
                </ul>
            </div>
            {% endfor %}
        </div>
        
        <div class="text-center mt-8">
            <a href="/roadmap/" class="btn btn-secondary">Detailed Roadmap</a>
        </div>
    </div>
</section>

<section class="section">
    <div class="container text-center">
        <h2 class="mb-6">Help Build the Future</h2>
        <p class="text-lg mb-8" style="color: var(--color-text-secondary); max-width: 600px; margin: 0 auto;">
            RustDHT is a community-owned project. Every contribution strengthens the network and advances user-owned data infrastructure.
        </p>
        
        <div style="display: flex; gap: var(--space-4); justify-content: center; flex-wrap: wrap;">
            <a href="/community/" class="btn btn-primary">Get Involved</a>
            <a href="{{ site.github }}/issues" class="btn btn-secondary" target="_blank" rel="noopener">
                Report Issues
                <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                    <path d="M6 6H10V10M10 6L6 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </a>
            <a href="{{ site.github }}/discussions" class="btn btn-secondary" target="_blank" rel="noopener">
                Join Discussion
                <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
                    <path d="M6 6H10V10M10 6L6 10" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </a>
        </div>
    </div>
</section>
