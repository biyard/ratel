'use client';

import { AuthProvider } from '@/app/_providers/auth-provider';
import { client } from '@/lib/apollo';
import { PopupProvider } from '@/lib/contexts/popup-service';
import { TeamProvider } from '@/lib/service/team-provider';
import { ApolloProvider } from '@apollo/client';
import { hydrate, QueryClientProvider } from '@tanstack/react-query';
import { getQueryClient } from './getQueryClient';
import { ThemeProvider } from '@/lib/contexts/theme-context';

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
    <ApolloProvider client={client}>
      <QueryClientProvider client={queryClient}>
        <AuthProvider>
          <PopupProvider>
            <ThemeProvider>
              <TeamProvider>{children}</TeamProvider>
            </ThemeProvider>
          </PopupProvider>
        </AuthProvider>
      </QueryClientProvider>
    </ApolloProvider>
  );
}
