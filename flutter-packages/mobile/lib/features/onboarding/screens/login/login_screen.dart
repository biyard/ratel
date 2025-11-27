import 'package:ratel/exports.dart';

class LoginScreen extends GetWidget<LoginController> {
  const LoginScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<LoginController>(
      scrollable: true,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            AppTopBar(
              onBack: () => {},
              title: "Sign in",
              rightLabel: "Sign up",
              onRight: controller.goToSignup,
              enableBack: false,
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
            AppTextField(
              hint: 'Email',
              controller: controller.emailCtrl,
              keyboardType: TextInputType.emailAddress,
              onChanged: (val) => controller.email.value = val.trim(),
            ),
            20.vgap,
            Obx(
              () => AppTextField(
                hint: 'Password',
                controller: controller.passwordCtrl,
                obscureText: !controller.showPassword.value,
                onChanged: (val) => controller.password.value = val.trim(),
              ),
            ),
            20.vgap,
            SizedBox(
              width: double.infinity,
              child: Obx(
                () => ElevatedButton(
                  onPressed: controller.isFormValid && !controller.isBusy.value
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
                            color: Color(0xff1d1d1d),
                          ),
                        )
                      : Text(
                          'Sign in',
                          style: AppFonts.textTheme.titleMedium?.copyWith(
                            color: Color(0xff1d1d1d),
                            fontWeight: FontWeight.w700,
                            fontSize: 16,
                          ),
                        ),
                ),
              ),
            ),
            40.vgap,
          ],
        ),
      ),
    );
  }
}
