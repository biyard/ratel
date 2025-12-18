import 'package:ratel/exports.dart';

class ChoiceBox extends StatelessWidget {
  const ChoiceBox({
    super.key,
    required this.child,
    required this.selected,
    required this.enabled,
    required this.onTap,
  });

  final Widget child;
  final bool selected;
  final bool enabled;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    final tap = enabled ? onTap : null;

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: tap,
      child: Container(
        width: double.infinity,
        decoration: BoxDecoration(
          color: const Color(0xFF171717),
          borderRadius: BorderRadius.circular(10),
        ),
        padding: const EdgeInsets.symmetric(vertical: 20),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 15),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              Container(
                width: 20,
                height: 20,
                decoration: BoxDecoration(
                  color: selected ? AppColors.primary : const Color(0xFF101010),
                  borderRadius: BorderRadius.circular(4),
                  border: Border.all(
                    width: 2,
                    color: selected
                        ? AppColors.primary
                        : const Color(0xFF737373),
                  ),
                ),
                alignment: Alignment.center,
                child: selected
                    ? const Icon(
                        Icons.check,
                        size: 14,
                        color: Color(0xFF1D1D1D),
                      )
                    : null,
              ),
              20.gap,
              Expanded(child: child),
            ],
          ),
        ),
      ),
    );
  }
}
