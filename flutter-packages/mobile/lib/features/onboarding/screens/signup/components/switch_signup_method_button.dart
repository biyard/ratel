import 'package:ratel/exports.dart';

class SwitchSignupMethodButton extends StatelessWidget {
  const SwitchSignupMethodButton({
    super.key,
    required this.icon,
    required this.label,
    required this.onTap,
  });

  final String icon;
  final String label;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      behavior: HitTestBehavior.opaque,
      onTap: onTap,
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 30, vertical: 17),
        decoration: BoxDecoration(
          color: const Color(0xFF171717),
          borderRadius: BorderRadius.circular(100),
          border: Border.all(color: const Color(0xFF464646), width: 0.5),
        ),
        child: Row(
          children: [
            SvgPicture.asset(icon, width: 18, height: 18),
            12.gap,
            Expanded(
              child: Text(
                label,
                style: TextStyle(
                  color: Colors.white,
                  fontWeight: FontWeight.w600,
                  fontSize: 15,
                  height: 23 / 15,
                ),
              ),
            ),
            const Icon(Icons.chevron_right, color: Color(0xFFD4D4D4)),
          ],
        ),
      ),
    );
  }
}
