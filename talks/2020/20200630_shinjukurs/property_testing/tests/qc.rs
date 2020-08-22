use property_testing::*;

use quickcheck_macros::quickcheck;

#[quickcheck]
fn qc_search_all_satisfy_item_in_result(bs: Vec<Baggage>, q: Query) -> bool {
    let result = search(&bs, &q);
    bs.into_iter()
        .filter(|b| q.is_satisfied(b))
        .all(|b| result.contains(&b))
}
#[quickcheck]
fn qc_search_any_unsatisfy_item_not_in_result(bs: Vec<Baggage>, q: Query) -> bool {
    let result = search(&bs, &q);
    bs.into_iter()
        .filter(|b| !q.is_satisfied(b))
        .all(|b| !result.contains(&b))
}
// search
// #[quickcheck]
// fn qc_search_all_baggages_satisfy_empty_conditions(bs: Vec<Baggage>) -> bool {
//     let q = Query { ..Query::default() };
//     let result = search(&bs, &q);
//     bs.into_iter().eq(result)
// }
// #[quickcheck]
// fn qc_search_all_results_satisfy_conditions(bs: Vec<Baggage>, q: Query) -> bool {
//     let result = search(&bs, &q);
//     search(&result, &q).into_iter().eq(result)
// }
