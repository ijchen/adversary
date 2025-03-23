use crate::report::Report;

use super::ReportRenderer;

use std::fmt::Write as _;

macro_rules! writeln_string {
    ($string:ident$(, $($rest:tt)*)?) => {{
        let _: &mut String = $string;
        writeln!($string$(, $($rest)*)?).expect("writing to a String cannot fail")
    }}
}

macro_rules! write_string {
    ($string:ident$(, $($rest:tt)*)?) => {{
        let _: &mut String = $string;
        write!($string$(, $($rest)*)?).expect("writing to a String cannot fail")
    }}
}

// Converts a number of passing runs to a comma-separated string representing
// the number of *total* runs (passing runs + 1)
//
// Ex: 3982551 -> "3,982,552 runs"
// Ex: 0 -> "1 run"
// Ex: u64::MAX - 1 -> "18,446,744,073,709,551,615 runs"
// Ex: u64::MAX -> "over 18,446,744,073,709,551,615 runs"
fn format_run_count(passing_runs: u64) -> String {
    // TODO: Locale considerations (https://crates.io/crates/num-format)
    fn thousands_separate(n: u64) -> String {
        // From https://stackoverflow.com/a/67834588
        n.to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join(",")
    }

    let runs_maybe_singular = if passing_runs.checked_add(1).is_some_and(|runs| runs == 1) {
        "run"
    } else {
        "runs"
    };

    match passing_runs.checked_add(1) {
        None => format!(
            "over {} {runs_maybe_singular}",
            thousands_separate(passing_runs)
        ),
        Some(total_runs) => format!("{} {runs_maybe_singular}", thousands_separate(total_runs)),
    }
}

// Outputs this section:
// Test 'my_test_name' failed after 3,982,552 runs. The simplest failing input found was:
// [0, 0, 3]
fn failure_summary<T>(output: &mut String, report: &Report<T>, converter: impl Fn(&T) -> String) {
    write_string!(output, "Test ");

    if let Some(test_name) = &report.test_name {
        write_string!(output, "'{}' ", test_name);
    }

    let formatted_run_count = format_run_count(report.passing_runs);
    writeln_string!(
        output,
        "failed after {formatted_run_count}. The simplest failing input found was:",
    );

    writeln_string!(output, "{}", converter(report.simplest_failing_value()));
}

// Outputs the details section
fn details<T>(output: &mut String, report: &Report<T>, converter: impl Fn(&T) -> String) {
    writeln_string!(output, "# Details");

    // Test: my_test_name
    if let Some(test_name) = &report.test_name {
        writeln_string!(output, "Test name: {test_name}");
    }

    if let Some(panic_info) = &report.panic_info {
        // Panic message: "Some optional string here, newlines and non-printables escaped"
        if let Some(panic_message) = &panic_info.message {
            let escaped_panic_message = panic_message; // TODO: escape
            writeln_string!(output, "Panic message: {escaped_panic_message}");
        }

        // Panic location: src/something/whatever/foo.rs:100:24
        if let Some(panic_location) = &panic_info.location {
            writeln_string!(
                output,
                "Panic location: {}:{}:{}",
                panic_location.file,
                panic_location.line,
                panic_location.col
            );
        }
    }

    // Attempts taken to fail: 3,982,552
    let formatted_run_count = format_run_count(report.passing_runs);
    writeln_string!(output, "Attempts taken to fail: {formatted_run_count}");

    // Simplest failing input: [0, 0, 3]
    // TODO: consider trimming if line exceeds certain length
    writeln_string!(
        output,
        "Simplest failing input: {}",
        converter(report.simplest_failing_value())
    );

    // Original failing input: [948752093874, 94357209348671097, 437685812673...
    // TODO: consider trimming if line exceeds certain length
    // TODO: consider different formatting (or even omitting) if steps is empty
    writeln_string!(
        output,
        "Original failing input: {}",
        converter(report.original_failing_value())
    );
    // TODO: observations, ex:
    // Observations:
    // - The vector length was reduced to 3, but lengths under 3 started passing
    // - The value at index 0 probably doesn't matter - it was instantly reduced to the simplest value
    // - The value at index 1 probably doesn't matter - it was instantly reduced to the simplest value
    // - The value at index 2 was reduced to 3, but values under 3 started passing
    // - A second pass over the vector was unable to shrink any elements further.
    // - The test seems to fail consistently - 1,000 runs all failed

    // Full shrinking steps:
    // - FAIL: ...
    // - PASS: ...
    // - Fail: ...
    // ...
    if !report.shrink_steps.is_empty() {
        // TODO: make this configurable
        const STEPS_PER_SIDE: usize = 5;

        // TODO: allow the user to force a full output
        if report.shrink_steps.len() <= STEPS_PER_SIDE * 2 {
            writeln_string!(output, "Full shrinking steps:");

            for step in &report.shrink_steps {
                let passfail = if step.test_passed { "PASS" } else { "FAIL" };
                // TODO: consider trimming if line exceeds certain length
                let value = converter(&step.value);
                writeln_string!(output, "- {passfail}: {value}");
            }
        } else {
            writeln_string!(output, "Shrinking steps (trimmed):");

            for step in &report.shrink_steps[..STEPS_PER_SIDE] {
                let passfail = if step.test_passed { "PASS" } else { "FAIL" };
                // TODO: consider trimming if line exceeds certain length
                let value = converter(&step.value);
                writeln_string!(output, "- {passfail}: {value}");
            }
            writeln_string!(
                output,
                "... ({} steps omitted)",
                report.shrink_steps.len() - STEPS_PER_SIDE * 2
            );
            for step in &report.shrink_steps[report.shrink_steps.len() - STEPS_PER_SIDE..] {
                let passfail = if step.test_passed { "PASS" } else { "FAIL" };
                // TODO: consider trimming if line exceeds certain length
                let value = converter(&step.value);
                writeln_string!(output, "- {passfail}: {value}");
            }
        }
    }
}

// TODO(ichen): colors? maybe even a custom `Terminal` report renderer?

pub struct Plaintext;
impl ReportRenderer for Plaintext {
    type Output = String;
    type Converted = String;

    fn render<T>(report: &Report<T>, converter: impl Fn(&T) -> Self::Converted) -> Self::Output {
        let mut rendered_report = String::new();
        let output = &mut rendered_report;

        failure_summary(output, report, &converter);
        writeln_string!(output);

        details(output, report, &converter);

        assert_eq!(rendered_report.pop(), Some('\n'));

        rendered_report
    }
}

// TODO: update example report based on changes since it was initially drafted
/*
EXAMPLE REPORT
(Can also be output in various formats, like HTML, markdown, text, etc.)

--------------------------------------------------------------------------------
Test 'my_test_name' failed after 3,982,552 runs. The simplest failing input found was:
[0, 0, 3]

# Details
Test: my_test_name
Panic message: "Some optional string here, newlines and non-printables escaped"
Panic location: src/something/whatever/foo.rs:100:24
Attempts taken to fail: 3,982,552
Simplest failing input: [0, 0, 3]
Original failing input: [948752093874, 94357209348671097, 43768581267384, 293...
Observations:
- The vector length was reduced to 3, but lengths under 3 started passing
- The value at index 0 probably doesn't matter - it was instantly reduced to the simplest value
- The value at index 1 probably doesn't matter - it was instantly reduced to the simplest value
- The value at index 2 was reduced to 3, but values under 3 started passing
- A second pass over the vector was unable to shrink any elements further.
- The test seems to fail consistently - 1,000 runs all failed
Full shrinking steps:
- FAIL: [948752093874, 94357209348671097, 43768581267384, 2939157349818, 8176...
- PASS: []
- FAIL: [948752093874, 94357209348671097, 43768581267384, 2939157349818, 8176...
- FAIL: [948752093874, 94357209348671097, 43768581267384, 2939157349818, 8176...
- FAIL: [948752093874, 94357209348671097, 43768581267384, 2939157349818]
- PASS: [948752093874, 94357209348671097]
- FAIL: [948752093874, 94357209348671097, 43768581267384]
- FAIL: [0, 94357209348671097, 43768581267384]
- FAIL: [0, 0, 43768581267384]
- PASS: [0, 0, 0]
- FAIL: [0, 0, 43768581267384]
- FAIL: [0, 0, 21884290633692]
- FAIL: [0, 0, 10942145316846]
- FAIL: [0, 0, 5471072658423]
- FAIL: [0, 0, 2735536329211]
- FAIL: [0, 0, 1367768164605]
- FAIL: [0, 0, 683884082302]
- FAIL: [0, 0, 341942041151]
- FAIL: [0, 0, 170971020575]
- FAIL: [0, 0, 85485510287]
- FAIL: [0, 0, 42742755143]
- FAIL: [0, 0, 21371377571]
- FAIL: [0, 0, 10685688785]
- FAIL: [0, 0, 5342844392]
- FAIL: [0, 0, 2671422196]
- FAIL: [0, 0, 1335711098]
- FAIL: [0, 0, 667855549]
- FAIL: [0, 0, 333927774]
- FAIL: [0, 0, 166963887]
- FAIL: [0, 0, 83481943]
- FAIL: [0, 0, 41740971]
- FAIL: [0, 0, 20870485]
- FAIL: [0, 0, 10435242]
- FAIL: [0, 0, 5217621]
- FAIL: [0, 0, 2608810]
- FAIL: [0, 0, 1304405]
- FAIL: [0, 0, 652202]
- FAIL: [0, 0, 326101]
- FAIL: [0, 0, 163050]
- FAIL: [0, 0, 81525]
- FAIL: [0, 0, 40762]
- FAIL: [0, 0, 20381]
- FAIL: [0, 0, 10190]
- FAIL: [0, 0, 5095]
- FAIL: [0, 0, 2547]
- FAIL: [0, 0, 1273]
- FAIL: [0, 0, 636]
- FAIL: [0, 0, 318]
- FAIL: [0, 0, 159]
- FAIL: [0, 0, 79]
- FAIL: [0, 0, 39]
- FAIL: [0, 0, 19]
- FAIL: [0, 0, 9]
- FAIL: [0, 0, 4]
- PASS: [0, 0, 2]
- FAIL: [0, 0, 3]
--------------------------------------------------------------------------------
*/
