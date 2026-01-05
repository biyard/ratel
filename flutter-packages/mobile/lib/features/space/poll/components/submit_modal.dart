import 'package:ratel/exports.dart';

class SubmitModal extends StatelessWidget {
  const SubmitModal({super.key, required this.onConfirm, this.onCancel});

  final VoidCallback onConfirm;
  final VoidCallback? onCancel;

  static Future<T?> show<T>({
    required VoidCallback onConfirm,
    VoidCallback? onCancel,
    bool barrierDismissible = true,
  }) {
    return Get.dialog<T>(
      SubmitModal(onConfirm: onConfirm, onCancel: onCancel),
      barrierDismissible: barrierDismissible,
      barrierColor: Colors.black.withAlpha(160),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(minWidth: 300, maxWidth: 350),
        child: Container(
          padding: const EdgeInsets.fromLTRB(20, 30, 20, 20),
          decoration: BoxDecoration(
            color: const Color(0xFF171717),
            borderRadius: BorderRadius.circular(20),
            boxShadow: const [
              BoxShadow(
                color: Color.fromRGBO(255, 206, 71, 0.25),
                blurRadius: 100,
                spreadRadius: 0,
                offset: Offset(0, 0),
              ),
            ],
          ),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: const [
                  Text(
                    'Submit response?',
                    textAlign: TextAlign.center,
                    style: TextStyle(
                      fontSize: 24,
                      height: 32 / 24,
                      fontWeight: FontWeight.w700,
                      color: Colors.white,
                      decoration: TextDecoration.none,
                    ),
                  ),
                  SizedBox(height: 24),
                  Text(
                    "Once submitted, you won’t be able to edit your response.",
                    textAlign: TextAlign.center,
                    style: TextStyle(
                      fontSize: 15,
                      height: 22 / 15,
                      fontWeight: FontWeight.w400,
                      color: Color(0xFFD4D4D4),
                      decoration: TextDecoration.none,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 35),
              Row(
                children: [
                  SizedBox(
                    width: 110,
                    child: _ModalButton(
                      label: 'Cancel',
                      onTap: () {
                        Get.back();
                        onCancel?.call();
                      },
                      variant: _ModalButtonVariant.cancel,
                    ),
                  ),
                  10.gap,
                  Expanded(
                    child: _ModalButton(
                      label: 'Confirm',
                      onTap: () {
                        Get.back();
                        onConfirm();
                      },
                      variant: _ModalButtonVariant.confirm,
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

enum _ModalButtonVariant { cancel, confirm }

class _ModalButton extends StatelessWidget {
  const _ModalButton({
    required this.label,
    required this.onTap,
    required this.variant,
  });

  final String label;
  final VoidCallback onTap;
  final _ModalButtonVariant variant;

  @override
  Widget build(BuildContext context) {
    final isConfirm = variant == _ModalButtonVariant.confirm;

    final bg = isConfirm ? const Color(0xFFFCB300) : Colors.transparent;
    final fg = isConfirm ? const Color(0xFF1D1D1D) : const Color(0xFFD4D4D4);

    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: onTap,
      child: Container(
        padding: isConfirm
            ? const EdgeInsets.symmetric(horizontal: 40, vertical: 14)
            : const EdgeInsets.symmetric(horizontal: 20, vertical: 14),
        decoration: BoxDecoration(
          color: bg,
          borderRadius: BorderRadius.circular(10),
        ),
        child: Center(
          child: Text(
            label,
            style: TextStyle(
              fontWeight: FontWeight.w700,
              fontSize: 16,
              height: 19 / 16,
              color: fg,
              decoration: TextDecoration.none,
            ),
          ),
        ),
      ),
    );
  }
}
