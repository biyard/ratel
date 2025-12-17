import 'package:ratel/exports.dart';

class CommentMoreBottomSheet extends StatelessWidget {
  const CommentMoreBottomSheet({super.key, required this.onReport});

  final VoidCallback onReport;

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      top: false,
      child: Container(
        width: double.infinity,
        padding: const EdgeInsets.fromLTRB(20, 12, 20, 24),
        decoration: const BoxDecoration(
          color: Color(0xFF191919),
          borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Center(
              child: Container(
                width: 50,
                height: 4,
                decoration: BoxDecoration(
                  color: const Color(0xFF3A3A3A),
                  borderRadius: BorderRadius.circular(999),
                ),
              ),
            ),
            20.vgap,
            InkWell(
              borderRadius: BorderRadius.circular(12),
              onTap: onReport,
              child: SizedBox(
                height: 48,
                child: Row(
                  children: [
                    SvgPicture.asset(Assets.report, width: 20, height: 20),
                    5.gap,
                    const Text(
                      'Report comment',
                      style: TextStyle(
                        color: Color(0xFFEF4444),
                        fontSize: 16,
                        fontWeight: FontWeight.w600,
                        height: 24 / 16,
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
