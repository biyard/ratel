const en = {
  btn_prev: 'Prev',
  btn_next: 'Next',
  btn_submit: 'Submit',
  btn_update: 'Update',
  btn_login: 'Log in to Submit',

  is_required_false_label: 'Optional',
  is_required_true_label: 'Required',

  single_choice_label: 'Single Choice',
  multiple_choice_label: 'Multiple Choice',
  linear_scale_label: 'Linear Scale',
  checkbox_label: 'Checkbox',
  subjective_label: 'Subjective',
  short_answer_label: 'Short Answer',
  dropdown_label: 'Dropdown',

  delete_button_label: 'Delete',

  question_title_placeholder: 'Question Title',
  dropdown_select_placeholder: 'Select an option',
  subjective_input_placeholder: 'Please share your opinion.',
  option_input_placeholder: 'Enter an option',
  add_option_button_label: 'Add Option',

  no_questions: 'No questions available.',

  success_update_time: 'Success to update time range.',
  failed_update_time: 'Failed to update time range. please try later.',
  success_change_response: 'Success to save poll.',
  failed_change_response: 'Failed to save poll. please try later.',
  success_submit_answer: 'Success to submit answer.',
  failed_submit_answer: 'Failed to submit answer. please try later.',

  no_questions_error: 'Please add at least one question before saving.',
  invalid_question_error:
    'Please ensure all questions have valid titles and options.',

  save: 'Save',
  cancel: 'Cancel',
  modal_title: 'Submit final survey',
  modal_desc:
    'Once you submit your response, it cannot be changed. \nPlease double-check before submitting.',
};

const ko = {
  btn_prev: '이전',
  btn_next: '다음',
  btn_submit: '제출',
  btn_update: '수정',
  btn_login: '로그인 후 제출',

  is_required_false_label: '선택',
  is_required_true_label: '필수',

  single_choice_label: '단일 선택',
  multiple_choice_label: '다중 선택',
  linear_scale_label: '리니어 스케일',
  checkbox_label: '체크박스',
  subjective_label: '주관식',
  short_answer_label: '단답형',
  dropdown_label: '선택형',

  delete_button_label: '삭제',

  question_title_placeholder: '질문 제목',
  dropdown_select_placeholder: '옵션을 선택하세요',
  subjective_input_placeholder: '의견을 공유해주세요.', // ??
  option_input_placeholder: '옵션을 입력하세요',

  add_option_button_label: '옵션 추가하기',

  no_questions: '질문이 없습니다.',

  success_update_time: '성공적으로 시간을 설정하셨습니다.',
  failed_update_time: '시간 설정에 실패했습니다. 잠시 후 다시 시도해주세요.',
  success_change_response: '성공적으로 설문을 저장했습니다.',
  failed_change_response:
    '설문 저장에 실패했습니다. 잠시 후 다시 시도해주세요.',
  success_submit_answer: '성공적으로 응답을 저장했습니다.',
  failed_submit_answer: '응답 저장에 실패했습니다. 잠시 후 다시 시도해주세요.',

  no_questions_error: '저장하기 전에 질문을 하나 이상 추가해주세요.',
  invalid_question_error:
    '모든 질문에 유효한 제목과 옵션이 있는지 확인해주세요.',

  save: '저장',
  cancel: '취소',
  modal_title: '최종 설문 제출',
  modal_desc:
    '한번 제출한 응답은 변경할 수 없습니다. \n제출하기 전에 다시 확인해주세요.',
};
const i18n = {
  en,
  ko,
};

export default i18n;
