import 'package:ratel/exports.dart';

class SpaceService extends GetxService {
  static void init() {
    Get.put<SpaceApi>(SpaceApi());
    Get.put<SpaceService>(SpaceService());
  }

  final SpaceApi _spaceApi = Get.find<SpaceApi>();

  final _spaces = <String, Rxn<SpaceModel>>{}.obs;
  final _isLoading = <String, RxBool>{}.obs;

  Rxn<SpaceModel> spaceOf(String pk) {
    return _spaces.putIfAbsent(pk, () => Rxn<SpaceModel>());
  }

  RxBool isLoadingOf(String pk) {
    return _isLoading.putIfAbsent(pk, () => false.obs);
  }

  Future<void> loadSpace(String pk) async {
    final loading = isLoadingOf(pk);
    final rx = spaceOf(pk);

    if (loading.value) return;
    try {
      loading.value = true;
      final result = await _spaceApi.getSpace(pk);
      rx.value = result;
    } catch (e, s) {
      Get.log('Failed to load space $pk: $e\n$s');
      rx.value = null;
    } finally {
      loading.value = false;
    }
  }
}
