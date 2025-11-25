import 'package:ratel/exports.dart';

class EmailLoginForm extends GetWidget<LoginController> {
  const EmailLoginForm({super.key});

  @override
  Widget build(BuildContext context) {
    final viewInsets = MediaQuery.of(context).viewInsets.bottom;

    return Padding(
      padding: EdgeInsets.only(left: 20, right: 20, bottom: viewInsets),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          AppTopBar(
            onBack: controller.closeEmailForm,
            title: "Sign in",
            rightLabel: "Sign up",
            onRight: controller.goToSignup,
          ),
          20.vgap,
          Text(
            "Sign in to\nyour account",
            style: AppFonts.textTheme.headlineMedium?.copyWith(
              color: Colors.white,
              height: 1.1,
              fontSize: 34,
              fontWeight: FontWeight.w900,
            ),
          ),
          30.vgap,
          Row(
            children: [
              InkWell(
                onTap: controller.signInWithApple,
                borderRadius: BorderRadius.circular(999),
                child: Container(
                  width: 58,
                  height: 58,
                  decoration: const BoxDecoration(
                    color: Colors.white,
                    shape: BoxShape.circle,
                  ),
                  child: Center(
                    child: SvgPicture.asset(
                      Assets.apple,
                      width: 24,
                      height: 24,
                      colorFilter: const ColorFilter.mode(
                        Colors.black,
                        BlendMode.srcIn,
                      ),
                    ),
                  ),
                ),
              ),
              15.gap,
              InkWell(
                onTap: controller.signInWithGoogle,
                borderRadius: BorderRadius.circular(999),
                child: Container(
                  width: 58,
                  height: 58,
                  decoration: BoxDecoration(
                    color: AppColors.neutral900,
                    shape: BoxShape.circle,
                    border: Border.all(
                      color: AppColors.borderPrimary,
                      width: 1,
                    ),
                  ),
                  child: Center(
                    child: SvgPicture.asset(
                      Assets.google,
                      width: 24,
                      height: 24,
                    ),
                  ),
                ),
              ),
            ],
          ),
          16.vgap,
          Expanded(
            child: SingleChildScrollView(
              child: Column(
                children: [
                  AppTextField(
                    hint: 'Email',
                    controller: controller.emailCtrl,
                    focusNode: controller.emailFocus,
                    autofocus: true,
                    keyboardType: TextInputType.emailAddress,
                    onChanged: (val) => controller.email.value = val.trim(),
                  ),
                  20.vgap,
                  Obx(
                    () => AppTextField(
                      hint: 'Password',
                      controller: controller.passwordCtrl,
                      focusNode: controller.passwordFocus,
                      obscureText: !controller.showPassword.value,
                      onChanged: (val) =>
                          controller.password.value = val.trim(),
                    ),
                  ),
                  20.vgap,
                  SizedBox(
                    width: double.infinity,
                    child: Obx(
                      () => ElevatedButton(
                        onPressed:
                            controller.isFormValid && !controller.isBusy.value
                            ? controller.signIn
                            : null,
                        style: ElevatedButton.styleFrom(
                          backgroundColor: AppColors.primary,
                          disabledBackgroundColor: AppColors.primary.withValues(
                            alpha: 0.6,
                          ),
                          foregroundColor: Colors.black,
                          padding: const EdgeInsets.symmetric(vertical: 16.5),
                          shape: RoundedRectangleBorder(
                            borderRadius: BorderRadius.circular(10),
                          ),
                        ),
                        child: controller.isBusy.value
                            ? const SizedBox(
                                height: 20,
                                width: 20,
                                child: CircularProgressIndicator(
                                  strokeWidth: 2,
                                  color: AppColors.backgroundColor,
                                ),
                              )
                            : Text(
                                'Sign in',
                                style: AppFonts.textTheme.titleMedium?.copyWith(
                                  color: AppColors.backgroundColor,
                                  fontWeight: FontWeight.w700,
                                  fontSize: 16,
                                ),
                              ),
                      ),
                    ),
                  ),
                  30.vgap,
                  Text(
                    "Forgot password?",
                    style: AppFonts.textTheme.bodyMedium?.copyWith(
                      color: AppColors.neutral300,
                      fontSize: 15,
                      fontWeight: FontWeight.w400,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }
}
