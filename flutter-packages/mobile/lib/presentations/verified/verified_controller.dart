import 'package:ratel/exports.dart';

class VerifiedController extends BaseController {
  final RxString didId = ''.obs;
  final Rxn<DidDocument> didDocument = Rxn<DidDocument>();
  final Rx<UserAttributes> attributes = UserAttributes.empty.obs;

  late final UserService _userService;
  late final UserApi _userApi;

  final isLoading = false.obs;

  @override
  void onInit() {
    super.onInit();
    _userService = Get.find<UserService>();
    _userApi = Get.find<UserApi>();
    _initAll();
  }

  Future<void> _initAll() async {
    isLoading.value = true;
    try {
      await Future.wait([_initDid(), _fetchDidAndAttributes()]);
    } catch (e, s) {
      logger.e('Failed to init VerifiedController: $e', stackTrace: s);
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> _initDid() async {
    var user = _userService.user.value;

    if (user.nickname.isEmpty) {
      user = await _userService.getUser();
    }

    final domain = Config.signDomain;
    final nickname = user.nickname;

    didId.value = 'did:mobile:$domain:$nickname';
  }

  Future<void> _fetchDidAndAttributes() async {
    try {
      final remoteDid = await _userApi.getOrCreateDid();
      if (remoteDid != null) {
        didDocument.value = remoteDid;
      }

      final attrs = await _userApi.getAttributes();
      attributes.value = attrs;
    } catch (e, s) {
      logger.e('Failed to fetch did/attributes: $e', stackTrace: s);
    }
  }

  Future<void> refreshDid() async {
    await _initDid();
    await _fetchDidAndAttributes();
  }

  Future<UserAttributes> signAttributesWithCode(String code) async {
    try {
      final attrs = await _userApi.signAttributesWithCode(code);

      if (attrs.gender == null && attrs.university == null) {
        return attrs;
      }

      attributes.value = attrs;
      return attrs;
    } catch (e, s) {
      logger.e('Failed to sign attributes with code: $e', stackTrace: s);
      return attributes.value;
    }
  }
}
