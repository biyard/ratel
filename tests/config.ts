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
    BASE_URL: "http://localhost:8080",
  },
};
