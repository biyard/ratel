export const CONFIGS ={
    PAGE_WAIT_TIME: 600000,
    MODAL_WAIT_TIME: 600000,
    SELECTOR_WAIT_TIME: 7000,
    DEVICE_SCREEN_SIZES:{
        MOBILE: 768
    },
    PLAYWRIGHT:{
        TIMEOUT: 6000000,
        NAVIGATION_TIME_OUT: 6000000,
        BASE_URL: "https://dev.ratel.foundation"
    },
    credentials: {
        email: process.env.TEST_EMAIL || "",
        password: process.env.TEST_PASSWORD || "",  
        username: process.env.TEST_USERNAME || "",
        displayName: process.env.TEST_DISPLAY_NAME || "",
        verificationCode: process.env.TEST_VERIFICATION_CODE || "123456",
        newUserEmail: process.env.TEST_NEW_USER_EMAIL || "",

    }
}