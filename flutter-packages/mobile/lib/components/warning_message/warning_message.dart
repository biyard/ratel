import 'package:ratel/exports.dart';

class WarningMessage extends StatelessWidget {
  final String message;

  const WarningMessage({super.key, required this.message});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.fromLTRB(10, 10, 10, 10),
      decoration: BoxDecoration(
        color: const Color(0x40EF4444),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Row(
        children: [
          Positioned.fill(
            child: SvgPicture.asset(Assets.warning, width: 16, height: 16),
          ),
          4.gap,
          Expanded(
            child: Text(
              message,
              style: const TextStyle(
                color: Colors.white,
                fontSize: 13,
                fontWeight: FontWeight.w700,
                height: 16 / 13,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
