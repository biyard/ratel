import 'package:ratel/exports.dart';

class ChoiceBox extends StatelessWidget {
  const ChoiceBox({
    super.key,
    required this.selected,
    required this.child,
    this.onTap,
    this.enabled = true,
  });

  final bool selected;
  final Widget child;
  final VoidCallback? onTap;
  final bool enabled;

  @override
  Widget build(BuildContext context) {
    final selectedColor = enabled
        ? AppColors.primary
        : AppColors.primary.withAlpha(125);

    final checkBg = selected ? selectedColor : const Color(0xFF101010);
    final checkBorder = selected ? Colors.transparent : const Color(0xFF737373);

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: enabled ? onTap : null,
      child: Container(
        height: 72,
        padding: const EdgeInsets.symmetric(horizontal: 15),
        decoration: BoxDecoration(
          color: const Color(0xFF171717),
          borderRadius: BorderRadius.circular(10),
        ),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Container(
              width: 20,
              height: 20,
              alignment: Alignment.center,
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(4),
                border: Border.all(color: checkBorder, width: 2),
                color: checkBg,
              ),
              child: selected
                  ? const Icon(Icons.check, size: 16, color: Color(0xFF0A0A0A))
                  : null,
            ),
            20.gap,
            Expanded(child: child),
          ],
        ),
      ),
    );
  }
}
