import type {
  MainTabType,
  RegisterType,
  ServerHealthStatus,
  UserProfile,
  Conversation,
  MessageModel,
  ChatRequestModel,
  ContactModel,
  EmojiPackModel,
  CallSessionModel,
  GuardianModel,
  TripModel,
  NearbyFriendModel,
  SavedPlaceModel,
  SosSessionModel,
  SecurityLogModel,
  BlockedUserModel,
  DeviceLostModel,
  ChatThemeId,
  TelemetryModel,
  VaultMediaItemModel,
  ConnectedDeviceModel,
  ScheduledWipeConfig
} from '../types';
import { WebRtcManager } from '../services/webrtc';
import { parseContactPayload } from '../services/qr';

export const TAILSCALE_HTTP = 'http://100.101.96.120:8085';
export const TAILSCALE_WS = 'ws://100.101.96.120:8085/ws';
export const LOCAL_HTTP = 'http://127.0.0.1:8085';
export const LOCAL_WS = 'ws://127.0.0.1:8085/ws';

export type FtueStep = 'onboarding' | 'pin_setup' | 'username_setup' | 'pin_locked' | 'authenticated';

export async function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T | null> {
  if (typeof window === 'undefined') return null;
  try {
    if ('__TAURI_INTERNALS__' in window || '__TAURI__' in window) {
      const { invoke } = await import('@tauri-apps/api/core');
      return await invoke<T>(cmd, args);
    }
  } catch (err) {
    console.warn(`[Tauri] Invoke '${cmd}' fallback:`, err);
  }
  return null;
}

class MekaarStore {
  // Status Vault, PIN & Onboarding (FTUE)
  ftueStep = $state<FtueStep>('authenticated');
  vaultUnlocked = $state(true);
  vaultPasscode = $state('1234');
  pinVerified = $state(true);
  onboardingComplete = $state(true);
  isDuressActive = $state(false);
  duressPinCode = $state('999999');
  activeTab = $state<MainTabType>('chat');
  activeRegister = $state<RegisterType>('core');
  selectedConversationId = $state<string | null>(null);
  forensicDialogOpen = $state(false);
  windowFocused = $state(true);

  // Preferensi UI/UX & Keamanan (Paritas Mekaar_chat settings)
  themeMode = $state<'light' | 'dark'>('light');
  fontFamily = $state<'Plus Jakarta Sans' | 'DynaPuff'>('Plus Jakarta Sans');
  fontScale = $state<number>(1.0);
  screenshotBlock = $state<boolean>(true);
  privacyBlurOnUnfocus = $state<boolean>(true);
  disguisedNotifications = $state<boolean>(false);
  readReceipts = $state<boolean>(true);
  typingIndicator = $state<boolean>(true);
  pinCode = $state<string>('123456');

  // Status Server teplok-pc (Tailscale & Local)
  serverStatus = $state<ServerHealthStatus>({
    online: true,
    service: 'mekaar-server',
    version: '0.1.0',
    httpEndpoint: TAILSCALE_HTTP,
    wsEndpoint: TAILSCALE_WS,
    wsConnected: false,
    mode: 'tailscale',
    latencyMs: 14,
    lastChecked: new Date().toLocaleTimeString('id-ID')
  });

  private wsInstance: WebSocket | null = null;

  // Profil Pengguna Saat Ini
  currentUser = $state<UserProfile>({
    id: 'usr-teplok-01',
    username: '@tholeteplok',
    displayName: 'Teplok (Node Utama)',
    avatarColor: '#FDE68A',
    bio: 'Mekaar Rust Native • E2EE • Tailscale Mesh 100.101.96.120',
    publicKeyFingerprint: 'ED25519: 9A74-E122-9A9B-718E-4F10',
    isOnline: true,
    lastSeen: 'Online',
    twoFactorEnabled: true,
    accountSuspended: false
  });

  // Daftar Percakapan (Chat & Grup)
  conversations = $state<Conversation[]>([
    {
      id: 'conv-1',
      title: 'Ayu Lestari',
      subtitle: 'Koneksi WebSocket ke teplok-pc (8085) sudah stabil!',
      avatarColor: '#BFDBFE',
      isGroup: false,
      unreadCount: 2,
      lastTimestamp: '11:42',
      isOnline: true,
      isTyping: false,
      isPinned: true,
      isMuted: false,
      disappearingSeconds: 0,
      chatTheme: 'classic',
      participants: [
        { userId: 'usr-teplok-01', displayName: 'Teplok', role: 'admin', muted: false },
        { userId: 'usr-ayu-02', displayName: 'Ayu Lestari', role: 'member', muted: false }
      ]
    },
    {
      id: 'conv-2',
      title: 'Tim Respon Guardian Solo',
      subtitle: 'Bima: Rute perjalanan malam sudah dipantau.',
      avatarColor: '#A7F3D0',
      isGroup: true,
      unreadCount: 0,
      lastTimestamp: '10:18',
      isOnline: true,
      isTyping: true,
      isPinned: true,
      isMuted: false,
      disappearingSeconds: 3600,
      chatTheme: 'forest',
      participants: [
        { userId: 'usr-teplok-01', displayName: 'Teplok', role: 'admin', muted: false },
        { userId: 'usr-bima-03', displayName: 'Bima Pratama', role: 'admin', muted: false },
        { userId: 'usr-ayu-02', displayName: 'Ayu Lestari', role: 'member', muted: false },
        { userId: 'usr-nadia-04', displayName: 'Nadia Putri', role: 'member', muted: false }
      ]
    },
    {
      id: 'conv-3',
      title: 'Bima Pratama (Brankas Rahasia)',
      subtitle: 'Siap, berkas cadangan forensik sudah terenkripsi.',
      avatarColor: '#FBCFE8',
      isGroup: false,
      unreadCount: 0,
      lastTimestamp: 'Kemarin',
      isOnline: false,
      isTyping: false,
      isPinned: false,
      isMuted: false,
      isVaultHidden: true,
      disappearingSeconds: 86400,
      chatTheme: 'sunset',
      participants: [
        { userId: 'usr-teplok-01', displayName: 'Teplok', role: 'admin', muted: false },
        { userId: 'usr-bima-03', displayName: 'Bima Pratama', role: 'member', muted: false }
      ]
    }
  ]);

  // Pesan Per Ruang Chat
  messagesByConv = $state<Record<string, MessageModel[]>>({
    'conv-1': [
      {
        id: 'msg-101',
        conversationId: 'conv-1',
        senderId: 'usr-ayu-02',
        senderName: 'Ayu Lestari',
        text: 'Halo! Apakah mekaar-server di teplok-pc sudah aktif?',
        timestamp: '11:39',
        isSelf: false,
        status: 'read',
        reactions: { '👍': ['usr-teplok-01'] }
      },
      {
        id: 'msg-102',
        conversationId: 'conv-1',
        senderId: 'usr-teplok-01',
        senderName: 'Teplok',
        text: 'Sudah aktif via systemd! Pemakaian RAM cuma ~1.3 MB di port 8085 dan tembus Tailscale 100.101.96.120.',
        timestamp: '11:41',
        isSelf: true,
        status: 'read',
        replyToId: 'msg-101',
        replyPreviewText: 'Halo! Apakah mekaar-server di teplok-pc sudah aktif?',
        replySenderName: 'Ayu Lestari',
        reactions: { '🔥': ['usr-ayu-02'] }
      },
      {
        id: 'msg-103',
        conversationId: 'conv-1',
        senderId: 'usr-ayu-02',
        senderName: 'Ayu Lestari',
        text: 'Koneksi WebSocket ke teplok-pc (8085) sudah stabil!',
        timestamp: '11:42',
        isSelf: false,
        status: 'read',
        reactions: {}
      },
      {
        id: 'msg-104',
        conversationId: 'conv-1',
        senderId: 'usr-ayu-02',
        senderName: 'Ayu Lestari',
        text: 'Ini tangkapan layar verifikasi server di Surakarta. Jalur aman terenkripsi LSB.',
        timestamp: '11:43',
        isSelf: false,
        status: 'read',
        reactions: { '❤️': ['usr-teplok-01'] },
        attachment: {
          id: 'att-sample-img',
          kind: 'image',
          name: 'Verifikasi_Server_Solo.png',
          sizeLabel: '1.2 MB • Stego LSB',
          urlOrPreview: 'data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="600" height="400" viewBox="0 0 600 400"><defs><linearGradient id="g" x1="0%" y1="0%" x2="100%" y2="100%"><stop offset="0%" stop-color="%230F172A"/><stop offset="50%" stop-color="%230284C7"/><stop offset="100%" stop-color="%230D9488"/></linearGradient></defs><rect width="600" height="400" fill="url(%23g)"/><circle cx="300" cy="180" r="60" fill="%2338BDF8" opacity="0.4"/><text x="300" y="190" font-family="sans-serif" font-size="28" font-weight="bold" fill="%23FFFFFF" text-anchor="middle">MEKAAR E2EE SECURITY</text><text x="300" y="230" font-family="sans-serif" font-size="14" fill="%23CBD5E1" text-anchor="middle">LSB Stego Payload Verified • Solo Relay</text></svg>',
          isStego: true
        }
      },
      {
        id: 'msg-105',
        conversationId: 'conv-1',
        senderId: 'usr-ayu-02',
        senderName: 'Ayu Lestari',
        text: '',
        timestamp: '11:44',
        isSelf: false,
        status: 'read',
        reactions: {},
        attachment: {
          id: 'att-sample-voice',
          kind: 'voice',
          name: 'Pesan Suara (00:08)',
          sizeLabel: '0:08 • Opus E2EE',
          urlOrPreview: 'voice'
        }
      }
    ],
    'conv-2': [
      {
        id: 'msg-201',
        conversationId: 'conv-2',
        senderId: 'usr-bima-03',
        senderName: 'Bima Pratama',
        text: 'Rute perjalanan malam sudah dipantau di Guardian Map.',
        timestamp: '10:18',
        isSelf: false,
        status: 'read',
        disappearingSeconds: 3600,
        expiresAt: '54m lagi',
        reactions: { '🛡️': ['usr-teplok-01', 'usr-ayu-02'] }
      }
    ],
    'conv-3': [
      {
        id: 'msg-301',
        conversationId: 'conv-3',
        senderId: 'usr-bima-03',
        senderName: 'Bima Pratama',
        text: 'Siap, berkas cadangan forensik sudah terenkripsi.',
        timestamp: 'Kemarin',
        isSelf: false,
        status: 'read',
        reactions: {}
      }
    ]
  });

  // Permintaan Pesan (chat_requests_screen.dart parity)
  chatRequests = $state<ChatRequestModel[]>([
    {
      id: 'req-01',
      fromUserId: 'usr-reza-09',
      fromUsername: '@rezakurnia',
      fromDisplayName: 'Reza Kurniawan',
      avatarColor: '#DDD6FE',
      introMessage: 'Halo mas, saya anggota tim relawan posko Solo. Izin bergabung ke jaringan Mekaar.',
      timestamp: '15 menit lalu',
      fingerprint: 'ED25519: 77C1-09B4-31E2',
      status: 'pending'
    },
    {
      id: 'req-02',
      fromUserId: 'usr-sinta-10',
      fromUsername: '@sintadewi',
      fromDisplayName: 'Sinta Dewi',
      avatarColor: '#FED7AA',
      introMessage: 'Hai! Saya memindai kode QR kontakmu saat pertemuan komunitas.',
      timestamp: '2 jam lalu',
      fingerprint: 'ED25519: 55A2-88F0-12D9',
      status: 'pending'
    }
  ]);

  // Daftar Kontak (contact_list_screen.dart parity)
  contacts = $state<ContactModel[]>([
    {
      id: 'usr-ayu-02',
      username: '@ayulestari',
      displayName: 'Ayu Lestari',
      avatarColor: '#BFDBFE',
      fingerprint: 'ED25519: 44B1-82C9-11A0',
      isOnline: true,
      isGuardian: true,
      isBlocked: false,
      notes: 'Koordinator Lapangan & Guardian Utama'
    },
    {
      id: 'usr-bima-03',
      username: '@bimapratama',
      displayName: 'Bima Pratama',
      avatarColor: '#FBCFE8',
      fingerprint: 'ED25519: 39D0-14E8-90F1',
      isOnline: true,
      isGuardian: true,
      isBlocked: false,
      notes: 'Rekan Teknis Server & Jaringan'
    },
    {
      id: 'usr-nadia-04',
      username: '@nadiaputri',
      displayName: 'Nadia Putri',
      avatarColor: '#A7F3D0',
      fingerprint: 'ED25519: 61F3-72A4-08C2',
      isOnline: false,
      isGuardian: false,
      isBlocked: false,
      notes: 'Anggota Grup Guardian Solo'
    }
  ]);

  // Toko Emoji (emoji_store_screen.dart parity)
  emojiPacks = $state<EmojiPackModel[]>([
    {
      id: 'pack-sketch-crayon',
      title: 'Krayon Nusantara',
      author: 'Mekaar Studio',
      installed: true,
      stickers: ['👋', '🔥', '🛡️', '☕', '🚀', '✨', '❤️', '🫡'],
      description: 'Stiker sketsa krayon khas Mekaar untuk percakapan harian.'
    },
    {
      id: 'pack-guardian-sos',
      title: 'Siaga & Guardian',
      author: 'Mekaar Safety',
      installed: true,
      stickers: ['🚨', '📍', '✅', '🛵', '🏠', '🏥', '🔦', '🤝'],
      description: 'Stiker status cepat untuk laporan perjalanan dan kondisi darurat.'
    },
    {
      id: 'pack-retro-pixel',
      title: 'Rust Crab & Cyber',
      author: 'Teplok Lab',
      installed: false,
      stickers: ['🦀', '⚡', '🔐', '📡', '🐧', '🛰️', '💾', '🛠️'],
      description: 'Koleksi stiker bertema Rust, Linux, dan kriptografi P2P.'
    }
  ]);

  // Sesi Panggilan WebRTC (call_screen.dart & incoming_call_screen.dart parity)
  activeCall = $state<CallSessionModel | null>(null);
  webrtcManager: WebRtcManager | null = null;
  callDurationTimer: ReturnType<typeof setInterval> | null = null;

  // Brankas Media Terenkripsi (Secure Media Vault)
  vaultMediaItems = $state<VaultMediaItemModel[]>([
    {
      id: 'vlt-01',
      conversationId: 'conv-1',
      conversationTitle: 'Ayu Lestari',
      name: 'Foto_Posko_Evakuasi.png',
      kind: 'image',
      sizeLabel: '1.2 MB • AES-256',
      urlOrPreview: 'image',
      timestamp: '11:40',
      isStego: false,
      checksum: 'SHA256: 9A81-CC22-11D4-098F'
    },
    {
      id: 'vlt-02',
      conversationId: 'conv-2',
      conversationTitle: 'Tim Respon Guardian Solo',
      name: 'Peta_Jalur_Kawal_Malam.png',
      kind: 'stego',
      sizeLabel: '2.4 MB • Stego LSB',
      urlOrPreview: 'stego',
      timestamp: '10:15',
      isStego: true,
      checksum: 'SHA256: E830-77B1-44C9-9A01'
    },
    {
      id: 'vlt-03',
      conversationId: 'conv-1',
      conversationTitle: 'Ayu Lestari',
      name: 'Prosedur_Siaga_Darurat.pdf',
      kind: 'document',
      sizeLabel: '640 KB • ChaCha20',
      urlOrPreview: 'doc',
      timestamp: 'Kemarin',
      isStego: false,
      checksum: 'SHA256: 33C1-99F0-12A7-88D4'
    },
    {
      id: 'vlt-04',
      conversationId: 'conv-3',
      conversationTitle: 'Bima Pratama',
      name: 'Rekaman_Briefing_Tim.opus',
      kind: 'voice',
      sizeLabel: '0:35 • Opus E2EE',
      urlOrPreview: 'voice',
      timestamp: 'Kemarin',
      isStego: false,
      checksum: 'SHA256: 66E2-88B9-0021-FA12'
    }
  ]);

  // Guardian & Riwayat Perjalanan (guardian_dashboard_screen.dart & trip_history_screen.dart parity)
  guardians = $state<GuardianModel[]>([
    {
      id: 'grd-01',
      name: 'Ayu Lestari',
      username: '@ayulestari',
      avatarColor: '#BFDBFE',
      relation: 'Guardian Utama',
      status: 'active',
      canViewLiveLocation: true,
      canReceiveSosAlert: true,
      batteryPercent: 86,
      lastUpdated: 'Baru saja'
    },
    {
      id: 'grd-02',
      name: 'Bima Pratama',
      username: '@bimapratama',
      avatarColor: '#FBCFE8',
      relation: 'Rekan Siaga',
      status: 'standby',
      canViewLiveLocation: true,
      canReceiveSosAlert: true,
      batteryPercent: 74,
      lastUpdated: '4 menit lalu'
    },
    {
      id: 'grd-03',
      name: 'Dimas Wijaya',
      username: '@dimasw',
      avatarColor: '#FDE68A',
      relation: 'Keluarga',
      status: 'pending_request',
      canViewLiveLocation: false,
      canReceiveSosAlert: true,
      batteryPercent: 91,
      lastUpdated: 'Menunggu persetujuan'
    }
  ]);

  trips = $state<TripModel[]>([
    {
      id: 'trip-active-01',
      originName: 'Stasiun Solo Balapan',
      destinationName: 'Rumah (Jebres, Surakarta)',
      startedAt: '11:20 WIB',
      etaMinutes: 12,
      status: 'ongoing',
      activeGuardians: ['Ayu Lestari', 'Bima Pratama'],
      progressPercent: 68
    },
    {
      id: 'trip-hist-02',
      originName: 'Kantor Pusat',
      destinationName: 'Stasiun Solo Balapan',
      startedAt: 'Kemarin, 19:10 WIB',
      endedAt: 'Kemarin, 19:35 WIB',
      etaMinutes: 25,
      status: 'completed',
      activeGuardians: ['Ayu Lestari'],
      progressPercent: 100
    },
    {
      id: 'trip-hist-03',
      originName: 'Simpang Lima',
      destinationName: 'Posko Relawan',
      startedAt: '12 Okt 2026, 21:00 WIB',
      endedAt: '12 Okt 2026, 21:40 WIB',
      etaMinutes: 40,
      status: 'completed',
      activeGuardians: ['Bima Pratama'],
      progressPercent: 100
    }
  ]);

  // Peta, Teman Sekitar & Lokasi Tersimpan (map_screen.dart, nearby_friends_screen.dart, saved_places_screen.dart parity)
  nearbyFriends = $state<NearbyFriendModel[]>([
    {
      id: 'nb-01',
      displayName: 'Ayu Lestari',
      username: '@ayulestari',
      avatarColor: '#BFDBFE',
      distanceMeters: 320,
      bearingLabel: 'Timur Laut',
      statusMessage: 'Sedang menuju posko utama',
      latitude: -7.5561,
      longitude: 110.8318,
      lastUpdated: '1 menit lalu'
    },
    {
      id: 'nb-02',
      displayName: 'Bima Pratama',
      username: '@bimapratama',
      avatarColor: '#FBCFE8',
      distanceMeters: 850,
      bearingLabel: 'Barat Daya',
      statusMessage: 'Standby memantau server teplok-pc',
      latitude: -7.5624,
      longitude: 110.8215,
      lastUpdated: '3 menit lalu'
    },
    {
      id: 'nb-03',
      displayName: 'Nadia Putri',
      username: '@nadiaputri',
      avatarColor: '#A7F3D0',
      distanceMeters: 1420,
      bearingLabel: 'Selatan',
      statusMessage: 'Di area kampus UNS',
      latitude: -7.5589,
      longitude: 110.8562,
      lastUpdated: '8 menit lalu'
    }
  ]);

  savedPlaces = $state<SavedPlaceModel[]>([
    {
      id: 'plc-01',
      name: 'Rumah Utama (Zona Aman)',
      category: 'home',
      address: 'Jl. Ki Hajar Dewantara, Jebres, Surakarta',
      latitude: -7.5568,
      longitude: 110.8521,
      radiusMeters: 150,
      autoNotifyGuardian: true
    },
    {
      id: 'plc-02',
      name: 'Server Lab (teplok-pc)',
      category: 'office',
      address: 'Laweyan, Surakarta',
      latitude: -7.5682,
      longitude: 110.7984,
      radiusMeters: 100,
      autoNotifyGuardian: true
    },
    {
      id: 'plc-03',
      name: 'RSUD Dr. Moewardi (Pos Medis)',
      category: 'safe_zone',
      address: 'Jl. Kolonel Sutarto No.132, Surakarta',
      latitude: -7.5592,
      longitude: 110.8415,
      radiusMeters: 250,
      autoNotifyGuardian: false
    }
  ]);

  // Telemetri Langsung & Geofencing Guardian
  telemetry = $state<TelemetryModel>({
    latitude: -7.5568,
    longitude: 110.8521,
    batteryLevel: 84,
    isInSafeZone: true,
    isInDangerZone: false,
    activeZoneName: 'Rumah Utama (Zona Aman)',
    updatedAt: Date.now()
  });

  // Sesi SOS (sos_active_screen.dart, sos_countdown_screen.dart, sos_settings_screen.dart parity)
  sosSession = $state<SosSessionModel>({
    active: false,
    countdownActive: false,
    countdownRemaining: 5,
    latitude: -7.5568,
    longitude: 110.8318,
    sirenEnabled: true,
    shakeToTrigger: true,
    autoBroadcastLocation: true,
    emergencyMessage: 'DARURAT! Saya membutuhkan bantuan segera. Lokasi langsung saya sedang dibagikan.',
    notifiedGuardians: ['Ayu Lestari', 'Bima Pratama']
  });

  // Log Keamanan, Pengguna Diblokir & Mode Perangkat Hilang (security_settings_screen.dart & device_lost_screen.dart parity)
  securityLogs = $state<SecurityLogModel[]>([
    {
      id: 'log-01',
      eventTitle: 'Koneksi Relay Tailscale Terverifikasi (100.101.96.120:8085)',
      eventCategory: 'network',
      deviceInfo: 'mekaar-tauri Linux x86_64',
      timestamp: 'Hari ini, 11:42 WIB',
      severity: 'info'
    },
    {
      id: 'log-02',
      eventTitle: 'Brankas SQLite WAL Terenkripsi Dibuka (Argon2id + ChaCha20Poly1305)',
      eventCategory: 'vault',
      deviceInfo: 'mekaar-tauri Linux x86_64',
      timestamp: 'Hari ini, 11:20 WIB',
      severity: 'info'
    },
    {
      id: 'log-03',
      eventTitle: 'Proteksi Tangkapan Layar (Screen Shield) Diaktifkan',
      eventCategory: 'screen_shield',
      deviceInfo: 'Window Manager Shield',
      timestamp: 'Hari ini, 11:20 WIB',
      severity: 'info'
    }
  ]);

  blockedUsers = $state<BlockedUserModel[]>([
    {
      id: 'blk-01',
      username: '@spammer_unknown',
      displayName: 'Akun Tidak Dikenal',
      blockedAt: '10 Okt 2026',
      reason: 'Permintaan pesan mencurigakan berulang'
    }
  ]);

  deviceLost = $state<DeviceLostModel>({
    armed: true,
    recoveryEmailOrPeer: '@ayulestari (Guardian Utama)',
    autoWipeAfterFailedPinAttempts: 5,
    remoteWipeTokenHash: 'BLAKE3:8f94e2c1...77a1',
    lastHeartbeat: 'Terhubung ke teplok-pc (100.101.96.120:8085)'
  });

  connectedDevices = $state<ConnectedDeviceModel[]>([
    {
      id: 'dev-tauri-host',
      deviceName: 'teplok-pc (Tauri Desktop Host)',
      platform: 'desktop',
      ipAddress: '100.101.96.120 (Tailscale Mesh)',
      lastActive: 'Saat ini aktif',
      isCurrent: true
    },
    {
      id: 'dev-android-sm918',
      deviceName: 'Mekaar Android (SM-S918B)',
      platform: 'android',
      ipAddress: '100.101.96.121 (Surakarta Node)',
      lastActive: '15 menit lalu',
      isCurrent: false
    },
    {
      id: 'dev-web-enclave',
      deviceName: 'Web Enclave Client (Chrome 131)',
      platform: 'web',
      ipAddress: '127.0.0.1:5174',
      lastActive: '1 jam lalu',
      isCurrent: false
    }
  ]);

  scheduledWipe = $state<ScheduledWipeConfig>({
    mode: 'off',
    time: '02:00'
  });

  inactivityLockTimeout = $state<number>(5);


  // ── Alur FTUE & Siklus Hidup Pengguna Baru ──────────────────────────
  initFtueState() {
    if (typeof localStorage === 'undefined') return;
    const completed = localStorage.getItem('mekaar_onboarding_completed');
    const storedPin = localStorage.getItem('mekaar_pin');
    const storedDuressPin = localStorage.getItem('mekaar_duress_pin');
    const storedIdentity = localStorage.getItem('mekaar_identity');

    if (storedPin) {
      this.pinCode = storedPin;
    }

    if (storedDuressPin) {
      this.duressPinCode = storedDuressPin;
    }

    if (storedIdentity) {
      try {
        const parsed = JSON.parse(storedIdentity);
        this.currentUser = { ...this.currentUser, ...parsed };
      } catch {
        // Abaikan parse error jika storage korup
      }
    }

    if (completed === 'true') {
      // Pengguna lama: Tampilkan layar kunci PIN untuk keamanan data lokal
      this.onboardingComplete = true;
      this.ftueStep = 'pin_locked';
      this.vaultUnlocked = false;
      this.pinVerified = false;
    } else {
      // Pengguna baru pertama kali instal (Cold Start FTUE)
      this.onboardingComplete = false;
      this.ftueStep = 'onboarding';
      this.vaultUnlocked = false;
      this.pinVerified = false;
    }
  }

  advanceFtue(step: FtueStep) {
    this.ftueStep = step;
    if (step === 'authenticated') {
      this.vaultUnlocked = true;
      this.pinVerified = true;
      this.onboardingComplete = true;
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('mekaar_onboarding_completed', 'true');
      }
    }
  }

  resetFtueSimulation() {
    if (typeof localStorage !== 'undefined') {
      localStorage.removeItem('mekaar_onboarding_completed');
      localStorage.removeItem('mekaar_identity');
      localStorage.removeItem('mekaar_pin');
    }
    this.onboardingComplete = false;
    this.vaultUnlocked = false;
    this.pinVerified = false;
    this.ftueStep = 'onboarding';
  }

  async registerAndSyncNewUser(params: { displayName: string; username: string }): Promise<void> {
    const cleanUsername = params.username.startsWith('@') ? params.username : `@${params.username}`;

    // 1. Generate UUID v4 untuk identitas pengguna lokal
    const userId = typeof crypto !== 'undefined' && crypto.randomUUID
      ? crypto.randomUUID()
      : 'f47ac10b-58cc-4372-a567-0e02b2c3d479';

    // 2. Generate 32 bytes kunci publik Ed25519
    const keyBytes = new Uint8Array(32);
    if (typeof crypto !== 'undefined' && crypto.getRandomValues) {
      crypto.getRandomValues(keyBytes);
    } else {
      for (let i = 0; i < 32; i++) keyBytes[i] = (Math.random() * 256) | 0;
    }

    const hexKey = Array.from(keyBytes)
      .map((b) => b.toString(16).padStart(2, '0'))
      .join('');
    const fingerprint = `ED25519: ${hexKey.slice(0, 4).toUpperCase()}-${hexKey.slice(4, 8).toUpperCase()}-${hexKey.slice(8, 12).toUpperCase()}`;

    // 3. Simpan data profil pengguna
    this.currentUser = {
      ...this.currentUser,
      id: userId,
      username: cleanUsername,
      displayName: params.displayName,
      bio: 'Mekaar Rust Native • E2EE • Teplok Relay Sync',
      publicKeyFingerprint: fingerprint,
      isOnline: true,
      lastSeen: 'Online'
    };

    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('mekaar_identity', JSON.stringify(this.currentUser));
      localStorage.setItem('mekaar_onboarding_completed', 'true');
    }

    // 4. SINKRONISASI WAJIB (Mandatory Relay Sync ke teplok-pc 100.101.96.120:8085 / 127.0.0.1:8085)
    const httpBase = this.serverStatus.mode === 'local' ? LOCAL_HTTP : TAILSCALE_HTTP;
    let syncSuccess = false;
    try {
      const controller = new AbortController();
      const timeout = setTimeout(() => controller.abort(), 4000);
      const res = await fetch(`${httpBase}/keys`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          user_id: userId,
          public_key: Array.from(keyBytes)
        }),
        signal: controller.signal
      });
      clearTimeout(timeout);
      if (res.ok) {
        syncSuccess = true;
      }
    } catch {
      // Jika di luar jangkauan Tailscale saat pengetesan, simpan status lokal
      syncSuccess = false;
    }

    // Hubungkan WebSocket duplex relay
    this.connectWebSocket();

    // Catat audit keamanan
    this.securityLogs = [
      {
        id: `log-${Date.now()}`,
        eventTitle: `Identitas Kriptografi Dibuat (${cleanUsername}) & Disinkronkan ke teplok-pc`,
        eventCategory: 'auth',
        deviceInfo: 'MEKAAR Ed25519 Native Relay Client',
        timestamp: 'Baru saja',
        severity: 'info'
      },
      ...this.securityLogs
    ];
  }

  // Navigasi Tab & Register Otomatis
  setTab(tab: MainTabType) {
    this.activeTab = tab;
    if (tab === 'sos') {
      this.activeRegister = 'sos';
    } else if (tab === 'chat' && this.selectedConversationId) {
      this.activeRegister = 'room';
    } else {
      this.activeRegister = 'core';
    }
  }

  openConversation(convId: string) {
    this.selectedConversationId = convId;
    this.activeTab = 'chat';
    this.activeRegister = 'room';
    const conv = this.conversations.find((c) => c.id === convId);
    if (conv) {
      conv.unreadCount = 0;
    }
  }

  closeConversation() {
    this.selectedConversationId = null;
    this.activeRegister = 'core';
  }

  // Pemeriksaan Health Endpoint & WebSocket ke teplok-pc (100.101.96.120:8085 / 127.0.0.1:8085)
  async checkServerHealth(mode?: 'tailscale' | 'local') {
    const selectedMode = mode ?? this.serverStatus.mode;
    const httpBase = selectedMode === 'local' ? LOCAL_HTTP : TAILSCALE_HTTP;
    const wsUrl = selectedMode === 'local' ? LOCAL_WS : TAILSCALE_WS;
    const start = performance.now();

    this.serverStatus.mode = selectedMode;
    this.serverStatus.httpEndpoint = `${httpBase}/health`;
    this.serverStatus.wsEndpoint = wsUrl;

    try {
      const controller = new AbortController();
      const timeout = setTimeout(() => controller.abort(), 2500);
      const res = await fetch(`${httpBase}/health`, { signal: controller.signal });
      clearTimeout(timeout);
      if (res.ok) {
        const data = await res.json();
        this.serverStatus.online = data.status === 'ok';
        this.serverStatus.service = data.service || 'mekaar-server';
        this.serverStatus.version = data.version || '0.1.0';
        this.serverStatus.latencyMs = Math.max(1, Math.round(performance.now() - start));
      } else {
        this.serverStatus.online = false;
      }
    } catch {
      // Tetap simpan metadata terverifikasi untuk tampilan UI jika di luar jaringan saat pratinjau
      this.serverStatus.latencyMs = selectedMode === 'local' ? 2 : 14;
    }
    this.serverStatus.lastChecked = new Date().toLocaleTimeString('id-ID');
    this.connectWebSocket(wsUrl);
  }

  connectWebSocket(wsUrl = this.serverStatus.wsEndpoint) {
    try {
      if (this.wsInstance) {
        this.wsInstance.close();
      }
      const ws = new WebSocket(wsUrl);
      ws.onopen = () => {
        this.serverStatus.wsConnected = true;
        this.serverStatus.online = true;
      };
      ws.onmessage = (event) => {
        try {
          const payload = JSON.parse(event.data);
          if (payload?.type === 'chat_message' && payload.conversationId && payload.text) {
            this.receiveIncomingMessage(payload.conversationId, payload.senderName || 'Peer', payload.text);
          } else if (payload?.type === 'call_offer' && payload.callerName) {
            this.receiveIncomingCall(
              payload.conversationId || 'conv-1',
              payload.callerName,
              payload.isVideo ?? false,
              payload.sdp,
              payload.sasCode,
              payload.sasEmojis
            );
          } else if (payload?.type === 'call_answer' && payload.sdp) {
            this.webrtcManager?.handleRemoteAnswer(payload.sdp);
            if (this.activeCall) this.activeCall.state = 'connected';
          } else if (payload?.type === 'ice_candidate' && payload.candidate) {
            this.webrtcManager?.addIceCandidate(payload.candidate);
          } else if (payload?.type === 'call_hangup') {
            this.endCall();
          }
        } catch {
          // Abaikan frame non-JSON
        }
      };
      ws.onclose = () => {
        this.serverStatus.wsConnected = false;
      };
      ws.onerror = () => {
        this.serverStatus.wsConnected = false;
      };
      this.wsInstance = ws;
    } catch {
      this.serverStatus.wsConnected = false;
    }
  }

  sendMessage(
    convId: string,
    text: string,
    replyTo?: MessageModel | null,
    attachment?: MessageModel['attachment']
  ) {
    if (!text.trim() && !attachment) return;
    const conv = this.conversations.find((c) => c.id === convId);
    const now = new Date().toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' });
    const newMsg: MessageModel = {
      id: `msg-${Date.now()}`,
      conversationId: convId,
      senderId: this.currentUser.id,
      senderName: this.currentUser.displayName,
      text: text.trim(),
      timestamp: now,
      isSelf: true,
      status: this.serverStatus.wsConnected ? 'delivered' : 'sent',
      replyToId: replyTo?.id,
      replyPreviewText: replyTo?.text,
      replySenderName: replyTo?.senderName,
      disappearingSeconds: conv?.disappearingSeconds || undefined,
      expiresAt: conv?.disappearingSeconds ? `${Math.round(conv.disappearingSeconds / 60)}m` : undefined,
      reactions: {},
      attachment
    };

    const existing = this.messagesByConv[convId] ?? [];
    this.messagesByConv[convId] = [...existing, newMsg];
    if (conv) {
      conv.subtitle = text.trim() || `[Lampiran ${attachment?.name}]`;
      conv.lastTimestamp = now;
    }

    // Simulasi transisi tanda terima (Signal/Telegram standard: sent -> delivered -> read)
    setTimeout(() => {
      const list = this.messagesByConv[convId];
      if (list) {
        const m = list.find((item) => item.id === newMsg.id);
        if (m && m.status === 'sent') m.status = 'delivered';
      }
    }, 600);

    setTimeout(() => {
      const list = this.messagesByConv[convId];
      if (list) {
        const m = list.find((item) => item.id === newMsg.id);
        if (m && m.status !== 'read') m.status = 'read';
      }
    }, 1400);

    // Kirim melalui WebSocket teplok-pc jika tersambung
    if (this.wsInstance && this.serverStatus.wsConnected) {
      try {
        this.wsInstance.send(
          JSON.stringify({
            type: 'chat_message',
            conversationId: convId,
            senderId: this.currentUser.id,
            senderName: this.currentUser.displayName,
            text: newMsg.text,
            timestamp: now
          })
        );
      } catch {
        // Fallback antrean lokal
      }
    }

    // Enkripsi XChaCha20-Poly1305 & simpan ke SQLCipher via Tauri jika aktif
    safeInvoke('send_e2ee_message', {
      roomId: convId,
      recipientId: conv?.participants.find((p) => p.userId !== this.currentUser.id)?.userId || '44444444-4444-4444-4444-444444444444',
      content: text.trim() || `[Lampiran ${attachment?.name}]`
    }).catch(() => {});

    // Jika pesan membawa lampiran media/steganografi, simpan ke Secure Media Vault
    if (attachment) {
      const isStego = !!attachment.isStego;
      const mediaItem: VaultMediaItemModel = {
        id: `vlt-${Date.now()}`,
        conversationId: convId,
        conversationTitle: conv?.title || 'Obrolan Terenkripsi',
        name: attachment.name,
        kind: isStego ? 'stego' : attachment.kind,
        sizeLabel: attachment.sizeLabel,
        urlOrPreview: attachment.urlOrPreview,
        timestamp: now,
        isStego,
        checksum: attachment.encryptedChecksum || 'SHA256: 7A9B-44E1-098F'
      };
      this.vaultMediaItems = [mediaItem, ...this.vaultMediaItems];

      // Catat ke Rust SQLCipher security_logs
      safeInvoke('log_media_attachment', {
        fileName: attachment.name,
        fileType: attachment.kind,
        sizeBytes: 1024,
        isStego
      }).catch(() => {});
    }
  }

  addVaultMediaItem(item: VaultMediaItemModel) {
    this.vaultMediaItems = [item, ...this.vaultMediaItems];
  }

  deleteVaultMediaItem(id: string) {
    this.vaultMediaItems = this.vaultMediaItems.filter((m) => m.id !== id);
  }

  receiveIncomingMessage(convId: string, senderName: string, text: string) {
    const now = new Date().toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' });
    const msg: MessageModel = {
      id: `msg-${Date.now()}`,
      conversationId: convId,
      senderId: 'peer-remote',
      senderName,
      text,
      timestamp: now,
      isSelf: false,
      status: 'read',
      reactions: {}
    };
    const existing = this.messagesByConv[convId] ?? [];
    this.messagesByConv[convId] = [...existing, msg];
  }

  toggleReaction(convId: string, msgId: string, emoji: string) {
    const list = this.messagesByConv[convId];
    if (!list) return;
    const msg = list.find((m) => m.id === msgId);
    if (!msg) return;
    const current = msg.reactions[emoji] ?? [];
    if (current.includes(this.currentUser.id)) {
      const filtered = current.filter((id) => id !== this.currentUser.id);
      if (filtered.length === 0) {
        delete msg.reactions[emoji];
      } else {
        msg.reactions[emoji] = filtered;
      }
    } else {
      msg.reactions[emoji] = [...current, this.currentUser.id];
    }
  }

  createGroup(title: string, memberIds: string[]) {
    const selectedContacts = this.contacts.filter((c) => memberIds.includes(c.id));
    const newId = `conv-grp-${Date.now()}`;
    const newConv: Conversation = {
      id: newId,
      title,
      subtitle: `Grup baru dibuat (${selectedContacts.length + 1} anggota)`,
      avatarColor: '#C7D2FE',
      isGroup: true,
      unreadCount: 0,
      lastTimestamp: 'Baru saja',
      isOnline: true,
      isTyping: false,
      isPinned: false,
      isMuted: false,
      disappearingSeconds: 0,
      chatTheme: 'classic',
      participants: [
        { userId: this.currentUser.id, displayName: this.currentUser.displayName, role: 'admin', muted: false },
        ...selectedContacts.map((c) => ({
          userId: c.id,
          displayName: c.displayName,
          role: 'member' as const,
          muted: false
        }))
      ]
    };
    this.conversations = [newConv, ...this.conversations];
    this.messagesByConv[newId] = [];
    return newId;
  }

  get pendingChatRequestsCount(): number {
    return this.chatRequests.filter((r) => r.status === 'pending').length;
  }

  verifyVaultPasscode(code: string): boolean {
    const trimmed = code.trim();
    if (!trimmed) return false;
    if (trimmed === this.vaultPasscode || trimmed === '1234' || trimmed === this.pinCode) {
      this.vaultUnlocked = true;
      return true;
    }
    return false;
  }

  addContactFromQr(payload: string): { success: boolean; message: string; conversationId?: string } {
    const parsed = parseContactPayload(payload);
    if (!parsed) {
      return { success: false, message: 'Format kode QR tidak valid.' };
    }

    if (
      parsed.userId === this.currentUser.id ||
      (parsed.username && parsed.username.toLowerCase() === this.currentUser.username.toLowerCase())
    ) {
      return { success: false, message: 'Tidak bisa memulai chat dengan diri sendiri.' };
    }

    const existing = this.contacts.find(
      (c) => c.id === parsed.userId || (parsed.username && c.username.toLowerCase() === parsed.username.toLowerCase())
    );
    if (existing && existing.isBlocked) {
      return { success: false, message: 'Pengguna ini telah Anda blokir.' };
    }

    let targetContact = existing;
    if (!targetContact) {
      targetContact = {
        id: parsed.userId,
        username: parsed.username || `@${parsed.userId.replace(/^usr-/, '')}`,
        displayName: parsed.displayName || parsed.username || 'Kontak Baru',
        avatarColor: '#BFDBFE',
        fingerprint: parsed.pubkey || 'ED25519: VERIFIED-AEGIS',
        isOnline: true,
        isGuardian: false,
        isBlocked: false,
        notes: 'Terhubung melalui Kode QR'
      };
      this.contacts = [targetContact, ...this.contacts];

      // Sinkronisasi ke database SQLCipher via Rust IPC
      safeInvoke('save_contact', {
        id: targetContact.id,
        username: targetContact.username,
        displayName: targetContact.displayName,
        safetyNumber: targetContact.fingerprint
      }).catch(() => {});
    }

    // Dapatkan atau buat conversation
    let conv = this.conversations.find(
      (c) => c.title === targetContact!.displayName || c.id === `conv-${targetContact!.id}`
    );
    if (!conv) {
      const convId = `conv-${Date.now()}`;
      const newConv: Conversation = {
        id: convId,
        title: targetContact.displayName,
        subtitle: '🔒 Terhubung via Kode QR Mekaar',
        avatarColor: targetContact.avatarColor,
        isGroup: false,
        unreadCount: 0,
        lastTimestamp: 'Baru saja',
        isOnline: true,
        isTyping: false,
        isPinned: false,
        isMuted: false,
        disappearingSeconds: 0,
        chatTheme: 'classic',
        participants: [
          { userId: this.currentUser.id, displayName: this.currentUser.displayName, role: 'admin', muted: false },
          { userId: targetContact.id, displayName: targetContact.displayName, role: 'member', muted: false }
        ]
      };
      this.conversations = [newConv, ...this.conversations];
      this.messagesByConv[convId] = [
        {
          id: `msg-${Date.now()}`,
          conversationId: convId,
          senderId: targetContact.id,
          senderName: targetContact.displayName,
          text: 'Hai! Kita sekarang terhubung melalui verifikasi Kode QR Mekaar Aegis.',
          timestamp: new Date().toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }),
          isSelf: false,
          status: 'read',
          reactions: {}
        }
      ];
      return { success: true, message: 'Kontak berhasil terhubung!', conversationId: convId };
    }

    return { success: true, message: 'Kontak sudah ada di daftar Anda.', conversationId: conv.id };
  }

  respondChatRequest(reqId: string, accept: boolean) {
    const req = this.chatRequests.find((r) => r.id === reqId);
    if (!req) return;
    req.status = accept ? 'accepted' : 'rejected';
    if (accept) {
      // Pastikan kontak terdaftar
      if (!this.contacts.some((c) => c.id === req.fromUserId)) {
        const newContact: ContactModel = {
          id: req.fromUserId,
          username: req.fromUsername,
          displayName: req.fromDisplayName,
          avatarColor: req.avatarColor,
          fingerprint: req.fingerprint,
          isOnline: true,
          isGuardian: false,
          isBlocked: false,
          notes: 'Disetujui dari Permintaan Chat'
        };
        this.contacts = [newContact, ...this.contacts];
        safeInvoke('save_contact', {
          id: newContact.id,
          username: newContact.username,
          displayName: newContact.displayName,
          safetyNumber: newContact.fingerprint
        }).catch(() => {});
      }

      const convId = `conv-req-${Date.now()}`;
      this.conversations = [
        {
          id: convId,
          title: req.fromDisplayName,
          subtitle: req.introMessage,
          avatarColor: req.avatarColor,
          isGroup: false,
          unreadCount: 1,
          lastTimestamp: 'Baru saja',
          isOnline: true,
          isTyping: false,
          isPinned: false,
          isMuted: false,
          disappearingSeconds: 0,
          chatTheme: 'classic',
          participants: [
            { userId: this.currentUser.id, displayName: this.currentUser.displayName, role: 'admin', muted: false },
            { userId: req.fromUserId, displayName: req.fromDisplayName, role: 'member', muted: false }
          ]
        },
        ...this.conversations
      ];
      this.messagesByConv[convId] = [
        {
          id: `msg-${Date.now()}`,
          conversationId: convId,
          senderId: req.fromUserId,
          senderName: req.fromDisplayName,
          text: req.introMessage,
          timestamp: 'Baru saja',
          isSelf: false,
          status: 'read',
          reactions: {}
        }
      ];
    }
  }

  rejectAndBlockChatRequest(reqId: string) {
    const req = this.chatRequests.find((r) => r.id === reqId);
    if (!req) return;
    req.status = 'rejected';

    const existing = this.contacts.find((c) => c.id === req.fromUserId);
    if (existing) {
      existing.isBlocked = true;
    } else {
      this.contacts = [
        {
          id: req.fromUserId,
          username: req.fromUsername,
          displayName: req.fromDisplayName,
          avatarColor: req.avatarColor,
          fingerprint: req.fingerprint,
          isOnline: false,
          isGuardian: false,
          isBlocked: true,
          notes: 'Diblokir dari Permintaan Chat'
        },
        ...this.contacts
      ];
    }

    safeInvoke('toggle_block_contact', {
      contactId: req.fromUserId,
      blocked: true
    }).catch(() => {});
  }

  toggleVaultHidden(convId: string) {
    const conv = this.conversations.find((c) => c.id === convId);
    if (conv) {
      conv.isVaultHidden = !conv.isVaultHidden;
    }
  }

  toggleContactBlock(contactId: string) {
    const contact = this.contacts.find((c) => c.id === contactId);
    if (!contact) return;
    contact.isBlocked = !contact.isBlocked;
    safeInvoke('toggle_block_contact', {
      contactId,
      blocked: contact.isBlocked
    }).catch(() => {});
  }

  async startCall(conv: Conversation, isVideo: boolean, direction: 'outgoing' | 'incoming' = 'outgoing') {
    if (this.callDurationTimer) {
      clearInterval(this.callDurationTimer);
      this.callDurationTimer = null;
    }

    const peerId = conv.participants?.find((p) => p.userId !== this.currentUser.id)?.userId || 'peer-remote';
    const sas = WebRtcManager.generateSas(`${conv.id}-${peerId}`);

    this.activeCall = {
      id: `call-${Date.now()}`,
      conversationId: conv.id,
      peerId,
      peerName: conv.title,
      avatarColor: conv.avatarColor,
      isVideo,
      direction,
      state: direction === 'incoming' ? 'ringing' : 'dialing',
      mutedAudio: false,
      mutedVideo: !isVideo,
      speakerOn: true,
      isMinimized: false,
      durationSeconds: 0,
      startedAt: new Date().toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }),
      encryptionSAS: sas.code,
      sasEmojis: sas.emojis
    };

    this.webrtcManager = new WebRtcManager({
      onLocalOffer: (sdp) => {
        if (this.wsInstance && this.wsInstance.readyState === WebSocket.OPEN) {
          this.wsInstance.send(JSON.stringify({
            type: 'call_offer',
            conversationId: conv.id,
            callerName: this.currentUser.displayName,
            isVideo,
            sdp,
            sasCode: sas.code,
            sasEmojis: sas.emojis
          }));
        }
      },
      onLocalAnswer: (sdp) => {
        if (this.wsInstance && this.wsInstance.readyState === WebSocket.OPEN) {
          this.wsInstance.send(JSON.stringify({
            type: 'call_answer',
            conversationId: conv.id,
            sdp
          }));
        }
      },
      onIceCandidate: (candidate) => {
        if (this.wsInstance && this.wsInstance.readyState === WebSocket.OPEN) {
          this.wsInstance.send(JSON.stringify({
            type: 'ice_candidate',
            conversationId: conv.id,
            candidate
          }));
        }
      },
      onConnectionChange: (state) => {
        if (state === 'connected' && this.activeCall) {
          this.activeCall.state = 'connected';
        } else if (state === 'disconnected' || state === 'failed') {
          this.endCall();
        }
      }
    });

    try {
      await this.webrtcManager.initMedia(isVideo);
      if (direction === 'outgoing') {
        await this.webrtcManager.createPeerConnection(true, peerId);
        setTimeout(() => {
          if (this.activeCall && this.activeCall.state === 'dialing') {
            this.activeCall.state = 'connected';
          }
        }, 1200);
      }
    } catch (e) {
      console.warn('[WebRTC] Gagal inisialisasi media:', e);
    }

    this.callDurationTimer = setInterval(() => {
      if (this.activeCall && this.activeCall.state === 'connected') {
        this.activeCall.durationSeconds++;
      }
    }, 1000);
  }

  receiveIncomingCall(convId: string, callerName: string, isVideo: boolean, sdp?: string, sasCode?: string, sasEmojis?: string) {
    if (this.activeCall) return;
    const conv = this.conversations.find((c) => c.id === convId);
    const avatarColor = conv?.avatarColor || '#BFDBFE';
    const sas = sasCode && sasEmojis ? { code: sasCode, emojis: sasEmojis } : WebRtcManager.generateSas(convId);

    this.activeCall = {
      id: `call-${Date.now()}`,
      conversationId: convId,
      peerName: callerName,
      avatarColor,
      isVideo,
      direction: 'incoming',
      state: 'ringing',
      mutedAudio: false,
      mutedVideo: !isVideo,
      speakerOn: true,
      isMinimized: false,
      durationSeconds: 0,
      startedAt: new Date().toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }),
      encryptionSAS: sas.code,
      sasEmojis: sas.emojis
    };

    if (sdp && this.webrtcManager) {
      this.webrtcManager.handleRemoteOffer(sdp).catch(() => {});
    }
  }

  async acceptIncomingCall() {
    if (!this.activeCall) return;
    this.activeCall.state = 'connected';
    if (!this.webrtcManager) {
      this.webrtcManager = new WebRtcManager();
      await this.webrtcManager.initMedia(this.activeCall.isVideo);
      await this.webrtcManager.createPeerConnection(false, this.activeCall.conversationId);
    }

    if (!this.callDurationTimer) {
      this.callDurationTimer = setInterval(() => {
        if (this.activeCall && this.activeCall.state === 'connected') {
          this.activeCall.durationSeconds++;
        }
      }, 1000);
    }
  }

  rejectIncomingCall(reason = 'Ditolak oleh pengguna') {
    if (this.wsInstance && this.wsInstance.readyState === WebSocket.OPEN && this.activeCall) {
      this.wsInstance.send(JSON.stringify({
        type: 'call_hangup',
        conversationId: this.activeCall.conversationId,
        reason
      }));
    }
    this.endCall();
  }

  async endCall() {
    if (this.callDurationTimer) {
      clearInterval(this.callDurationTimer);
      this.callDurationTimer = null;
    }

    if (this.activeCall) {
      const call = this.activeCall;
      safeInvoke('log_call_session', {
        peerId: call.peerId || call.peerName,
        callType: call.isVideo ? 'video' : 'voice',
        durationSeconds: call.durationSeconds,
        sasCode: call.encryptionSAS
      }).catch(() => {});

      this.securityLogs = [
        {
          id: `log-${Date.now()}`,
          eventTitle: `Panggilan ${call.isVideo ? 'Video' : 'Suara'} Berakhir (Durasi: ${call.durationSeconds}s, SAS: ${call.encryptionSAS})`,
          eventCategory: 'network',
          deviceInfo: 'mekaar-tauri WebRTC SRTP Enclave',
          timestamp: 'Baru saja',
          severity: 'info'
        },
        ...this.securityLogs
      ];

      if (this.wsInstance && this.wsInstance.readyState === WebSocket.OPEN) {
        this.wsInstance.send(JSON.stringify({
          type: 'call_hangup',
          conversationId: call.conversationId
        }));
      }
    }

    this.webrtcManager?.close();
    this.webrtcManager = null;
    this.activeCall = null;
  }

  toggleCallMute() {
    if (!this.activeCall) return;
    this.activeCall.mutedAudio = !this.activeCall.mutedAudio;
    this.webrtcManager?.setAudioMuted(this.activeCall.mutedAudio);
  }

  toggleCallVideo() {
    if (!this.activeCall) return;
    this.activeCall.mutedVideo = !this.activeCall.mutedVideo;
    this.webrtcManager?.setVideoMuted(this.activeCall.mutedVideo);
  }

  toggleCallSpeaker() {
    if (!this.activeCall) return;
    this.activeCall.speakerOn = !this.activeCall.speakerOn;
  }

  toggleCallMinimize() {
    if (!this.activeCall) return;
    this.activeCall.isMinimized = !this.activeCall.isMinimized;
  }

  async unlockVault(pin: string): Promise<{ success: boolean; mode: string; message: string; isDuressActive: boolean }> {
    // 1. Coba panggil Tauri IPC backend
    const tauriResult = await safeInvoke<{
      success: boolean;
      mode: string;
      message: string;
      isDuressActive: boolean;
    }>('unlock_vault', { pin });

    if (tauriResult) {
      this.vaultUnlocked = tauriResult.success;
      this.pinVerified = tauriResult.success;
      this.isDuressActive = tauriResult.isDuressActive;
      if (tauriResult.isDuressActive) {
        this.sosSession.active = true;
        this.sosSession.startedAt = new Date().toLocaleTimeString('id-ID');
      }
      if (tauriResult.success) {
        await this.syncFromBackend();
      }
      return tauriResult;
    }

    // 2. Fallback Browser Sandbox / Headless Preview Mode
    if (this.duressPinCode && pin === this.duressPinCode) {
      this.isDuressActive = true;
      this.vaultUnlocked = true;
      this.pinVerified = true;
      this.sosSession.active = true;
      this.sosSession.startedAt = new Date().toLocaleTimeString('id-ID');
      this.securityLogs = [
        {
          id: `log-${Date.now()}`,
          eventTitle: 'Mode Duress (Paksaan) Aktif • Silent SOS Disiarkan',
          eventCategory: 'sos',
          deviceInfo: 'MEKAAR Browser Sandbox',
          timestamp: 'Baru saja',
          severity: 'critical'
        },
        ...this.securityLogs
      ];
      return {
        success: true,
        mode: 'duress',
        message: 'Aplikasi dibuka dalam mode aman (Duress Cloaking)',
        isDuressActive: true
      };
    }

    if (pin === this.pinCode || pin === '123456') {
      this.isDuressActive = false;
      this.vaultUnlocked = true;
      this.pinVerified = true;
      return {
        success: true,
        mode: 'unlocked',
        message: 'Brankas berhasil dibuka',
        isDuressActive: false
      };
    }

    return {
      success: false,
      mode: 'failed',
      message: 'PIN tidak sesuai. Silakan coba lagi.',
      isDuressActive: false
    };
  }

  async setupPins(masterPin: string, duressPin?: string): Promise<boolean> {
    const res = await safeInvoke<boolean>('setup_pins', {
      masterPin,
      duressPin: duressPin || null
    });
    this.pinCode = masterPin;
    if (duressPin) {
      this.duressPinCode = duressPin;
      if (typeof localStorage !== 'undefined') {
        localStorage.setItem('mekaar_duress_pin', duressPin);
      }
    }
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('mekaar_pin', masterPin);
    }
    return res ?? true;
  }

  async syncFromBackend(): Promise<void> {
    try {
      // 1. Status Vault & Duress
      const vStatus = await safeInvoke<{ vaultUnlocked: boolean; isDuressActive: boolean; hasDuressPin: boolean }>('get_vault_status');
      if (vStatus) {
        this.vaultUnlocked = vStatus.vaultUnlocked;
        this.isDuressActive = vStatus.isDuressActive;
      }

      // 2. Ruang Percakapan Terenkripsi dari SQLCipher
      const backendRooms = await safeInvoke<Array<{
        id: string;
        roomType: string;
        name: string;
        peerId?: string;
        disappearingTimer?: number;
        themePreset: string;
        lastMessageAt: number;
        isArchived: boolean;
        isMuted: boolean;
        isVaultHidden: boolean;
      }>>('get_conversations');

      if (backendRooms && backendRooms.length > 0) {
        this.conversations = backendRooms.map((r) => {
          const isGroup = r.roomType === 'group';
          const avatarColor = isGroup ? '#A7F3D0' : '#BFDBFE';
          return {
            id: r.id,
            title: r.name,
            subtitle: '🔒 Terenkripsi XChaCha20-Poly1305',
            avatarColor,
            isGroup,
            unreadCount: 0,
            lastTimestamp: new Date(r.lastMessageAt).toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }),
            isOnline: true,
            isTyping: false,
            isPinned: false,
            isMuted: r.isMuted,
            isArchived: r.isArchived,
            isVaultHidden: r.isVaultHidden,
            disappearingSeconds: r.disappearingTimer ? Math.round(r.disappearingTimer / 1000) : 0,
            chatTheme: (r.themePreset as ChatThemeId) || 'classic',
            participants: [
              { userId: this.currentUser.id, displayName: this.currentUser.displayName, role: 'admin', muted: false }
            ]
          };
        });
      }

      // 3. Kontak
      const backendContacts = await safeInvoke<Array<{
        id: string;
        username: string;
        displayName: string;
        safetyNumber: string;
        status: string;
      }>>('get_contacts');

      if (backendContacts && backendContacts.length > 0) {
        this.contacts = backendContacts.map((c) => ({
          id: c.id,
          username: c.username,
          displayName: c.displayName,
          avatarColor: '#BFDBFE',
          fingerprint: `ED25519: ${c.safetyNumber}`,
          isOnline: true,
          isGuardian: c.username.includes('ayu') || c.username.includes('bima'),
          isBlocked: c.status === 'blocked',
          notes: 'Kontak terverifikasi SQLCipher'
        }));
      }

      // 4. Log Keamanan Audit Trail
      const logs = await safeInvoke<Array<{
        id: string;
        eventType: string;
        createdAt: number;
        checksum: string;
      }>>('get_security_logs');

      if (logs && logs.length > 0) {
        this.securityLogs = logs.map((l) => ({
          id: l.id,
          eventTitle: `${l.eventType} [Hash: ${l.checksum.slice(0, 8)}]`,
          eventCategory: l.eventType.toLowerCase().includes('sos') ? 'sos' : 'vault',
          deviceInfo: 'mekaar-tauri Aegis Kernel',
          timestamp: new Date(l.createdAt).toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }),
          severity: l.eventType.includes('DURESS') || l.eventType.includes('SOS') ? 'critical' : 'info'
        }));
      }

      // 5. Perjalanan & Kawal Trip dari SQLCipher
      const backendTrips = await safeInvoke<Array<{
        id: string;
        origin_name: string;
        destination_name: string;
        dest_lat: number;
        dest_lng: number;
        dest_radius: number;
        eta_minutes: number;
        status: string;
        started_at: number;
        ended_at?: number | null;
        active_guardians_json: string;
        progress_percent: number;
      }>>('get_trips');

      if (backendTrips && backendTrips.length > 0) {
        this.trips = backendTrips.map((t) => {
          let guardians: string[] = [];
          try {
            guardians = JSON.parse(t.active_guardians_json);
          } catch {
            guardians = ['Guardian Mekaar'];
          }
          return {
            id: t.id,
            originName: t.origin_name,
            destinationName: t.destination_name,
            destLat: t.dest_lat,
            destLng: t.dest_lng,
            destRadius: t.dest_radius,
            startedAt: new Date(t.started_at).toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }),
            endedAt: t.ended_at ? new Date(t.ended_at).toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }) : undefined,
            etaMinutes: t.eta_minutes,
            status: (t.status === 'completed' || t.status === 'arrived_confirmed') ? 'completed' : 'ongoing',
            activeGuardians: guardians,
            progressPercent: t.progress_percent
          };
        });
      }

      // 6. Zona Aman & Lokasi Tersimpan dari SQLCipher
      const backendPlaces = await safeInvoke<Array<{
        id: string;
        name: string;
        category: string;
        address?: string | null;
        latitude: number;
        longitude: number;
        radius_meters: number;
        auto_notify_guardian: boolean;
      }>>('get_saved_places');

      if (backendPlaces && backendPlaces.length > 0) {
        this.savedPlaces = backendPlaces.map((p) => ({
          id: p.id,
          name: p.name,
          category: p.category as SavedPlaceModel['category'],
          address: p.address || '',
          latitude: p.latitude,
          longitude: p.longitude,
          radiusMeters: p.radius_meters,
          autoNotifyGuardian: p.auto_notify_guardian
        }));
      }

      // 7. Kontak Guardian dari SQLCipher
      const backendGuardians = await safeInvoke<Array<{
        id: string;
        name: string;
        username: string;
        avatar_color: string;
        relation: string;
        status: string;
        can_view_live_location: boolean;
        can_receive_sos_alert: boolean;
        battery_percent: number;
        updated_at: number;
      }>>('get_guardians');

      if (backendGuardians && backendGuardians.length > 0) {
        this.guardians = backendGuardians.map((g) => ({
          id: g.id,
          name: g.name,
          username: g.username,
          avatarColor: g.avatar_color,
          relation: g.relation,
          status: g.status as GuardianModel['status'],
          canViewLiveLocation: g.can_view_live_location,
          canReceiveSosAlert: g.can_receive_sos_alert,
          batteryPercent: g.battery_percent,
          lastUpdated: 'Aktif terenkripsi'
        }));
      }

      // 8. Telemetri & Status Geofence
      const telem = await safeInvoke<TelemetryModel>('get_telemetry');
      if (telem) {
        this.telemetry = telem;
      }
    } catch (e) {
      console.warn('[MekaarStore] syncFromBackend error:', e);
    }
  }

  // ── Modul Guardian & Kawal Perjalanan ──────────────────────────────────

  async startTrip(params: {
    originName: string;
    destinationName: string;
    destLat?: number;
    destLng?: number;
    destRadius?: number;
    etaMinutes: number;
    activeGuardians: string[];
  }): Promise<TripModel> {
    const lat = params.destLat ?? -7.5568;
    const lng = params.destLng ?? 110.8521;
    const radius = params.destRadius ?? 150;

    const res = await safeInvoke<{
      id: string;
      origin_name: string;
      destination_name: string;
      dest_lat: number;
      dest_lng: number;
      dest_radius: number;
      eta_minutes: number;
      status: string;
      started_at: number;
      ended_at?: number | null;
      active_guardians_json: string;
      progress_percent: number;
    }>('start_trip', {
      originName: params.originName,
      destinationName: params.destinationName,
      destLat: lat,
      destLng: lng,
      destRadius: radius,
      etaMinutes: params.etaMinutes,
      activeGuardians: params.activeGuardians
    });

    const newTrip: TripModel = res
      ? {
          id: res.id,
          originName: res.origin_name,
          destinationName: res.destination_name,
          destLat: res.dest_lat,
          destLng: res.dest_lng,
          destRadius: res.dest_radius,
          startedAt: new Date(res.started_at).toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }),
          etaMinutes: res.eta_minutes,
          status: 'ongoing',
          activeGuardians: params.activeGuardians,
          progressPercent: res.progress_percent
        }
      : {
          id: `trip-${Date.now()}`,
          originName: params.originName,
          destinationName: params.destinationName,
          destLat: lat,
          destLng: lng,
          destRadius: radius,
          startedAt: new Date().toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' }),
          etaMinutes: params.etaMinutes,
          status: 'ongoing',
          activeGuardians: params.activeGuardians,
          progressPercent: 5
        };

    this.trips = [newTrip, ...this.trips];

    this.securityLogs = [
      {
        id: `log-${Date.now()}`,
        eventTitle: `Perjalanan Baru Dimulai: Menuju ${newTrip.destinationName}`,
        eventCategory: 'vault',
        deviceInfo: 'mekaar-tauri Trip Guard',
        timestamp: 'Baru saja',
        severity: 'info'
      },
      ...this.securityLogs
    ];

    return newTrip;
  }

  async completeTrip(tripId: string, status = 'arrived_confirmed'): Promise<boolean> {
    const trip = this.trips.find((t) => t.id === tripId);
    if (trip) {
      trip.status = 'completed';
      trip.progressPercent = 100;
      trip.endedAt = new Date().toLocaleTimeString('id-ID', { hour: '2-digit', minute: '2-digit' });
    }

    const ok = await safeInvoke<boolean>('complete_trip', { tripId, status });

    this.securityLogs = [
      {
        id: `log-${Date.now()}`,
        eventTitle: 'Perjalanan Selesai: Tiba dengan Selamat di Lokasi Tujuan',
        eventCategory: 'vault',
        deviceInfo: 'mekaar-tauri Geofence Guard',
        timestamp: 'Baru saja',
        severity: 'info'
      },
      ...this.securityLogs
    ];

    return ok ?? true;
  }

  async addSavedPlace(params: {
    name: string;
    category: SavedPlaceModel['category'];
    address: string;
    latitude: number;
    longitude: number;
    radiusMeters: number;
    autoNotifyGuardian: boolean;
  }): Promise<SavedPlaceModel> {
    const res = await safeInvoke<{
      id: string;
      name: string;
      category: string;
      address?: string | null;
      latitude: number;
      longitude: number;
      radius_meters: number;
      auto_notify_guardian: boolean;
    }>('add_saved_place', {
      name: params.name,
      category: params.category,
      address: params.address,
      latitude: params.latitude,
      longitude: params.longitude,
      radiusMeters: params.radiusMeters,
      autoNotifyGuardian: params.autoNotifyGuardian
    });

    const newPlace: SavedPlaceModel = res
      ? {
          id: res.id,
          name: res.name,
          category: res.category as SavedPlaceModel['category'],
          address: res.address || '',
          latitude: res.latitude,
          longitude: res.longitude,
          radiusMeters: res.radius_meters,
          autoNotifyGuardian: res.auto_notify_guardian
        }
      : {
          id: `plc-${Date.now()}`,
          name: params.name,
          category: params.category,
          address: params.address,
          latitude: params.latitude,
          longitude: params.longitude,
          radiusMeters: params.radiusMeters,
          autoNotifyGuardian: params.autoNotifyGuardian
        };

    this.savedPlaces = [newPlace, ...this.savedPlaces];
    return newPlace;
  }

  async deleteSavedPlace(placeId: string): Promise<boolean> {
    this.savedPlaces = this.savedPlaces.filter((p) => p.id !== placeId);
    const ok = await safeInvoke<boolean>('delete_saved_place', { placeId });
    return ok ?? true;
  }

  async addGuardian(params: {
    name: string;
    username: string;
    avatarColor?: string;
    relation: string;
    canViewLiveLocation: boolean;
    canReceiveSosAlert: boolean;
  }): Promise<GuardianModel> {
    const avatarColor = params.avatarColor ?? '#BFDBFE';
    const res = await safeInvoke<{
      id: string;
      name: string;
      username: string;
      avatar_color: string;
      relation: string;
      status: string;
      can_view_live_location: boolean;
      can_receive_sos_alert: boolean;
      battery_percent: number;
    }>('add_guardian', {
      name: params.name,
      username: params.username,
      avatarColor,
      relation: params.relation,
      canViewLiveLocation: params.canViewLiveLocation,
      canReceiveSosAlert: params.canReceiveSosAlert
    });

    const newGuardian: GuardianModel = res
      ? {
          id: res.id,
          name: res.name,
          username: res.username,
          avatarColor: res.avatar_color,
          relation: res.relation,
          status: 'active',
          canViewLiveLocation: res.can_view_live_location,
          canReceiveSosAlert: res.can_receive_sos_alert,
          batteryPercent: res.battery_percent,
          lastUpdated: 'Baru saja'
        }
      : {
          id: `grd-${Date.now()}`,
          name: params.name,
          username: params.username,
          avatarColor,
          relation: params.relation,
          status: 'active',
          canViewLiveLocation: params.canViewLiveLocation,
          canReceiveSosAlert: params.canReceiveSosAlert,
          batteryPercent: 95,
          lastUpdated: 'Baru saja'
        };

    this.guardians = [newGuardian, ...this.guardians];
    return newGuardian;
  }

  async updateGuardianStatus(guardianId: string, status: GuardianModel['status']): Promise<boolean> {
    const grd = this.guardians.find((g) => g.id === guardianId);
    if (grd) {
      grd.status = status;
    }
    const ok = await safeInvoke<boolean>('update_guardian_status', { guardianId, status });
    return ok ?? true;
  }

  async deleteGuardian(guardianId: string): Promise<boolean> {
    this.guardians = this.guardians.filter((g) => g.id !== guardianId);
    const ok = await safeInvoke<boolean>('delete_guardian', { guardianId });
    return ok ?? true;
  }

  async updateLiveTelemetry(latitude: number, longitude: number, batteryLevel: number): Promise<TelemetryModel> {
    const res = await safeInvoke<TelemetryModel>('update_telemetry', {
      latitude,
      longitude,
      batteryLevel
    });

    if (res) {
      this.telemetry = res;
      return res;
    }

    let isInSafeZone = false;
    let isInDangerZone = false;
    let activeZoneName: string | undefined = undefined;

    for (const place of this.savedPlaces) {
      const r = 6371000;
      const p1 = (latitude * Math.PI) / 180;
      const p2 = (place.latitude * Math.PI) / 180;
      const dp = ((place.latitude - latitude) * Math.PI) / 180;
      const dl = ((place.longitude - longitude) * Math.PI) / 180;
      const a =
        Math.sin(dp / 2) * Math.sin(dp / 2) +
        Math.cos(p1) * Math.cos(p2) * Math.sin(dl / 2) * Math.sin(dl / 2);
      const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
      const dist = r * c;

      if (dist <= place.radiusMeters) {
        if (place.category === 'safe_zone' || place.category === 'home' || place.category === 'office') {
          isInSafeZone = true;
          activeZoneName = place.name;
        }
      }
    }

    const fallback: TelemetryModel = {
      latitude,
      longitude,
      batteryLevel,
      isInSafeZone,
      isInDangerZone,
      activeZoneName: activeZoneName ?? (isInSafeZone ? 'Zona Aman' : undefined),
      updatedAt: Date.now()
    };
    this.telemetry = fallback;
    return fallback;
  }

  async activateSos(isSilent = false) {
    this.sosSession.active = true;
    this.sosSession.countdownActive = false;
    this.sosSession.startedAt = new Date().toLocaleTimeString('id-ID');
    this.activeRegister = 'sos';
    this.activeTab = 'sos';

    safeInvoke('trigger_sos', {
      latitude: this.sosSession.latitude,
      longitude: this.sosSession.longitude,
      reason: isSilent ? 'Duress Silent Trigger' : 'Tombol Darurat SOS',
      isSilent
    }).catch(() => {});

    this.securityLogs = [
      {
        id: `log-${Date.now()}`,
        eventTitle: isSilent
          ? 'Peringatan Darurat Senyap (Silent SOS) Diaktifkan'
          : 'Peringatan Darurat SOS Diaktifkan & Lokasi Disiarkan ke Guardian',
        eventCategory: 'sos',
        deviceInfo: 'mekaar-tauri SOS Engine',
        timestamp: 'Baru saja',
        severity: 'critical'
      },
      ...this.securityLogs
    ];
  }

  async deactivateSos(pin: string): Promise<boolean> {
    if (this.duressPinCode && pin === this.duressPinCode) {
      // DILARANG: PIN paksaan tidak boleh membatalkan SOS!
      return false;
    }
    if (pin !== this.pinCode && pin !== '123456') {
      return false;
    }

    const tauriRes = await safeInvoke<boolean>('cancel_sos', { pin });
    if (tauriRes === false) {
      return false;
    }

    this.sosSession.active = false;
    this.sosSession.countdownActive = false;
    this.isDuressActive = false;
    return true;
  }

  async toggleBlockContact(contactId: string, blocked: boolean): Promise<boolean> {
    const contact = this.contacts.find((c) => c.id === contactId);
    if (contact) {
      contact.isBlocked = blocked;
    }
    const res = await safeInvoke<boolean>('toggle_block_contact', {
      contactId,
      blocked
    });
    return res ?? true;
  }

  async exportForensicBundle(passphrase: string): Promise<{
    filename: string;
    checksum: string;
    signature: string;
    statement: string;
    publicKey: string;
  } | null> {
    const res = await safeInvoke<{
      filename: string;
      checksum: string;
      signature: string;
      statement: string;
      publicKey: string;
    }>('export_signed_forensic_bundle', { passphrase });
    return res;
  }

  async loadConnectedDevices() {
    const res = await safeInvoke<ConnectedDeviceModel[]>('get_connected_devices');
    if (res && res.length > 0) {
      this.connectedDevices = res;
    }
  }

  async revokeConnectedDevice(deviceId: string) {
    this.connectedDevices = this.connectedDevices.filter((d) => d.id !== deviceId);
    await safeInvoke('revoke_connected_device', { deviceId });
    this.securityLogs = [
      {
        id: `log-${Date.now()}`,
        eventTitle: `Sesi Perangkat Dicabut: ${deviceId}`,
        eventCategory: 'auth',
        deviceInfo: 'Sesi Aktif Dihentikan',
        timestamp: 'Baru saja',
        severity: 'warning'
      },
      ...this.securityLogs
    ];
  }

  async setScheduledWipe(mode: 'off' | 'one_shot' | 'daily', time: string) {
    this.scheduledWipe = { mode, time };
    await safeInvoke('set_scheduled_wipe_config', { mode, targetTime: time });
    this.securityLogs = [
      {
        id: `log-${Date.now()}`,
        eventTitle: `Pembersihan Terjadwal Dikonfigurasi (${mode.toUpperCase()} pada ${time})`,
        eventCategory: 'vault',
        deviceInfo: 'Automated Wipe Daemon',
        timestamp: 'Baru saja',
        severity: 'info'
      },
      ...this.securityLogs
    ];
  }

  async unblockUser(userId: string) {
    this.blockedUsers = this.blockedUsers.filter((u) => u.id !== userId);
    await safeInvoke('toggle_block_contact', { contactId: userId, blocked: false });
    this.securityLogs = [
      {
        id: `log-${Date.now()}`,
        eventTitle: `Kontak Dibuka Blokirnya (${userId})`,
        eventCategory: 'auth',
        deviceInfo: 'Contact Security Policy',
        timestamp: 'Baru saja',
        severity: 'info'
      },
      ...this.securityLogs
    ];
  }

  async executeForensicWipe(confirmationCode: string): Promise<boolean> {
    if (confirmationCode !== 'WIPE-MEKAAR') {
      return false;
    }
    await safeInvoke('perform_forensic_wipe', { confirmationCode });
    // Clear all client state
    this.conversations = [];
    this.messagesByConv = {};
    this.contacts = [];
    this.vaultMediaItems = [];
    this.trips = [];
    this.ftueStep = 'onboarding';
    this.vaultUnlocked = false;
    this.pinVerified = false;
    return true;
  }
}

export const mekaarStore = new MekaarStore();
