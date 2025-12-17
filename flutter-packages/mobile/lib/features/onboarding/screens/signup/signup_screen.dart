import 'package:ratel/exports.dart';

class SignupScreen extends GetWidget<SignupController> {
  const SignupScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SignupController>(
      scrollable: false,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            AppTopBar(
              onBack: () {
                Get.rootDelegate.offNamed(loginScreen);
              },
              enableBack: true,
              title: "Sign up",
            ),
            40.vgap,
            Center(
              child: Text(
                'Enter your phone number',
                style: AppFonts.textTheme.headlineMedium?.copyWith(
                  color: Colors.white,
                  fontSize: 22,
                  fontWeight: FontWeight.w700,
                ),
                textAlign: TextAlign.center,
              ),
            ),
            30.vgap,
            Obx(
              () => _CountrySelector(
                countryName: controller.selectedCountry.value.name,
                dialCode: controller.selectedCountry.value.dialCode,
                onTap: () => _showCountryPicker(context, controller),
              ),
            ),
            15.vgap,
            AppTextField(
              hint: 'Phone Number',
              controller: controller.phoneCtrl,
              keyboardType: TextInputType.phone,
              onChanged: controller.onPhoneChanged,
              rounded: 10,
              bgColor: const Color(0xFF101010),
              enabledBorderOverride: OutlineInputBorder(
                borderRadius: BorderRadius.circular(10),
                borderSide: const BorderSide(
                  color: Color(0xFF2A2A2A),
                  width: 1,
                ),
              ),
              focusedBorderOverride: OutlineInputBorder(
                borderRadius: BorderRadius.circular(10),
                borderSide: const BorderSide(
                  color: AppColors.primary,
                  width: 1,
                ),
              ),
            ),
            const Spacer(),
            SizedBox(
              width: double.infinity,
              child: Obx(
                () => ElevatedButton(
                  onPressed:
                      controller.isPhoneStepValid && !controller.isBusy.value
                      ? controller.next
                      : null,
                  style: ElevatedButton.styleFrom(
                    backgroundColor: AppColors.primary,
                    disabledBackgroundColor: AppColors.primary.withValues(
                      alpha: 0.4,
                    ),
                    foregroundColor: Colors.black,
                    padding: const EdgeInsets.symmetric(vertical: 16),
                    shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(12),
                    ),
                  ),
                  child: controller.isBusy.value
                      ? const SizedBox(
                          height: 22,
                          width: 22,
                          child: CircularProgressIndicator(
                            strokeWidth: 2,
                            color: Colors.black,
                          ),
                        )
                      : const Text(
                          'OK',
                          style: TextStyle(
                            fontSize: 16,
                            fontWeight: FontWeight.w700,
                            color: Colors.black,
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

class _CountrySelector extends StatelessWidget {
  final String countryName;
  final String dialCode;
  final VoidCallback onTap;

  const _CountrySelector({
    required this.countryName,
    required this.dialCode,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(8),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
        decoration: BoxDecoration(
          color: AppColors.background,
          borderRadius: BorderRadius.circular(8),
          border: Border.all(
            color: AppColors.neutral600.withOpacity(0.6),
            width: 1,
          ),
        ),
        child: Row(
          children: [
            Expanded(
              child: Text(
                countryName,
                style: const TextStyle(color: Colors.white, fontSize: 14),
              ),
            ),
            Text(
              '+$dialCode',
              style: const TextStyle(
                color: AppColors.primary,
                fontWeight: FontWeight.w600,
              ),
            ),
            const SizedBox(width: 4),
            const Icon(Icons.arrow_drop_down, color: Colors.white),
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

Future<void> _showCountryPicker(
  BuildContext context,
  SignupController controller,
) async {
  final selected = await showCountryPickerBottomSheet(context);

  if (selected != null) {
    controller.selectCountry(selected);
  }
}
