import 'package:ratel/exports.dart';

class ModalTitle extends StatelessWidget {
  final String text;

  const ModalTitle({super.key, required this.text});

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        Text("Join the Movement", style: AppFonts.modalHeaderTextStyle),
        GestureDetector(
          onTap: () => Navigator.pop(context),
          child: const Icon(Icons.close, size: 24, color: AppColors.neutral80),
        ),
      ],
    );
  }
}
