import 'package:ratel/exports.dart';

class MySpaceController extends BaseController {
  final spaceApi = Get.find<SpaceApi>();
  final pendingSpaces = <MySpaceItem>[].obs;
  final participatingSpaces = <MySpaceItem>[].obs;

  final isLoading = false.obs;
  final isRefreshing = false.obs;
  final isLoadingMore = false.obs;

  String? _bookmark;
  bool get hasMore => _bookmark != null && _bookmark!.isNotEmpty;

  @override
  void onInit() {
    super.onInit();
    loadInitialSpaces();
  }

  Future<void> loadInitialSpaces() async {
    await _fetchSpaces(reset: true);
  }

  Future<void> refreshSpaces() async {
    isLoadingMore.value = false;
    _bookmark = null;
    await _fetchSpaces(reset: true, refreshing: true);
  }

  Future<void> loadMoreSpaces() async {
    if (!hasMore || isLoadingMore.value) return;

    isLoadingMore.value = true;
    try {
      await _fetchSpaces(reset: false);
    } finally {
      isLoadingMore.value = false;
    }
  }

  Future<void> _fetchSpaces({
    required bool reset,
    bool refreshing = false,
  }) async {
    if (reset) {
      _bookmark = null;
    }

    if (refreshing) {
      isRefreshing.value = true;
    } else if (!isLoadingMore.value) {
      // 첫 페이지 로딩일 때만 isLoading 사용
      isLoading.value = true;
    }

    try {
      final res = await spaceApi.getMySpaces(
        bookmark: reset ? null : _bookmark,
      );

      final newItems = res.items;

      final nextBookmark = res.bookmark;
      if (newItems.isEmpty || nextBookmark == null || nextBookmark.isEmpty) {
        _bookmark = null;
      } else {
        _bookmark = nextBookmark;
      }

      final pending = <MySpaceItem>[];
      final participating = <MySpaceItem>[];

      for (final item in newItems) {
        switch (item.invitationStatus) {
          case MySpaceInvitationStatus.pending:
            pending.add(item);
            break;
          case MySpaceInvitationStatus.participating:
            participating.add(item);
            break;
        }
      }

      if (reset) {
        pendingSpaces.assignAll(pending);
        participatingSpaces.assignAll(participating);
      } else {
        pendingSpaces.addAll(pending);
        participatingSpaces.addAll(participating);
      }
    } catch (e, s) {
      logger.e('Failed to fetch my spaces: $e', stackTrace: s);
    } finally {
      isLoading.value = false;
      isRefreshing.value = false;
    }
  }
}
