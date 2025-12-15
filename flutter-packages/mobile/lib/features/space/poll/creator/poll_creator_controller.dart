import 'package:ratel/exports.dart';

class PollCreatorController extends BaseController {
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
      logger.w('PollCreatorController: spacePk is null or empty in parameters');
      return;
    }

    spacePk = Uri.decodeComponent(rawPk);

    final spaceController = Get.find<SpaceController>();
    pollSk = spaceController.selectedPollSk.value;

    logger.d(
      'PollCreatorController initialized with spacePk=$spacePk pollSk=$pollSk',
    );

    if (pollSk != null && pollSk!.isNotEmpty) {
      _loadPoll();
    } else {
      logger.w('PollCreatorController: pollSk is null or empty, skip load');
    }
  }

  Future<void> _loadPoll() async {
    try {
      isLoading.value = true;
      final result = await _pollsApi.getPoll(spacePk, pollSk!);
      poll.value = result;
      logger.d(
        'Loaded poll: sk=${result.sk}, status=${pollStatusToString(result.status)}, questions=${result.questions.length}',
      );
    } catch (e) {
      logger.e('Failed to load poll for spacePk=$spacePk pollSk=$pollSk: $e');
    } finally {
      isLoading.value = false;
    }
  }

  Future<void> respondAnswers(List<Answer> answers) async {
    if (pollSk == null || pollSk!.isEmpty) {
      logger.w(
        'PollCreatorController.respondAnswers: pollSk is null or empty, skip submit',
      );
      return;
    }

    try {
      isSubmitting.value = true;

      final res = await _pollsApi.respondPoll(spacePk, pollSk!, answers);

      logger.d('Responded poll in creator: poll_space_pk=${res.pollSpacePk}');

      Biyard.info("successfully submitted your responses.");
      await _loadPoll();
    } catch (e) {
      logger.e(
        'Failed to respond poll in creator, spacePk=$spacePk pollSk=$pollSk: $e',
      );
      Biyard.error(
        "Failed to Submit Responses",
        "Failed to submit your responses. Please try again.",
      );
      rethrow;
    } finally {
      isSubmitting.value = false;
    }
  }
}
