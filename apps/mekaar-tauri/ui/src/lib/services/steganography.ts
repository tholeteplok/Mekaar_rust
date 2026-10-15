/**
 * Mekaar Steganography Engine (LSB - Least Significant Bit)
 * 
 * Menyisipkan dan mengekstrak pesan teks rahasia ke dalam bit warna piksel
 * citra kanvas HTML5 secara deterministik dan lossless (PNG).
 * Memberikan perlindungan "plausible deniability" pada saat inspeksi paksa.
 */

const MAGIC_HEADER = 'MKR_STEGO:';

function xorCipher(text: string, key: string): string {
  if (!key) return text;
  let result = '';
  for (let i = 0; i < text.length; i++) {
    const charCode = text.charCodeAt(i) ^ key.charCodeAt(i % key.length);
    result += String.fromCharCode(charCode);
  }
  return result;
}

/**
 * Menyisipkan pesan rahasia ke dalam gambar (Carrier Image) menggunakan metode LSB
 * @param imageSource URL data / objek gambar dasar
 * @param secretMessage Pesan rahasia yang ingin disembunyikan
 * @param passkey Kunci sandi rahasia opsional
 * @returns Promise<string> Data URL PNG citra steganografi
 */
export async function encodeStegoMessage(
  imageSource: string,
  secretMessage: string,
  passkey?: string
): Promise<string> {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.crossOrigin = 'anonymous';

    img.onload = () => {
      try {
        const canvas = document.createElement('canvas');
        canvas.width = img.width;
        canvas.height = img.height;
        const ctx = canvas.getContext('2d');
        if (!ctx) {
          reject(new Error('Gagal menginisialisasi konteks Canvas 2D'));
          return;
        }

        ctx.drawImage(img, 0, 0);
        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const data = imageData.data;

        // Siapkan payload dengan format terstruktur
        const isEncrypted = !!passkey && passkey.trim().length > 0;
        const processedBody = isEncrypted ? xorCipher(secretMessage, passkey.trim()) : secretMessage;
        const fullPayload = `${MAGIC_HEADER}${isEncrypted ? 'ENC:' : 'RAW:'}${processedBody}`;

        // Ubah payload menjadi representasi bit (UTF-8 bytes -> 8 bits each)
        const encoder = new TextEncoder();
        const payloadBytes = encoder.encode(fullPayload);
        const totalBits = payloadBytes.length * 8;

        // Kapasitas maksimum: 3 kanal per piksel (R, G, B)
        // Kita simpan 32 bit panjang payload terlebih dahulu
        const availableBits = (data.length / 4) * 3;
        if (totalBits + 32 > availableBits) {
          reject(
            new Error(
              `Kapasitas gambar terlalu kecil (${availableBits} bit) untuk menyembunyikan pesan (${totalBits + 32} bit). Gunakan gambar dengan resolusi lebih besar.`
            )
          );
          return;
        }

        let bitIndex = 0;

        // 1. Tulis 32-bit panjang payload di awal
        for (let i = 0; i < 32; i++) {
          const bit = (payloadBytes.length >> (31 - i)) & 1;
          const pixelIndex = Math.floor(bitIndex / 3);
          const channelOffset = bitIndex % 3; // 0=R, 1=G, 2=B
          const dataPos = pixelIndex * 4 + channelOffset;

          data[dataPos] = (data[dataPos] & ~1) | bit;
          bitIndex++;
        }

        // 2. Tulis byte payload bit demi bit ke dalam LSB kanal RGB
        for (let b = 0; b < payloadBytes.length; b++) {
          const byteVal = payloadBytes[b];
          for (let bitPos = 7; bitPos >= 0; bitPos--) {
            const bit = (byteVal >> bitPos) & 1;
            const pixelIndex = Math.floor(bitIndex / 3);
            const channelOffset = bitIndex % 3;
            const dataPos = pixelIndex * 4 + channelOffset;

            data[dataPos] = (data[dataPos] & ~1) | bit;
            bitIndex++;
          }
        }

        ctx.putImageData(imageData, 0, 0);
        // PNG wajib digunakan agar kompresi tidak mengubah bit warna (lossless)
        const resultDataUrl = canvas.toDataURL('image/png');
        resolve(resultDataUrl);
      } catch (err: any) {
        reject(new Error('Gagal menyisipkan steganografi: ' + (err?.message || err)));
      }
    };

    img.onerror = () => {
      reject(new Error('Gagal memuat berkas gambar pembawa (carrier image)'));
    };

    img.src = imageSource;
  });
}

/**
 * Mendekode dan mengekstrak pesan tersembunyi dari citra steganografi LSB
 * @param imageSource URL data atau objek gambar
 * @param passkey Kunci sandi jika pesan dienkripsi
 */
export async function decodeStegoMessage(
  imageSource: string,
  passkey?: string
): Promise<{ success: boolean; message: string; isEncrypted?: boolean }> {
  return new Promise((resolve) => {
    const img = new Image();
    img.crossOrigin = 'anonymous';

    img.onload = () => {
      try {
        const canvas = document.createElement('canvas');
        canvas.width = img.width;
        canvas.height = img.height;
        const ctx = canvas.getContext('2d');
        if (!ctx) {
          resolve({ success: false, message: 'Gagal menginisialisasi Canvas' });
          return;
        }

        ctx.drawImage(img, 0, 0);
        const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const data = imageData.data;

        // 1. Baca 32 bit panjang payload
        let payloadLen = 0;
        let bitIndex = 0;
        for (let i = 0; i < 32; i++) {
          const pixelIndex = Math.floor(bitIndex / 3);
          const channelOffset = bitIndex % 3;
          const dataPos = pixelIndex * 4 + channelOffset;
          const bit = data[dataPos] & 1;

          payloadLen = (payloadLen << 1) | bit;
          bitIndex++;
        }

        // Validasi batas kewajaran panjang payload
        const maxPossibleBytes = Math.floor(((data.length / 4) * 3 - 32) / 8);
        if (payloadLen <= 0 || payloadLen > maxPossibleBytes || payloadLen > 500000) {
          resolve({
            success: false,
            message: 'Tidak ditemukan penanda pesan steganografi yang valid pada citra ini.'
          });
          return;
        }

        // 2. Baca bytes payload
        const bytes = new Uint8Array(payloadLen);
        for (let b = 0; b < payloadLen; b++) {
          let byteVal = 0;
          for (let bitPos = 0; bitPos < 8; bitPos++) {
            const pixelIndex = Math.floor(bitIndex / 3);
            const channelOffset = bitIndex % 3;
            const dataPos = pixelIndex * 4 + channelOffset;
            const bit = data[dataPos] & 1;

            byteVal = (byteVal << 1) | bit;
            bitIndex++;
          }
          bytes[b] = byteVal;
        }

        const decoder = new TextDecoder();
        const extractedString = decoder.decode(bytes);

        // 3. Periksa magic header
        if (!extractedString.startsWith(MAGIC_HEADER)) {
          resolve({
            success: false,
            message: 'Gambar tidak mengandung header steganografi Mekaar Aegis.'
          });
          return;
        }

        const rest = extractedString.substring(MAGIC_HEADER.length);
        if (rest.startsWith('ENC:')) {
          const cipherText = rest.substring('ENC:'.length);
          if (!passkey || passkey.trim().length === 0) {
            resolve({
              success: false,
              isEncrypted: true,
              message: 'Pesan steganografi dilindungi kata sandi. Masukkan kunci sandi untuk mendekripsi.'
            });
            return;
          }
          const decrypted = xorCipher(cipherText, passkey.trim());
          resolve({
            success: true,
            isEncrypted: true,
            message: decrypted
          });
        } else if (rest.startsWith('RAW:')) {
          const plainText = rest.substring('RAW:'.length);
          resolve({
            success: true,
            isEncrypted: false,
            message: plainText
          });
        } else {
          resolve({
            success: true,
            isEncrypted: false,
            message: rest
          });
        }
      } catch (err: any) {
        resolve({
          success: false,
          message: 'Kesalahan saat ekstraksi steganografi: ' + (err?.message || err)
        });
      }
    };

    img.onerror = () => {
      resolve({ success: false, message: 'Gagal memuat citra untuk analisis steganografi' });
    };

    img.src = imageSource;
  });
}

/**
 * Menghasilkan gambar pembawa sederhana (synthetic canvas image) jika pengguna tidak memilih gambar dari galeri
 */
export function createSyntheticCarrierImage(
  title = 'Laporan Lapangan',
  subtitle = 'Mekaar Aegis Secure Carrier'
): string {
  const canvas = document.createElement('canvas');
  canvas.width = 400;
  canvas.height = 300;
  const ctx = canvas.getContext('2d')!;

  // Background gradient modern
  const grad = ctx.createLinearGradient(0, 0, 400, 300);
  grad.addColorStop(0, '#0F172A');
  grad.addColorStop(0.5, '#1E293B');
  grad.addColorStop(1, '#0284C7');
  ctx.fillStyle = grad;
  ctx.fillRect(0, 0, 400, 300);

  // Subtle geometric grid
  ctx.strokeStyle = 'rgba(255, 255, 255, 0.08)';
  ctx.lineWidth = 1;
  for (let x = 0; x < 400; x += 30) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, 300);
    ctx.stroke();
  }
  for (let y = 0; y < 300; y += 30) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(400, y);
    ctx.stroke();
  }

  // Shield Icon
  ctx.fillStyle = '#38BDF8';
  ctx.font = 'bold 36px sans-serif';
  ctx.textAlign = 'center';
  ctx.fillText('🛡️', 200, 110);

  // Title
  ctx.fillStyle = '#FFFFFF';
  ctx.font = 'bold 18px sans-serif';
  ctx.fillText(title, 200, 160);

  // Subtitle
  ctx.fillStyle = '#94A3B8';
  ctx.font = '13px sans-serif';
  ctx.fillText(subtitle, 200, 190);

  // Security Pill Stamp
  ctx.fillStyle = 'rgba(2, 132, 199, 0.3)';
  ctx.beginPath();
  ctx.roundRect(100, 220, 200, 32, 16);
  ctx.fill();
  ctx.strokeStyle = '#38BDF8';
  ctx.lineWidth = 1.5;
  ctx.stroke();

  ctx.fillStyle = '#38BDF8';
  ctx.font = 'bold 11px sans-serif';
  ctx.fillText('VERIFIED CARRIER PNG', 200, 240);

  return canvas.toDataURL('image/png');
}
