use adversary::prelude::*;

fn main() {
    fn f(list: Vec<u32>) -> bool {
        list.iter()
            .copied()
            .try_fold(0u32, |accum, elem| accum.checked_add(elem))
            .is_none_or(|n| n % 10 != 9 || n < 1000)
    }

    // let mut weird_count: u128 = 0;
    // let mut total_count: u128 = 0;
    loop {
        let report = adv::test_runners::run_test_bool(
            f,
            any(),
            &mut adv::rand::thread_rng(),
            adv::test_runners::TestConfig::default(),
        )
        .unwrap_report();

        // total_count += 1;
        if report.simplest_failing_value().iter().sum::<u32>() != 1009 {
            // weird_count += 1;

            // let percent = (weird_count as f64) / (total_count as f64) * 100.0;
            // print!("{percent:.2}% ({weird_count}/{total_count}) - ");
            // if report.simplest_failing_value() != report.original_failing_value() {
            //     println!(
            //         "{:?} -> {:?}",
            //         report.original_failing_value(),
            //         report.simplest_failing_value()
            //     );
            // } else {
            //     println!("{:?}", report.simplest_failing_value());
            // }

            println!(
                "{}",
                report.render::<adv::report::renderer::Plaintext>(|value| format!("{value:?}"))
            );
        }
    }
}
