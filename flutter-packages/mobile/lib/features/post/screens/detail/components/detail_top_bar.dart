import 'package:ratel/exports.dart';

class DetailTopBar extends StatelessWidget {
  const DetailTopBar({
    super.key,
    required this.isCreator,
    required this.onBack,
    required this.onExtra,
  });

  final bool isCreator;
  final VoidCallback onBack;
  final VoidCallback onExtra;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        15.vgap,
        Row(
          children: [
            16.gap,
            GestureDetector(
              onTap: onBack,
              child: SvgPicture.asset(Assets.back, width: 24, height: 24),
            ),
            const Spacer(),
            if (isCreator) ...[
              GestureDetector(
                onTap: onExtra,
                child: SvgPicture.asset(Assets.extra, width: 24, height: 24),
              ),
              16.gap,
            ],
          ],
        ),
        15.vgap,
      ],
    );
  }
}
