import 'package:ratel/exports.dart';

class CodeVerifyDialog extends StatefulWidget {
  final VerifiedController controller;

  const CodeVerifyDialog({super.key, required this.controller});

  static Future<void> show({
    required BuildContext context,
    required VerifiedController controller,
  }) {
    return showDialog(
      context: context,
      barrierDismissible: false,
      builder: (_) => CodeVerifyDialog(controller: controller),
    );
  }

  @override
  State<CodeVerifyDialog> createState() => CodeVerifyDialogState();
}

class CodeVerifyDialogState extends State<CodeVerifyDialog> {
  final _codeController = TextEditingController();
  bool _submitting = false;
  bool _isValid = false;

  @override
  void dispose() {
    _codeController.dispose();
    super.dispose();
  }

  Future<void> _onSubmit() async {
    final raw = _codeController.text.trim();
    if (raw.isEmpty || _submitting) return;

    FocusScope.of(context).unfocus();

    setState(() {
      _submitting = true;
    });

    try {
      final before = widget.controller.attributes.value;
      final after = await widget.controller.signAttributesWithCode(raw);

      final changed =
          before.age != after.age ||
          before.gender != after.gender ||
          before.university != after.university;

      if (!changed || (after.gender == null && after.university == null)) {
        Biyard.error('Verification failed', 'Please check your code.');
        return;
      }

      Biyard.info('Your DID attributes have been updated.');

      if (!mounted) return;
      Navigator.of(context).pop();
    } catch (e, s) {
      logger.e('Failed to verify DID code: $e', stackTrace: s);
      Biyard.error('Verification failed', 'Please check your code.');
    } finally {
      if (mounted) {
        setState(() {
          _submitting = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);

    return Dialog(
      backgroundColor: Colors.white,
      insetPadding: const EdgeInsets.symmetric(horizontal: 32),
      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 360),
        child: Padding(
          padding: const EdgeInsets.fromLTRB(20, 20, 20, 16),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Text(
                'Input Authorization code',
                style: theme.textTheme.titleMedium?.copyWith(
                  color: const Color(0xFF4B5563),
                  fontWeight: FontWeight.w700,
                  fontSize: 16,
                ),
              ),
              15.vgap,
              TextField(
                controller: _codeController,
                autofocus: true,
                onChanged: (value) {
                  setState(() {
                    _isValid = value.trim().isNotEmpty;
                  });
                },
                style: const TextStyle(color: Colors.black, fontSize: 14),
                decoration: InputDecoration(
                  hintText: 'Input your code.',
                  hintStyle: const TextStyle(
                    color: AppColors.neutral500,
                    fontSize: 14,
                  ),
                  contentPadding: const EdgeInsets.symmetric(
                    horizontal: 12,
                    vertical: 10,
                  ),
                  enabledBorder: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(6),
                    borderSide: const BorderSide(
                      color: AppColors.primary,
                      width: 1.5,
                    ),
                  ),
                  focusedBorder: OutlineInputBorder(
                    borderRadius: BorderRadius.circular(6),
                    borderSide: const BorderSide(
                      color: AppColors.primary,
                      width: 1.5,
                    ),
                  ),
                ),
              ),
              15.vgap,
              Row(
                mainAxisAlignment: MainAxisAlignment.end,
                children: [
                  TextButton(
                    onPressed: _submitting ? null : () => Get.back(),
                    child: Text(
                      'Cancel',
                      style: theme.textTheme.bodyMedium?.copyWith(
                        color: Colors.black,
                      ),
                    ),
                  ),
                  10.gap,
                  SizedBox(
                    height: 36,
                    child: ElevatedButton(
                      onPressed: (!_isValid || _submitting) ? null : _onSubmit,
                      style: ElevatedButton.styleFrom(
                        backgroundColor: AppColors.primary,
                        disabledBackgroundColor: const Color(0xFFE5E7EB),
                        foregroundColor: const Color(0xFF111827),
                        disabledForegroundColor: const Color(0xFF9CA3AF),
                        elevation: 0,
                        padding: const EdgeInsets.symmetric(
                          horizontal: 22,
                          vertical: 8,
                        ),
                        shape: RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(18),
                        ),
                      ),
                      child: _submitting
                          ? const SizedBox(
                              width: 16,
                              height: 16,
                              child: CircularProgressIndicator(strokeWidth: 2),
                            )
                          : const Text(
                              'Submit',
                              style: TextStyle(
                                fontWeight: FontWeight.w600,
                                fontSize: 14,
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
  }
}
