import { getAnalytics, logEvent as firebaseLogEvent, setUserId, setUserProperties, type Analytics } from 'firebase/analytics';
import { getApp } from 'firebase/app';
import { logger } from '../logger';

let analytics: Analytics | null = null;

/**
 * Initialize Firebase Analytics
 * Should be called after Firebase app is initialized
 */
export const initializeAnalytics = (): Analytics | null => {
  try {
    const app = getApp();
    analytics = getAnalytics(app);
    logger.info('📊 Firebase Analytics initialized successfully');
    return analytics;
  } catch (error) {
    logger.error('📊 Failed to initialize Firebase Analytics:', error);
    return null;
  }
};

/**
 * Get the Analytics instance
 */
export const getAnalyticsInstance = (): Analytics | null => {
  return analytics;
};

/**
 * Log a custom event to Google Analytics
 */
export const logEvent = (eventName: string, eventParams?: Record<string, any>) => {
  if (!analytics) {
    logger.warn('📊 Analytics not initialized, event not logged:', eventName);
    return;
  }

  try {
    firebaseLogEvent(analytics, eventName, eventParams);
    logger.debug('📊 Event logged:', eventName, eventParams);
  } catch (error) {
    logger.error('📊 Failed to log event:', eventName, error);
  }
};

/**
 * Set user ID for analytics tracking
 */
export const setAnalyticsUserId = (userId: string | null) => {
  if (!analytics) {
    logger.warn('📊 Analytics not initialized, user ID not set');
    return;
  }

  try {
    setUserId(analytics, userId);
    logger.debug('📊 User ID set:', userId);
  } catch (error) {
    logger.error('📊 Failed to set user ID:', error);
  }
};

/**
 * Set user properties for analytics
 */
export const setAnalyticsUserProperties = (properties: Record<string, any>) => {
  if (!analytics) {
    logger.warn('📊 Analytics not initialized, user properties not set');
    return;
  }

  try {
    setUserProperties(analytics, properties);
    logger.debug('📊 User properties set:', properties);
  } catch (error) {
    logger.error('📊 Failed to set user properties:', error);
  }
};

// Common event tracking functions
export const trackPageView = (pagePath: string, pageTitle?: string) => {
  logEvent('page_view', {
    page_path: pagePath,
    page_title: pageTitle || document.title,
  });
};

export const trackLogin = (method: string) => {
  logEvent('login', { method });
};

export const trackSignUp = (method: string) => {
  logEvent('sign_up', { method });
};

export const trackSearch = (searchTerm: string) => {
  logEvent('search', { search_term: searchTerm });
};

export const trackShare = (contentType: string, itemId: string, method: string) => {
  logEvent('share', {
    content_type: contentType,
    item_id: itemId,
    method,
  });
};

export const trackError = (errorMessage: string, fatal: boolean = false) => {
  logEvent('exception', {
    description: errorMessage,
    fatal,
  });
};
