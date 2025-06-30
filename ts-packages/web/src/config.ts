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
  graphql_url: string;
};

export enum Env {
  Local = 'local',
  Dev = 'dev',
  Staging = 'stg',
  Prod = 'prod',
}

export const config: Config = {
  env: (process.env.NEXT_PUBLIC_ENV || 'dev') as Env,
  firebase_api_key: process.env.NEXT_PUBLIC_FIREBASE_API_KEY || '',
  firebase_auth_domain: process.env.NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN || '',
  firebase_project_id: process.env.NEXT_PUBLIC_FIREBASE_PROJECT_ID || '',
  firebase_storage_bucket:
    process.env.NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET || '',
  firebase_messaging_sender_id:
    process.env.NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID || '',
  firebase_app_id: process.env.NEXT_PUBLIC_FIREBASE_APP_ID || '',
  firebase_measurement_id:
    process.env.NEXT_PUBLIC_FIREBASE_MEASUREMENT_ID || '',
  api_url:
    process.env.NEXT_PUBLIC_API_URL || 'https://api.dev.ratel.foundation',

  log_level: process.env.NEXT_PUBLIC_LOG_LEVEL || 'info',
  sign_domain: process.env.NEXT_PUBLIC_SIGN_DOMAIN || 'dev.ratel.foundation',
  experiment: process.env.NEXT_PUBLIC_EXPERIMENT === 'true',
  graphql_url:
    process.env.NEXT_PUBLIC_GRAPHQL_URL ||
    'https://graphql.dev.ratel.foundation/v1/graphql',
};
