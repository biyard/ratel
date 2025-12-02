import 'package:ratel/exports.dart';

class DetailTopBar extends StatelessWidget {
  const DetailTopBar({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 72,
      padding: const EdgeInsets.symmetric(horizontal: 20),
      color: const Color(0xFF1D1D1D),
      child: Row(
        children: [
          GestureDetector(
            onTap: () => Get.back(),
            child: Container(
              width: 32,
              height: 32,
              alignment: Alignment.center,
              child: SvgPicture.asset(Assets.back, width: 24, height: 24),
            ),
          ),
          // const Spacer(),
          // Row(
          //   children: const [
          //     SizedBox(
          //       width: 24,
          //       height: 24,
          //       child: Icon(
          //         Icons.chat_bubble_outline_rounded,
          //         size: 20,
          //         color: Color(0xFF737373),
          //       ),
          //     ),
          //     SizedBox(width: 20),
          //     SizedBox(
          //       width: 24,
          //       height: 24,
          //       child: Icon(
          //         Icons.more_horiz_rounded,
          //         size: 20,
          //         color: Color(0xFF737373),
          //       ),
          //     ),
          //   ],
          // ),
        ],
      ),
    );
  }
}
