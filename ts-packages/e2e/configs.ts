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
    email: "hi+user1@biyard.co",
    username: "user1",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER2: {
    email: "hi+user2@biyard.co",
    username: "user2",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER3: {
    email: "hi+user3@biyard.co",
    username: "user3",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER4: {
    email: "hi+user4@biyard.co",
    username: "user4",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER5: {
    email: "hi+user5@biyard.co",
    username: "user5",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER6: {
    email: "hi+user6@biyard.co",
    username: "user6",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER7: {
    email: "hi+user7@biyard.co",
    username: "user7",
    password: process.env.RATEL_TEST_USER_PASSWORD || "",
  },
  USER8: {
    email: "hi+user8@biyard.co",
    username: "user8",
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
