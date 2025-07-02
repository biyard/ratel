import 'package:ratel/components/indicator/biyard_indicator.dart';
import 'package:ratel/exports.dart';
import 'package:ratel/services/firebase/firebase.dart';

enum BiyardSnackbarType { info, error, warning, reference }

class Biyard {
  static SnackbarController snackbar(
    String message, [
    BiyardSnackbarType type = BiyardSnackbarType.error,
  ]) {
    Color backgroundColor = Colors.red.withOpacity(0.3);

    if (type == BiyardSnackbarType.info ||
        type == BiyardSnackbarType.reference) {
      backgroundColor = Colors.blue.withOpacity(0.3);
    } else if (type == BiyardSnackbarType.warning) {
      backgroundColor = Colors.yellow.withOpacity(0.3);
    }
    String title = type == BiyardSnackbarType.error
        ? AppLocalization.errorTitle
        : type == BiyardSnackbarType.info
        ? AppLocalization.infoTitle
        : type == BiyardSnackbarType.reference
        ? AppLocalization.referenceTitle
        : AppLocalization.warningTitle;

    return Get.snackbar(
      title,
      message,
      snackPosition: SnackPosition.TOP,
      backgroundColor: backgroundColor,
      colorText: backgroundColor.withOpacity(1),
      margin: const EdgeInsets.all(10),
      borderRadius: 10,
      duration: const Duration(seconds: 3),
    );
  }

  static SnackbarController info(String message) {
    return snackbar(message, BiyardSnackbarType.info);
  }

  static SnackbarController reference(String message) {
    return snackbar(message, BiyardSnackbarType.reference);
  }

  static Future error(dynamic err, String message) async {
    ByFirebase firebaseService = Get.find<ByFirebase>();
    logger.e('$err : $message');
    await firebaseService.analytics.logEvent(
      name: "biyard-error",
      parameters: {"error": "$err", "message": message},
    );

    snackbar(message);
  }

  static Future<T?> catchAsync<T>(
    Future<T> Function() handle,
    String failureMessage, [
    String? completedMessage,
  ]) async {
    try {
      final ret = await handle();
      if (completedMessage != null) {
        Biyard.info(completedMessage);
      }
      return ret;
    } catch (e) {
      Biyard.error(e, failureMessage);

      return null;
    }
  }

  static Widget indicator() {
    return const BiyardProgressIndicator();
  }
}
