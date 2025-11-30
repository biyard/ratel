import 'dart:convert';

import 'package:flutter_quill/flutter_quill.dart' as quill;

class RichEditorController {
  final quill.QuillController quillController;

  RichEditorController() : quillController = quill.QuillController.basic();

  String toHtml() {
    final delta = quillController.document.toDelta();
    return jsonEncode(delta.toJson());
  }
}
