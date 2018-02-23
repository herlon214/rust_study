extern crate sum_numbers;

#[test]
fn should_sum() {
  assert_eq!(4, sum_numbers::sum(2, 2));
}