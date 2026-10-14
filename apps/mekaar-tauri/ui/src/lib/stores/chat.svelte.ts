import type { Conversation, Message } from '../types';

class ChatStore {
  conversations = $state<Conversation[]>([]);
  activeRoomId = $state<string | null>(null);
  messages = $state<Record<string, Message[]>>({});
  disappearingTimerSecs = $state<number | null>(null);
  isLoading = $state<boolean>(false);
  private isListening = false;

  get activeConversation(): Conversation | null {
    if (!this.activeRoomId) return null;
    return this.conversations.find((c) => c.id === this.activeRoomId) ?? null;
  }

  get activeMessages(): Message[] {
    if (!this.activeRoomId) return [];
    return this.messages[this.activeRoomId] ?? [];
  }

  async initRealtimeListener() {
    if (this.isListening) return;
    const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
    if (!isTauri) return;

    try {
      const { listen } = await import('@tauri-apps/api/event');
      await listen<{
        id: string;
        room_id: string;
        sender_id: string;
        content: string;
        timestamp: number;
        disappearing_secs?: number;
      }>('mekaar:incoming-message', (event) => {
        const raw = event.payload;
        const msg: Message = {
          id: raw.id,
          roomId: raw.room_id,
          senderId: raw.sender_id,
          isMe: false,
          content: raw.content,
          timestamp: raw.timestamp,
          disappearingSecs: raw.disappearing_secs,
          isEncrypted: true,
        };

        if (!this.messages[msg.roomId]) {
          this.messages[msg.roomId] = [];
        }
        this.messages[msg.roomId].push(msg);

        const conv = this.conversations.find((c) => c.id === msg.roomId);
        if (conv) {
          conv.lastMessage = msg.content;
          conv.lastTimestamp = msg.timestamp;
          if (this.activeRoomId !== msg.roomId) {
            conv.unreadCount += 1;
          }
        }
      });
      this.isListening = true;
    } catch {
      // Ignored
    }
  }

  async loadConversations() {
    this.isLoading = true;
    await this.initRealtimeListener();
    try {
      const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
      if (isTauri) {
        const { invoke } = await import('@tauri-apps/api/core');
        const raw = await invoke<Array<{
          id: string;
          peer_name: string;
          peer_public_key: string;
          last_message: string;
          last_timestamp: number;
          unread_count: number;
          is_ratchet_active: boolean;
        }>>('list_conversations');

        this.conversations = raw.map((c) => ({
          id: c.id,
          peerName: c.peer_name,
          peerPublicKey: c.peer_public_key,
          lastMessage: c.last_message,
          lastTimestamp: c.last_timestamp,
          unreadCount: c.unread_count,
          isRatchetActive: c.is_ratchet_active,
        }));
      } else {
        // Fallback mockup
        this.conversations = [
          {
            id: 'room_guard_1',
            peerName: 'Guardian Utama (Budi)',
            peerPublicKey: 'x25519_pk_budi_92a83f...',
            lastMessage: 'Kunci E2EE telah diverifikasi via QR.',
            lastTimestamp: Date.now() - 60000,
            unreadCount: 0,
            isRatchetActive: true,
          },
          {
            id: 'room_sec_2',
            peerName: 'Ruang Rahasia Citra',
            peerPublicKey: 'x25519_pk_citra_47e11c...',
            lastMessage: 'Pesan ini memiliki timer 30 detik.',
            lastTimestamp: Date.now() - 300000,
            unreadCount: 2,
            isRatchetActive: true,
          },
        ];
      }
    } finally {
      this.isLoading = false;
    }
  }

  async selectRoom(roomId: string) {
    this.activeRoomId = roomId;
    if (!this.messages[roomId]) {
      await this.loadMessages(roomId);
    }
  }

  async loadMessages(roomId: string) {
    const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
    if (isTauri) {
      const { invoke } = await import('@tauri-apps/api/core');
      const raw = await invoke<Array<{
        id: string;
        room_id: string;
        sender_id: string;
        is_me: boolean;
        content: string;
        timestamp: number;
        disappearing_secs?: number;
        is_encrypted: boolean;
      }>>('list_messages', { roomId });

      this.messages[roomId] = raw.map((m) => ({
        id: m.id,
        roomId: m.room_id,
        senderId: m.sender_id,
        isMe: m.is_me,
        content: m.content,
        timestamp: m.timestamp,
        disappearingSecs: m.disappearing_secs,
        isEncrypted: m.is_encrypted,
      }));
    } else {
      // Mockup messages
      if (roomId === 'room_guard_1') {
        this.messages[roomId] = [
          {
            id: 'msg_1',
            roomId,
            senderId: 'peer_budi',
            isMe: false,
            content: 'Halo! Protokol Double Ratchet MEKAAR aktif dan terverifikasi.',
            timestamp: Date.now() - 120000,
            isEncrypted: true,
          },
          {
            id: 'msg_2',
            roomId,
            senderId: 'me',
            isMe: true,
            content: 'Kunci E2EE telah diverifikasi via QR.',
            timestamp: Date.now() - 60000,
            isEncrypted: true,
          },
        ];
      } else {
        this.messages[roomId] = [
          {
            id: 'msg_c1',
            roomId,
            senderId: 'peer_citra',
            isMe: false,
            content: 'Pesan ini memiliki timer 30 detik.',
            timestamp: Date.now() - 300000,
            disappearingSecs: 30,
            isEncrypted: true,
          },
        ];
      }
    }
  }

  async sendMessage(text: string): Promise<boolean> {
    if (!this.activeRoomId || !text.trim()) return false;
    const roomId = this.activeRoomId;

    const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
    let newMsg: Message;

    if (isTauri) {
      const { invoke } = await import('@tauri-apps/api/core');
      const raw = await invoke<{
        id: string;
        room_id: string;
        sender_id: string;
        is_me: boolean;
        content: string;
        timestamp: number;
        disappearing_secs?: number;
        is_encrypted: boolean;
      }>('send_message', {
        payload: {
          room_id: roomId,
          text: text.trim(),
          disappearing_secs: this.disappearingTimerSecs,
        },
      });

      newMsg = {
        id: raw.id,
        roomId: raw.room_id,
        senderId: raw.sender_id,
        isMe: raw.is_me,
        content: raw.content,
        timestamp: raw.timestamp,
        disappearingSecs: raw.disappearing_secs,
        isEncrypted: raw.is_encrypted,
      };
    } else {
      newMsg = {
        id: `msg_${Date.now()}`,
        roomId,
        senderId: 'me',
        isMe: true,
        content: text.trim(),
        timestamp: Date.now(),
        disappearingSecs: this.disappearingTimerSecs ?? undefined,
        isEncrypted: true,
      };
    }

    if (!this.messages[roomId]) {
      this.messages[roomId] = [];
    }
    this.messages[roomId].push(newMsg);

    // Update conversation lastMessage
    const conv = this.conversations.find((c) => c.id === roomId);
    if (conv) {
      conv.lastMessage = newMsg.content;
      conv.lastTimestamp = newMsg.timestamp;
    }

    return true;
  }

  setDisappearingTimer(secs: number | null) {
    this.disappearingTimerSecs = secs;
  }
}

export const chatStore = new ChatStore();
