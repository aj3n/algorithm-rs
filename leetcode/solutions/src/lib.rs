#![allow(dead_code)]
#![allow(clippy::type_complexity)]
#![allow(clippy::comparison_chain)]
#[macro_use]
pub mod util;
mod n0001_two_sum;
mod n0005_longest_palindromic_substring;
mod n0006_zigzag_conversion;
mod n0007_reverse_integer;
mod n0010_regular_expression_matching;
mod n0012_integer_to_roman;
mod n0015_3sum;
mod n0017_letter_combinations_of_a_phone_number;
mod n0019_remove_nth_node_from_end_of_list;
mod n0020_valid_parentheses;
mod n0021_merge_two_sorted_lists;
mod n0022_generate_parentheses;
mod n0023_merge_k_sorted_lists;
mod n0024_swap_nodes_in_pairs;
mod n0026_remove_duplicates_from_sorted_array;
mod n0028_implement_strstr;
mod n0033_search_in_rotated_sorted_array;
mod n0034_find_first_and_last_position_of_element_in_sorted_array;
mod n0035_search_insert_position;
mod n0036_valid_sudoku;
mod n0037_sudoku_solver;
mod n0039_combination_sum;
mod n0041_first_missing_positive;
mod n0042_trapping_rain_water;
mod n0045_jump_game_ii;
mod n0046_permutations;
mod n0048_rotate_image;
mod n0049_group_anagrams;
mod n0050_powx_n;
mod n0051_n_queens;
mod n0053_maximum_subarray;
mod n0054_spiral_matrix;
mod n0055_jump_game;
mod n0057_insert_interval;
mod n0058_length_of_last_word;
mod n0059_spiral_matrix_ii;
mod n0062_unique_paths;
mod n0063_unique_paths_ii;
mod n0064_minimum_path_sum;
mod n0065_valid_number;
mod n0067_add_binary;
mod n0068_text_justification;
mod n0070_climbing_stairs;
mod n0071_simplify_path;
mod n0072_edit_distance;
mod n0074_search_a_2d_matrix;
mod n0076_minimum_window_substring;
mod n0077_combinations;
mod n0078_subsets;
mod n0079_word_search;
mod n0081_search_in_rotated_sorted_array_ii;
mod n0084_largest_rectangle_in_histogram;
mod n0085_maximal_rectangle;
mod n0086_partition_list;
mod n0087_scramble_string;
mod n0089_gray_code;
mod n0090_subsets_ii;
mod n0091_decode_ways;
mod n0092_reverse_linked_list_ii;
mod n0093_restore_ip_addresses;
mod n0095_unique_binary_search_trees_ii;
mod n0097_interleaving_string;
mod n0100_same_tree;
mod n0101_symmetric_tree;
mod n0102_binary_tree_level_order_traversal;
mod n0103_binary_tree_zigzag_level_order_traversal;
mod n0104_maximum_depth_of_binary_tree;
mod n0105_construct_binary_tree_from_preorder_and_inorder_traversal;
mod n0106_construct_binary_tree_from_inorder_and_postorder_traversal;
mod n0107_binary_tree_level_order_traversal_ii;
mod n0109_convert_sorted_list_to_binary_search_tree;
mod n0111_minimum_depth_of_binary_tree;
mod n0112_path_sum;
mod n0113_path_sum_ii;
mod n0114_flatten_binary_tree_to_linked_list;
mod n0118_pascals_triangle;
mod n0119_pascals_triangle_ii;
mod n0120_triangle;
mod n0121_best_time_to_buy_and_sell_stock;
mod n0122_best_time_to_buy_and_sell_stock_ii;
mod n0123_best_time_to_buy_and_sell_stock_iii;
mod n0124_binary_tree_maximum_path_sum;
mod n0126_word_ladder_ii;
mod n0129_sum_root_to_leaf_numbers;
mod n0131_palindrome_partitioning;
mod n0132_palindrome_partitioning_ii;
mod n0134_gas_station;
mod n0135_candy;
mod n0136_single_number;
mod n0137_single_number_ii;
mod n0139_word_break;
mod n0140_word_break_ii;
mod n0143_reorder_list;
mod n0144_binary_tree_preorder_traversal;
mod n0146_lru_cache;
mod n0148_sort_list;
mod n0149_max_points_on_a_line;
mod n0150_evaluate_reverse_polish_notation;
mod n0152_maximum_product_subarray;
mod n0153_find_minimum_in_rotated_sorted_array;
mod n0154_find_minimum_in_rotated_sorted_array_ii;
mod n0155_min_stack;
mod n0162_find_peak_element;
mod n0164_maximum_gap;
mod n0165_compare_version_numbers;
mod n0166_fraction_to_recurring_decimal;
mod n0167_two_sum_ii_input_array_is_sorted;
mod n0168_excel_sheet_column_title;
mod n0169_majority_element;
mod n0171_excel_sheet_column_number;
mod n0172_factorial_trailing_zeroes;
mod n0173_binary_search_tree_iterator;
mod n0187_repeated_dna_sequences;
mod n0188_best_time_to_buy_and_sell_stock_iv;
mod n0191_number_of_1_bits;
mod n0198_house_robber;
mod n0199_binary_tree_right_side_view;
mod n0200_number_of_islands;
mod n0201_bitwise_and_of_numbers_range;
mod n0202_happy_number;
mod n0204_count_primes;
mod n0205_isomorphic_strings;
mod n0206_reverse_linked_list;
mod n0207_course_schedule;
mod n0208_implement_trie_prefix_tree;
mod n0209_minimum_size_subarray_sum;
mod n0210_course_schedule_ii;
mod n0211_design_add_and_search_words_data_structure;
mod n0212_word_search_ii;
mod n0213_house_robber_ii;
mod n0215_kth_largest_element_in_an_array;
mod n0216_combination_sum_iii;
mod n0218_the_skyline_problem;
mod n0220_contains_duplicate_iii;
mod n0221_maximal_square;
mod n0222_count_complete_tree_nodes;
mod n0223_rectangle_area;
mod n0224_basic_calculator;
mod n0225_implement_stack_using_queues;
mod n0226_invert_binary_tree;
mod n0227_basic_calculator_ii;
mod n0228_summary_ranges;
mod n0231_power_of_two;
mod n0232_implement_queue_using_stacks;
mod n0233_number_of_digit_one;
mod n0234_palindrome_linked_list;
mod n0235_lowest_common_ancestor_of_a_binary_search_tree;
mod n0236_lowest_common_ancestor_of_a_binary_tree;
mod n0238_product_of_array_except_self;
mod n0239_sliding_window_maximum;
mod n0241_different_ways_to_add_parentheses;
mod n0242_valid_anagram;
mod n0257_binary_tree_paths;
mod n0258_add_digits;
mod n0263_ugly_number;
mod n0264_ugly_number_ii;
mod n0268_missing_number;
mod n0274_h_index;
mod n0275_h_index_ii;
mod n0279_perfect_squares;
mod n0282_expression_add_operators;
mod n0283_move_zeroes;
mod n0287_find_the_duplicate_number;
mod n0289_game_of_life;
mod n0290_word_pattern;
mod n0292_nim_game;
mod n0295_find_median_from_data_stream;
mod n0297_serialize_and_deserialize_binary_tree;
mod n0299_bulls_and_cows;
mod n0300_longest_increasing_subsequence;
mod n0301_remove_invalid_parentheses;
mod n0304_range_sum_query_2d_immutable;
mod n0306_additive_number;
mod n0309_best_time_to_buy_and_sell_stock_with_cooldown;
mod n0310_minimum_height_trees;
mod n0312_burst_balloons;
mod n0313_super_ugly_number;
mod n0316_remove_duplicate_letters;
mod n0318_maximum_product_of_word_lengths;
mod n0319_bulb_switcher;
mod n0321_create_maximum_number;
mod n0322_coin_change;
mod n0326_power_of_three;
mod n0328_odd_even_linked_list;
mod n0329_longest_increasing_path_in_a_matrix;
mod n0332_reconstruct_itinerary;
mod n0334_increasing_triplet_subsequence;
mod n0337_house_robber_iii;
mod n0338_counting_bits;
mod n0341_flatten_nested_list_iterator;
mod n0342_power_of_four;
mod n0345_reverse_vowels_of_a_string;
mod n0347_top_k_frequent_elements;
mod n0349_intersection_of_two_arrays;
mod n0352_data_stream_as_disjoint_intervals;
mod n0365_water_and_jug_problem;
mod n0367_valid_perfect_square;
mod n0368_largest_divisible_subset;
mod n0373_find_k_pairs_with_smallest_sums;
mod n0374_guess_number_higher_or_lower;
mod n0375_guess_number_higher_or_lower_ii;
mod n0377_combination_sum_iv;
mod n0378_kth_smallest_element_in_a_sorted_matrix;
mod n0380_insert_delete_getrandom_o1;
mod n0381_insert_delete_getrandom_o1_duplicates_allowed;
mod n0382_linked_list_random_node;
mod n0383_ransom_note;
mod n0387_first_unique_character_in_a_string;
mod n0389_find_the_difference;
mod n0391_perfect_rectangle;
mod n0392_is_subsequence;
mod n0397_integer_replacement;
mod n0399_evaluate_division;
mod n0400_nth_digit;
mod n0402_remove_k_digits;
mod n0403_frog_jump;
mod n0404_sum_of_left_leaves;
mod n0407_trapping_rain_water_ii;
mod n0410_split_array_largest_sum;
mod n0423_reconstruct_original_digits_from_english;
mod n0424_longest_repeating_character_replacement;
mod n0433_minimum_genetic_mutation;
mod n0435_non_overlapping_intervals;
mod n0437_path_sum_iii;
mod n0438_find_all_anagrams_in_a_string;
mod n0441_arranging_coins;
mod n0442_find_all_duplicates_in_an_array;
mod n0443_string_compression;
mod n0445_add_two_numbers_ii;
mod n0446_arithmetic_slices_ii_subsequence;
mod n0451_sort_characters_by_frequency;
mod n0452_minimum_number_of_arrows_to_burst_balloons;
mod n0453_minimum_moves_to_equal_array_elements;
mod n0454_4sum_ii;
mod n0456_132_pattern;
mod n0460_lfu_cache;
mod n0462_minimum_moves_to_equal_array_elements_ii;
mod n0463_island_perimeter;
mod n0472_concatenated_words;
mod n0473_matchsticks_to_square;
mod n0474_ones_and_zeroes;
mod n0476_number_complement;
mod n0486_predict_the_winner;
mod n0491_non_decreasing_subsequences;
mod n0493_reverse_pairs;
mod n0494_target_sum;
mod n0495_teemo_attacking;
mod n0501_find_mode_in_binary_search_tree;
mod n0502_ipo;
mod n0509_fibonacci_number;
mod n0514_freedom_trail;
mod n0515_find_largest_value_in_each_tree_row;
mod n0516_longest_palindromic_subsequence;
mod n0518_coin_change_ii;
mod n0520_detect_capital;
mod n0523_continuous_subarray_sum;
mod n0524_longest_word_in_dictionary_through_deleting;
mod n0525_contiguous_array;
mod n0530_minimum_absolute_difference_in_bst;
mod n0540_single_element_in_a_sorted_array;
mod n0542_01_matrix;
mod n0543_diameter_of_binary_tree;
mod n0547_number_of_provinces;
mod n0554_brick_wall;
mod n0560_subarray_sum_equals_k;
mod n0563_binary_tree_tilt;
mod n0567_permutation_in_string;
mod n0575_distribute_candies;
mod n0576_out_of_boundary_paths;
mod n0583_delete_operation_for_two_strings;
mod n0600_non_negative_integers_without_consecutive_ones;
mod n0605_can_place_flowers;
mod n0606_construct_string_from_binary_tree;
mod n0611_valid_triangle_number;
mod n0623_add_one_row_to_tree;
mod n0629_k_inverse_pairs_array;
mod n0630_course_schedule_iii;
mod n0637_average_of_levels_in_binary_tree;
mod n0645_set_mismatch;
mod n0646_maximum_length_of_pair_chain;
mod n0647_palindromic_substrings;
mod n0649_dota2_senate;
mod n0652_find_duplicate_subtrees;
mod n0662_maximum_width_of_binary_tree;
mod n0665_non_decreasing_array;
mod n0673_number_of_longest_increasing_subsequence;
mod n0678_valid_parenthesis_string;
mod n0684_redundant_connection;
mod n0688_knight_probability_in_chessboard;
mod n0689_maximum_sum_of_3_non_overlapping_subarrays;
mod n0699_falling_squares;
mod n0703_kth_largest_element_in_a_stream;
mod n0704_binary_search;
mod n0705_design_hashset;
mod n0712_minimum_ascii_delete_sum_for_two_strings;
mod n0713_subarray_product_less_than_k;
mod n0714_best_time_to_buy_and_sell_stock_with_transaction_fee;
mod n0715_range_module;
mod n0719_find_k_th_smallest_pair_distance;
mod n0725_split_linked_list_in_parts;
mod n0733_flood_fill;
mod n0735_asteroid_collision;
mod n0739_daily_temperatures;
mod n0744_find_smallest_letter_greater_than_target;
mod n0745_prefix_and_suffix_search;
mod n0746_min_cost_climbing_stairs;
mod n0752_open_the_lock;
mod n0763_partition_labels;
mod n0766_toeplitz_matrix;
mod n0767_reorganize_string;
mod n0771_jewels_and_stones;
mod n0779_k_th_symbol_in_grammar;
mod n0783_minimum_distance_between_bst_nodes;
mod n0785_is_graph_bipartite;
mod n0786_k_th_smallest_prime_fraction;
mod n0787_cheapest_flights_within_k_stops;
mod n0790_domino_and_tromino_tiling;
mod n0791_custom_sort_string;
mod n0792_number_of_matching_subsequences;
mod n0794_valid_tic_tac_toe_state;
mod n0795_number_of_subarrays_with_bounded_maximum;
mod n0797_all_paths_from_source_to_target;
mod n0798_smallest_rotation_with_highest_score;
mod n0799_champagne_tower;
mod n0802_find_eventual_safe_states;
mod n0805_split_array_with_same_average;
mod n0807_max_increase_to_keep_city_skyline;
mod n0808_soup_servings;
mod n0810_chalkboard_xor_game;
mod n0815_bus_routes;
mod n0823_binary_trees_with_factors;
mod n0834_sum_of_distances_in_tree;
mod n0836_rectangle_overlap;
mod n0837_new_21_game;
mod n0839_similar_string_groups;
mod n0841_keys_and_rooms;
mod n0844_backspace_string_compare;
mod n0847_shortest_path_visiting_all_nodes;
mod n0852_peak_index_in_a_mountain_array;
mod n0859_buddy_strings;
mod n0863_all_nodes_distance_k_in_binary_tree;
mod n0864_shortest_path_to_get_all_keys;
mod n0871_minimum_number_of_refueling_stops;
mod n0872_leaf_similar_trees;
mod n0875_koko_eating_bananas;
mod n0876_middle_of_the_linked_list;
mod n0877_stone_game;
mod n0879_profitable_schemes;
mod n0880_decoded_string_at_index;
mod n0881_boats_to_save_people;
mod n0886_possible_bipartition;
mod n0889_construct_binary_tree_from_preorder_and_postorder_traversal;
mod n0890_find_and_replace_pattern;
mod n0894_all_possible_full_binary_trees;
mod n0896_monotonic_array;
mod n0897_increasing_order_search_tree;
mod n0898_bitwise_ors_of_subarrays;
mod n0899_orderly_queue;
mod n0901_online_stock_span;
mod n0904_fruit_into_baskets;
mod n0907_sum_of_subarray_minimums;
mod n0909_snakes_and_ladders;
mod n0912_sort_an_array;
mod n0916_word_subsets;
mod n0918_maximum_sum_circular_subarray;
mod n0919_complete_binary_tree_inserter;
mod n0923_3sum_with_multiplicity;
mod n0926_flip_string_to_monotone_increasing;
mod n0927_three_equal_parts;
mod n0930_binary_subarrays_with_sum;
mod n0931_minimum_falling_path_sum;
mod n0934_shortest_bridge;
mod n0935_knight_dialer;
mod n0938_range_sum_of_bst;
mod n0944_delete_columns_to_make_sorted;
mod n0946_validate_stack_sequences;
mod n0947_most_stones_removed_with_same_row_or_column;
mod n0948_bag_of_tokens;
mod n0950_reveal_cards_in_increasing_order;
mod n0952_largest_component_size_by_common_factor;
mod n0953_verifying_an_alien_dictionary;
mod n0957_prison_cells_after_n_days;
mod n0958_check_completeness_of_a_binary_tree;
mod n0974_subarray_sums_divisible_by_k;
mod n0976_largest_perimeter_triangle;
mod n0977_squares_of_a_sorted_array;
mod n0980_unique_paths_iii;
mod n0983_minimum_cost_for_tickets;
mod n0988_smallest_string_starting_from_leaf;
mod n0989_add_to_array_form_of_integer;
mod n0992_subarrays_with_k_different_integers;
mod n0997_find_the_town_judge;
mod n1004_max_consecutive_ones_iii;
mod n1008_construct_binary_search_tree_from_preorder_traversal;
mod n1011_capacity_to_ship_packages_within_d_days;
mod n1013_partition_array_into_three_parts_with_equal_sum;
mod n1020_number_of_enclaves;
mod n1026_maximum_difference_between_node_and_ancestor;
mod n1027_longest_arithmetic_subsequence;
mod n1034_coloring_a_border;
mod n1035_uncrossed_lines;
mod n1043_partition_array_for_maximum_sum;
mod n1046_last_stone_weight;
mod n1047_remove_all_adjacent_duplicates_in_string;
mod n1048_longest_string_chain;
mod n1049_last_stone_weight_ii;
mod n1061_lexicographically_smallest_equivalent_string;
mod n1071_greatest_common_divisor_of_strings;
mod n1074_number_of_submatrices_that_sum_to_target;
mod n1091_shortest_path_in_binary_matrix;
mod n1095_find_in_mountain_array;
mod n1125_smallest_sufficient_team;
mod n1129_shortest_path_with_alternating_colors;
mod n1137_n_th_tribonacci_number;
mod n1140_stone_game_ii;
mod n1143_longest_common_subsequence;
mod n1155_number_of_dice_rolls_with_target_sum;
mod n1161_maximum_level_sum_of_a_binary_tree;
mod n1162_as_far_from_land_as_possible;
mod n1171_remove_zero_sum_consecutive_nodes_from_linked_list;
mod n1187_make_array_strictly_increasing;
mod n1190_reverse_substrings_between_each_pair_of_parentheses;
mod n1201_ugly_number_iii;
mod n1203_sort_items_by_groups_respecting_dependencies;
mod n1206_design_skiplist;
mod n1207_unique_number_of_occurrences;
mod n1218_longest_arithmetic_subsequence_of_given_difference;
mod n1220_count_vowels_permutation;
mod n1232_check_if_it_is_a_straight_line;
mod n1235_maximum_profit_in_job_scheduling;
mod n1239_maximum_length_of_a_concatenated_string_with_unique_characters;
mod n1249_minimum_remove_to_make_valid_parentheses;
mod n1254_number_of_closed_islands;
mod n1269_number_of_ways_to_stay_in_the_same_place_after_some_steps;
mod n1282_group_the_people_given_the_group_size_they_belong_to;
mod n1287_element_appearing_more_than_25_in_sorted_array;
mod n1289_minimum_falling_path_sum_ii;
mod n1291_sequential_digits;
mod n1293_shortest_path_in_a_grid_with_obstacles_elimination;
mod n1312_minimum_insertion_steps_to_make_a_string_palindrome;
mod n1318_minimum_flips_to_make_a_or_b_equal_to_c;
mod n1319_number_of_operations_to_make_network_connected;
mod n1323_maximum_69_number;
mod n1326_minimum_number_of_taps_to_open_to_water_a_garden;
mod n1335_minimum_difficulty_of_a_job_schedule;
mod n1337_the_k_weakest_rows_in_a_matrix;
mod n1339_maximum_product_of_splitted_binary_tree;
mod n1344_angle_between_hands_of_a_clock;
mod n1345_jump_game_iv;
mod n1351_count_negative_numbers_in_a_sorted_matrix;
mod n1354_construct_target_array_with_multiple_sums;
mod n1356_sort_integers_by_the_number_of_1_bits;
mod n1359_count_all_valid_pickup_and_delivery_options;
mod n1363_largest_multiple_of_three;
mod n1368_minimum_cost_to_make_at_least_one_valid_path_in_a_grid;
mod n1372_longest_zigzag_path_in_a_binary_tree;
mod n1376_time_needed_to_inform_all_employees;
mod n1396_design_underground_system;
mod n1402_reducing_dishes;
mod n1406_stone_game_iii;
mod n1416_restore_the_array;
mod n1420_build_array_where_you_can_find_the_maximum_exactly_k_comparisons;
mod n1422_maximum_score_after_splitting_a_string;
mod n1423_maximum_points_you_can_obtain_from_cards;
mod n1424_diagonal_traverse_ii;
mod n1425_constrained_subsequence_sum;
mod n1426_counting_elements;
mod n1427_perform_string_shifts;
mod n1431_kids_with_the_greatest_number_of_candies;
mod n1436_destination_city;
mod n1441_build_an_array_with_stack_operations;
mod n1442_count_triplets_that_can_form_two_arrays_of_equal_xor;
mod n1443_minimum_time_to_collect_all_apples_in_a_tree;
mod n1444_number_of_ways_of_cutting_a_pizza;
mod n1446_consecutive_characters;
mod n1456_maximum_number_of_vowels_in_a_substring_of_given_length;
mod n1457_pseudo_palindromic_paths_in_a_binary_tree;
mod n1458_max_dot_product_of_two_subsequences;
mod n1463_cherry_pickup_ii;
mod n1466_reorder_routes_to_make_all_paths_lead_to_the_city_zero;
mod n1470_shuffle_the_array;
mod n1472_design_browser_history;
mod n1480_running_sum_of_1d_array;
mod n1481_least_number_of_unique_integers_after_k_removals;
mod n1483_kth_ancestor_of_a_tree_node;
mod n1489_find_critical_and_pseudo_critical_edges_in_minimum_spanning_tree;
mod n1491_average_salary_excluding_the_minimum_and_maximum_salary;
mod n1492_the_kth_factor_of_n;
mod n1493_longest_subarray_of_1s_after_deleting_one_element;
mod n1498_number_of_subsequences_that_satisfy_the_given_sum_condition;
mod n1502_can_make_arithmetic_progression_from_sequence;
mod n1514_path_with_maximum_probability;
mod n1519_number_of_nodes_in_the_sub_tree_with_the_same_label;
mod n1523_count_odd_numbers_in_an_interval_range;
mod n1529_minimum_suffix_flips;
mod n1531_string_compression_ii;
mod n1535_find_the_winner_of_an_array_game;
mod n1539_kth_missing_positive_number;
mod n1544_make_the_string_great;
mod n1547_minimum_cost_to_cut_a_stick;
mod n1557_minimum_number_of_vertices_to_reach_all_nodes;
mod n1561_maximum_number_of_coins_you_can_get;
mod n1569_number_of_ways_to_reorder_array_to_get_same_bst;
mod n1572_matrix_diagonal_sum;
mod n1575_count_all_possible_routes;
mod n1578_minimum_time_to_make_rope_colorful;
mod n1579_remove_max_number_of_edges_to_keep_graph_fully_traversable;
mod n1582_special_positions_in_a_binary_matrix;
mod n1584_min_cost_to_connect_all_points;
mod n1601_maximum_number_of_achievable_transfer_requests;
mod n1603_design_parking_system;
mod n1609_even_odd_tree;
mod n1614_maximum_nesting_depth_of_the_parentheses;
mod n1626_best_team_with_no_conflicts;
mod n1627_graph_connectivity_with_threshold;
mod n1630_arithmetic_subarrays;
mod n1631_path_with_minimum_effort;
mod n1639_number_of_ways_to_form_a_target_string_given_a_dictionary;
mod n1642_furthest_building_you_can_reach;
mod n1647_minimum_deletions_to_make_character_frequencies_unique;
mod n1657_determine_if_two_strings_are_close;
mod n1658_minimum_operations_to_reduce_x_to_zero;
mod n1662_check_if_two_string_arrays_are_equivalent;
mod n1669_merge_in_between_linked_lists;
mod n1670_design_front_middle_back_queue;
mod n1671_minimum_number_of_removals_to_make_mountain_array;
mod n1675_minimize_deviation_in_array;
mod n1679_max_number_of_k_sum_pairs;
mod n1685_sum_of_absolute_differences_in_a_sorted_array;
mod n1697_checking_existence_of_edge_length_limited_paths;
mod n1700_number_of_students_unable_to_eat_lunch;
mod n1704_determine_if_string_halves_are_alike;
mod n1706_where_will_the_ball_fall;
mod n1716_calculate_money_in_leetcode_bank;
mod n1721_swapping_nodes_in_a_linked_list;
mod n1727_largest_submatrix_with_rearrangements;
mod n1732_find_the_highest_altitude;
mod n1738_find_kth_largest_xor_coordinate_value;
mod n1743_restore_the_array_from_adjacent_pairs;
mod n1744_can_you_eat_your_favorite_candy_on_your_favorite_day;
mod n1745_palindrome_partitioning_iv;
mod n1750_minimum_length_of_string_after_deleting_similar_ends;
mod n1751_maximum_number_of_events_that_can_be_attended_ii;
mod n1759_count_number_of_homogenous_substrings;
mod n1768_merge_strings_alternately;
mod n1793_maximum_score_of_a_good_subarray;
mod n1799_maximize_score_after_n_operations;
mod n1802_maximum_value_at_a_given_index_in_a_bounded_array;
mod n1814_count_nice_pairs_in_an_array;
mod n1822_sign_of_the_product_of_an_array;
mod n1833_maximum_ice_cream_bars;
mod n1834_single_threaded_cpu;
mod n1838_frequency_of_the_most_frequent_element;
mod n1845_seat_reservation_manager;
mod n1846_maximum_element_after_decreasing_and_rearranging;
mod n1856_maximum_subarray_min_product;
mod n1857_largest_color_value_in_a_directed_graph;
mod n1870_minimum_speed_to_arrive_on_time;
mod n1877_minimize_maximum_pair_sum_in_array;
mod n1882_process_tasks_using_servers;
mod n1887_reduction_operations_to_make_the_array_elements_equal;
mod n1889_minimum_space_wasted_from_packaging;
mod n1913_maximum_product_difference_between_two_pairs;
mod n1921_eliminate_maximum_number_of_monsters;
mod n1926_nearest_exit_from_entrance_in_maze;
mod n1930_unique_length_3_palindromic_subsequences;
mod n1962_remove_stones_to_minimize_the_total;
mod n1964_find_the_longest_valid_obstacle_course_at_each_position;
mod n1970_last_day_where_you_can_still_cross;
mod n1971_find_if_path_exists_in_graph;
mod n1980_find_unique_binary_string;
mod n1992_find_all_groups_of_farmland;
mod n1998_gcd_sort_of_an_array;
mod n2009_minimum_number_of_operations_to_make_array_continuous;
mod n2024_maximize_the_confusion_of_an_exam;
mod n2050_parallel_courses_iii;
mod n2073_time_needed_to_buy_tickets;
mod n2090_k_radius_subarray_averages;
mod n2092_find_all_people_with_secret;
mod n2095_delete_the_middle_node_of_a_linked_list;
mod n2100_find_good_days_to_rob_the_bank;
mod n2101_detonate_the_maximum_bombs;
mod n2108_find_first_palindromic_string_in_the_array;
mod n2125_number_of_laser_beams_in_a_bank;
mod n2130_maximum_twin_sum_of_a_linked_list;
mod n2131_longest_palindrome_by_concatenating_two_letter_words;
mod n2136_earliest_possible_day_of_full_bloom;
mod n2140_solving_questions_with_brainpower;
mod n2147_number_of_ways_to_divide_a_long_corridor;
mod n2163_minimum_difference_in_sums_after_removal_of_elements;
mod n2187_minimum_time_to_complete_trips;
mod n2215_find_the_difference_of_two_arrays;
mod n2218_maximum_value_of_k_coins_from_piles;
mod n2225_find_players_with_zero_or_one_losses;
mod n2244_minimum_rounds_to_complete_all_tasks;
mod n2246_longest_path_with_different_adjacent_characters;
mod n2251_number_of_flowers_in_full_bloom;
mod n2256_minimum_average_difference;
mod n2264_largest_3_same_digit_number_in_string;
mod n2265_count_nodes_equal_to_average_of_subtree;
mod n2272_substring_with_largest_variance;
mod n2279_maximum_bags_with_full_capacity_of_rocks;
mod n2300_successful_pairs_of_spells_and_potions;
mod n2302_count_subarrays_with_score_less_than_k;
mod n2305_fair_distribution_of_cookies;
mod n2306_naming_a_company;
mod n2316_count_unreachable_pairs_of_nodes_in_an_undirected_graph;
mod n2328_number_of_increasing_paths_in_a_grid;
mod n2336_smallest_number_in_infinite_set;
mod n2348_number_of_zero_filled_subarrays;
mod n2352_equal_row_and_column_pairs;
mod n2353_design_a_food_rating_system;
mod n2359_find_closest_node_to_given_two_nodes;
mod n2360_longest_cycle_in_a_graph;
mod n2366_minimum_replacements_to_sort_the_array;
mod n2369_check_if_there_is_a_valid_partition_for_the_array;
mod n2370_longest_ideal_subsequence;
mod n2385_amount_of_time_for_binary_tree_to_be_infected;
mod n2389_longest_subsequence_with_limited_sum;
mod n2390_removing_stars_from_a_string;
mod n2391_minimum_amount_of_time_to_collect_garbage;
mod n2401_longest_nice_subarray;
mod n2402_meeting_rooms_iii;
mod n2405_optimal_partition_of_string;
mod n2421_number_of_good_paths;
mod n2433_find_the_original_array_of_prefix_xor;
mod n2439_minimize_maximum_of_array;
mod n2444_count_subarrays_with_fixed_bounds;
mod n2448_minimum_cost_to_make_array_equal;
mod n2460_apply_operations_to_an_array;
mod n2461_maximum_sum_of_distinct_subarrays_with_length_k;
mod n2462_total_cost_to_hire_k_workers;
mod n2465_number_of_distinct_averages;
mod n2466_count_ways_to_build_good_strings;
mod n2467_most_profitable_path_in_a_tree;
mod n2468_split_message_based_on_limit;
mod n2469_convert_the_temperature;
mod n2470_number_of_subarrays_with_lcm_equal_to_k;
mod n2471_minimum_number_of_operations_to_sort_a_binary_tree_by_level;
mod n2472_maximum_number_of_non_overlapping_palindrome_substrings;
mod n2475_number_of_unequal_triplets_in_array;
mod n2476_closest_nodes_queries_in_a_binary_search_tree;
mod n2477_minimum_fuel_cost_to_report_to_the_capital;
mod n2478_number_of_beautiful_partitions;
mod n2482_difference_between_ones_and_zeros_in_row_and_column;
mod n2483_minimum_penalty_for_a_shop;
mod n2485_find_the_pivot_integer;
mod n2486_append_characters_to_string_to_make_subsequence;
mod n2487_remove_nodes_from_linked_list;
mod n2488_count_subarrays_with_median_k;
mod n2490_circular_sentence;
mod n2491_divide_players_into_teams_of_equal_skill;
mod n2492_minimum_score_of_a_path_between_two_cities;
mod n2500_delete_greatest_value_in_each_row;
mod n2501_longest_square_streak_in_an_array;
mod n2502_design_memory_allocator;
mod n2503_maximum_number_of_points_from_grid_queries;
mod n2506_count_pairs_of_similar_strings;
mod n2507_smallest_value_after_replacing_with_sum_of_prime_factors;
mod n2509_cycle_length_queries_in_a_tree;
mod n2515_shortest_distance_to_target_string_in_a_circular_array;
mod n2516_take_k_of_each_character_from_left_and_right;
mod n2518_number_of_great_partitions;
mod n2520_count_the_digits_that_divide_a_number;
mod n2521_distinct_prime_factors_of_product_of_array;
mod n2522_partition_string_into_substrings_with_values_at_most_k;
mod n2523_closest_prime_numbers_in_range;
mod n2525_categorize_box_according_to_criteria;
mod n2526_find_consecutive_integers_from_a_data_stream;
mod n2527_find_xor_beauty_of_array;
mod n2528_maximize_the_minimum_powered_city;
mod n2529_maximum_count_of_positive_integer_and_negative_integer;
mod n2530_maximal_score_after_applying_k_operations;
mod n2531_make_number_of_distinct_characters_equal;
mod n2532_time_to_cross_a_bridge;
mod n2535_difference_between_element_sum_and_digit_sum_of_an_array;
mod n2536_increment_submatrices_by_one;
mod n2537_count_the_number_of_good_subarrays;
mod n2540_minimum_common_value;
mod n2542_maximum_subsequence_score;
mod n2551_put_marbles_in_bags;
mod n2586_count_the_number_of_vowel_strings_in_range;
mod n2609_find_the_longest_balanced_substring_of_a_binary_string;
mod n2610_convert_an_array_into_a_2d_array_with_conditions;
mod n2611_mice_and_cheese;
mod n2616_minimize_the_maximum_difference_of_pairs;
mod n2642_design_graph_with_shortest_path_calculator;
mod n2670_find_the_distinct_difference_array;
mod n2671_frequency_tracker;
mod n2672_number_of_adjacent_elements_with_the_same_color;
mod n2673_make_costs_of_paths_equal_in_a_binary_tree;
mod n2682_find_the_losers_of_the_circular_game;
mod n2683_neighboring_bitwise_xor;
mod n2684_maximum_number_of_moves_in_a_grid;
mod n2685_count_the_number_of_complete_components;
mod n2696_minimum_string_length_after_removing_substrings;
mod n2697_lexicographically_smallest_palindrome;
mod n2698_find_the_punishment_number_of_an_integer;
mod n2706_buy_two_chocolates;
mod n2707_extra_characters_in_a_string;
mod n2708_maximum_strength_of_a_group;
mod n2709_greatest_common_divisor_traversal;
mod n2710_remove_trailing_zeros_from_a_string;
mod n2711_difference_of_number_of_distinct_values_on_diagonals;
mod n2712_minimum_cost_to_make_all_characters_equal;
mod n2713_maximum_strictly_increasing_cells_in_a_matrix;
mod n2716_minimize_string_length;
mod n2717_semi_ordered_permutation;
mod n2718_sum_of_matrix_after_queries;
mod n2742_painting_the_walls;
mod n2770_maximum_number_of_jumps_to_reach_the_last_index;
mod n2785_sort_vowels_in_a_string;
mod n2807_insert_greatest_common_divisors_in_linked_list;
mod n2815_max_pair_sum_in_an_array;
mod n2816_double_a_number_represented_as_a_linked_list;
mod n2817_minimum_absolute_difference_between_elements_with_constraint;
mod n2818_apply_operations_to_maximize_score;
mod n2849_determine_if_a_cell_is_reachable_at_a_given_time;
mod n2864_maximum_odd_binary_number;
mod n2870_minimum_number_of_operations_to_make_array_empty;
mod n2909_minimum_sum_of_mountain_triplets_ii;
mod n2910_minimum_number_of_groups_to_create_a_valid_assignment;
mod n2911_minimum_changes_to_make_k_semi_palindromes;
mod n2913_subarrays_distinct_element_sum_of_squares_i;
mod n2914_minimum_number_of_changes_to_make_binary_string_beautiful;
mod n2915_length_of_the_longest_subsequence_that_sums_to_target;
mod n2916_subarrays_distinct_element_sum_of_squares_ii;
mod n2917_find_the_k_or_of_an_array;
mod n2918_minimum_equal_sum_of_two_arrays_after_replacing_zeros;
mod n2919_minimum_increment_operations_to_make_array_beautiful;
mod n2920_maximum_points_after_collecting_coins_from_all_nodes;
mod n2923_find_champion_i;
mod n2924_find_champion_ii;
mod n2925_maximum_score_after_applying_operations_on_a_tree;
mod n2926_maximum_balanced_subsequence_sum;
mod n2928_distribute_candies_among_children_i;
mod n2929_distribute_candies_among_children_ii;
mod n2930_number_of_strings_which_can_be_rearranged_to_contain_substring;
mod n2931_maximum_spending_after_buying_items;
mod n2932_maximum_strong_pair_xor_i;
mod n2933_high_access_employees;
mod n2934_minimum_operations_to_maximize_last_elements_in_arrays;
mod n2935_maximum_strong_pair_xor_ii;
mod n2937_make_three_strings_equal;
mod n2938_separate_black_and_white_balls;
mod n2939_maximum_xor_product;
mod n2942_find_words_containing_character;
mod n2943_maximize_area_of_square_hole_in_grid;
mod n2944_minimum_number_of_coins_for_fruits;
mod n2946_matrix_similarity_after_cyclic_shifts;
mod n2947_count_beautiful_substrings_i;
mod n2948_make_lexicographically_smallest_array_by_swapping_elements;
mod n2949_count_beautiful_substrings_ii;
mod n2956_find_common_elements_between_two_arrays;
mod n2957_remove_adjacent_almost_equal_characters;
mod n2958_length_of_longest_subarray_with_at_most_k_frequency;
mod n2959_number_of_possible_sets_of_closing_branches;
mod n2960_count_tested_devices_after_test_operations;
mod n2961_double_modular_exponentiation;
mod n2962_count_subarrays_where_max_element_appears_at_least_k_times;
mod n2963_count_the_number_of_good_partitions;
mod n2970_count_the_number_of_incremovable_subarrays_i;
mod n2971_find_polygon_with_the_largest_perimeter;
mod n2972_count_the_number_of_incremovable_subarrays_ii;
mod n2973_find_number_of_coins_to_place_in_tree_nodes;
mod n2974_minimum_number_game;
mod n2975_maximum_square_area_by_removing_fences_from_a_field;
mod n2976_minimum_cost_to_convert_string_i;
mod n2977_minimum_cost_to_convert_string_ii;
mod n2980_check_if_bitwise_or_has_trailing_zeros;
mod n2981_find_longest_special_substring_that_occurs_thrice_i;
mod n2982_find_longest_special_substring_that_occurs_thrice_ii;
mod n2996_smallest_missing_integer_greater_than_sequential_prefix_sum;
mod n2997_minimum_number_of_operations_to_make_array_xor_equal_to_k;
mod n2998_minimum_number_of_operations_to_make_x_and_y_equal;
mod n3000_maximum_area_of_longest_diagonal_rectangle;
mod n3001_minimum_moves_to_capture_the_queen;
mod n3002_maximum_size_of_a_set_after_removals;
mod n3005_count_elements_with_maximum_frequency;
mod n3006_find_beautiful_indices_in_the_given_array_i;
mod n3007_maximum_number_that_sum_of_the_prices_is_less_than_or_equal_to_k;
mod n3008_find_beautiful_indices_in_the_given_array_ii;
mod n3010_divide_an_array_into_subarrays_with_minimum_cost_i;
mod n3011_find_if_array_can_be_sorted;
mod n3012_minimize_length_of_array_using_operations;
mod n3013_divide_an_array_into_subarrays_with_minimum_cost_ii;
mod n3014_minimum_number_of_pushes_to_type_word_i;
mod n3015_count_the_number_of_houses_at_a_certain_distance_i;
mod n3016_minimum_number_of_pushes_to_type_word_ii;
mod n3101_count_alternating_subarrays;
mod n3102_minimize_manhattan_distances;
