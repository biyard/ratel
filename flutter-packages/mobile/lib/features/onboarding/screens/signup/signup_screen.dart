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
      enableSafeArea: false,
      style: const LayoutStyle(background: pageBg),
      child: Container(
        color: pageBg,
        child: SafeArea(
          bottom: false,
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
                                      onSubmit: controller.onContinueTap,
                                    ),
                                    if (controller.showWarning.value) ...[
                                      const Padding(
                                        padding: EdgeInsets.only(top: 10),
                                        child: Align(
                                          alignment: Alignment.centerLeft,
                                          widthFactor: 1,
                                          child: IntrinsicWidth(
                                            child: WarningMessage(
                                              message:
                                                  'Missing required fields',
                                            ),
                                          ),
                                        ),
                                      ),
                                    ],
                                    10.vgap,
                                  ],
                                );
                              }),
                              40.vgap,
                            ],
                          ),
                        ),
                      ),
                      Obx(() {
                        final isPhone = controller.isPhone;

                        final canContinue = isPhone
                            ? controller.isPhoneStepValid
                            : controller.isEmailStepValid;

                        final bg = canContinue
                            ? AppColors.primary
                            : AppColors.primary.withValues(alpha: 0.6);
                        return Container(
                          width: double.infinity,
                          padding: EdgeInsets.fromLTRB(15, 0, 15, 0),
                          child: ElevatedButton(
                            onPressed: controller.onContinueTap,
                            style: ElevatedButton.styleFrom(
                              backgroundColor: bg,
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
                              style: AppFonts.textTheme.titleMedium?.copyWith(
                                fontWeight: FontWeight.w700,
                                fontSize: 16,
                                height: 18 / 16,
                                color: const Color(0xFF0A0A0A),
                              ),
                            ),
                          ),
                        );
                      }),
                      10.vgap,
                      SafeArea(
                        top: false,
                        child: Padding(
                          padding: const EdgeInsets.only(bottom: 20),
                          child: AlreadyHaveAccountRow(
                            onTapLogin: controller.goToLogin,
                          ),
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
