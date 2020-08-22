use property_testing::*;

use proptest::prelude::*;

proptest! {
    #[test]
    fn pt_is_valid_size_each(
        height in f32::EPSILON..1.7,
        width in f32::EPSILON..1.7,
        depth in f32::EPSILON..1.7,
    ){
        let b = Baggage {
            height: height,
            width: width,
            depth: depth,
            ..Baggage::default()
        };
        prop_assume!(height + width + depth < 2.0);
        prop_assert!(b.is_valid());
    }
    #[test]
    fn pt_is_valid_size_sum(
        sum3 in f32::EPSILON..2.0,
        rate_height in f32::EPSILON..1.0-f32::EPSILON,
        rate_width in f32::EPSILON..1.0-f32::EPSILON,
    ){
        let height= sum3*rate_height;
        let width=  (sum3-height) * rate_width;
        let depth = sum3 - height - width;
        let b = Baggage {
            height,
            width,
            depth,
            ..Baggage::default()
        };
        prop_assume!(height <= 1.7 && width <= 1.7 && depth <= 1.7);
        prop_assert!(b.is_valid());
    }
}
