import 'package:ratel/exports.dart';

class MethodTabs extends StatelessWidget {
  const MethodTabs({
    super.key,
    required this.leftLabel,
    required this.rightLabel,
    required this.leftSelected,
    required this.onLeftTap,
    required this.onRightTap,
  });

  final String leftLabel;
  final String rightLabel;
  final bool leftSelected;
  final VoidCallback onLeftTap;
  final VoidCallback onRightTap;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 15),
      decoration: const BoxDecoration(
        border: Border(bottom: BorderSide(color: Color(0xFF262626), width: 1)),
      ),
      child: Row(
        children: [
          Expanded(
            child: _TabItem(
              label: leftLabel,
              selected: leftSelected,
              onTap: onLeftTap,
            ),
          ),
          Expanded(
            child: _TabItem(
              label: rightLabel,
              selected: !leftSelected,
              onTap: onRightTap,
            ),
          ),
        ],
      ),
    );
  }
}

class _TabItem extends StatelessWidget {
  const _TabItem({
    required this.label,
    required this.selected,
    required this.onTap,
  });

  final String label;
  final bool selected;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: onTap,
      child: Column(
        mainAxisAlignment: MainAxisAlignment.end,
        children: [
          Text(
            label,
            style: AppFonts.textTheme.titleMedium?.copyWith(
              color: Colors.white,
              fontWeight: FontWeight.w700,
              fontSize: 16,
              height: 24 / 16,
            ),
          ),
          10.vgap,
          Container(
            height: 2,
            width: double.infinity,
            color: selected ? Colors.white : Colors.transparent,
          ),
        ],
      ),
    );
  }
}
