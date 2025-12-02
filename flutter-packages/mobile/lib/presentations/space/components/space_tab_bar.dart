import 'package:ratel/exports.dart';

class SpaceTabBar extends StatelessWidget {
  const SpaceTabBar({super.key, required this.controller});

  final SpaceController controller;

  @override
  Widget build(BuildContext context) {
    final tabs = controller.tabs;
    if (tabs.isEmpty) {
      return const SizedBox.shrink();
    }

    return SizedBox(
      height: 30,
      child: SingleChildScrollView(
        scrollDirection: Axis.horizontal,
        child: Obx(() {
          final current = controller.currentTab.value;

          return Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              for (var i = 0; i < tabs.length; i++) ...[
                SpaceTabButton(
                  label: tabs[i].label,
                  isSelected: current.id == tabs[i].id,
                  onTap: () => controller.onTabSelected(tabs[i]),
                ),
                if (i != tabs.length - 1) 10.gap,
              ],
            ],
          );
        }),
      ),
    );
  }
}

class SpaceTabButton extends StatelessWidget {
  const SpaceTabButton({
    super.key,
    required this.label,
    required this.isSelected,
    required this.onTap,
  });

  final String label;
  final bool isSelected;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 3),
        decoration: BoxDecoration(
          color: isSelected ? Colors.white : const Color(0xFF1D1D1D),
          borderRadius: BorderRadius.circular(50),
          border: Border.all(color: Colors.white, width: 0.5),
        ),
        child: Text(
          label,
          textAlign: TextAlign.center,
          style: TextStyle(
            fontFamily: 'Inter',
            fontWeight: FontWeight.w500,
            fontSize: 16,
            height: 24 / 16,
            color: isSelected ? const Color(0xFF262626) : Colors.white,
          ),
        ),
      ),
    );
  }
}
