import 'package:ratel/exports.dart';

class BookmarkLocalization {
  static final _bookmarks = s('bookmarksTitle', 'Bookmarks', 'Bookmarks');
  static String get bookmarks => _bookmarks.tr;

  static final _bookmarkError = s(
    'bookmarkError',
    'No bookmarks yet',
    'No bookmarks yet',
  );
  static String get bookmarkError => _bookmarkError.tr;
}
