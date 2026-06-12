import { useWebSocket } from '@vueuse/core';
import { Notification } from '@arco-design/web-vue';

// Use standard location to determine protocol and host
const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
const host = window.location.host;
const wsUrl = `${protocol}//${host}/api/monitor/ws`;

export function useGlobalWebSocket() {
  const { status, data, send, open, close } = useWebSocket(wsUrl, {
    autoReconnect: {
      retries: 3,
      delay: 5000,
      onFailed() {
        console.warn('[WebSocket] Failed to connect after 3 retries.');
      },
    },
    onMessage(ws, event) {
      try {
        const msg = JSON.parse(event.data);
        if (msg && msg.title && msg.content) {
          Notification.info({
            title: msg.title,
            content: msg.content,
            position: 'bottomRight',
            duration: 5000,
          });
        }
      } catch (error) {
        console.error('[WebSocket] Failed to parse message:', event.data);
      }
    },
    onConnected() {
      console.log('[WebSocket] Connected successfully to', wsUrl);
    },
    onDisconnected() {
      console.log('[WebSocket] Disconnected from', wsUrl);
    }
  });

  return {
    status,
    data,
    send,
    open,
    close,
  };
}
