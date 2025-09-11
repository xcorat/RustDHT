import init, { start_client, connect_to_server, send_message } from './pkg/rustdht.js';

let clientStarted = false;

// Log function that displays both in console and on page
function log(message) {
  console.log(message);
  const logDiv = document.getElementById('log');
  if (logDiv) {
    const timestamp = new Date().toLocaleTimeString();
    logDiv.textContent += `[${timestamp}] ${message}\n`;
    logDiv.scrollTop = logDiv.scrollHeight;
  }
}

function addMessage(peerId, message, isSent = false) {
  const messagesDiv = document.getElementById('messages');
  const messageDiv = document.createElement('div');
  messageDiv.className = `message ${isSent ? 'sent' : 'received'}`;
  messageDiv.textContent = `${peerId.slice(0, 8)}: ${message}`;
  messagesDiv.appendChild(messageDiv);
  messagesDiv.scrollTop = messagesDiv.scrollHeight;
}

async function run() {
  try {
    log('🚀 Initializing WASM module...');
    await init();
    log('✅ WASM module initialized successfully!');

    // Listen for P2P messages
    window.addEventListener('p2p-message', (event) => {
      const messageData = window.lastP2PMessage;
      if (messageData) {
        const [peerId, message] = messageData.split(':', 2);
        addMessage(peerId, message);
      }
    });
    
    // Set up event listeners
    document.getElementById('start-client').addEventListener('click', () => {
      if (!clientStarted) {
        log('🔄 Starting P2P client...');
        try {
          start_client();
          clientStarted = true;
          document.getElementById('client-status').textContent = 'Running ✅';
          document.getElementById('start-client').disabled = true;
          document.getElementById('restart-client').disabled = false;
          document.getElementById('connect-btn').disabled = false;
          document.getElementById('send-message').disabled = false;
          document.getElementById('message-input').disabled = false;
          log('✅ P2P client started successfully!');
        } catch (error) {
          log('❌ Failed to start client: ' + error);
        }
      }
    });

    document.getElementById('restart-client').addEventListener('click', () => {
      log('🔄 Restarting client... (reload page for full restart)');
      location.reload();
    });

    document.getElementById('connect-btn').addEventListener('click', () => {
      const addr = document.getElementById('server-addr').value.trim();
      if (addr) {
        if (!clientStarted) {
          log('⚠️  Please start the client first!');
          return;
        }
        log(`🔗 Attempting to connect to: ${addr}`);
        try {
          connect_to_server(addr);
          log('📡 Connection request sent!');
        } catch (error) {
          log('❌ Connection failed: ' + error);
        }
      } else {
        log('⚠️  Please enter a server address');
      }
    });

    document.getElementById('clear-log').addEventListener('click', () => {
      document.getElementById('log').textContent = '';
    });

    // Handle message sending
    document.getElementById('send-message').addEventListener('click', () => {
      const messageInput = document.getElementById('message-input');
      const message = messageInput.value.trim();
      if (message) {
        try {
          send_message(message);
          addMessage('You', message, true);
          messageInput.value = '';
        } catch (error) {
          log('❌ Failed to send message: ' + error);
        }
      }
    });

    // Handle Enter key in message input
    document.getElementById('message-input').addEventListener('keypress', (event) => {
      if (event.key === 'Enter') {
        document.getElementById('send-message').click();
      }
    });

    log('🌟 Ready! Click "Start P2P Client" to begin.');
    
  } catch (error) {
    log('❌ Failed to initialize: ' + error);
  }
}

run();
