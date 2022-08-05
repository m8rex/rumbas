pub struct NumbasDefaults {
    pub navigation_reverse: bool,
    pub navigation_browsing_enabled: bool,
    pub navigation_allow_steps: bool,
    pub navigation_prevent_leaving: bool,
    pub navigation_show_results_page: crate::exam::navigation::ShowResultsPage,
    pub navigation_show_names_of_question_groups: bool,
    pub navigation_start_password: String,
    pub navigation_on_leave: crate::exam::navigation::LeaveAction,
    pub question_navigation_prevent_leaving: bool,
    pub basic_settings_show_student_name: bool,
    pub basic_settings_allow_printing: bool,
    pub builtin_constants_e: bool,
    pub builtin_constants_i: bool,
    pub builtin_constants_pi: bool,
    pub number_entry_correct_answer_style: crate::support::answer_style::AnswerStyle,
    pub length_restriction_length: usize,
    pub number_entry_fractions_must_be_reduced: bool,
    pub number_entry_partial_credit_if_fraction_not_reduced: crate::support::primitive::Number,
    pub number_entry_hint_fraction: bool,

    pub choose_one_has_to_select_option: bool,
    pub choose_one_show_cell_answer_state: bool,
    pub gapfill_sort_answers: bool,
    pub match_answers_with_items_min_answers: crate::support::primitive::SafeNatural,
    pub choose_multiple_min_answers: crate::support::primitive::SafeNatural,

    pub pattern_match_case_sensitive: bool,
    pub pattern_match_partial_credit: crate::support::primitive::SafeFloat,
}

// TODO default values
pub const DEFAULTS: NumbasDefaults = NumbasDefaults {
    navigation_reverse: true,
    navigation_browsing_enabled: true,
    navigation_allow_steps: true,
    navigation_prevent_leaving: true,
    navigation_show_results_page: crate::exam::navigation::ShowResultsPage::Never,
    navigation_show_names_of_question_groups: true,
    navigation_start_password: String::new(),
    navigation_on_leave: crate::exam::navigation::LeaveAction::None {
        message: String::new(),
    },
    question_navigation_prevent_leaving: true,
    basic_settings_show_student_name: true,
    basic_settings_allow_printing: true,
    builtin_constants_e: false,
    builtin_constants_i: false,
    builtin_constants_pi: false,
    number_entry_correct_answer_style: crate::support::answer_style::AnswerStyle::EnglishPlain,
    length_restriction_length: 0,

    choose_one_has_to_select_option: true,
    choose_one_show_cell_answer_state: true,
    gapfill_sort_answers: true,
    match_answers_with_items_min_answers: crate::support::primitive::SafeNatural(0),
    choose_multiple_min_answers: crate::support::primitive::SafeNatural(0),

    pattern_match_case_sensitive: false,
    pattern_match_partial_credit: crate::support::primitive::SafeFloat(0.0),

    number_entry_fractions_must_be_reduced: false,
    number_entry_partial_credit_if_fraction_not_reduced: crate::support::primitive::Number::Integer(
        0,
    ),
    number_entry_hint_fraction: true
    };
