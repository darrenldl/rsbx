#!/bin/bash

tests=(
    "check_from_to_force_misalign"
    "check_from_to_rounding"
    "check_from_to_tests"
    "check_ref_from_to_tests"
    "check_ref_from_to_tests_force_misalign"
    "check_ref_from_to_tests_rounding"
    "compare_encode_file_and_stdin"
    "decode_from_to_tests_corruption_based"
    "decode_from_to_tests_corruption_based_force_misalign"
    "decode_from_to_tests_corruption_based_rounding"
    "decode_from_to_tests_decode_stdout"
    "decode_from_to_tests_decode_stdout_force_misalign"
    "decode_from_to_tests_decode_stdout_rounding"
    "decode_ref_from_to_tests"
    "decode_ref_from_to_tests_force_misalign"
    "decode_ref_from_to_tests_rounding"
    "hash_tests_decode_stdout"
    "nometa_tests"
    "nometa_tests_decode_stdout"
    "nometa_tests_encode_stdin"
    "out_file_logic_tests"
    "rescue_from_to_tests"
    "rescue_from_to_tests_encode_stdin"
    "rescue_from_to_tests_force_misalign"
    "rescue_from_to_tests_rounding"
    "rescue_pick_uid_tests"
    "rescue_pick_uid_tests_decode_stdout"
    "rescue_pick_uid_tests_encode_stdin"
    "rescue_tests_decode_stdout"
    "show_from_to_tests"
    "show_from_to_tests_force_misalign"
    "show_from_to_tests_rounding"
    "show_pick_uid_tests"
    "sort_ref_from_to_tests"
    "sort_ref_from_to_tests_force_misalign"
    "sort_ref_from_to_tests_rounding"
    "verify_encode_help_msg_consistent_w_actual_defaults"
    "check_hash_only_tests"
    "check_hash_tests"
    "check_hash_tests_manual_burst"
    "compare_decode_file_and_stdout"
    "compare_decode_file_and_stdout_nometa"
    "decode_blanks"
    "decode_blanks_decode_stdout"
    "decode_manual_burst_decode_stdout"
    "file_size_calc_tests"
    "hash_tests"
    "hash_tests_encode_stdin"
    "repair_truncated_tests_decode_stdout"
    "rescue_tests"
    "rescue_tests_encode_stdin"
    "sort_from_to_tests"
    "sort_from_to_tests_force_misalign"
    "sort_from_to_tests_rounding"
    "update_fnm_tests"
    "update_no_fnm_tests"
    "update_no_hsh_tests"
    "update_no_snm_tests"
    "update_snm_tests"
    "version_tests"
    "version_tests_decode_stdout"
    "version_tests_encode_stdin"
    "burst_corruption_tests"
    "corruption_tests_encode_stdin"
    "decode_manual_burst"
    "decode_manual_burst_encode_stdin"
    "decode_multi_pass_no_skip"
    "repair_truncated_tests"
    "repair_truncated_tests_encode_stdin"
    "sort_dry_run"
    "burst_corruption_tests_decode_stdout"
    "burst_corruption_tests_encode_stdin"
    "corruption_tests"
    "corruption_tests_decode_stdout"
    "decode_from_to_tests"
    "repair_manual_burst_decode_stdout"
    "sort_stats_tests"
    "decode_from_to_tests_force_misalign"
    "decode_from_to_tests_rounding"
    "decode_guess_burst_force_misalign"
    "repair_manual_burst"
    "repair_manual_burst_encode_stdin"
    "show_guess_burst_force_misalign"
    "sort_tests_decode_stdout"
    "sort_tests"
    "sort_tests_encode_stdin"
    "compare_decode_file_and_stdout_corrupted_container"
    "update_hash_tests"
    "sort_guess_burst_force_misalign"
    "decode_multi_pass"
    "sort_multi_pass_no_skip"
    "sort_multi_pass"
)
