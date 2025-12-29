import 'package:ratel/exports.dart';

class SignupEmailForm extends StatelessWidget {
  const SignupEmailForm({
    super.key,
    required this.emailController,
    required this.passwordController,
    required this.onEmailChanged,
    required this.onPasswordChanged,
    required this.onSubmit,
  });

  final TextEditingController emailController;
  final TextEditingController passwordController;
  final ValueChanged<String> onEmailChanged;
  final ValueChanged<String> onPasswordChanged;
  final VoidCallback onSubmit;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        AppTextField(
          hint: 'Email',
          controller: emailController,
          keyboardType: TextInputType.emailAddress,
          onChanged: (v) => onEmailChanged(v.trim()),
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
        15.vgap,
        AppTextField(
          hint: 'Password',
          controller: passwordController,
          obscureText: true,
          keyboardType: TextInputType.visiblePassword,
          onChanged: (v) => onPasswordChanged(v.trim()),
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
      ],
    );
  }
}
