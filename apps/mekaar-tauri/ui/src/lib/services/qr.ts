import QRCode from 'qrcode';

export interface ParsedContactQr {
  userId: string;
  username?: string;
  displayName?: string;
  pubkey?: string;
}

/**
 * Menghasilkan string payload terstandarisasi untuk Kode QR Kontak Mekaar
 * Selaras dengan format MyQrScreen.payloadFor pada Flutter mekaar_chat.
 */
export function generateContactPayload(
  userId: string,
  username?: string,
  displayName?: string,
  pubkey?: string
): string {
  const base = `mekaar://user/${encodeURIComponent(userId.trim())}`;
  const params = new URLSearchParams();
  if (username) params.set('username', username.trim().replace(/^@/, ''));
  if (displayName) params.set('name', displayName.trim());
  if (pubkey) params.set('pubkey', pubkey.trim());

  const qs = params.toString();
  return qs ? `${base}?${qs}` : base;
}

/**
 * Mengurai string mentah dari pemindaian QR menjadi data kontak
 */
export function parseContactPayload(raw: string): ParsedContactQr | null {
  const text = raw.trim();
  if (!text) return null;

  // 1. Cek JSON format
  if (text.startsWith('{') && text.endsWith('}')) {
    try {
      const parsed = JSON.parse(text);
      if (parsed.userId || parsed.id) {
        return {
          userId: parsed.userId || parsed.id,
          username: parsed.username,
          displayName: parsed.displayName || parsed.name,
          pubkey: parsed.pubkey || parsed.fingerprint
        };
      }
    } catch (_) {}
  }

  // 2. Cek format skema URI mekaar://user/<userId>?...
  const prefix = 'mekaar://user/';
  if (text.toLowerCase().startsWith(prefix)) {
    const withoutPrefix = text.substring(prefix.length);
    const [pathPart, queryPart] = withoutPrefix.split('?');
    const userId = decodeURIComponent(pathPart.trim());
    if (!userId) return null;

    let username: string | undefined;
    let displayName: string | undefined;
    let pubkey: string | undefined;

    if (queryPart) {
      const params = new URLSearchParams(queryPart);
      username = params.get('username') || undefined;
      displayName = params.get('name') || undefined;
      pubkey = params.get('pubkey') || undefined;
    }

    return {
      userId,
      username: username ? `@${username.replace(/^@/, '')}` : undefined,
      displayName,
      pubkey
    };
  }

  // 3. Cek format username / ID langsung (fallback off-grid)
  if (text.startsWith('usr-') || text.startsWith('@') || /^[a-zA-Z0-9_-]{4,}$/.test(text)) {
    return {
      userId: text.startsWith('usr-') ? text : `usr-${text.replace(/^@/, '')}`,
      username: text.startsWith('@') ? text : `@${text.replace(/^usr-/, '')}`,
      displayName: text.replace(/^usr-/, '').replace(/^@/, '')
    };
  }

  return null;
}

/**
 * Menghasilkan representasi vektor SVG dari teks QR Code secara lokal dan instan
 */
export async function generateQrSvg(
  text: string,
  options?: {
    size?: number;
    margin?: number;
    darkColor?: string;
    lightColor?: string;
  }
): Promise<string> {
  const svg = await QRCode.toString(text, {
    type: 'svg',
    errorCorrectionLevel: 'M',
    margin: options?.margin ?? 2,
    width: options?.size ?? 220,
    color: {
      dark: options?.darkColor ?? '#0F172A',
      light: options?.lightColor ?? '#FFFFFF'
    }
  });
  return svg;
}
