import 'package:ratel/exports.dart';

class AnalyzeCreatorController extends BaseController {
  final SpacePollsApi _pollsApi = Get.find<SpacePollsApi>();

  late final String spacePk;
  late final String? pollSk;

  final poll = Rxn<PollModel>();
  final pollResult = Rxn<PollResult>();
  final isLoading = false.obs;

  @override
  void onInit() {
    super.onInit();

    final rawPk = Get.parameters['spacePk'];
    if (rawPk == null || rawPk.isEmpty) {
      logger.w(
        'AnalyzeCreatorController: spacePk is null or empty in parameters',
      );
      return;
    }

    spacePk = Uri.decodeComponent(rawPk);

    final spaceController = Get.find<SpaceController>();
    pollSk = spaceController.selectedPollSk.value;

    logger.d(
      'AnalyzeCreatorController initialized with spacePk=$spacePk pollSk=$pollSk',
    );

    if (pollSk != null && pollSk!.isNotEmpty) {
      loadPollResult();
    } else {
      logger.w('AnalyzeCreatorController: pollSk is null or empty, skip load');
    }
  }

  Future<void> loadPollResult() async {
    if (pollSk == null || pollSk!.isEmpty) {
      logger.w(
        'AnalyzeCreatorController.loadPollResult: pollSk is null or empty, skip',
      );
      return;
    }

    try {
      isLoading.value = true;
      final pollFuture = _pollsApi.getPoll(spacePk, pollSk!);
      final resultFuture = _pollsApi.getPollResult(spacePk, pollSk!);
      final loadedPoll = await pollFuture;
      final result = await resultFuture;

      poll.value = loadedPoll;
      pollResult.value = result;

      logger.d(
        'Loaded poll result: questions=${loadedPoll.questions.length}, summaries=${result.summaries.length}, sample=${result.sampleAnswers.length}, final=${result.finalAnswers.length}',
      );
    } catch (e) {
      logger.e(
        'Failed to load poll result for spacePk=$spacePk pollSk=$pollSk: $e',
      );
    } finally {
      isLoading.value = false;
    }
  }
}
