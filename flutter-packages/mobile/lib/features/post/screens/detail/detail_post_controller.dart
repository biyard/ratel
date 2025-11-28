import 'package:ratel/exports.dart';

class DetailPostController extends BaseController {
  final feedsApi = Get.find<FeedsApi>();

  late final String postPk;

  final feed = Rxn<FeedV2Model>();
  final isLoading = false.obs;

  @override
  void onInit() {
    super.onInit();

    final raw = Get.parameters['pk'];
    if (raw == null) {
      logger.e('post pk is null. route: ${Get.currentRoute}');
      return;
    }

    postPk = Uri.decodeComponent(raw);
    logger.d('DetailPostController postPk = $postPk');

    loadFeed();
  }

  Future<void> loadFeed() async {
    try {
      isLoading.value = true;

      final result = await feedsApi.getFeedV2(postPk);
      feed.value = result;
      logger.d("feed results: ${result}");
    } catch (e, s) {
      logger.e('Failed to load feed detail: $e', stackTrace: s);
    } finally {
      isLoading.value = false;
    }
  }
}
