import 'package:ratel/exports.dart';

class SpaceTopBar extends StatelessWidget {
  const SpaceTopBar({
    super.key,
    required this.onBack,
    this.showMore = false,
    this.onMore,
  });

  final VoidCallback onBack;
  final bool showMore;
  final VoidCallback? onMore;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 72,
      padding: const EdgeInsets.symmetric(horizontal: 20),
      color: const Color(0xFF1D1D1D),
      child: Row(
        children: [
          // Expanded(
          //   child: InkWell(
          //     onTap: onBack,
          //     child: Align(
          //       alignment: Alignment.centerLeft,
          //       child: SvgPicture.asset(Assets.back, width: 24, height: 24),
          //     ),
          //   ),
          // ),
          if (showMore && onMore != null) const Spacer(),
          if (showMore && onMore != null)
            InkWell(
              onTap: onMore,
              child: Container(
                width: 32,
                height: 32,
                alignment: Alignment.center,
                child: SvgPicture.asset(Assets.extra, width: 24, height: 24),
              ),
            ),
        ],
      ),
    );
  }
}
