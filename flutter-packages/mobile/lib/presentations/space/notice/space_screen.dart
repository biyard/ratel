import 'package:ratel/exports.dart';

class NoticeSpaceScreen extends GetWidget<NoticeSpaceController> {
  const NoticeSpaceScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<NoticeSpaceController>(
      scrollable: false,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.all(20),
            child: Row(
              children: [
                InkWell(
                  onTap: () => controller.goBack(),
                  child: RoundContainer(
                    color: Colors.white.withAlpha(50),
                    radius: 100,
                    child: Padding(
                      padding: const EdgeInsets.all(5.0),
                      child: SvgPicture.asset(
                        Assets.back,
                        width: 20,
                        height: 20,
                      ),
                    ),
                  ),
                ),
              ],
            ),
          ),

          Padding(
            padding: const EdgeInsets.fromLTRB(10, 10, 10, 20),
            child: Text(
              "notice space",
              style: TextStyle(
                color: Colors.white,
                fontWeight: FontWeight.w600,
                fontSize: 12,
              ),
            ),
          ),
        ],
      ),
    );
  }
}
