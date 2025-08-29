import 'package:ratel/exports.dart';

class DraftLocalization {
  static final _draftMyDraft = s('draftMyDraft', 'My Drafts', 'My Drafts');
  static String get draftMyDraft => _draftMyDraft.tr;

  static final _draftDeleteDraft = s(
    'draftDeleteDraft',
    'Delete Draft',
    'Delete Draft',
  );
  static String get draftDeleteDraft => _draftDeleteDraft.tr;

  static final _draftDeleteDraftDescription = s(
    'draftDeleteDraftDescription',
    'Could you remove this draft? This action cannot be undone.',
    'Could you remove this draft? This action cannot be undone.',
  );
  static String get draftDeleteDraftDescription =>
      _draftDeleteDraftDescription.tr;

  static final _draftCancel = s('draftCancel', 'Cancel', 'Cancel');
  static String get draftCancel => _draftCancel.tr;

  static final _draftRemove = s('draftRemove', 'Remove', 'Remove');
  static String get draftRemove => _draftRemove.tr;

  static final _draftAdd = s('draftAdd', 'Add', 'Add');
  static String get draftAdd => _draftAdd.tr;

  static final _draftPost = s('draftPost', 'Post', 'Post');
  static String get draftPost => _draftPost.tr;

  static final _draftTypeTitle = s(
    'draftTypeTitle',
    'Type a title',
    'Type a title',
  );
  static String get draftTypeTitle => _draftTypeTitle.tr;

  static final _draftTitleWarning = s(
    'draftTitleWarning',
    'Enter at least 10 characters to continue',
    'Enter at least 10 characters to continue',
  );
  static String get draftTitleWarning => _draftTitleWarning.tr;

  static final _draftCategory = s('draftCategory', 'Category', 'Category');
  static String get draftCategory => _draftCategory.tr;

  static final _draftAddCategory = s(
    'draftAddCategory',
    'Add categories...',
    'Add categories...',
  );
  static String get draftAddCategory => _draftAddCategory.tr;

  static final _draftCategoryWarning = s(
    'draftCategoryWarning',
    'At least one category is required',
    'At least one category is required',
  );
  static String get draftCategoryWarning => _draftCategoryWarning.tr;

  static final _draftTypeSomething = s(
    'draftTypeSomething',
    'Type something...',
    'Type something...',
  );
  static String get draftTypeSomething => _draftTypeSomething.tr;

  static final _draftDescriptionWarning = s(
    'draftDescriptionWarning',
    'Enter at least 100 characters to continue',
    'Enter at least 100 characters to continue',
  );
  static String get draftDescriptionWarning => _draftDescriptionWarning.tr;
}
