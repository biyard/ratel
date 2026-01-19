import { useState, useEffect } from 'react';

type ChatState = 'collapsed' | 'sidebar';

interface ChatPreference {
  chatState: ChatState;
  setChatState: (state: ChatState) => void;
  sidebarWidth: number;
  setSidebarWidth: (width: number) => void;
}

const CHAT_STATE_KEY = 'pdf-ai-chat-state';
const SIDEBAR_WIDTH_KEY = 'pdf-ai-sidebar-width';
const DEFAULT_SIDEBAR_WIDTH = 400;

export function useChatPreference(): ChatPreference {
  const [chatState, setChatStateInternal] = useState<ChatState>(() => {
    const stored = localStorage.getItem(CHAT_STATE_KEY);
    return (stored as ChatState) || 'collapsed';
  });

  const [sidebarWidth, setSidebarWidthInternal] = useState<number>(() => {
    const stored = localStorage.getItem(SIDEBAR_WIDTH_KEY);
    return stored ? parseInt(stored, 10) : DEFAULT_SIDEBAR_WIDTH;
  });

  const setChatState = (state: ChatState) => {
    setChatStateInternal(state);
    localStorage.setItem(CHAT_STATE_KEY, state);
  };

  const setSidebarWidth = (width: number) => {
    setSidebarWidthInternal(width);
    localStorage.setItem(SIDEBAR_WIDTH_KEY, width.toString());
  };

  useEffect(() => {
    // Sync with localStorage changes from other tabs
    const handleStorageChange = (e: StorageEvent) => {
      if (e.key === CHAT_STATE_KEY && e.newValue) {
        setChatStateInternal(e.newValue as ChatState);
      } else if (e.key === SIDEBAR_WIDTH_KEY && e.newValue) {
        setSidebarWidthInternal(parseInt(e.newValue, 10));
      }
    };

    window.addEventListener('storage', handleStorageChange);
    return () => window.removeEventListener('storage', handleStorageChange);
  }, []);

  return {
    chatState,
    setChatState,
    sidebarWidth,
    setSidebarWidth,
  };
}
