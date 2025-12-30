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
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          Positioned.fill(
            child: SvgPicture.asset(Assets.warning, width: 16, height: 16),
          ),
          4.gap,
          Text(
            message,
            style: const TextStyle(
              color: Color(0xffef4444),
              fontSize: 13,
              fontWeight: FontWeight.w700,
              height: 16 / 13,
            ),
          ),
        ],
      ),
    );
  }
}
