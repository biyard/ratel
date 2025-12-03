import 'package:ratel/exports.dart';

class BoardViewerController extends BaseController {
  final SpaceBoardsApi _boardsApi = Get.find<SpaceBoardsApi>();

  late final String spacePk;
  late final String postPk;

  final post = Rxn<SpacePostModel>();
  final isLoading = false.obs;

  @override
  void onInit() {
    super.onInit();

    final sk = Get.parameters['spacePk'];
    final pk = Get.parameters['postPk'];

    if (sk == null || pk == null) {
      logger.e(
        'BoardViewerController: spacePk/postPk is null. '
        'route: ${Get.currentRoute}',
      );
      return;
    }

    spacePk = Uri.decodeComponent(sk);
    postPk = Uri.decodeComponent(pk);

    logger.d(
      'BoardViewerController initialized '
      'spacePk=$spacePk, postPk=$postPk',
    );

    _loadPost();
  }

  Future<void> _loadPost() async {
    if (isLoading.value) return;

    try {
      isLoading.value = true;
      final res = await _boardsApi.getPost(spacePk, postPk);
      post.value = res;

      logger.d(
        'BoardViewerController: loaded post '
        'pk=${res.pk}, title=${res.title}',
      );
    } catch (e) {
      logger.e(
        'BoardViewerController: failed to load post '
        'spacePk=$spacePk postPk=$postPk: $e',
      );
      Biyard.error(
        'Failed to load post',
        'Failed to load the board post. Please try again later.',
      );
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> refresh() async {
    await _loadPost();
  }
}
