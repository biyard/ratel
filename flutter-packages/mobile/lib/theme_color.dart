import 'package:flutter/material.dart';

@immutable
class ThemeColors extends ThemeExtension<ThemeColors> {
  final Color background;

  const ThemeColors({required this.background});

  static const light = ThemeColors(background: Colors.white);

  static const dark = ThemeColors(background: Colors.black);

  @override
  ThemeColors copyWith({Color? background}) {
    return ThemeColors(background: background ?? this.background);
  }

  @override
  ThemeColors lerp(ThemeExtension<ThemeColors>? other, double t) {
    if (other is! ThemeColors) return this;
    return ThemeColors(
      background: Color.lerp(background, other.background, t)!,
    );
  }
}
