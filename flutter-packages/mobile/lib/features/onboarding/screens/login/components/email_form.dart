import 'package:ratel/exports.dart';

class EmailForm extends StatelessWidget {
  const EmailForm({
    super.key,
    required this.emailController,
    required this.passwordController,
    required this.showPassword,
    required this.onEmailChanged,
    required this.onPasswordChanged,
    required this.showWarning,
  });

  final TextEditingController emailController;
  final TextEditingController passwordController;

  final bool showPassword;
  final ValueChanged<String> onEmailChanged;
  final ValueChanged<String> onPasswordChanged;

  final bool showWarning;

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisAlignment: MainAxisAlignment.start,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        AppTextField(
          hint: 'Email',
          controller: emailController,
          keyboardType: TextInputType.emailAddress,
          onChanged: onEmailChanged,
          rounded: 10,
          bgColor: const Color(0xFF101010),
          enabledBorderOverride: OutlineInputBorder(
            borderRadius: BorderRadius.circular(10),
            borderSide: const BorderSide(color: Color(0xFF2A2A2A), width: 1),
          ),
          focusedBorderOverride: OutlineInputBorder(
            borderRadius: BorderRadius.circular(10),
            borderSide: const BorderSide(color: AppColors.primary, width: 1),
          ),
        ),
        14.vgap,
        AppTextField(
          hint: 'Password',
          controller: passwordController,
          obscureText: !showPassword,
          keyboardType: TextInputType.visiblePassword,
          onChanged: onPasswordChanged,
          rounded: 10,
          bgColor: const Color(0xFF101010),
          enabledBorderOverride: OutlineInputBorder(
            borderRadius: BorderRadius.circular(10),
            borderSide: const BorderSide(color: Color(0xFF2A2A2A), width: 1),
          ),
          focusedBorderOverride: OutlineInputBorder(
            borderRadius: BorderRadius.circular(10),
            borderSide: const BorderSide(color: AppColors.primary, width: 1),
          ),
        ),
        if (showWarning) ...[
          const Padding(
            padding: EdgeInsets.only(top: 10),
            child: WarningMessage(message: 'Missing required fields'),
          ),
        ],
      ],
    );
  }
}
