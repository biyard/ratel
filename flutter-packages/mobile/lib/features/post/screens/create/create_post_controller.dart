import 'dart:async';
import 'package:ratel/exports.dart';

class CreatePostController extends BaseController {
  CreatePostController({required this.postPk});

  final String? postPk;

  final feedApi = Get.find<FeedsApi>();
  final titleController = TextEditingController();
  final bodyController = TextEditingController();

  String bodyHtml = '';

  static const _autoSaveDelay = Duration(seconds: 5);
  Timer? _autoSaveTimer;
  bool _isModified = false;
  DateTime? lastSavedAt;

  @override
  void onInit() {
    super.onInit();

    logger.d('CreatePostController initialized with postPk: $postPk');

    titleController.addListener(_onTextChanged);
    bodyController.addListener(_onTextChanged);

    _autoSaveTimer = Timer.periodic(_autoSaveDelay, (_) => _autoSave());
  }

  @override
  void onClose() {
    titleController.dispose();
    bodyController.dispose();
    _autoSaveTimer?.cancel();
    super.onClose();
  }

  void _onTextChanged() {
    _isModified = true;
  }

  void onBodyHtmlChanged(String html) {
    bodyHtml = html;
    _isModified = true;
  }

  Future<void> _saveDraft({required String title, required String html}) async {
    logger.d('[autosave] saveDraft called');
    logger.d('[autosave] title: $title');
    logger.d('[autosave] html: $html');
    logger.d('[autosave] postPk: $postPk');

    if (postPk == null || postPk!.isEmpty) {
      logger.w('[autosave] skip: postPk is null or empty');
      return;
    }

    final res = await feedApi.updatePost(
      postPk: postPk!,
      title: title,
      content: html,
    );

    logger.d("save draft res: $res");
  }

  Future<void> _autoSave() async {
    if (!_isModified) return;

    final title = titleController.text.trim();
    final html = bodyHtml.trim();

    if (title.isEmpty && html.isEmpty) return;

    try {
      logger.d('[autosave] start');
      await _saveDraft(title: title, html: html);
      _isModified = false;
      lastSavedAt = DateTime.now();
      logger.d('[autosave] success at $lastSavedAt');
    } catch (e) {
      logger.e('[autosave] failed $e');
    }
  }

  Future<void> submit() async {
    final title = titleController.text.trim();
    final html = bodyHtml.trim();

    await _autoSave();

    logger.d('[submit] title: $title');
    logger.d('[submit] html: $html');
  }
}
