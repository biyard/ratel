import 'package:ratel/exports.dart';
import 'package:ratel/features/onboarding/screens/login/components/email_form.dart';
import 'package:ratel/features/onboarding/screens/login/components/login_top_bar.dart';
import 'package:ratel/features/onboarding/screens/login/components/method_tab.dart';

class LoginScreen extends GetWidget<LoginController> {
  const LoginScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<LoginController>(
      scrollable: false,
      child: SafeArea(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Expanded(
              child: SingleChildScrollView(
                physics: const BouncingScrollPhysics(),
                padding: const EdgeInsets.only(bottom: 24),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    LoginTopBar(
                      title: 'Login',
                      onBack: () => Get.back(),
                      enableBack: true,
                    ),
                    6.vgap,
                    Obx(
                      () => MethodTabs(
                        leftLabel: 'Phone',
                        rightLabel: 'Email',
                        leftSelected: controller.isPhone,
                        onLeftTap: () =>
                            controller.selectMethod(LoginMethod.phone),
                        onRightTap: () =>
                            controller.selectMethod(LoginMethod.email),
                      ),
                    ),
                    22.vgap,
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 20),
                      child: Obx(() {
                        if (controller.isPhone) {
                          final cc = controller.selectedCountry.value;
                          return Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              PhoneNumberField(
                                countryCode: cc.code,
                                dialCode: cc.dialCode,
                                controller: controller.phoneCtrl,
                                onTapCountry: () async {
                                  final selected =
                                      await showCountryPickerBottomSheet(
                                        context,
                                      );
                                  if (selected != null) {
                                    controller.selectCountry(selected);
                                  }
                                },
                                onChanged: controller.onPhoneChanged,
                                onSubmit: controller.submit,
                              ),
                              if (controller.showWarning.value) ...[
                                const Padding(
                                  padding: EdgeInsets.only(top: 10),
                                  child: WarningMessage(
                                    message: 'Missing required fields',
                                  ),
                                ),
                              ],
                            ],
                          );
                        }

                        return EmailForm(
                          emailController: controller.emailCtrl,
                          passwordController: controller.passwordCtrl,
                          showPassword: controller.showPassword.value,
                          onEmailChanged: controller.onEmailChanged,
                          onPasswordChanged: controller.onPasswordChanged,
                          showWarning: controller.showWarning.value,
                        );
                      }),
                    ),
                  ],
                ),
              ),
            ),
            Padding(
              padding: const EdgeInsets.fromLTRB(20, 0, 20, 24),
              child: Column(
                children: [
                  Obx(() {
                    final enabled =
                        controller.isFormValid && !controller.isBusy.value;

                    return _ContinueButton(
                      enabled: enabled,
                      loading: controller.isBusy.value,
                      onTap: enabled
                          ? controller.submit
                          : controller.markWarningIfInvalid,
                    );
                  }),
                  10.vgap,
                  _SignupInlineRow(onTap: controller.goToSignup),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _SignupInlineRow extends StatelessWidget {
  const _SignupInlineRow({required this.onTap});

  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 22,
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Text(
            "Don’t have an account?",
            style: AppFonts.textTheme.bodyMedium?.copyWith(
              color: AppColors.neutral300,
              fontWeight: FontWeight.w400,
              fontSize: 15,
              height: 22 / 15,
            ),
          ),
          10.gap,
          GestureDetector(
            behavior: HitTestBehavior.opaque,
            onTap: onTap,
            child: Text(
              'Sign up',
              style: AppFonts.textTheme.bodyMedium?.copyWith(
                color: AppColors.primary,
                fontWeight: FontWeight.w400,
                fontSize: 15,
                height: 22 / 15,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _ContinueButton extends StatelessWidget {
  const _ContinueButton({
    required this.enabled,
    required this.loading,
    required this.onTap,
  });

  final bool enabled;
  final bool loading;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final bg = enabled
        ? AppColors.primary
        : AppColors.primary.withValues(alpha: 0.6);

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: loading ? null : onTap,
      child: Container(
        width: double.infinity,
        padding: const EdgeInsets.symmetric(horizontal: 25, vertical: 15),
        decoration: BoxDecoration(
          color: bg,
          borderRadius: BorderRadius.circular(12),
        ),
        child: Center(
          child: loading
              ? const SizedBox(
                  height: 20,
                  width: 20,
                  child: CircularProgressIndicator(
                    strokeWidth: 2,
                    color: Color(0xFF1D1D1D),
                  ),
                )
              : Text(
                  'Continue',
                  style: AppFonts.textTheme.titleMedium?.copyWith(
                    fontWeight: FontWeight.w700,
                    fontSize: 16,
                    height: 18 / 16,
                    color: const Color(0xFF0A0A0A),
                  ),
                ),
        ),
      ),
    );
  }
}
