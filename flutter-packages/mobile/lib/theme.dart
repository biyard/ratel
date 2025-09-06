import 'package:ratel/exports.dart';

final darkTheme = ThemeData(
  brightness: Brightness.dark,
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
  primaryColor: AppColors.primary,
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

// ThemeData for light theme.
ThemeData _getLightTheme() {
  return darkTheme.copyWith(brightness: Brightness.light);
}

// Returns the theme data for the application by Brightness.
ThemeData getThemeData(Brightness brightness) {
  switch (brightness) {
    case Brightness.light:
      return _getLightTheme();
    case Brightness.dark:
      return darkTheme;
  }
}
