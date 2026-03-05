export const CONFIGS = {
  TIMEOUT: Number(process.env.PLAYWRIGHT_TIMEOUT) || 5000,
  BASE_URL: process.env.PLAYWRIGHT_BASE_URL || "http://localhost:8000",
  ID: process.env.PLAYWRIGHT_ID || Date.now().toString(),
};
