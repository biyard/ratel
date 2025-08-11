import 'package:ratel/exports.dart';

class CustomCheckbox extends StatelessWidget {
  final bool value;
  final VoidCallback onChanged;

  const CustomCheckbox({
    super.key,
    required this.value,
    required this.onChanged,
  });

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onChanged,
      borderRadius: BorderRadius.circular(6),
      child: Container(
        width: 24,
        height: 24,
        decoration: BoxDecoration(
          color: value ? AppColors.primary : Colors.transparent,
          borderRadius: BorderRadius.circular(6),
          border: Border.all(
            color: value ? AppColors.primary : AppColors.neutral300,
            width: 2,
          ),
        ),
        child: value
            ? const Icon(Icons.check, color: Colors.black, size: 18)
            : null,
      ),
    );
  }
}
