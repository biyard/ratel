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
        email: process.env.TEST_EMAIL as string,
        password: process.env.TEST_PASSWORD as string,  
        username: process.env.TEST_USERNAME as string,
        displayName: process.env.TEST_DISPLAY_NAME as string,
        verificationCode: process.env.TEST_VERIFICATION_CODE as string,
        newUserEmail: process.env.TEST_NEW_USER_EMAIL as string,

    }
}