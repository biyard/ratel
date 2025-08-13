import 'package:ratel/exports.dart';

class PostScreen extends GetWidget<PostController> {
  const PostScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Layout<PostController>(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [Text("Post", style: TextStyle(color: Colors.white))],
      ),
    );
  }
}
