import 'package:logger/logger.dart';

// BiyardLogger logger = BiyardLogger(Logger(), false);
Logger logger = Logger();

void initializeLogger(String level, [bool isRelease = false]) {
  Level lvl = Level.error;

  switch (level) {
    case "debug":
      lvl = Level.debug;
      break;
    case "trace":
      lvl = Level.trace;
      break;
    case "warn":
      lvl = Level.warning;
      break;
    case "error":
      lvl = Level.error;
      break;
    default:
      lvl = Level.info;
  }

  // logger = BiyardLogger(
  //     Logger(
  //       level: lvl,
  //       printer: PrettyPrinter(
  //         methodCount: lvl == Level.debug ? 8 : 2,
  //         errorMethodCount: 8,
  //         lineLength: 120,
  //         colors: true,
  //         printEmojis: true,
  //         printTime: true,
  //       ),
  //     ),
  //     isRelease);

  logger = Logger(
    level: lvl,
    printer: PrettyPrinter(
      methodCount: lvl == Level.debug ? 8 : 2,
      errorMethodCount: 8,
      lineLength: 120,
      colors: true,
      printEmojis: true,
      printTime: true,
    ),
  );
}

class BiyardLogger {
  final Logger l;
  final bool isRelease;

  BiyardLogger(this.l, this.isRelease);

  void d(dynamic message) {
    isRelease ? print('[DEBUG] $message') : l.d(message);
  }

  void i(dynamic message) {
    isRelease ? print('[INFO] $message') : l.i(message);
  }

  void w(dynamic message) {
    isRelease ? print('[WARN] $message') : l.w(message);
  }

  void e(dynamic message) {
    isRelease ? print('[ERROR] $message') : l.e(message);
  }
}
