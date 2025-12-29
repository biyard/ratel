import 'package:ratel/exports.dart';

class LoginTopBar extends StatelessWidget {
  const LoginTopBar({
    super.key,
    required this.title,
    required this.onBack,
    this.enableBack = true,
  });

  final String title;
  final VoidCallback onBack;
  final bool enableBack;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 72,
      padding: const EdgeInsets.symmetric(horizontal: 20),
      child: Row(
        children: [
          // if (enableBack)
          //   GestureDetector(
          //     behavior: HitTestBehavior.opaque,
          //     onTap: onBack,
          //     child: Container(
          //       width: 32,
          //       height: 32,
          //       decoration: BoxDecoration(
          //         color: const Color(0xFF171717),
          //         borderRadius: BorderRadius.circular(100),
          //       ),
          //       child: const Center(
          //         child: Icon(
          //           Icons.arrow_back_ios_new,
          //           size: 16,
          //           color: Colors.white,
          //         ),
          //       ),
          //     ),
          //   )
          // else
          //   const SizedBox(width: 32, height: 32),
          12.gap,
          Expanded(
            child: Text(
              title,
              style: AppFonts.textTheme.titleMedium?.copyWith(
                color: Colors.white,
                fontWeight: FontWeight.w600,
                fontSize: 18,
                height: 24 / 18,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
