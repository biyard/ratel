import 'package:ratel/exports.dart';

class BoardsViewerController extends BaseController {
  final SpaceBoardsApi _boardsApi = Get.find<SpaceBoardsApi>();

  late final String spacePk;

  final categories = <String>[].obs;
  final selectedCategory = RxnString();

  final posts = <SpacePostModel>[].obs;
  final isLoading = false.obs;
  final isLoadingMore = false.obs;

  String? _bookmark;

  bool get hasMore => _bookmark != null && _bookmark!.isNotEmpty;

  @override
  void onInit() {
    super.onInit();

    final rawPk = Get.parameters['spacePk'];
    if (rawPk == null || rawPk.isEmpty) {
      logger.w(
        'BoardsViewerController: spacePk is null or empty in parameters',
      );
      return;
    }

    spacePk = Uri.decodeComponent(rawPk);
    logger.d('BoardsViewerController initialized with spacePk=$spacePk');

    _loadInitial();
  }

  Future<void> _loadInitial() async {
    await Future.wait([loadCategories(), loadPosts(reset: true)]);
  }

  Future<void> loadCategories() async {
    try {
      final list = await _boardsApi.listCategories(spacePk);
      categories.assignAll(list);
    } catch (e) {
      logger.e('Failed to load categories for spacePk=$spacePk: $e');
    }
  }

  Future<void> loadPosts({bool reset = false}) async {
    if (reset) {
      _bookmark = null;
      posts.clear();
    }

    if (isLoading.value) return;

    try {
      isLoading.value = true;

      final res = await _boardsApi.listPosts(
        spacePk,
        bookmark: _bookmark,
        category: selectedCategory.value,
      );

      _bookmark = res.bookmark;

      if (reset) {
        posts.assignAll(res.posts);
      } else {
        posts.addAll(res.posts);
      }

      logger.d(
        'Loaded posts (viewer) for spacePk=$spacePk, count=${res.posts.length}, bookmark=$_bookmark',
      );
    } catch (e) {
      logger.e('Failed to load posts (viewer) for spacePk=$spacePk: $e');
      Biyard.error(
        'Failed to load posts',
        'Failed to load posts for space. Please try again later.',
      );
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> loadMore() async {
    if (!hasMore || isLoadingMore.value) return;

    try {
      isLoadingMore.value = true;

      final res = await _boardsApi.listPosts(
        spacePk,
        bookmark: _bookmark,
        category: selectedCategory.value,
      );

      _bookmark = res.bookmark;
      posts.addAll(res.posts);

      logger.d(
        'Loaded more posts (viewer) for spacePk=$spacePk, added=${res.posts.length}, bookmark=$_bookmark',
      );
    } catch (e) {
      logger.e('Failed to load more posts (viewer) for spacePk=$spacePk: $e');
    } finally {
      isLoadingMore.value = false;
    }
  }

  void onCategorySelected(String? category) {
    final value = category?.isNotEmpty == true ? category : null;
    if (selectedCategory.value == value) return;

    selectedCategory.value = value;
    loadPosts(reset: true);
  }

  Future<void> refreshAll() async {
    await _loadInitial();
  }
}
