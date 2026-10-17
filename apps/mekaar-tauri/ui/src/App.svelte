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
  import { onBackButtonPress, exit } from '@tauri-apps/api/app';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import { isTauri, type PluginListener } from '@tauri-apps/api/core';

  async function minimizeOrExit() {
    try {
      const appWindow = getCurrentWindow();
      await appWindow.minimize();
    } catch {
      try {
        await exit(0);
      } catch (err) {
        console.warn('Unable to minimize or exit app:', err);
      }
    }
  }

  async function handleSmartBack() {
    // 1. Dismiss forensic export / overlay dialog if open
    if (mekaarStore.forensicDialogOpen) {
      mekaarStore.forensicDialogOpen = false;
      return;
    }

    // 2. Handle onboarding / auth setup stages
    if (mekaarStore.ftueStep !== 'authenticated') {
      if (mekaarStore.ftueStep === 'auth') {
        mekaarStore.ftueStep = 'onboarding';
        return;
      }
      if (mekaarStore.ftueStep === 'username_setup') {
        mekaarStore.ftueStep = 'pin_setup';
        return;
      }
      // In splash, onboarding, or pin_locked -> minimize / exit
      await minimizeOrExit();
      return;
    }

    // 3. If inside a conversation (ChatRoomScreen), return to conversation list
    if (mekaarStore.selectedConversationId) {
      mekaarStore.selectedConversationId = null;
      return;
    }

    // 4. If in Settings advanced sub-screen, return to main settings first
    if (mekaarStore.activeTab === 'settings' && mekaarStore.settingsSubView !== 'main') {
      mekaarStore.settingsSubView = 'main';
      return;
    }

    // 5. User requirement: "jika back dari kontak/profil/pengaturan akan kembali ke home(tab pesan)"
    if (mekaarStore.activeTab !== 'chat') {
      mekaarStore.activeTab = 'chat';
      return;
    }

    // 5. User requirement: "back dari home - close/minimize"
    await minimizeOrExit();
  }

  onMount(() => {
    mekaarStore.initFtueState();
    mekaarStore.checkServerHealth('tailscale');
    const handleFocus = () => (mekaarStore.windowFocused = true);
    const handleBlur = () => (mekaarStore.windowFocused = false);
    window.addEventListener('focus', handleFocus);
    window.addEventListener('blur', handleBlur);

    // Register Android native Smart Back gesture & hardware back button
    let backListener: PluginListener | undefined;
    if (isTauri()) {
      onBackButtonPress(() => {
        void handleSmartBack();
      })
        .then((listener) => {
          backListener = listener;
        })
        .catch((err) => {
          console.debug('onBackButtonPress not registered:', err);
        });
    }

    // Cross-platform keyboard Escape key support for consistent smart back
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        void handleSmartBack();
      }
    };
    window.addEventListener('keydown', handleKeyDown);

    return () => {
      window.removeEventListener('focus', handleFocus);
      window.removeEventListener('blur', handleBlur);
      window.removeEventListener('keydown', handleKeyDown);
      if (backListener) {
        void backListener.unregister();
      }
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
