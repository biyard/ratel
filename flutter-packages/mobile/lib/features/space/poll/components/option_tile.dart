import 'package:ratel/exports.dart';

class OptionTile extends StatelessWidget {
  const OptionTile({
    super.key,
    required this.label,
    required this.selected,
    required this.onTap,
    this.enabled = true,
  });

  final String label;
  final bool selected;
  final VoidCallback onTap;
  final bool enabled;

  @override
  Widget build(BuildContext context) {
    final Color fillColor = selected
        ? (enabled ? AppColors.primary : AppColors.primary.withOpacity(0.5))
        : Colors.transparent;

    final Color borderColor = enabled
        ? (selected ? AppColors.primary : AppColors.neutral80)
        : AppColors.neutral80.withOpacity(0.5);

    final Color textColor = enabled ? Colors.white : const Color(0xFF9CA3AF);

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: enabled ? onTap : null,
      child: Row(
        children: [
          Container(
            width: 20,
            height: 20,
            decoration: BoxDecoration(
              color: fillColor,
              borderRadius: BorderRadius.circular(4),
              border: Border.all(color: borderColor, width: 1.2),
            ),
            child: selected
                ? const Icon(Icons.check, size: 16, color: Color(0xFF1D1D1D))
                : null,
          ),
          10.gap,
          if (label != 'Others')
            Expanded(
              child: Text(
                label,
                style: TextStyle(
                  fontFamily: 'Inter',
                  fontSize: 14,
                  color: textColor,
                ),
              ),
            ),
        ],
      ),
    );
  }
}
