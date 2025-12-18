import 'package:ratel/exports.dart';

class AppTextField extends StatelessWidget {
  final String hint;
  final TextEditingController? controller;
  final bool obscureText;
  final Widget? suffixIcon;
  final TextInputType keyboardType;
  final double rounded;
  final ValueChanged<String>? onChanged;
  final bool readOnly;
  final InputBorder? border;
  final Color? bgColor;

  final InputBorder? enabledBorderOverride;
  final InputBorder? focusedBorderOverride;

  const AppTextField({
    super.key,
    required this.hint,
    this.controller,
    this.obscureText = false,
    this.suffixIcon,
    this.keyboardType = TextInputType.text,
    this.onChanged,
    this.rounded = 8,
    this.readOnly = false,
    this.border,
    this.bgColor,

    this.enabledBorderOverride,
    this.focusedBorderOverride,
  });

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: controller,
      obscureText: obscureText,
      keyboardType: keyboardType,
      onChanged: onChanged,
      readOnly: readOnly,
      style: TextStyle(
        fontStyle: FontStyle.normal,
        color: readOnly ? AppColors.neutral600 : Colors.white,
        fontWeight: FontWeight.w500,
        fontSize: 15,
      ),
      decoration: InputDecoration(
        hintText: hint,
        hintStyle: TextStyle(
          fontStyle: FontStyle.normal,
          color: AppColors.neutral600,
          fontWeight: FontWeight.w500,
          fontSize: 15,
        ),
        filled: true,
        fillColor: bgColor ?? AppColors.background,
        suffixIcon: suffixIcon,
        contentPadding: const EdgeInsets.symmetric(
          horizontal: 20,
          vertical: 10,
        ),
        enabledBorder:
            enabledBorderOverride ??
            border ??
            OutlineInputBorder(
              borderRadius: BorderRadius.circular(rounded),
              borderSide: BorderSide.none,
            ),
        focusedBorder:
            focusedBorderOverride ??
            border ??
            OutlineInputBorder(
              borderRadius: BorderRadius.circular(rounded),
              borderSide: const BorderSide(color: AppColors.primary),
            ),
      ),
    );
  }
}
