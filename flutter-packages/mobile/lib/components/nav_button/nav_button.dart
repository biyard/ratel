import 'package:ratel/exports.dart';

enum NavButtonVariant { outline, primary, submit }

class NavButton extends StatelessWidget {
  const NavButton({
    super.key,
    required this.label,
    required this.enabled,
    this.tapEnabled,
    required this.onTap,
    required this.isPrimary,
    this.variant = NavButtonVariant.primary,
    this.horizontalPadding = 25,
    this.verticalPadding = 12,
    this.radius = 12,
    this.fontSize = 16,
    this.fontWeight = FontWeight.w700,
    this.height = 18 / 16,
    this.iconSize = 24,
  });

  final String label;
  final bool enabled;
  final bool? tapEnabled;
  final VoidCallback onTap;
  final bool isPrimary;
  final NavButtonVariant variant;

  final double horizontalPadding;
  final double verticalPadding;
  final double radius;
  final double fontSize;
  final FontWeight fontWeight;
  final double height;
  final double iconSize;

  @override
  Widget build(BuildContext context) {
    final isOutline = variant == NavButtonVariant.outline;
    final isSubmit = variant == NavButtonVariant.submit;

    final bgColor = isSubmit
        ? const Color(0xFFFCB300)
        : (isPrimary ? Colors.white : Colors.transparent);

    final textColor = isSubmit
        ? const Color(0xFF0A0A0A)
        : (isPrimary ? const Color(0xFF0A0A0A) : Colors.white);

    final disabledBg = isSubmit
        ? const Color(0xFFFCB300).withAlpha(140)
        : (isPrimary ? Colors.white.withAlpha(125) : Colors.transparent);

    final effectiveBg = enabled ? bgColor : disabledBg;
    final effectiveText = enabled
        ? textColor
        : (isSubmit ? const Color(0xFF0A0A0A) : const Color(0xFF737373));

    final borderColor = isOutline
        ? (enabled ? Colors.white : const Color(0xFF737373))
        : Colors.transparent;

    Widget chevron() {
      if (isSubmit) return const SizedBox.shrink();
      final iconColor = enabled
          ? const Color(0xFF737373)
          : const Color(0xFF404040);
      return Icon(
        isPrimary ? Icons.chevron_right : Icons.chevron_left,
        size: iconSize,
        color: iconColor,
      );
    }

    final canTap = tapEnabled ?? enabled;

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: canTap ? onTap : null,
      child: Container(
        padding: EdgeInsets.symmetric(
          horizontal: horizontalPadding,
          vertical: verticalPadding,
        ),
        decoration: BoxDecoration(
          color: effectiveBg,
          borderRadius: BorderRadius.circular(radius),
          border: Border.all(color: borderColor, width: 1),
        ),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            if (!isPrimary && !isSubmit) ...[chevron(), 8.gap],
            Text(
              label,
              style: TextStyle(
                fontWeight: fontWeight,
                fontSize: fontSize,
                height: height,
                color: effectiveText,
              ),
            ),
            if (isPrimary && !isSubmit) ...[8.gap, chevron()],
          ],
        ),
      ),
    );
  }
}
