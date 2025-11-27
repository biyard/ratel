import 'package:ratel/exports.dart';

class CreatePostController extends BaseController {
  final titleController = TextEditingController();
  final bodyController = TextEditingController();

  String bodyHtml = '';
  void onBodyHtmlChanged(String html) {
    bodyHtml = html;
  }

  Future<void> submit() async {
    final title = titleController.text.trim();
    final html = bodyHtml;

    logger.d("title: $title");
    logger.d("html: $html");
  }
}
