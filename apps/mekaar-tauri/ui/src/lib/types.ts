export type ScreenType = 'vault' | 'conversations' | 'chat' | 'sos';
export type TriRegister = 'core' | 'room' | 'sos';

export interface VaultSession {
  unlocked: boolean;
  isDuress: boolean;
  identityFingerprint: string;
  sessionToken: string;
}

export interface Conversation {
  id: string;
  peerName: string;
  peerPublicKey: string;
  lastMessage: string;
  lastTimestamp: number;
  unreadCount: number;
  isRatchetActive: boolean;
}

export interface Message {
  id: string;
  roomId: string;
  senderId: string;
  isMe: boolean;
  content: string;
  timestamp: number;
  disappearingSecs?: number;
  isEncrypted: boolean;
}

export interface Guardian {
  id: string;
  name: string;
  publicKey: string;
  isOnline: boolean;
  role: string;
}

export interface SosAlert {
  alertId: string;
  status: string;
  timestamp: number;
  dispatchedTo: string[];
}

export interface AuditLogEntry {
  timestamp: number;
  event_type: string;
  target: string;
  signature_hex: string;
}

export interface ForensicAuditReport {
  export_id: string;
  generated_at: number;
  identity_fingerprint: string;
  total_records: number;
  root_hash_sha256: string;
  integrity_verified: boolean;
  log_entries: AuditLogEntry[];
}
