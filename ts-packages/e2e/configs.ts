export const BASE_URL = process.env.BASE_URL || "http://localhost:8080";

export const DESKTOP_VIEWPORT = { width: 1920, height: 1080 };
export const MOBILE_VIEWPORT = { width: 412, height: 915 };
export const TIMEOUT = 30000;

export const ADMIN_ACCOUNTS = {
  ADMIN1: {
    email: process.env.RATEL_TEST_ADMIN1_EMAIL || "hi+admin1@biyard.co",
    password: process.env.RATEL_TEST_ADMIN_PASSWORD || "",
    username: "관리자1",
  },
  ADMIN2: {
    email: process.env.RATEL_TEST_ADMIN2_EMAIL || "hi+admin2@biyard.co",
    password: process.env.RATEL_TEST_ADMIN_PASSWORD || "",
    username: "관리자2",
  },
};

export const USER_ACCOUNTS = {
  USER1: {
    email: process.env.RATEL_TEST_USER1_EMAIL || "hi+user1@biyard.co",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER2: {
    email: process.env.RATEL_TEST_USER2_EMAIL || "hi+user2@biyard.co",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER3: {
    email: process.env.RATEL_TEST_USER3_EMAIL || "hi+user3@biyard.co",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER4: {
    email: process.env.RATEL_TEST_USER4_EMAIL || "hi+user4@biyard.co",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER5: {
    email: process.env.RATEL_TEST_USER5_EMAIL || "hi+user5@biyard.co",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER6: {
    email: process.env.RATEL_TEST_USER6_EMAIL || "hi+user6@biyard.co",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER7: {
    email: process.env.RATEL_TEST_USER7_EMAIL || "hi+user7@biyard.co",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER8: {
    email: process.env.RATEL_TEST_USER8_EMAIL || "hi+user8@biyard.co",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
};

export const ATTRIBUTE = {
  SOGANG: {
    MALE: "j94EA1",
    FEMALE: "bIFviB",
  },
  Konkuk: {
    MALE: "bVn0Vq",
    FEMALE: "wKFegq",
  },
};
