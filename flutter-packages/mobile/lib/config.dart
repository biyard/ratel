class Config {
  static const env = String.fromEnvironment('ENV', defaultValue: 'dev');
  static const logLevel = String.fromEnvironment(
    'LOG_LEVEL',
    defaultValue: 'debug',
  );
  static const redirectUrl = String.fromEnvironment(
    'REDIRECT_URL',
    defaultValue: 'https://dev.ratel.foundation',
  );
  static const apiEndpoint = String.fromEnvironment(
    'API_ENDPOINT',
    defaultValue: 'https://api.dev.ratel.foundation',
  );
  static const signDomain = String.fromEnvironment(
    'SIGN_DOMAIN',
    defaultValue: 'https://api.dev.ratel.foundation',
  );
}
