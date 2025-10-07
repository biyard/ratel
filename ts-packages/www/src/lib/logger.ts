/* eslint-disable @typescript-eslint/no-explicit-any */
import { config } from '@/config';

type LogLevel = 'debug' | 'info' | 'warn' | 'error' | 'silent';

const levels = ['debug', 'info', 'warn', 'error'];

function shouldLog(level: LogLevel) {
  return levels.indexOf(level) >= levels.indexOf(config.log_level as LogLevel);
}

export const logger = {
  debug: (...args: any[]) =>
    shouldLog('debug') && console.debug('%c[DEBUG]', 'color: gray', ...args),
  info: (...args: any[]) =>
    shouldLog('info') && console.info('%c[INFO]', 'color: blue', ...args),
  warn: (...args: any[]) =>
    shouldLog('warn') && console.warn('%c[WARN]', 'color: orange', ...args),
  error: (...args: any[]) =>
    shouldLog('error') &&
    console?.error('%c[ERROR]', 'color: red; font-weight: bold', ...args),
};
