import { useEffect } from 'react';
import { useLocation } from 'react-router';
import {
  logEvent,
  trackPageView,
  trackLogin,
  trackSignUp,
  trackSearch,
  trackShare,
  trackError,
  setAnalyticsUserId,
  setAnalyticsUserProperties,
} from '@/lib/service/analytics-service';

/**
 * Hook for tracking page views automatically
 */
export const usePageTracking = () => {
  const location = useLocation();

  useEffect(() => {
    // Track page view on route change
    trackPageView(location.pathname + location.search);
  }, [location]);
};

/**
 * Hook for analytics tracking functions
 */
export const useAnalytics = () => {
  return {
    logEvent,
    trackPageView,
    trackLogin,
    trackSignUp,
    trackSearch,
    trackShare,
    trackError,
    setUserId: setAnalyticsUserId,
    setUserProperties: setAnalyticsUserProperties,
  };
};
