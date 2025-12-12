import 'package:ratel/exports.dart';

class FeedsService extends GetxService {
  final feedsApi = Get.find<FeedsApi>();

  final RxList<FeedSummaryModel> summaries = <FeedSummaryModel>[].obs;
  final RxMap<String, FeedV2Model> details = <String, FeedV2Model>{}.obs;

  final RxList<FeedSummaryModel> drafts = <FeedSummaryModel>[].obs;

  String? bookmark;
  String? draftBookmark;

  static void init() {
    Get.put<FeedsApi>(FeedsApi());
    Get.put<FeedsService>(FeedsService());
  }

  bool get hasMore => bookmark != null;
  bool get hasMoreDrafts => draftBookmark != null;

  Future<void> loadInitial() async {
    bookmark = null;
    await _loadFeeds(reset: true);
  }

  Future<void> loadMore() async {
    if (!hasMore) return;
    await _loadFeeds(reset: false);
  }

  Future<void> _loadFeeds({required bool reset}) async {
    final result = await feedsApi.listPostsV2(bookmark: bookmark);
    bookmark = result.bookmark;

    if (reset) {
      summaries.assignAll(result.items);
    } else {
      summaries.addAll(result.items);
    }

    summaries.refresh();
  }

  Future<FeedV2Model> fetchDetail(
    String postPk, {
    bool forceRefresh = false,
  }) async {
    if (!forceRefresh && details.containsKey(postPk)) {
      return details[postPk]!;
    }

    final result = await feedsApi.getFeedV2(postPk);
    details[postPk] = result;
    _syncSummaryFromDetail(result);
    return result;
  }

  void updateDetail(FeedV2Model model) {
    final pk = model.post.pk;
    details[pk] = model;
    _syncSummaryFromDetail(model);
  }

  Future<bool> deletePost(String postPk) async {
    final ok = await feedsApi.deletePostV2(postPk);
    if (!ok) return false;

    details.remove(postPk);

    final idx = summaries.indexWhere((e) => e.pk == postPk);
    if (idx >= 0) {
      summaries.removeAt(idx);
      summaries.refresh();
    }

    removeDraftLocally(postPk);
    return true;
  }

  Future<void> loadDraftsInitial() async {
    draftBookmark = null;
    await _loadDrafts(reset: true);
  }

  Future<void> loadDraftsMore() async {
    if (!hasMoreDrafts) return;
    await _loadDrafts(reset: false);
  }

  Future<void> _loadDrafts({required bool reset}) async {
    final result = await feedsApi.listDraftsV2(bookmark: draftBookmark);
    draftBookmark = result.bookmark;

    if (reset) {
      drafts.assignAll(result.items);
    } else {
      drafts.addAll(result.items);
    }

    drafts.refresh();
  }

  Future<bool> deleteDraft(String pk) async {
    final ok = await feedsApi.deletePostV2(pk);
    if (!ok) return false;

    removeDraftLocally(pk);
    return true;
  }

  void removeDraftLocally(String pk) {
    drafts.removeWhere((e) => e.pk == pk);
    drafts.refresh();
  }

  void _syncSummaryFromDetail(FeedV2Model detail) {
    final pk = detail.post.pk;
    final idx = summaries.indexWhere((e) => e.pk == pk);
    if (idx < 0) return;

    final s = summaries[idx];

    s.title = detail.post.title;
    s.likes = detail.post.likes;
    s.comments = detail.post.comments;

    summaries.refresh();
  }
}
