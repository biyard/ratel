// login_screen.dart
import 'package:ratel/exports.dart';

class LoginScreen extends GetWidget<LoginController> {
  const LoginScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<LoginController>(
      child: SizedBox(
        width: double.infinity,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SizedBox(
              width: double.infinity,
              height: 100,
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.center,
                children: [30.vgap, Image.asset(Assets.logoLetter, width: 182)],
              ),
            ),
            SizedBox(
              height: MediaQuery.of(context).size.height - 250,
              child: Container(
                margin: const EdgeInsets.symmetric(horizontal: 20),
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(
                      "Sign in to your account",
                      style: TextStyle(
                        fontStyle: FontStyle.normal,
                        color: Colors.white,
                        fontWeight: FontWeight.w600,
                        fontSize: 14,
                      ),
                    ),
                    30.vgap,
                    AppTextField(
                      hint: 'Email',
                      controller: controller.emailCtrl,
                      keyboardType: TextInputType.emailAddress,
                      onChanged: (val) => controller.email.value = val.trim(),
                    ),
                    30.vgap,
                    Obx(
                      () => AppTextField(
                        hint: 'Password',
                        controller: controller.passwordCtrl,
                        obscureText: !controller.showPassword.value,
                        onChanged: (val) =>
                            controller.password.value = val.trim(),
                      ),
                    ),
                    30.vgap,
                    Container(
                      margin: const EdgeInsets.symmetric(horizontal: 6),
                      width: double.infinity,
                      child: Obx(
                        () => ElevatedButton(
                          onPressed:
                              controller.isFormValid && !controller.isBusy.value
                              ? controller.signIn
                              : null,
                          style: ElevatedButton.styleFrom(
                            backgroundColor: AppColors.primary,
                            disabledBackgroundColor: AppColors.primary
                                .withValues(alpha: 0.6),
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
                                  'SIGN IN',
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
                    30.vgap,
                    SizedBox(
                      width: double.infinity,
                      child: Column(
                        mainAxisAlignment: MainAxisAlignment.center,
                        crossAxisAlignment: CrossAxisAlignment.center,
                        children: [
                          Text(
                            "- Or Sign in with -",
                            style: TextStyle(
                              fontStyle: FontStyle.normal,
                              color: Colors.white,
                              fontWeight: FontWeight.w600,
                              fontSize: 14,
                            ),
                          ),
                          20.vgap,
                          Row(
                            mainAxisAlignment: MainAxisAlignment.center,
                            crossAxisAlignment: CrossAxisAlignment.center,
                            children: [
                              InkWell(
                                onTap: controller.signInWithGoogle,
                                child: Assets.googleImage,
                              ),
                              10.gap,
                              InkWell(
                                onTap: controller.signInWithApple,
                                child: Assets.appleImage,
                              ),
                            ],
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            ),
            SizedBox(
              width: double.infinity,
              height: 100,
              child: Column(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Text(
                        "Don't have an account?  ",
                        style: TextStyle(
                          fontStyle: FontStyle.normal,
                          fontWeight: FontWeight.w400,
                          fontSize: 12,
                          color: Colors.white,
                        ),
                      ),
                      10.gap,
                      GestureDetector(
                        onTap: controller.goToSignup,
                        child: const Text(
                          'Sign up',
                          style: TextStyle(
                            fontStyle: FontStyle.normal,
                            color: AppColors.primary,
                            fontWeight: FontWeight.w600,
                            fontSize: 12,
                          ),
                        ),
                      ),
                    ],
                  ),
                  50.vgap,
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
