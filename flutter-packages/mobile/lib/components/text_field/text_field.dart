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
  final FocusNode? focusNode;
  final bool autofocus;

  const AppTextField({
    super.key,
    required this.hint,
    this.controller,
    this.obscureText = false,
    this.suffixIcon,
    this.keyboardType = TextInputType.text,
    this.onChanged,
    this.rounded = 10,
    this.readOnly = false,
    this.border,
    this.bgColor,
    this.focusNode,
    this.autofocus = false,
  });

  @override
  Widget build(BuildContext context) {
    final defaultBorder = OutlineInputBorder(
      borderRadius: BorderRadius.circular(rounded),
      borderSide: BorderSide(color: AppColors.mobileFormline, width: 1),
    );

    return ConstrainedBox(
      constraints: const BoxConstraints(minHeight: 52),
      child: TextField(
        autofocus: autofocus,
        focusNode: focusNode,
        controller: controller,
        obscureText: obscureText,
        keyboardType: keyboardType,
        onChanged: onChanged,
        readOnly: readOnly,
        cursorColor: Colors.white,
        style: TextStyle(
          fontStyle: FontStyle.normal,
          color: readOnly ? AppColors.neutral600 : Colors.white,
          fontWeight: FontWeight.w500,
          fontSize: 15,
          height: 1.2,
        ),
        decoration: InputDecoration(
          hintText: hint,
          hintStyle: TextStyle(
            fontStyle: FontStyle.normal,
            color: AppColors.mobileFormPlaceholder,
            fontWeight: FontWeight.w500,
            fontSize: 15,
            height: 1.2,
          ),
          filled: true,
          fillColor: bgColor ?? AppColors.mobileFormBg,
          suffixIcon: suffixIcon,
          isDense: true,
          contentPadding: const EdgeInsets.symmetric(
            horizontal: 20,
            vertical: 14.5,
          ),
          enabledBorder: border ?? defaultBorder,
          focusedBorder:
              border ??
              OutlineInputBorder(
                borderRadius: BorderRadius.circular(rounded),
                borderSide: const BorderSide(
                  color: AppColors.primary,
                  width: 1,
                ),
              ),
        ),
      ),
    );
  }
}
