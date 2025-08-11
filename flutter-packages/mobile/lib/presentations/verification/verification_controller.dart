import 'package:ratel/exports.dart';

class VerificationController extends BaseController {
  final signupService = Get.find<SignupService>();

  final fields = List.generate(6, (_) => TextEditingController());
  final nodes = List.generate(6, (_) => FocusNode());

  final isBusy = false.obs;
  final code = List.generate(6, (_) => '').obs;

  String get email => signupService.email.value;
  bool get isComplete => code.length == 6 && code.every((c) => c.isNotEmpty);

  final List<TextInputFormatter> codeInputFormatters = [
    FilteringTextInputFormatter.allow(RegExp(r'[A-Za-z0-9]')),
    LengthLimitingTextInputFormatter(1),
  ];

  void onChanged(int index, String value) {
    final v = value.replaceAll(RegExp(r'[^A-Za-z0-9]'), '');
    fields[index].value = fields[index].value.copyWith(
      text: v,
      selection: TextSelection.collapsed(offset: v.length),
    );
    code[index] = v;

    if (v.isNotEmpty) {
      if (index < 5) {
        nodes[index + 1].requestFocus();
      } else {
        nodes[index].unfocus();
      }
    }
  }

  KeyEventResult onKey(int index, KeyEvent e) {
    if (e is KeyDownEvent && e.logicalKey == LogicalKeyboardKey.backspace) {
      if (fields[index].text.isEmpty && index > 0) {
        nodes[index - 1].requestFocus();
        fields[index - 1].clear();
        code[index - 1] = '';
      }
    }
    return KeyEventResult.ignored;
  }

  void goBack() {
    Get.rootDelegate.offNamed(AppRoutes.signupScreen);
  }

  Future<void> verify() async {
    if (!isComplete || isBusy.value) return;
    isBusy.value = true;
    try {
      final pin = code.join();
      logger.d("pin value: ${pin}");
      await Future.delayed(const Duration(milliseconds: 800));
      Get.rootDelegate.offAndToNamed(AppRoutes.setupProfileScreen);
    } finally {
      isBusy.value = false;
    }
  }

  Future<void> resend() async {
    if (isBusy.value) return;
    Get.snackbar('Verification', 'Code resent to $email');
  }

  @override
  void onClose() {
    for (final c in fields) {
      c.dispose();
    }
    for (final n in nodes) {
      n.dispose();
    }
    super.onClose();
  }
}
