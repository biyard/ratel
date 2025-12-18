import 'package:ratel/exports.dart';
import 'package:ratel/features/onboarding/screens/signup/components/already_have_account_row.dart';
import 'package:ratel/features/onboarding/screens/signup/components/or_divider.dart';
import 'package:ratel/features/onboarding/screens/signup/components/signup_email_form.dart';
import 'package:ratel/features/onboarding/screens/signup/components/signup_logo.dart';
import 'package:ratel/features/onboarding/screens/signup/components/signup_phone_form.dart';
import 'package:ratel/features/onboarding/screens/signup/components/switch_signup_method_button.dart';

import 'signup_controller.dart';

class SignupScreen extends GetWidget<SignupController> {
  const SignupScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SignupController>(
      scrollable: false,
      child: SafeArea(
        child: Column(
          children: [
            Expanded(
              child: SingleChildScrollView(
                physics: const BouncingScrollPhysics(),
                padding: const EdgeInsets.only(top: 20),
                child: Column(
                  children: [
                    const SignupLogo(),
                    55.vgap,
                    Text(
                      'Sign up',
                      style: TextStyle(
                        fontSize: 24,
                        fontWeight: FontWeight.w800,
                        color: Colors.white,
                        height: 32 / 24,
                      ),
                    ),
                    20.vgap,
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 15),
                      child: Obx(() {
                        final isPhone = controller.isPhone;

                        return Column(
                          crossAxisAlignment: CrossAxisAlignment.stretch,
                          children: [
                            isPhone
                                ? SignupPhoneForm(
                                    countryCode:
                                        controller.selectedCountry.value.code,
                                    dialCode: controller
                                        .selectedCountry
                                        .value
                                        .dialCode,
                                    phoneController: controller.phoneCtrl,
                                    onTapCountry: () async {
                                      final selected =
                                          await showCountryPickerBottomSheet(
                                            context,
                                          );
                                      if (selected != null)
                                        controller.selectCountry(selected);
                                    },
                                    onPhoneChanged: controller.onPhoneChanged,
                                    onSubmit: controller.nextPhone,
                                  )
                                : SignupEmailForm(
                                    emailController: controller.emailCtrl,
                                    passwordController: controller.passwordCtrl,
                                    onEmailChanged: controller.onEmailChanged,
                                    onPasswordChanged:
                                        controller.onPasswordChanged,
                                    onSubmit: controller.nextEmail,
                                  ),

                            10.vgap,

                            SizedBox(
                              width: double.infinity,
                              child: Obx(() {
                                final canContinue = isPhone
                                    ? controller.isPhoneStepValid
                                    : controller.isEmailStepValid;

                                return ElevatedButton(
                                  onPressed: canContinue
                                      ? (isPhone
                                            ? controller.nextPhone
                                            : controller.nextEmail)
                                      : null,
                                  style: ElevatedButton.styleFrom(
                                    backgroundColor: AppColors.primary,
                                    disabledBackgroundColor: AppColors.primary
                                        .withValues(alpha: 0.6),
                                    foregroundColor: const Color(0xFF0A0A0A),
                                    padding: const EdgeInsets.symmetric(
                                      horizontal: 25,
                                      vertical: 15,
                                    ),
                                    shape: RoundedRectangleBorder(
                                      borderRadius: BorderRadius.circular(12),
                                    ),
                                    elevation: 0,
                                  ),
                                  child: Text(
                                    'Continue',
                                    style: AppFonts.textTheme.titleMedium
                                        ?.copyWith(
                                          fontWeight: FontWeight.w700,
                                          fontSize: 16,
                                          height: 18 / 16,
                                          color: const Color(0xFF0A0A0A),
                                        ),
                                  ),
                                );
                              }),
                            ),
                          ],
                        );
                      }),
                    ),
                    18.vgap,
                    const OrDivider(),
                    14.vgap,
                    Padding(
                      padding: const EdgeInsets.symmetric(horizontal: 15),
                      child: Obx(
                        () => controller.isPhone
                            ? SwitchSignupMethodButton(
                                icon: Assets.email,
                                label: 'Continue with email',
                                onTap: () =>
                                    controller.selectMethod(SignupMethod.email),
                              )
                            : SwitchSignupMethodButton(
                                icon: Assets.mobile,
                                label: 'Continue with phone',
                                onTap: () =>
                                    controller.selectMethod(SignupMethod.phone),
                              ),
                      ),
                    ),
                    40.vgap,
                  ],
                ),
              ),
            ),

            Padding(
              padding: const EdgeInsets.only(bottom: 18),
              child: AlreadyHaveAccountRow(onTapLogin: controller.goToLogin),
            ),
          ],
        ),
      ),
    );
  }
}

Future<CountryCode?> showCountryPickerBottomSheet(BuildContext context) {
  return showAppBottomSheet<CountryCode>(
    context: context,
    child: const CountryPickerSheet(),
  );
}
