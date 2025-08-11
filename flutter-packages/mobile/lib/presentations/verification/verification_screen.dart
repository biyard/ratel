import 'package:ratel/exports.dart';

class VerificationScreen extends GetWidget<VerificationController> {
  const VerificationScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<VerificationController>(
      child: SafeArea(
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
                      'Email verification',
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
                    const Text(
                      'Check your email',
                      style: TextStyle(
                        color: Colors.white,
                        fontSize: 36,
                        fontWeight: FontWeight.w900,
                        height: 1.22,
                      ),
                    ),
                    30.vgap,

                    Obx(
                      () => RichText(
                        text: TextSpan(
                          style: TextStyle(
                            color: AppColors.neutral300,
                            fontSize: 12,
                            height: 1.4,
                            fontWeight: FontWeight.w400,
                          ),
                          children: [
                            const TextSpan(text: 'Please check your email '),
                            TextSpan(
                              text: controller.email,
                              style: const TextStyle(
                                color: Colors.white,
                                fontWeight: FontWeight.w700,
                                fontSize: 12,
                                height: 1.4,
                              ),
                            ),
                            const TextSpan(
                              text:
                                  ' inbox.\nIf you couldn’t find email, it may be inside spam.',
                            ),
                          ],
                        ),
                      ),
                    ),

                    30.vgap,

                    Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      crossAxisAlignment: CrossAxisAlignment.center,
                      children: List.generate(6, (i) {
                        return _CodeBox(
                          controller: controller.fields[i],
                          focusNode: controller.nodes[i],
                          inputFormatters: controller.codeInputFormatters,
                          onChanged: (v) => controller.onChanged(i, v),
                          onKey: (e) => controller.onKey(i, e),
                        );
                      }),
                    ),

                    30.vgap,

                    SizedBox(
                      width: double.infinity,
                      child: Obx(
                        () => ElevatedButton(
                          onPressed:
                              controller.isComplete && !controller.isBusy.value
                              ? controller.verify
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
                                  'VERIFY',
                                  style: TextStyle(
                                    color: AppColors.bg,
                                    fontSize: 16,
                                    fontWeight: FontWeight.w700,
                                  ),
                                ),
                        ),
                      ),
                    ),

                    30.vgap,

                    Row(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Text(
                          "Don't receive verification code?  ",
                          style: TextStyle(
                            color: AppColors.neutral300,
                            fontWeight: FontWeight.w400,
                            fontSize: 12,
                          ),
                        ),
                        10.gap,
                        InkWell(
                          onTap: () => {
                            showResendModal(context, controller.email),
                          },
                          child: const Text(
                            'Resend',
                            style: TextStyle(
                              color: AppColors.primary,
                              fontWeight: FontWeight.w400,
                              fontSize: 12,
                            ),
                          ),
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
    );
  }
}

void showResendModal(BuildContext ctx, String email) {
  final controller = Get.find<VerificationController>();

  showDialog(
    context: ctx,
    // barrierDismissible: false,
    builder: (BuildContext context) {
      return AlertDialog(
        backgroundColor: AppColors.bg,
        surfaceTintColor: AppColors.bg,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(20.0),
        ),
        content: FittedBox(
          fit: BoxFit.cover,
          child: SizedBox(
            width: 350,
            child: Column(
              children: [
                Text(
                  "Resend email",
                  style: TextStyle(
                    color: Colors.white,
                    fontWeight: FontWeight.w700,
                    fontSize: 24,
                    height: 1.33,
                  ),
                ),
                24.vgap,
                Text(
                  "It will send verification code to",
                  style: TextStyle(
                    color: AppColors.neutral300,
                    fontWeight: FontWeight.w400,
                    fontSize: 12,
                    height: 1.33,
                  ),
                ),
                10.vgap,
                Text(
                  email,
                  style: TextStyle(
                    color: AppColors.neutral300,
                    fontWeight: FontWeight.w700,
                    fontSize: 12,
                    height: 1.33,
                  ),
                ),
                35.vgap,
                Row(
                  mainAxisAlignment: MainAxisAlignment.center,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    InkWell(
                      onTap: () => Navigator.pop(context),
                      child: RoundContainer(
                        width: 95,
                        height: 50,
                        color: Colors.transparent,
                        radius: 10,
                        child: Padding(
                          padding: EdgeInsets.fromLTRB(20, 15, 20, 15),
                          child: Text(
                            "Cancel",
                            style: TextStyle(
                              color: AppColors.neutral300,
                              fontWeight: FontWeight.w700,
                              fontSize: 16,
                            ),
                          ),
                        ),
                      ),
                    ),
                    10.gap,
                    InkWell(
                      onTap: controller.isBusy.value
                          ? null
                          : () async {
                              await controller.resend();
                              Navigator.pop(context);
                            },
                      child: RoundContainer(
                        width: 180,
                        height: 50,
                        color: AppColors.primary,
                        radius: 10,
                        child: Center(
                          child: Padding(
                            padding: EdgeInsets.fromLTRB(20, 15, 20, 15),
                            child: Text(
                              "Resend",
                              style: TextStyle(
                                color: AppColors.bg,
                                fontWeight: FontWeight.w700,
                                fontSize: 16,
                              ),
                            ),
                          ),
                        ),
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      );
    },
  );
}

class _CodeBox extends StatelessWidget {
  const _CodeBox({
    required this.controller,
    required this.focusNode,
    required this.inputFormatters,
    required this.onChanged,
    required this.onKey,
  });

  final TextEditingController controller;
  final FocusNode focusNode;
  final List<TextInputFormatter> inputFormatters;
  final ValueChanged<String> onChanged;
  final KeyEventResult Function(KeyEvent) onKey;

  @override
  Widget build(BuildContext context) {
    return Container(
      width: 42,
      height: 48,
      margin: const EdgeInsets.only(right: 10),
      child: KeyboardListener(
        focusNode: FocusNode(),
        onKeyEvent: onKey,
        child: TextField(
          controller: controller,
          focusNode: focusNode,
          keyboardType: TextInputType.visiblePassword,
          textAlign: TextAlign.center,
          style: const TextStyle(
            color: Colors.white,
            fontSize: 18,
            fontWeight: FontWeight.w600,
          ),
          inputFormatters: inputFormatters,
          onChanged: onChanged,
          decoration: InputDecoration(
            counterText: '',
            filled: true,
            fillColor: AppColors.background,
            contentPadding: EdgeInsets.zero,
            enabledBorder: OutlineInputBorder(
              borderRadius: BorderRadius.circular(4),
              borderSide: BorderSide(color: Colors.white),
            ),
            focusedBorder: OutlineInputBorder(
              borderRadius: BorderRadius.circular(4),
              borderSide: BorderSide(color: AppColors.primary),
            ),
          ),
        ),
      ),
    );
  }
}
