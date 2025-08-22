import 'package:ratel/exports.dart';

class SignupScreen extends GetWidget<SignupController> {
  const SignupScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<SignupController>(
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SizedBox(
              width: double.infinity,
              height: 70,
              child: Row(
                mainAxisSize: MainAxisSize.min,
                mainAxisAlignment: MainAxisAlignment.start,
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [
                  InkWell(onTap: controller.goBack, child: Assets.backIcon),
                  10.gap,
                  const Text(
                    'Sign up',
                    style: TextStyle(
                      fontStyle: FontStyle.normal,
                      color: Colors.white,
                      fontWeight: FontWeight.w600,
                      fontSize: 14,
                    ),
                  ),
                ],
              ),
            ),

            SizedBox(
              width: double.infinity,
              height: MediaQuery.of(context).size.height - 120,
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'Create\nyour account',
                    style: TextStyle(
                      color: Colors.white,
                      fontSize: 36,
                      fontWeight: FontWeight.w900,
                      height: 1.22,
                    ),
                  ),

                  30.vgap,

                  AppTextField(
                    hint: 'Email',
                    controller: controller.emailCtrl,
                    keyboardType: TextInputType.emailAddress,
                    onChanged: controller.onEmailChanged,
                  ),

                  30.vgap,

                  Obx(
                    () => AppTextField(
                      hint: 'Password',
                      controller: controller.passwordCtrl,
                      obscureText: !controller.showPassword.value,
                      onChanged: controller.onPasswordChanged,
                    ),
                  ),

                  30.vgap,

                  Obx(
                    () => AppTextField(
                      hint: 'Confirm Password',
                      controller: controller.confirmCtrl,
                      obscureText: !controller.showConfirm.value,
                      onChanged: controller.onConfirmChanged,
                    ),
                  ),

                  30.vgap,

                  if (!controller.isFormFilled) ...[
                    Text(
                      "Password must be at least 8 characters long and contain a combination of numbers, letters, and special characters.",
                      style: TextStyle(
                        fontSize: 12,
                        fontWeight: FontWeight.normal,
                        color: Colors.redAccent,
                      ),
                    ),
                    30.vgap,
                  ],

                  SizedBox(
                    width: double.infinity,
                    child: Obx(
                      () => ElevatedButton(
                        onPressed:
                            (controller.isFormFilled &&
                                !controller.isBusy.value)
                            ? controller.next
                            : null,
                        style: ElevatedButton.styleFrom(
                          backgroundColor: AppColors.primary,
                          disabledBackgroundColor: AppColors.primary.withValues(
                            alpha: 0.6,
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
                                'NEXT',
                                style: TextStyle(
                                  fontStyle: FontStyle.normal,
                                  color: AppColors.bg,
                                  fontWeight: FontWeight.w700,
                                  fontSize: 16,
                                ),
                              ),
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
