#[derive(Debug, Clone)]
pub struct Report<T> {
    pub test_name: Option<String>,
    pub panic_message: Option<String>,
    pub passing_runs: u64,
    pub observations: Vec<Observation>,
    // First elem is original failing input, last elem is the step right before
    // the simplest failing input
    pub shrink_steps: Vec<ShrinkStep<T>>,
    pub simplest_failing_input: T,
}

// TODO: split this into its own submodule
pub trait RenderReport {
    fn render<T>(report: &Report<T>) -> String;
}

pub struct Plaintext;
impl RenderReport for Plaintext {
    fn render<T>(_report: &Report<T>) -> String {
        // TODO: do this for real
        "your test failed lol".to_string()
    }
}

impl<T> Report<T> {
    pub fn render<R: RenderReport>(&self) -> String {
        R::render(self)
    }
}

#[derive(Debug, Clone)]
pub struct Observation {
    pub contents: String,
    pub importance: Importance,
}

impl Observation {
    pub fn new(contents: impl Into<String>, importance: Importance) -> Self {
        Self {
            contents: contents.into(),
            importance,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Importance {
    Important,           // Always displayed to the user
    MaybeRelevant,       // Sometimes displayed to the user
    ProbablyUnimportant, // Never displayed to the user
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShrinkStep<T> {
    pub value: T,
    pub just_informational: bool,
    pub test_passed: bool,
}

impl<T> ShrinkStep<T> {
    pub fn new(value: T, just_informational: bool, test_passed: bool) -> Self {
        Self {
            value,
            just_informational,
            test_passed,
        }
    }
}

/*
EXAMPLE REPORT
Can also be output in various formats, like HTML, markdown, text, etc.

Test failed!

Test 'my_test_name' failed after 3,982,552 runs. The simplest failing input found was:
[0, 0, 3]

# Details
Test: my_test_name
Panic message: "Some optional string here, newlines and non-printables escaped"
Number of passing attempts before failure: 3,982,552
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
- FAIL: [948752093874, 94357209348671097, 43768581267384, 2939157349818, 8176518...
- PASS: []
- FAIL: [948752093874, 94357209348671097, 43768581267384, 2939157349818, 8176518...
- FAIL: [948752093874, 94357209348671097, 43768581267384, 2939157349818, 8176518...
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
*/
