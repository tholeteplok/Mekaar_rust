<script lang="ts">
  let {
    open = false,
    onselect,
    onclose
  }: {
    open: boolean;
    onselect: (emoji: string) => void;
    onclose: () => void;
  } = $props();

  let searchQuery = $state('');
  let activeCategory = $state<'all' | 'smileys' | 'gestures' | 'hearts' | 'objects'>('all');

  const emojiDatabase = [
    // Smileys
    { char: '😀', name: 'senyum lebar', cat: 'smileys' },
    { char: '😃', name: 'senyum gembira', cat: 'smileys' },
    { char: '😄', name: 'tertawa bahagia', cat: 'smileys' },
    { char: '😁', name: 'senyum mata sipit', cat: 'smileys' },
    { char: '😆', name: 'terbahak-bahak', cat: 'smileys' },
    { char: '😅', name: 'tertawa keringat dingin', cat: 'smileys' },
    { char: '😂', name: 'tertawa air mata menangis', cat: 'smileys' },
    { char: '🤣', name: 'guling guling ketawa', cat: 'smileys' },
    { char: '😊', name: 'senyum manis merona', cat: 'smileys' },
    { char: '😇', name: 'senyum malaikat suci', cat: 'smileys' },
    { char: '🙂', name: 'senyum ramah santai', cat: 'smileys' },
    { char: '🙃', name: 'terbalik bercanda', cat: 'smileys' },
    { char: '😉', name: 'kedip mata genit', cat: 'smileys' },
    { char: '😌', name: 'lega santai', cat: 'smileys' },
    { char: '😍', name: 'mata hati cinta suka', cat: 'smileys' },
    { char: '🥰', name: 'penuh cinta sayang', cat: 'smileys' },
    { char: '😘', name: 'ciup ciuman sayang', cat: 'smileys' },
    { char: '😋', name: 'enak lezat melet', cat: 'smileys' },
    { char: '😎', name: 'keren kacamata hitam', cat: 'smileys' },
    { char: '🤓', name: 'kutu buku nerd pintar', cat: 'smileys' },
    { char: '🧐', name: 'meneliti monokel selidik', cat: 'smileys' },
    { char: '🤔', name: 'berpikir heran ragu', cat: 'smileys' },
    { char: '🤫', name: 'rahasia diam senyap', cat: 'smileys' },
    { char: '🤭', name: 'tutup mulut kaget tertawa', cat: 'smileys' },
    { char: '🤐', name: 'resleting diam bungkam', cat: 'smileys' },
    { char: '🤨', name: 'curiga alis terangkat', cat: 'smileys' },
    { char: '😐', name: 'datar netral', cat: 'smileys' },
    { char: '😑', name: 'tanpa ekspresi jengkel', cat: 'smileys' },
    { char: '😶', name: 'tanpa mulut diam', cat: 'smileys' },
    { char: '😏', name: 'senyum licik seringai', cat: 'smileys' },
    { char: '😒', name: 'tidak puas sinis', cat: 'smileys' },
    { char: '🙄', name: 'putar mata malas bosan', cat: 'smileys' },
    { char: '😬', name: 'meringis canggung', cat: 'smileys' },
    { char: '🤥', name: 'bohong pinokio hidung panjang', cat: 'smileys' },
    { char: '😴', name: 'tidur nyenyak', cat: 'smileys' },
    { char: '😷', name: 'masker sakit virus', cat: 'smileys' },
    { char: '🤒', name: 'termometer demam flu', cat: 'smileys' },
    { char: '🤕', name: 'perban luka kepala cedera', cat: 'smileys' },
    { char: '🤢', name: 'mual mau muntah jijik', cat: 'smileys' },
    { char: '🤮', name: 'muntah hijau mabuk', cat: 'smileys' },
    { char: '🥵', name: 'panas kepanasan gerah merah', cat: 'smileys' },
    { char: '🥶', name: 'dingin kedinginan beku biru', cat: 'smileys' },
    { char: '🥴', name: 'pusing mabuk oleng', cat: 'smileys' },
    { char: '😵', name: 'pingsan linglung mutar', cat: 'smileys' },
    { char: '🤯', name: 'meledak kepala takjub', cat: 'smileys' },
    { char: '🥳', name: 'pesta perayaan selamat ultah', cat: 'smileys' },
    { char: '🥸', name: 'samaran kumis hidung kacamata', cat: 'smileys' },
    { char: '🥺', name: 'memohon tatapan sedih kasihan', cat: 'smileys' },
    { char: '😢', name: 'menangis air mata sedih', cat: 'smileys' },
    { char: '😭', name: 'menangis kencang histeris', cat: 'smileys' },
    { char: '😤', name: 'marah mendengus bangga', cat: 'smileys' },
    { char: '😠', name: 'marah kesal cemberut', cat: 'smileys' },
    { char: '😡', name: 'sangat marah merah murka', cat: 'smileys' },
    { char: '🤬', name: 'mengumpat sensor sumpah', cat: 'smileys' },
    { char: '😈', name: 'iblis ungu jahat seringai', cat: 'smileys' },
    { char: '👿', name: 'iblis marah murka setan', cat: 'smileys' },
    { char: '💀', name: 'tengkorak mati dead', cat: 'smileys' },
    { char: '💩', name: 'kotoran poop lucu', cat: 'smileys' },
    { char: '🤡', name: 'badut clown komedi', cat: 'smileys' },
    { char: '👻', name: 'hantu ghost gentayangan', cat: 'smileys' },
    { char: '👽', name: 'alien ufo luar angkasa', cat: 'smileys' },
    { char: '🤖', name: 'robot mecha ai mesin', cat: 'smileys' },

    // Gestures
    { char: '👋', name: 'lambaian tangan halo dadah', cat: 'gestures' },
    { char: '🤚', name: 'tangan terangkat stop', cat: 'gestures' },
    { char: '🖐️', name: 'lima jari telapak tangan', cat: 'gestures' },
    { char: '✋', name: 'tangan tegak berhenti', cat: 'gestures' },
    { char: '🖖', name: 'vulcan spock salam', cat: 'gestures' },
    { char: '👌', name: 'oke setuju mantap', cat: 'gestures' },
    { char: '🤌', name: 'jari mengerucut italia apa maksudmu', cat: 'gestures' },
    { char: '🤏', name: 'sedikit secuil kecil', cat: 'gestures' },
    { char: '✌️', name: 'damai peace dua jari menang', cat: 'gestures' },
    { char: '🤞', name: 'silang jari semoga beruntung', cat: 'gestures' },
    { char: '🤟', name: 'tanda cinta i love you', cat: 'gestures' },
    { char: '🤘', name: 'rock metal tanduk', cat: 'gestures' },
    { char: '🤙', name: 'telepon santai shaka', cat: 'gestures' },
    { char: '👈', name: 'tunjuk kiri', cat: 'gestures' },
    { char: '👉', name: 'tunjuk kanan', cat: 'gestures' },
    { char: '👆', name: 'tunjuk atas', cat: 'gestures' },
    { char: '👇', name: 'tunjuk bawah', cat: 'gestures' },
    { char: '☝️', name: 'satu jari telunjuk atas perhatian', cat: 'gestures' },
    { char: '👍', name: 'jempol atas suka setuju bagus', cat: 'gestures' },
    { char: '👎', name: 'jempol bawah tidak suka jelek', cat: 'gestures' },
    { char: '✊', name: 'kepalan tinju semangat perlawanan', cat: 'gestures' },
    { char: '👊', name: 'fist bump tinju depan toss', cat: 'gestures' },
    { char: '🤛', name: 'tinju kiri', cat: 'gestures' },
    { char: '🤜', name: 'tinju kanan', cat: 'gestures' },
    { char: '👏', name: 'tepuk tangan salut apresiasi', cat: 'gestures' },
    { char: '🙌', name: 'dua tangan angkat hore berkah', cat: 'gestures' },
    { char: '👐', name: 'tangan terbuka pelukan', cat: 'gestures' },
    { char: '🤲', name: 'tangan menengadah berdoa memohon', cat: 'gestures' },
    { char: '🤝', name: 'jabat tangan sepakat deal mitra', cat: 'gestures' },
    { char: '🙏', name: 'terima kasih tolong maaf doa bersyukur', cat: 'gestures' },

    // Hearts & Symbols
    { char: '❤️', name: 'hati merah cinta love jantung', cat: 'hearts' },
    { char: '🧡', name: 'hati oranye jingga', cat: 'hearts' },
    { char: '💛', name: 'hati kuning sahabat hangat', cat: 'hearts' },
    { char: '💚', name: 'hati hijau alam sehat', cat: 'hearts' },
    { char: '💙', name: 'hati biru tenang damai', cat: 'hearts' },
    { char: '💜', name: 'hati ungu mewah bangsawan', cat: 'hearts' },
    { char: '🖤', name: 'hati hitam gelap duka', cat: 'hearts' },
    { char: '🤍', name: 'hati putih suci bersih', cat: 'hearts' },
    { char: '🤎', name: 'hati cokelat bumi kayu', cat: 'hearts' },
    { char: '💔', name: 'patah hati kecewa sedih putus', cat: 'hearts' },
    { char: '❣️', name: 'tanda seru hati seru cinta', cat: 'hearts' },
    { char: '💕', name: 'dua hati merah muda bergetar', cat: 'hearts' },
    { char: '💞', name: 'hati berputar bergulung', cat: 'hearts' },
    { char: '💓', name: 'hati berdegup kencang cinta', cat: 'hearts' },
    { char: '💗', name: 'hati bertumbuh mekar sayang', cat: 'hearts' },
    { char: '💖', name: 'hati berkilau gemerlap bintang', cat: 'hearts' },
    { char: '💘', name: 'panah asmara cupid', cat: 'hearts' },
    { char: '💝', name: 'kado hati pita cinta', cat: 'hearts' },
    { char: '🔥', name: 'api membara semangat viral panas', cat: 'hearts' },
    { char: '✨', name: 'kilauan cahaya bintang ajaib bersih', cat: 'hearts' },
    { char: '🌟', name: 'bintang bersinar gemerlap juara', cat: 'hearts' },
    { char: '💯', name: 'seratus poin sempurna mantap', cat: 'hearts' },
    { char: '💥', name: 'ledakan tabrakan boom hancur', cat: 'hearts' },

    // Objects & Security
    { char: '🛡️', name: 'perisai perlindungan aegis aman enkripsi', cat: 'objects' },
    { char: '🔒', name: 'gembok kunci tertutup terenkripsi aman', cat: 'objects' },
    { char: '🔓', name: 'gembok terbuka tidak terkunci bebas', cat: 'objects' },
    { char: '🔑', name: 'kunci emas rahasia akses passkey', cat: 'objects' },
    { char: '🗝️', name: 'kunci antik kuno brankas', cat: 'objects' },
    { char: '🕵️', name: 'detektif mata-mata penyelidik intel steganografi', cat: 'objects' },
    { char: '🚀', name: 'roket meluncur cepat teknologi masa depan', cat: 'objects' },
    { char: '⚡', name: 'petir kilat listrik cepat energi', cat: 'objects' },
    { char: '📍', name: 'pin lokasi koordinat gps peta tempat', cat: 'objects' },
    { char: '🛵', name: 'motor skuter ojek kawal perjalanan', cat: 'objects' },
    { char: '🚨', name: 'lampu sirine polisi darurat bahaya sos', cat: 'objects' },
    { char: '📸', name: 'kamera foto jepret gambar', cat: 'objects' },
    { char: '🎙️', name: 'mikrofon suara rekaman audio podcast', cat: 'objects' },
    { char: '📁', name: 'folder berkas dokumen arsip', cat: 'objects' },
    { char: '📎', name: 'penjepit kertas klip lampiran file', cat: 'objects' },
    { char: '📦', name: 'paket kardus kotak pengiriman', cat: 'objects' }
  ];

  const filteredEmojis = $derived(
    emojiDatabase.filter((item) => {
      if (activeCategory !== 'all' && item.cat !== activeCategory) return false;
      if (!searchQuery.trim()) return true;
      const q = searchQuery.toLowerCase();
      return item.name.includes(q) || item.char === q;
    })
  );

  function pickEmoji(char: string) {
    onselect(char);
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="popover-backdrop" onclick={onclose}></div>

  <div class="emoji-popover-card">
    <div class="popover-header">
      <!-- svelte-ignore a11y_autofocus -->
      <input
        type="text"
        class="search-input"
        placeholder="Cari emoji (misal: senyum, hati, jempol)..."
        bind:value={searchQuery}
        autofocus
      />
    </div>

    <!-- Category Tabs -->
    <div class="category-tabs-row">
      <button
        type="button"
        class="cat-tab {activeCategory === 'all' ? 'active' : ''}"
        onclick={() => (activeCategory = 'all')}
        title="Semua"
      >
        Semua
      </button>
      <button
        type="button"
        class="cat-tab {activeCategory === 'smileys' ? 'active' : ''}"
        onclick={() => (activeCategory = 'smileys')}
        title="Wajah & Ekspresi"
      >
        😀
      </button>
      <button
        type="button"
        class="cat-tab {activeCategory === 'gestures' ? 'active' : ''}"
        onclick={() => (activeCategory = 'gestures')}
        title="Tangan & Gestur"
      >
        👍
      </button>
      <button
        type="button"
        class="cat-tab {activeCategory === 'hearts' ? 'active' : ''}"
        onclick={() => (activeCategory = 'hearts')}
        title="Hati & Simbol"
      >
        ❤️
      </button>
      <button
        type="button"
        class="cat-tab {activeCategory === 'objects' ? 'active' : ''}"
        onclick={() => (activeCategory = 'objects')}
        title="Objek & Keamanan"
      >
        🛡️
      </button>
    </div>

    <!-- Emoji Grid Feed -->
    <div class="emoji-grid-viewport">
      {#each filteredEmojis as item (item.char + item.name)}
        <button
          type="button"
          class="emoji-grid-btn"
          onclick={() => pickEmoji(item.char)}
          title={item.name}
          aria-label={item.name}
        >
          {item.char}
        </button>
      {:else}
        <div class="empty-search">
          <div style="font-size: 1.5rem; margin-bottom: 4px;">🔍</div>
          <div style="font-size: 0.78rem; color: var(--text-secondary);">
            Emoji tidak ditemukan.
          </div>
        </div>
      {/each}
    </div>
  </div>
{/if}

<style>
  .popover-backdrop {
    position: fixed;
    inset: 0;
    z-index: 60;
    background: transparent;
  }

  .emoji-popover-card {
    position: absolute;
    bottom: 64px;
    left: 16px;
    width: 320px;
    max-height: 350px;
    background: #FFFFFF;
    border: 1px solid var(--border-light, #E2E8F0);
    border-radius: var(--radius-xl, 16px);
    box-shadow: 0 10px 30px rgba(15, 23, 42, 0.16);
    z-index: 70;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    animation: popIn 0.16s cubic-bezier(0.16, 1, 0.3, 1);
  }

  @keyframes popIn {
    from {
      opacity: 0;
      transform: translateY(12px) scale(0.96);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  .popover-header {
    padding: 10px 12px 6px 12px;
    border-bottom: 1px solid var(--border-light, #E2E8F0);
  }

  .search-input {
    width: 100%;
    padding: 6px 10px;
    border-radius: var(--radius-pill, 9999px);
    border: 1px solid var(--border-light, #E2E8F0);
    background-color: var(--bg-primary, #F8FAFC);
    font-size: 0.78rem;
    color: var(--text-primary, #0F172A);
    outline: none;
    box-sizing: border-box;
  }

  .search-input:focus {
    border-color: var(--brand-blue, #0284C7);
    background: #FFFFFF;
  }

  .category-tabs-row {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 6px 10px;
    border-bottom: 1px solid var(--border-light, #E2E8F0);
    background-color: var(--bg-primary, #F8FAFC);
  }

  .cat-tab {
    padding: 4px 8px;
    border-radius: var(--radius-md, 8px);
    border: none;
    background: transparent;
    font-size: 0.88rem;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .cat-tab:hover {
    background-color: rgba(2, 132, 199, 0.1);
  }

  .cat-tab.active {
    background-color: #FFFFFF;
    box-shadow: 0 1px 3px rgba(15, 23, 42, 0.08);
    font-weight: 700;
  }

  .emoji-grid-viewport {
    padding: 8px 10px;
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 4px;
    overflow-y: auto;
    max-height: 250px;
  }

  .emoji-grid-btn {
    width: 36px;
    height: 36px;
    border-radius: 8px;
    border: none;
    background: transparent;
    font-size: 1.35rem;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: transform 0.1s ease, background 0.15s ease;
  }

  .emoji-grid-btn:hover {
    background-color: rgba(2, 132, 199, 0.12);
    transform: scale(1.18);
  }

  .emoji-grid-btn:active {
    transform: scale(0.92);
  }

  .empty-search {
    grid-column: 1 / -1;
    padding: 30px 10px;
    text-align: center;
  }
</style>
