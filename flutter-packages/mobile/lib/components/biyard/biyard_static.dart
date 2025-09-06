import 'package:ratel/exports.dart';

enum BiyardSnackbarType { info, error, warning, reference }

class Biyard {
  static const _duration = Duration(seconds: 3);

  static SnackbarController snackbar(
    String message, [
    BiyardSnackbarType type = BiyardSnackbarType.error,
  ]) {
    final theme = ToastTheme.fromType(type);
    // final ctx = Get.overlayContext ?? Get.context;
    // final bottomInset = 0.0;

    return Get.rawSnackbar(
      titleText: const SizedBox.shrink(),
      messageText: ToastBody(
        title: theme.titleText,
        message: message,
        iconData: theme.icon,
        accent: theme.accent,
        start: theme.start,
        end: theme.end,
        duration: _duration,
      ),
      snackPosition: SnackPosition.BOTTOM,
      backgroundColor: Colors.transparent,
      margin: EdgeInsets.fromLTRB(15, 15, 15, 15),
      isDismissible: true,
      overlayBlur: 0.01,
      overlayColor: Colors.transparent,
      onTap: (snack) => Get.closeCurrentSnackbar(),
      borderRadius: 0,
      duration: _duration,
      forwardAnimationCurve: Curves.easeOutCubic,
      reverseAnimationCurve: Curves.easeInCubic,
    );
  }

  static SnackbarController info(String message) {
    return snackbar(message, BiyardSnackbarType.info);
  }

  static SnackbarController reference(String message) {
    return snackbar(message, BiyardSnackbarType.reference);
  }

  static Future error(dynamic err, String message) async {
    final firebase = Get.find<ByFirebase>();
    logger.e('$err : $message');
    await firebase.analytics.logEvent(
      name: "biyard-error",
      parameters: {"error": "$err", "message": message},
    );
    snackbar(message, BiyardSnackbarType.error);
  }

  static Future<T?> catchAsync<T>(
    Future<T> Function() handle,
    String failureMessage, [
    String? completedMessage,
  ]) async {
    try {
      final ret = await handle();
      if (completedMessage != null) info(completedMessage);
      return ret;
    } catch (e) {
      await error(e, failureMessage);
      return null;
    }
  }

  static Widget indicator() => const BiyardProgressIndicator();
}

class ToastTheme {
  final Color start;
  final Color end;
  final Color accent;
  final IconData icon;
  final String titleText;

  const ToastTheme({
    required this.start,
    required this.end,
    required this.accent,
    required this.icon,
    required this.titleText,
  });

  factory ToastTheme.fromType(BiyardSnackbarType t) {
    switch (t) {
      case BiyardSnackbarType.info:
        return ToastTheme(
          start: const Color(0xFF0F2C1F),
          end: const Color(0xFF17191C),
          accent: const Color(0xFF1ED760),
          icon: Icons.check_rounded,
          titleText: 'Saved Successfully',
        );
      case BiyardSnackbarType.reference:
        return ToastTheme(
          start: const Color(0xFF0F2C1F),
          end: const Color(0xFF17191C),
          accent: const Color(0xFF1ED760),
          icon: Icons.info_rounded,
          titleText: 'Saved Successfully',
        );
      case BiyardSnackbarType.warning:
        return ToastTheme(
          start: const Color(0xFF2E2203),
          end: const Color(0xFF17191C),
          accent: const Color(0xFFFFC024),
          icon: Icons.error_outline_rounded,
          titleText: 'Action Required',
        );
      case BiyardSnackbarType.error:
        return ToastTheme(
          start: const Color(0xFF2C0E0E),
          end: const Color(0xFF17191C),
          accent: const Color(0xFFFF4D4D),
          icon: Icons.close_rounded,
          titleText: 'Error Occurred',
        );
    }
  }
}

class ToastBody extends StatelessWidget {
  final String title;
  final String message;
  final IconData iconData;
  final Color accent;
  final Color start;
  final Color end;
  final Duration duration;

  const ToastBody({
    super.key,
    required this.title,
    required this.message,
    required this.iconData,
    required this.accent,
    required this.start,
    required this.end,
    required this.duration,
  });

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        Container(
          padding: const EdgeInsets.fromLTRB(16, 12, 16, 12),
          decoration: BoxDecoration(
            gradient: LinearGradient(
              begin: Alignment.centerLeft,
              end: Alignment.centerRight,
              colors: [start, end],
            ),
            borderRadius: BorderRadius.circular(16),
            boxShadow: [
              BoxShadow(
                color: Colors.black.withOpacity(.35),
                blurRadius: 18,
                offset: const Offset(0, 10),
              ),
            ],
          ),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Container(
                width: 20,
                height: 20,
                decoration: BoxDecoration(
                  color: accent,
                  shape: BoxShape.circle,
                  boxShadow: [
                    BoxShadow(
                      color: accent.withOpacity(.35),
                      blurRadius: 10,
                      offset: const Offset(0, 4),
                    ),
                  ],
                ),
                alignment: Alignment.center,
                child: Icon(iconData, size: 12, color: AppColors.neutral900),
              ),
              16.gap,
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      title,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                      style: const TextStyle(
                        color: Colors.white,
                        fontSize: 18,
                        fontWeight: FontWeight.w700,
                        height: 1.1,
                      ),
                    ),
                    3.vgap,
                    Text(
                      message,
                      maxLines: 2,
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        color: Colors.white,
                        fontSize: 12,
                        fontWeight: FontWeight.w600,
                        height: 1.1,
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ),
        ),
        Positioned(
          left: 12,
          right: 12,
          bottom: 0,
          child: SingleProgressBar(color: accent, duration: duration),
        ),
      ],
    );
  }
}

class SingleProgressBar extends StatelessWidget {
  final Color color;
  final Duration duration;
  const SingleProgressBar({
    super.key,
    required this.color,
    required this.duration,
  });

  @override
  Widget build(BuildContext context) {
    final br = const BorderRadius.only(
      bottomLeft: Radius.circular(16),
      bottomRight: Radius.circular(16),
    );

    return ClipRRect(
      borderRadius: br,
      child: Container(
        height: 2,
        color: color.withOpacity(.12),
        alignment: Alignment.bottomLeft,
        child: TweenAnimationBuilder<double>(
          tween: Tween(begin: 0, end: 1),
          duration: duration,
          curve: Curves.linear,
          builder: (_, v, __) {
            return FractionallySizedBox(
              widthFactor: v,
              child: Container(
                height: 4,
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(999),
                  gradient: LinearGradient(
                    colors: [color.withOpacity(.55), color],
                  ),
                  boxShadow: [
                    BoxShadow(
                      color: color.withOpacity(.28),
                      blurRadius: 8,
                      offset: const Offset(0, 1),
                    ),
                  ],
                ),
              ),
            );
          },
        ),
      ),
    );
  }
}
