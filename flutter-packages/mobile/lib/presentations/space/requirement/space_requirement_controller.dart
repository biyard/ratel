import 'package:ratel/exports.dart';

class SpaceRequirementController extends BaseController {
  final SpaceService _spaceService = Get.find<SpaceService>();
  final SpacePollsApi _pollsApi = Get.find<SpacePollsApi>();

  late final String spacePk;
  late final String? pollSk;

  Rxn<SpaceModel> get spaceRx => _spaceService.spaceOf(spacePk);
  SpaceModel? get space => spaceRx.value;

  final poll = Rxn<PollModel>();
  final isLoading = false.obs;
  final isSubmitting = false.obs;

  @override
  void onInit() {
    super.onInit();

    final rawPk = Get.parameters['spacePk'];
    if (rawPk == null || rawPk.isEmpty) {
      logger.w(
        'SpaceRequirementController: spacePk is null or empty in parameters',
      );
      return;
    }

    spacePk = Uri.decodeComponent(rawPk);

    final spaceController = Get.isRegistered<SpaceController>()
        ? Get.find<SpaceController>()
        : null;
    final fromSpace = spaceController?.selectedPollSk.value;

    pollSk = _buildDefaultPollSk(spacePk);

    logger.d(
      'SpaceRequirementController initialized with spacePk=$spacePk pollSk=$pollSk',
    );

    if (pollSk != null && pollSk!.isNotEmpty) {
      _loadPoll();
    } else {
      logger.w(
        'SpaceRequirementController: pollSk is null or empty, skip load',
      );
    }
  }

  String _buildDefaultPollSk(String rawSpacePk) {
    final idx = rawSpacePk.indexOf('#');
    if (idx == -1) {
      return '';
    }
    final suffix = rawSpacePk.substring(idx + 1);
    return 'SPACE_POLL#$suffix';
  }

  Future<void> _loadPoll() async {
    try {
      isLoading.value = true;
      final result = await _pollsApi.getPoll(spacePk, pollSk!);
      poll.value = result;
      logger.d(
        'Loaded poll for viewer: sk=${result.sk}, status=${pollStatusToString(result.status)}, questions=${result.questions.length}',
      );
    } catch (e) {
      logger.e(
        'Failed to load poll for viewer, spacePk=$spacePk pollSk=$pollSk: $e',
      );
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> reload() => _loadPoll();

  void goBack() {
    Get.rootDelegate.offNamed(AppRoutes.mainScreen);
  }

  Future<void> respondAnswers(List<Answer> answers) async {
    if (pollSk == null || pollSk!.isNotEmpty == false) {
      logger.w(
        'PollViewerController.respondAnswers: pollSk is null or empty, skip submit',
      );
      return;
    }

    try {
      isSubmitting.value = true;

      final res = await _pollsApi.respondPoll(spacePk, pollSk!, answers);

      logger.d('Responded poll in viewer: poll_space_pk=${res.pollSpacePk}');
      Biyard.info("successfully submitted your responses.");
      Get.rootDelegate.offNamed(spaceWithPk(spacePk));
    } catch (e) {
      logger.e(
        'Failed to respond poll in viewer, spacePk=$spacePk pollSk=$pollSk: $e',
      );
      Biyard.error(
        "Failed to Submit Respond",
        "Failed to submit your responses. Please try again.",
      );
      rethrow;
    } finally {
      isSubmitting.value = false;
    }
  }
}
