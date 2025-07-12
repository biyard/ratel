'use client';

import { useState } from 'react';
import UserSidemenu from './user-sidemenu';

export default function UserSidemenuClientWrapper() {
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);

  return (
    <UserSidemenu
      isOpen={isSidebarOpen}
      toggleSidebar={() => setIsSidebarOpen(!isSidebarOpen)}
    />
  );
}
