type Config = {
  env: Env;
  firebase_api_key: string;
  firebase_auth_domain: string;
  firebase_project_id: string;
  firebase_storage_bucket: string;
  firebase_messaging_sender_id: string;
  firebase_app_id: string;
  firebase_measurement_id: string;
  api_url: string;

  log_level: string;
  sign_domain: string;
  experiment: boolean;
  telegram_botname: string;
  version: string;
  storybookUrl: string;

  // Identity Verification
  portone_store_id: string;
  portone_inicis_channel_key: string;
};

export const Env = {
  Local: 'local',
  Dev: 'dev',
  Staging: 'stg',
  Prod: 'prod',
} as const;

export type Env = (typeof Env)[keyof typeof Env];

export const config: Config = {
  env: (import.meta.env.VITE_ENV || 'local') as Env,
  firebase_api_key: import.meta.env.VITE_FIREBASE_API_KEY || '',
  firebase_auth_domain: import.meta.env.VITE_FIREBASE_AUTH_DOMAIN || '',
  firebase_project_id: import.meta.env.VITE_FIREBASE_PROJECT_ID || '',
  firebase_storage_bucket: import.meta.env.VITE_FIREBASE_STORAGE_BUCKET || '',
  firebase_messaging_sender_id:
    import.meta.env.VITE_FIREBASE_MESSAGING_SENDER_ID || '',
  firebase_app_id: import.meta.env.VITE_FIREBASE_APP_ID || '',
  firebase_measurement_id: import.meta.env.VITE_FIREBASE_MEASUREMENT_ID || '',

  log_level: import.meta.env.VITE_LOG_LEVEL || 'info',
  sign_domain: import.meta.env.VITE_SIGN_DOMAIN || 'dev.ratel.foundation',
  experiment: import.meta.env.VITE_EXPERIMENT === 'true',

  version: import.meta.env.VITE_VERSION || 'unknown',

  // Server-customizable configuration
  api_url: import.meta.env.VITE_API_URL || '',
  telegram_botname:
    import.meta.env.VITE_TELEGRAM_BOTNAME || 'cryto_ratel_dev_bot',
  storybookUrl: import.meta.env.VITE_STORYBOOK_URL || 'http://localhost:6006',

  portone_store_id: import.meta.env.VITE_PORTONE_STORE_ID || '',
  portone_inicis_channel_key:
    import.meta.env.VITE_PORTONE_INICIS_CHANNEL_KEY || '',
};
