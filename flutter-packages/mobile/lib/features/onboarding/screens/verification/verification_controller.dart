import 'package:ratel/exports.dart';

class VerificationController extends BaseController {
  final signupService = Get.find<SignupService>();

  final fields = List.generate(6, (_) => TextEditingController());
  final nodes = List.generate(6, (_) => FocusNode());

  final isBusy = false.obs;
  final code = List.generate(6, (_) => '').obs;

  String get phone => signupService.phone.value;

  final _isComplete = false.obs;
  bool get isComplete => _isComplete.value;

  final List<TextInputFormatter> codeInputFormatters = [
    FilteringTextInputFormatter.digitsOnly,
  ];

  bool _updating = false;

  void onChanged(int index, String value) {
    if (_updating) return;

    final digits = value.replaceAll(RegExp(r'\D'), '');
    _updating = true;
    try {
      if (digits.isEmpty) {
        fields[index].value = const TextEditingValue(text: '');
        code[index] = '';
      } else if (digits.length == 1) {
        fields[index].value = TextEditingValue(
          text: digits,
          selection: const TextSelection.collapsed(offset: 1),
        );
        code[index] = digits;

        if (index < fields.length - 1) {
          nodes[index + 1].requestFocus();
        } else {
          nodes[index].unfocus();
        }
      } else {
        var cursor = index;
        for (
          var i = 0;
          i < digits.length && cursor < fields.length;
          i++, cursor++
        ) {
          final ch = digits[i];
          fields[cursor].value = TextEditingValue(
            text: ch,
            selection: const TextSelection.collapsed(offset: 1),
          );
          code[cursor] = ch;
        }
        for (; cursor < fields.length; cursor++) {
          fields[cursor].value = const TextEditingValue(text: '');
          code[cursor] = '';
        }

        if (index + digits.length >= fields.length) {
          nodes.last.unfocus();
        } else {
          nodes[index + digits.length].requestFocus();
        }
      }
    } finally {
      _updating = false;
    }

    _isComplete.value = code.every((c) => c.isNotEmpty);
  }

  KeyEventResult onKey(int index, KeyEvent e) {
    if (e is KeyDownEvent && e.logicalKey == LogicalKeyboardKey.backspace) {
      if (fields[index].text.isEmpty && index > 0) {
        nodes[index - 1].requestFocus();
        fields[index - 1].value = const TextEditingValue(text: '');
        code[index - 1] = '';
      }
    }
    _isComplete.value = code.every((c) => c.isNotEmpty);
    return KeyEventResult.ignored;
  }

  void goBack() {
    Get.back();
  }

  Future<void> verify() async {
    final auth = AuthApi();
    final authService = Get.find<AuthService>();

    if (!isComplete || isBusy.value) return;
    isBusy.value = true;
    try {
      final pin = code.join();
      logger.d("pin value: $pin");
      final res = await auth.verifyCode(phone, pin);
      final res2 = await auth.signup(phone, pin);
      await authService.loadAccounts(refresh: false);

      if (res != null && res2 != null) {
        logger.d("verification response: $res $res2");
        authService.neededSignup = false;
        Get.rootDelegate.offNamed(AppRoutes.mainScreen);
        Biyard.info("Verification Successed");
      } else {
        Biyard.error(
          "Failed to verify code",
          "Please check the response code again.",
        );
      }
    } finally {
      isBusy.value = false;
    }
  }

  Future<void> resend() async {
    final auth = AuthApi();
    if (isBusy.value) return;
    isBusy.value = true;

    try {
      final res = await auth.sendVerificationCode(phone);
      if (res != null) {
        Biyard.info("Success to resend verification code");
      } else {
        Biyard.error(
          "Failed to send authorization code",
          "Send Authorization code failed. Please try again later.",
        );
      }
    } finally {
      isBusy.value = false;
    }
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
