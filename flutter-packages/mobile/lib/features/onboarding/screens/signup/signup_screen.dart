import 'package:ratel/exports.dart';
import 'package:ratel/features/onboarding/screens/signup/components/already_have_account_row.dart';
import 'package:ratel/features/onboarding/screens/signup/components/signup_logo.dart';
import 'package:ratel/features/onboarding/screens/signup/components/signup_phone_form.dart';

class SignupScreen extends GetWidget<SignupController> {
  const SignupScreen({super.key});

  @override
  Widget build(BuildContext context) {
    const pageBg = Color(0xFF1D1D1D);
    const panelBg = Color(0xFF171717);

    return Layout<SignupController>(
      scrollable: false,
      child: Container(
        color: pageBg,
        child: SafeArea(
          child: Column(
            children: [
              Padding(
                padding: const EdgeInsets.only(top: 20),
                child: const SignupLogo(),
              ),
              25.vgap,
              Expanded(
                child: Container(
                  width: double.infinity,
                  decoration: const BoxDecoration(
                    color: panelBg,
                    borderRadius: BorderRadius.vertical(
                      top: Radius.circular(24),
                    ),
                  ),
                  child: Column(
                    children: [
                      Expanded(
                        child: SingleChildScrollView(
                          physics: const BouncingScrollPhysics(),
                          padding: const EdgeInsets.fromLTRB(15, 30, 15, 0),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.stretch,
                            children: [
                              Text(
                                'Sign up',
                                textAlign: TextAlign.center,
                                style: AppFonts.textTheme.titleLarge?.copyWith(
                                  fontSize: 24,
                                  fontWeight: FontWeight.w800,
                                  color: Colors.white,
                                  height: 32 / 24,
                                ),
                              ),
                              20.vgap,
                              Obx(() {
                                final isPhone = controller.isPhone;

                                return Column(
                                  crossAxisAlignment:
                                      CrossAxisAlignment.stretch,
                                  children: [
                                    SignupPhoneForm(
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
                                        if (selected != null) {
                                          controller.selectCountry(selected);
                                        }
                                      },
                                      onPhoneChanged: controller.onPhoneChanged,
                                      onSubmit: controller.nextPhone,
                                    ),
                                    10.vgap,
                                    Obx(() {
                                      final canContinue = isPhone
                                          ? controller.isPhoneStepValid
                                          : controller.isEmailStepValid;

                                      return SizedBox(
                                        width: double.infinity,
                                        child: ElevatedButton(
                                          onPressed: canContinue
                                              ? (isPhone
                                                    ? controller.nextPhone
                                                    : controller.nextEmail)
                                              : null,
                                          style: ElevatedButton.styleFrom(
                                            backgroundColor: AppColors.primary,
                                            disabledBackgroundColor: AppColors
                                                .primary
                                                .withValues(alpha: 0.6),
                                            foregroundColor: const Color(
                                              0xFF0A0A0A,
                                            ),
                                            padding: const EdgeInsets.symmetric(
                                              horizontal: 25,
                                              vertical: 15,
                                            ),
                                            shape: RoundedRectangleBorder(
                                              borderRadius:
                                                  BorderRadius.circular(12),
                                            ),
                                            elevation: 0,
                                          ),
                                          child: Text(
                                            'Continue',
                                            style: AppFonts
                                                .textTheme
                                                .titleMedium
                                                ?.copyWith(
                                                  fontWeight: FontWeight.w700,
                                                  fontSize: 16,
                                                  height: 18 / 16,
                                                  color: const Color(
                                                    0xFF0A0A0A,
                                                  ),
                                                ),
                                          ),
                                        ),
                                      );
                                    }),
                                  ],
                                );
                              }),
                              40.vgap,
                            ],
                          ),
                        ),
                      ),
                      Padding(
                        padding: const EdgeInsets.only(bottom: 20),
                        child: AlreadyHaveAccountRow(
                          onTapLogin: controller.goToLogin,
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
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
