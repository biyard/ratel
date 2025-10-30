import { useEffect } from 'react';
import { useNavigate } from 'react-router';
import { useUserInfo } from '@/hooks/use-user-info';
import { usePopup } from '@/lib/contexts/popup-service';
import { LoginModal } from '@/components/popup/login-popup';

interface RequireAuthProps {
  children: React.ReactNode;
}

/**
 * Wrapper component that requires authentication.
 * If user is not logged in, shows login popup and redirects to home.
 */
export function RequireAuth({ children }: RequireAuthProps) {
  const { data: user, isLoading } = useUserInfo();
  const navigate = useNavigate();
  const popup = usePopup();

  useEffect(() => {
    if (!isLoading && !user) {
      // Show login popup
      popup.open(<LoginModal disableClose={false} />);

      // Redirect to home
      navigate('/', { replace: true });
    }
  }, [user, isLoading, navigate, popup]);

  // Show loading state while checking auth
  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-foreground-muted">Loading...</div>
      </div>
    );
  }

  // Don't render children if not authenticated
  if (!user) {
    return null;
  }

  return <>{children}</>;
}
