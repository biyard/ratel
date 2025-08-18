import 'package:ratel/exports.dart';

class NetworkController extends BaseController {
  //FIXME: fix to real api
  RxList<InvitationModel> invitations = [
    InvitationModel(
      id: 1,
      nickname: 'Summer',
      profileUrl: '',
      username: 'biyard-summer',
      description:
          "Hi. I'd like to connect. Could you connect my invitation to re..",
    ),
    InvitationModel(
      id: 2,
      nickname: 'Ria',
      profileUrl: '',
      username: 'biyard-ria',
      description:
          "Hi. I'd like to connect. Could you connect my invitation to re..",
    ),
  ].obs;

  RxList<SuggestionModel> suggestions = [
    SuggestionModel(
      id: 3,
      nickname: 'Rosa',
      profileUrl: '',
      description: 'Project manager at Ratel foundation',
      spaces: 10,
      follows: null,
    ),
    SuggestionModel(
      id: 4,
      nickname: 'Victor',
      profileUrl: '',
      description: 'Software engineer at Biyard',
      spaces: 5,
      follows: null,
    ),
    SuggestionModel(
      id: 5,
      nickname: 'Ryan',
      profileUrl: '',
      description: 'Software engineer at Biyard',
      spaces: 10,
      follows: null,
    ),
    SuggestionModel(
      id: 6,
      nickname: 'Ratel',
      profileUrl: '',
      description: 'Smart community platform helping decision making..',
      spaces: null,
      follows: 100,
    ),
    SuggestionModel(
      id: 7,
      nickname: 'Ratel',
      profileUrl: '',
      description: 'Smart community platform helping decision making..',
      spaces: null,
      follows: 10,
    ),
    SuggestionModel(
      id: 8,
      nickname: 'Ratel',
      profileUrl: '',
      description: 'Smart community platform helping decision making..',
      spaces: null,
      follows: 20,
    ),
  ].obs;
}
