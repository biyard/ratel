import 'package:ratel/exports.dart';

class BoardsCategoryBar extends StatelessWidget {
  final RxList<String> categories;
  final RxnString selectedCategory;
  final void Function(String?) onCategorySelected;

  const BoardsCategoryBar({
    super.key,
    required this.categories,
    required this.selectedCategory,
    required this.onCategorySelected,
  });

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Obx(() {
      final items = <String>['Total', ...categories];
      final selected = selectedCategory.value;

      return SizedBox(
        height: 35,
        child: ListView.separated(
          scrollDirection: Axis.horizontal,
          itemCount: items.length,
          separatorBuilder: (_, __) => 5.gap,
          itemBuilder: (context, index) {
            final label = items[index];
            final isAll = index == 0;
            final value = isAll ? null : label;
            final isSelected = isAll ? selected == null : selected == value;

            final bg = isSelected ? AppColors.primary : AppColors.neutral700;
            final fg = isSelected ? Colors.black : Colors.white;

            return GestureDetector(
              onTap: () => onCategorySelected(value),
              child: AnimatedContainer(
                duration: const Duration(milliseconds: 160),
                padding: const EdgeInsets.symmetric(
                  horizontal: 12,
                  vertical: 3,
                ),
                decoration: BoxDecoration(
                  color: bg,
                  borderRadius: BorderRadius.circular(8),
                ),
                child: Center(
                  child: Text(
                    label,
                    style: theme.textTheme.labelMedium?.copyWith(
                      color: fg,
                      fontWeight: isSelected
                          ? FontWeight.w700
                          : FontWeight.w500,
                    ),
                  ),
                ),
              ),
            );
          },
        ),
      );
    });
  }
}
