// screen
import 'package:ratel/exports.dart';

class PostScreen extends GetWidget<PostController> {
  const PostScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<PostController>(
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 10),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Padding(
              padding: const EdgeInsets.fromLTRB(24, 20, 24, 20),
              child: Row(
                children: [
                  InkWell(
                    onTap: controller.goBack,
                    child: SvgPicture.asset(Assets.back, width: 24, height: 24),
                  ),
                  20.gap,
                  Text(
                    'My Posts',
                    style: TextStyle(
                      color: Colors.white,
                      fontSize: 24,
                      fontWeight: FontWeight.w700,
                      height: 1.2,
                    ),
                  ),
                ],
              ),
            ),
            Container(
              width: double.infinity,
              height: 1,
              color: Color(0xff464646),
            ),

            Padding(
              padding: const EdgeInsets.all(10.0),
              child: Obx(
                () => ListView.separated(
                  primary: false,
                  shrinkWrap: true,
                  itemCount: controller.feeds.length,
                  separatorBuilder: (_, __) => const SizedBox(height: 10),
                  itemBuilder: (_, i) => FeedCard(data: controller.feeds[i]),
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
