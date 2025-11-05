use crate::ast::base::money::*;
use rstest::rstest;

#[rstest]
#[case((1, Denomination::Gold), Denomination::Copper, 1f32 * GOLD_COPPER)]
#[case((10, Denomination::Gold), Denomination::Copper, 10f32 * GOLD_COPPER)]
#[case((20, Denomination::Gold), Denomination::Copper, 20f32 * GOLD_COPPER)]
#[case((20, Denomination::Silver), Denomination::Copper, 20f32 * SILVER_COPPER)]
fn convert_test(
    #[case] (value, from): (i64, Denomination),
    #[case] target: Denomination,
    #[case] expected_result: f32,
) {
    let result = from.exchange(target)(value);
    assert_eq!(
        expected_result, result,
        "Expected {expected_result} = {result}"
    );
}
