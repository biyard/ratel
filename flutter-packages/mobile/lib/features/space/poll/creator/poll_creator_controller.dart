import 'package:ratel/exports.dart';

class PollCreatorController extends BaseController {
  final SpacePollsApi _pollsApi = Get.find<SpacePollsApi>();

  late final String spacePk;
  late final String? pollSk;

  final poll = Rxn<PollModel>();
  final isLoading = false.obs;

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
}
