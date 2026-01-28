type Config = {
  env: Env;
  operator_address: string;
  rpc_url: string;
  block_explorer_url: string;
  usdt_address: string;
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
  portone_kpn_channel_key: string;

  // Business Information (for PG approval footer)
  company_name: string;
  company_ceo: string;
  business_registration: string;
  business_address: string;
  business_phone: string;
  business_email: string;
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
  operator_address: import.meta.env.VITE_OPERATOR_ADDRESS || '',
  rpc_url: import.meta.env.VITE_RPC_URL || '',
  block_explorer_url: import.meta.env.VITE_BLOCK_EXPLORER_URL || '',
  usdt_address: import.meta.env.VITE_USDT_ADDRESS || '',
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
  portone_kpn_channel_key: import.meta.env.VITE_PORTONE_KPN_CHANNEL_KEY || '',

  // Business Information (for PG approval footer)
  company_name: import.meta.env.VITE_COMPANY_NAME || 'Ratel Inc.',
  company_ceo: import.meta.env.VITE_COMPANY_CEO || '박혜진',
  business_registration:
    import.meta.env.VITE_BUSINESS_REGISTRATION || '591-87-01919',
  business_address:
    import.meta.env.VITE_BUSINESS_ADDRESS || 'Business Address, City, Country',
  business_phone: import.meta.env.VITE_BUSINESS_PHONE || '+82-2-1234-5678',
  business_email: import.meta.env.VITE_BUSINESS_EMAIL || 'contact@ratel.com',
};
