/*
    Copyright 2022 Helsing GmbH

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

use turtlefmt::{format_turtle, FormatOptions};

#[test]
fn test_format() {
    let input = include_str!("from.simple.ttl");
    let expected = include_str!("to.simple.ttl");
    assert_eq!(
        format_turtle(input, &FormatOptions::default()).unwrap(),
        expected
    );
}

#[test]
fn test_stable() {
    let file = include_str!("to.simple.ttl");
    assert_eq!(
        format_turtle(file, &FormatOptions::default()).unwrap(),
        file
    );
}

#[test]
fn test_format_default_inverted() {
    let input = include_str!("from.simple.ttl");
    let expected = include_str!("to.simple.default_inverted.ttl");
    let format_options = FormatOptions {
        indentation: 2,
        align_prefix_iris: true,
        unify_comment_indents: false,
        sort_subjects: true,
        sort_predicates: true,
        sort_objects: true,
        subject_dot_on_new_line: true,
        first_predicate_on_new_line: true,
        first_object_on_new_line: true,
        single_object_on_new_line: true,
        objects_on_separate_lines: true,
        collection_item_on_new_line: true,
        blank_node_predicates_on_separate_lines: true,
    };
    assert_eq!(format_turtle(input, &format_options).unwrap(), expected);
}
