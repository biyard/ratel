import 'package:ratel/exports.dart';

final darkTheme = ThemeData(
  brightness: Brightness.dark,
  pageTransitionsTheme: const PageTransitionsTheme(
    builders: {
      TargetPlatform.android: _SlideBuilder(),
      TargetPlatform.iOS: _SlideBuilder(),
    },
  ),
  visualDensity: VisualDensity.standard,
  fontFamily: "Raleway",
  fontFamilyFallback: ["Raleway"],
  textTheme: AppFonts.textTheme.apply(
    bodyColor: Colors.white,
    displayColor: Colors.white,
  ),
  scaffoldBackgroundColor: AppColors.bg,
  primaryColor: AppColors.primary,
  extensions: const <ThemeExtension<dynamic>>[ThemeColors.dark],
);

final lightTheme = darkTheme.copyWith(
  brightness: Brightness.light,
  scaffoldBackgroundColor: Colors.white,
  textTheme: AppFonts.textTheme.apply(
    bodyColor: AppColors.neutral900,
    displayColor: AppColors.neutral900,
  ),
  extensions: const <ThemeExtension<dynamic>>[ThemeColors.light],
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

ThemeData getThemeData(Brightness brightness) {
  return brightness == Brightness.dark ? darkTheme : lightTheme;
}
