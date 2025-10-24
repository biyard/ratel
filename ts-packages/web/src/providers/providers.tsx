import { AuthProvider } from '@/app/_providers/auth-provider';
import { PopupProvider } from '@/lib/contexts/popup-service';
import { TeamProvider } from '@/lib/service/team-provider';
import { QueryClientProvider } from '@tanstack/react-query';
import { getQueryClient } from './getQueryClient';

export default function Providers({ children }: { children: React.ReactNode }) {
  const queryClient = getQueryClient();

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
