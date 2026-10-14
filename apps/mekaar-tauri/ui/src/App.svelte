<script lang="ts">
  import { onMount } from 'svelte';
  import './tokens.css';
  import { mekaarStore } from './lib/stores';
  import { MekaarScaffold, ForensicExportDialog } from './lib/components';
  import {
    VaultScreen,
    ConversationListScreen,
    ChatRoomScreen,
    CallScreen,
    MapScreen,
    GuardianScreen,
    SosScreen,
    SettingsScreen
  } from './lib/screens';

  onMount(() => {
    mekaarStore.checkServerHealth('tailscale');
    const handleFocus = () => (mekaarStore.windowFocused = true);
    const handleBlur = () => (mekaarStore.windowFocused = false);
    window.addEventListener('focus', handleFocus);
    window.addEventListener('blur', handleBlur);
    return () => {
      window.removeEventListener('focus', handleFocus);
      window.removeEventListener('blur', handleBlur);
    };
  });
</script>

{#if !mekaarStore.vaultUnlocked || !mekaarStore.pinVerified}
  <VaultScreen />
{:else}
  <MekaarScaffold>
    {#if mekaarStore.activeTab === 'chat'}
      {#if mekaarStore.selectedConversationId}
        <ChatRoomScreen />
      {:else}
        <ConversationListScreen />
      {/if}
    {:else if mekaarStore.activeTab === 'map'}
      <MapScreen />
    {:else if mekaarStore.activeTab === 'guardian'}
      <GuardianScreen />
    {:else if mekaarStore.activeTab === 'sos'}
      <SosScreen />
    {:else if mekaarStore.activeTab === 'settings'}
      <SettingsScreen />
    {/if}
  </MekaarScaffold>

  <CallScreen />

  <ForensicExportDialog
    open={mekaarStore.forensicDialogOpen}
    onclose={() => (mekaarStore.forensicDialogOpen = false)}
  />
{/if}
