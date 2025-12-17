import 'package:ratel/exports.dart';

class FeedsService extends GetxService {
  final feedsApi = Get.find<FeedsApi>();
  final RxList<FeedSummaryModel> summaries = <FeedSummaryModel>[].obs;
  final RxList<FeedSummaryModel> homeFeeds = <FeedSummaryModel>[].obs;
  final RxMap<String, FeedModel> details = <String, FeedModel>{}.obs;
  final RxList<FeedSummaryModel> drafts = <FeedSummaryModel>[].obs;

  String? bookmark;
  String? homeBookmark;
  String? draftBookmark;

  static void init() {
    Get.put<FeedsApi>(FeedsApi());
    Get.put<FeedsService>(FeedsService());
  }

  bool get hasMore => bookmark != null;
  bool get hasMoreHome => homeBookmark != null;
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

  // ===== Home Feeds (listFeedsV2) =====

  Future<void> loadHomeInitial() async {
    homeBookmark = null;
    await _loadHomeFeeds(reset: true);
  }

  Future<void> loadHomeMore() async {
    if (!hasMoreHome) return;
    await _loadHomeFeeds(reset: false);
  }

  Future<void> _loadHomeFeeds({required bool reset}) async {
    final result = await feedsApi.listFeedsV2(bookmark: homeBookmark);
    homeBookmark = result.bookmark;

    if (reset) {
      homeFeeds.assignAll(result.items);
    } else {
      homeFeeds.addAll(result.items);
    }

    homeFeeds.refresh();
  }

  // ===== Detail =====

  Future<FeedModel> fetchDetail(
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

  void updateDetail(FeedModel model) {
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
    }

    final homeIdx = homeFeeds.indexWhere((e) => e.pk == postPk);
    if (homeIdx >= 0) {
      homeFeeds.removeAt(homeIdx);
    }

    summaries.refresh();
    homeFeeds.refresh();

    removeDraftLocally(postPk);
    return true;
  }

  // ===== Drafts =====

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

  void _syncSummaryFromDetail(FeedModel detail) {
    final pk = detail.post.pk;

    void updateList(RxList<FeedSummaryModel> list) {
      final idx = list.indexWhere((e) => e.pk == pk);
      if (idx < 0) return;

      final s = list[idx];
      s.title = detail.post.title;
      s.likes = detail.post.likes;
      s.comments = detail.post.comments;
      s.liked = detail.isLiked == true;
    }

    updateList(summaries);
    updateList(homeFeeds);

    summaries.refresh();
    homeFeeds.refresh();
  }

  void patchDetailFromSummary(FeedSummaryModel summary) {
    final pk = summary.pk;
    final detail = details[pk];
    if (detail == null) return;

    detail.post.likes = summary.likes;
    detail.post.comments = summary.comments;
    detail.isLiked = summary.liked == true;

    details[pk] = detail;
  }
}
