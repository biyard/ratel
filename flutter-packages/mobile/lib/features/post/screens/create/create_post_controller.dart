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

  final RxBool canSubmit = false.obs;
  final RxBool isEditorReady = false.obs;

  static const int minLength = 10;
  static const int maxLength = 1000;

  @override
  void onInit() {
    super.onInit();

    logger.d('CreatePostController initialized with postPk: $postPk');

    titleController.addListener(_onTextChanged);
    bodyController.addListener(_onTextChanged);

    _autoSaveTimer = Timer.periodic(_autoSaveDelay, (_) => _autoSave());

    if (postPk != null && postPk!.isNotEmpty) {
      _loadPost();
    } else {
      isEditorReady.value = true;
    }
  }

  Future<void> _loadPost() async {
    try {
      logger.d('[create] loadPost start pk=$postPk');
      final post = await feedApi.getFeedV2(postPk!);

      final title = post.post.title;
      final html = post.post.htmlContents.toString();

      titleController.text = title;
      bodyHtml = html;
      bodyController.text = html;

      _isModified = false;
      _validateCanSubmit();

      logger.d('[create] loadPost done title="$title" len=${html.length}');
    } catch (e, s) {
      logger.e('[create] loadPost failed: $e\n$s');
    } finally {
      isEditorReady.value = true;
    }
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
    _validateCanSubmit();
  }

  void onBodyHtmlChanged(String html) {
    bodyHtml = html;
    _isModified = true;
    _validateCanSubmit();
  }

  void _validateCanSubmit() {
    final titleLen = titleController.text.trim().length;
    final descLen = bodyController.text.trim().length;

    canSubmit.value =
        titleLen >= minLength &&
        titleLen <= maxLength &&
        descLen >= minLength &&
        descLen <= maxLength;
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

    logger.d('save draft res: $res');
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
    if (!canSubmit.value) {
      Biyard.error(
        'Cannot publish post.',
        'Title + content must be between $minLength and $maxLength characters.',
      );
      return;
    }

    final title = titleController.text.trim();
    final html = bodyHtml.trim();

    await _autoSave();

    logger.d('[submit] title: $title');
    logger.d('[submit] html: $html');

    final res = await feedApi.uploadPost(
      postPk: postPk!,
      title: title,
      content: html,
    );

    logger.d('save draft res: $res');

    if (!res) {
      Biyard.error('Failed to publish post.', 'Please try again later.');
      return;
    }

    Get.rootDelegate.offNamed(postWithPk(postPk!));
  }
}
