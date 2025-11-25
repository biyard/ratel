import 'package:ratel/exports.dart';
import 'package:ratel/theme_controller.dart';

class ThemeToggleButton extends StatelessWidget {
  const ThemeToggleButton({super.key});

  @override
  Widget build(BuildContext context) {
    return Obx(() {
      final isDark = ThemeController.to.themeMode.value == ThemeMode.dark;

      return IconButton(
        onPressed: ThemeController.to.toggleTheme,
        icon: Icon(isDark ? Icons.light_mode : Icons.dark_mode),
      );
    });
  }
}
