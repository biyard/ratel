'use client';

import { AuthProvider } from '@/app/_providers/auth-provider';
import { PopupProvider } from '@/lib/contexts/popup-service';
import { TeamProvider } from '@/lib/service/team-provider';
import { hydrate, QueryClientProvider } from '@tanstack/react-query';
import { getQueryClient } from './getQueryClient';

export default function Providers({
  children,
  dehydratedState,
}: {
  children: React.ReactNode;
  dehydratedState: unknown;
}) {
  const queryClient = getQueryClient();
  hydrate(queryClient, dehydratedState);
  return (
    <QueryClientProvider client={queryClient}>
      <AuthProvider>
        <PopupProvider>
          <TeamProvider>{children}</TeamProvider>
        </PopupProvider>
      </AuthProvider>
    </QueryClientProvider>
  );
}
