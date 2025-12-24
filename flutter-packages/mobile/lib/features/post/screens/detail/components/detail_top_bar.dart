import 'package:ratel/exports.dart';

class DetailTopBar extends StatelessWidget {
  const DetailTopBar({
    super.key,
    required this.isCreator,
    required this.isReport,
    required this.onBack,
    required this.onExtra,
  });

  final bool isCreator;
  final bool isReport;
  final VoidCallback onBack;
  final VoidCallback onExtra;

  @override
  Widget build(BuildContext context) {
    final showExtra = isCreator || !isReport;

    return Column(
      children: [
        15.vgap,
        Row(
          children: [
            16.gap,
            // Expanded(
            //   child: InkWell(
            //     onTap: onBack,
            //     child: Align(
            //       alignment: Alignment.centerLeft,
            //       child: SvgPicture.asset(Assets.back, width: 24, height: 24),
            //     ),
            //   ),
            // ),
            if (showExtra) const Spacer(),
            if (showExtra)
              InkWell(
                onTap: onExtra,
                child: SvgPicture.asset(Assets.extra, width: 24, height: 24),
              ),
            if (showExtra) 16.gap,
          ],
        ),
        15.vgap,
      ],
    );
  }
}
