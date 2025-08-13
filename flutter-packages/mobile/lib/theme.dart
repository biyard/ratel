import 'package:ratel/exports.dart';

final theme = ThemeData(
  pageTransitionsTheme: PageTransitionsTheme(
    builders: {
      TargetPlatform.android: const _SlideBuilder(),
      TargetPlatform.iOS: const _SlideBuilder(),
    },
  ),
  visualDensity: VisualDensity.standard,
  fontFamily: "Raleway",
  fontFamilyFallback: ["Raleway"],
  textTheme: AppFonts.textTheme,
  scaffoldBackgroundColor: AppColors.bg,
);

class _SlideBuilder extends PageTransitionsBuilder {
  const _SlideBuilder();
  @override
  Widget buildTransitions<T>(route, context, animation, secondary, child) {
    final tween = Tween(
      begin: const Offset(1, 0),
      end: Offset.zero,
    ).chain(CurveTween(curve: Curves.easeOutCubic));
    return SlideTransition(position: animation.drive(tween), child: child);
  }
}
