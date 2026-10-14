// Paritas 1:1 dengan Mekaar_chat/lib/data/models/*.dart

export type RegisterType = 'core' | 'room' | 'sos';
export type MainTabType = 'chat' | 'map' | 'guardian' | 'sos' | 'settings';
export type ChatThemeId = 'classic' | 'forest' | 'sunset' | 'ocean' | 'midnight';

export interface ServerHealthStatus {
  online: boolean;
  service: string;
  version: string;
  httpEndpoint: string;
  wsEndpoint: string;
  wsConnected: boolean;
  mode: 'tailscale' | 'local' | 'custom';
  latencyMs: number;
  lastChecked: string;
}

export interface UserProfile {
  id: string;
  username: string;
  displayName: string;
  avatarColor: string;
  bio: string;
  publicKeyFingerprint: string;
  isOnline: boolean;
  lastSeen: string;
  twoFactorEnabled: boolean;
  accountSuspended: boolean;
}

export interface AttachmentModel {
  id: string;
  kind: 'image' | 'document' | 'voice' | 'location';
  name: string;
  sizeLabel: string;
  urlOrPreview: string;
  durationSec?: number;
}

export interface MessageModel {
  id: string;
  conversationId: string;
  senderId: string;
  senderName: string;
  text: string;
  timestamp: string;
  isSelf: boolean;
  status: 'sending' | 'sent' | 'delivered' | 'read';
  replyToId?: string;
  replyPreviewText?: string;
  replySenderName?: string;
  disappearingSeconds?: number;
  expiresAt?: string;
  reactions: Record<string, string[]>;
  attachment?: AttachmentModel;
}

export interface RoomParticipantPreferences {
  userId: string;
  displayName: string;
  role: 'admin' | 'member';
  muted: boolean;
  customNickname?: string;
}

export interface Conversation {
  id: string;
  title: string;
  subtitle: string;
  avatarColor: string;
  isGroup: boolean;
  unreadCount: number;
  lastTimestamp: string;
  isOnline: boolean;
  isTyping: boolean;
  isPinned: boolean;
  isMuted: boolean;
  disappearingSeconds: number;
  chatTheme: ChatThemeId;
  participants: RoomParticipantPreferences[];
}

export interface ChatRequestModel {
  id: string;
  fromUserId: string;
  fromUsername: string;
  fromDisplayName: string;
  avatarColor: string;
  introMessage: string;
  timestamp: string;
  fingerprint: string;
  status: 'pending' | 'accepted' | 'rejected';
}

export interface ContactModel {
  id: string;
  username: string;
  displayName: string;
  avatarColor: string;
  fingerprint: string;
  isOnline: boolean;
  isGuardian: boolean;
  isBlocked: boolean;
  notes: string;
}

export interface EmojiPackModel {
  id: string;
  title: string;
  author: string;
  installed: boolean;
  stickers: string[];
  description: string;
}

export interface CallSessionModel {
  id: string;
  conversationId: string;
  peerName: string;
  avatarColor: string;
  isVideo: boolean;
  direction: 'incoming' | 'outgoing';
  state: 'ringing' | 'connected' | 'ended';
  mutedAudio: boolean;
  mutedVideo: boolean;
  speakerOn: boolean;
  startedAt: string;
  encryptionSAS: string;
}

export interface GuardianModel {
  id: string;
  name: string;
  username: string;
  avatarColor: string;
  relation: string;
  status: 'active' | 'standby' | 'pending_request';
  canViewLiveLocation: boolean;
  canReceiveSosAlert: boolean;
  batteryPercent: number;
  lastUpdated: string;
}

export interface TripPermissionModel {
  guardianId: string;
  guardianName: string;
  allowLiveTrack: boolean;
  allowAudioStreamOnSos: boolean;
  autoNotifyArrival: boolean;
}

export interface TripModel {
  id: string;
  originName: string;
  destinationName: string;
  startedAt: string;
  endedAt?: string;
  etaMinutes: number;
  status: 'ongoing' | 'completed' | 'sos_triggered';
  activeGuardians: string[];
  progressPercent: number;
}

export interface NearbyFriendModel {
  id: string;
  displayName: string;
  username: string;
  avatarColor: string;
  distanceMeters: number;
  bearingLabel: string;
  statusMessage: string;
  latitude: number;
  longitude: number;
  lastUpdated: string;
}

export interface SavedPlaceModel {
  id: string;
  name: string;
  category: 'home' | 'office' | 'safe_zone' | 'favorite';
  address: string;
  latitude: number;
  longitude: number;
  radiusMeters: number;
  autoNotifyGuardian: boolean;
}

export interface SosSessionModel {
  active: boolean;
  countdownActive: boolean;
  countdownRemaining: number;
  startedAt?: string;
  latitude: number;
  longitude: number;
  sirenEnabled: boolean;
  shakeToTrigger: boolean;
  autoBroadcastLocation: boolean;
  emergencyMessage: string;
  notifiedGuardians: string[];
}

export interface SecurityLogModel {
  id: string;
  eventTitle: string;
  eventCategory: 'vault' | 'auth' | 'network' | 'sos' | 'screen_shield';
  deviceInfo: string;
  timestamp: string;
  severity: 'info' | 'warning' | 'critical';
}

export interface BlockedUserModel {
  id: string;
  username: string;
  displayName: string;
  blockedAt: string;
  reason: string;
}

export interface DeviceLostModel {
  armed: boolean;
  recoveryEmailOrPeer: string;
  autoWipeAfterFailedPinAttempts: number;
  remoteWipeTokenHash: string;
  lastHeartbeat: string;
}
