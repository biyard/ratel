import 'package:ratel/exports.dart';

class PostMoreBottomSheet extends StatelessWidget {
  const PostMoreBottomSheet({
    super.key,
    required this.onUpdate,
    required this.onDelete,
  });

  final VoidCallback onUpdate;
  final VoidCallback onDelete;

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      top: false,
      child: Container(
        width: double.infinity,
        padding: const EdgeInsets.fromLTRB(20, 12, 20, 24),
        decoration: const BoxDecoration(
          color: Color(0xFF191919),
          borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Center(
              child: Container(
                width: 50,
                height: 4,
                decoration: BoxDecoration(
                  color: const Color(0xFF3A3A3A),
                  borderRadius: BorderRadius.circular(999),
                ),
              ),
            ),
            20.vgap,
            _SheetItem(
              icon: SvgPicture.asset(Assets.edit1, width: 20, height: 20),
              label: 'Update',
              labelColor: Colors.white,
              onTap: onUpdate,
            ),
            12.vgap,
            _SheetItem(
              icon: SvgPicture.asset(Assets.deleteRed, width: 20, height: 20),
              label: 'Delete post',
              labelColor: const Color(0xFFEF4444),
              onTap: onDelete,
            ),
          ],
        ),
      ),
    );
  }
}

class _SheetItem extends StatelessWidget {
  const _SheetItem({
    required this.icon,
    required this.label,
    required this.labelColor,
    required this.onTap,
  });

  final SvgPicture icon;
  final String label;
  final Color labelColor;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      borderRadius: BorderRadius.circular(12),
      onTap: onTap,
      child: SizedBox(
        height: 48,
        child: Row(
          children: [
            icon,
            5.gap,
            Text(
              label,
              style: TextStyle(
                color: labelColor,
                fontSize: 16,
                fontWeight: FontWeight.w600,
                height: 24 / 16,
              ),
            ),
          ],
        ),
      ),
    );
  }
}
