import init, { start_client, connect_to_server } from '../pkg/rustdht.js';

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

async function run() {
  try {
    log('🚀 Initializing WASM module...');
    await init();
    log('✅ WASM module initialized successfully!');
    
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

    log('🌟 Ready! Click "Start P2P Client" to begin.');
    
  } catch (error) {
    log('❌ Failed to initialize: ' + error);
  }
}

run();
