import 'package:ratel/exports.dart';

class BoardDetailTopBar extends StatelessWidget {
  final String categoryName;
  final VoidCallback onBackTap;
  final VoidCallback? onMoreTap;

  const BoardDetailTopBar({
    super.key,
    required this.categoryName,
    required this.onBackTap,
    this.onMoreTap,
  });

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(12, 20, 12, 10),
      child: Row(
        children: [
          // IconButton(
          //   icon: const Icon(Icons.arrow_back_ios, size: 18),
          //   color: Colors.white,
          //   onPressed: onBackTap,
          // ),
          10.gap,
          _BoardTagChip(label: 'Post'),
          if (categoryName.isNotEmpty) ...[
            6.gap,
            _BoardTagChip(label: categoryName),
          ],
          const Spacer(),
          if (onMoreTap != null)
            InkWell(
              onTap: onMoreTap,
              child: SvgPicture.asset(Assets.extra, width: 24, height: 24),
            ),
        ],
      ),
    );
  }
}

class _BoardTagChip extends StatelessWidget {
  final String label;

  const _BoardTagChip({required this.label});

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
      decoration: BoxDecoration(
        color: AppColors.neutral800,
        borderRadius: BorderRadius.circular(5),
        border: Border.all(color: AppColors.neutral700, width: 1),
      ),
      child: Text(
        label,
        style: theme.textTheme.labelSmall?.copyWith(
          color: Colors.white,
          fontWeight: FontWeight.w500,
        ),
      ),
    );
  }
}
