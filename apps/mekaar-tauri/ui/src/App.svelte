<script lang="ts">
  import { onMount } from 'svelte';
  import './tokens.css';
  import { mekaarStore } from './lib/stores';
  import { MekaarScaffold, ForensicExportDialog } from './lib/components';
  import {
    SplashScreen,
    OnboardingScreen,
    AuthScreen,
    VaultScreen,
    SetUsernameScreen,
    ConversationListScreen,
    ContactListScreen,
    ProfileScreen,
    SettingsScreen,
    ChatRoomScreen,
    CallScreen,
    MapScreen,
    GuardianScreen,
    SosScreen
  } from './lib/screens';

  onMount(() => {
    mekaarStore.initFtueState();
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

{#if mekaarStore.ftueStep === 'splash'}
  <SplashScreen />
{:else if mekaarStore.ftueStep === 'onboarding'}
  <OnboardingScreen />
{:else if mekaarStore.ftueStep === 'auth'}
  <AuthScreen />
{:else if mekaarStore.ftueStep === 'pin_setup'}
  <VaultScreen isSetup={true} />
{:else if mekaarStore.ftueStep === 'username_setup'}
  <SetUsernameScreen />
{:else if mekaarStore.ftueStep === 'pin_locked'}
  <VaultScreen isSetup={false} />
{:else if mekaarStore.selectedConversationId}
  <ChatRoomScreen />
{:else}
  <MekaarScaffold>
    {#if mekaarStore.activeTab === 'chat'}
      <ConversationListScreen />
    {:else if mekaarStore.activeTab === 'contacts'}
      <ContactListScreen />
    {:else if mekaarStore.activeTab === 'profile'}
      <ProfileScreen />
    {:else if mekaarStore.activeTab === 'settings'}
      <SettingsScreen />
    {:else if mekaarStore.activeTab === 'sos'}
      <SosScreen />
    {:else if mekaarStore.activeTab === 'guardian'}
      <GuardianScreen />
    {:else if mekaarStore.activeTab === 'map'}
      <MapScreen />
    {/if}
  </MekaarScaffold>
{/if}

<CallScreen />

<ForensicExportDialog
  open={mekaarStore.forensicDialogOpen}
  onclose={() => (mekaarStore.forensicDialogOpen = false)}
/>
