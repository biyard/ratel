import 'package:ratel/exports.dart';

class FileCreatorController extends BaseController {
  final SpaceFilesApi _filesApi = Get.find<SpaceFilesApi>();

  late final String spacePk;
  final files = <FileModel>[].obs;
  final isLoading = false.obs;

  @override
  void onInit() {
    super.onInit();
    logger.d('FileCreatorController initialized');

    final rawPk = Get.parameters['spacePk'];
    if (rawPk == null || rawPk.isEmpty) {
      logger.w('FileCreatorController: spacePk is null or empty in parameters');
      return;
    }

    spacePk = Uri.decodeComponent(rawPk);
    _loadFiles();
  }

  Future<void> _loadFiles() async {
    try {
      isLoading.value = true;
      final result = await _filesApi.listSpaceFiles(spacePk);
      files.assignAll(result);
    } catch (e) {
      logger.e('Failed to load space files for $spacePk $e');
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> refreshFiles() => _loadFiles();
}
