const timeout = 10000;
export const CONFIGS = {
  PAGE_WAIT_TIME: timeout,
  MODAL_WAIT_TIME: timeout,
  SELECTOR_WAIT_TIME: timeout,
  DEVICE_SCREEN_SIZES: {
    MOBILE: 768,
  },
  PLAYWRIGHT: {
    TIMEOUT: timeout,
    NAVIGATION_TIME_OUT: timeout,
    BASE_URL: process.env.RATEL_TEST_PLAYWRIGHT_URL || 'http://localhost:8080',
    ID: process.env.RATEL_TEST_PLAYWRIGHT_ID || 'playwrightuser',
  },
  SECRETS: {
    password: process.env.PASSWORD,
    testemail: process.env.EMAIL,
  },
  ADMIN: {
    id: process.env.RATEL_TEST_ADMIN_ID || 'admin@ratel.foundation',
    password: process.env.RATEL_TEST_ADMIN_PASSWORD || 'admin!234',
  },
};
