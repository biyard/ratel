class Config {
  static const env = String.fromEnvironment('ENV');
  static const logLevel = String.fromEnvironment('LOG_LEVEL');
  static const redirectUrl = String.fromEnvironment(
    'REDIRECT_URL',
    defaultValue: 'https://dev.ratel.foundation',
  );
}
