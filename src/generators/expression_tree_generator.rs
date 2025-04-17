use crate::{
    base_types::{
        expressions::{Expression, Operators},
        numbers::{CountdownNumberBaseType, NumberSystem},
    },
    timing::{MySender, caching::CachingTransiever},
};

use super::subset_permutation_generator::SubsetPermutationGenerator;

pub fn generate_tree<
    T: CountdownNumberBaseType,
    N: NumberSystem<T>,
    M: MySender<Expression<T>>,
>(
    source_numbers: &[T],
    number_system: &N,
    results: &mut M,
    operators: &Operators,
) {
    if source_numbers.len() == 1 {
        results.send(Expression::Value(source_numbers[0]));
        return;
    }
    let mut left = CachingTransiever::default();
    let mut right = CachingTransiever::default();
    for mid in 1..(source_numbers.len()) {
        let (l, r) = source_numbers.split_at(mid);
        generate_tree(l, number_system, &mut left, operators);
        generate_tree(r, number_system, &mut right, operators);
        for left_expr in left.as_ref().iter() {
            let left_value = left_expr.get_value();
            for right_expr in right.as_ref().iter() {
                let right_value = right_expr.get_value();
                for oper in *operators {
                    if let Some(a) = oper.apply(
                        number_system,
                        *left_value,
                        *right_value,
                    ) {
                        if a == T::ZERO {
                            continue;
                        }
                        let expr = Expression::Application(
                            oper,
                            Box::new(left_expr.clone()),
                            Box::new(right_expr.clone()),
                            a,
                        );
                        if expr.is_valid() {
                            results.send(expr);
                        }
                    }
                }
            }
        }

        left.as_mut().clear();
        right.as_mut().clear();
    }
}
pub fn find_expressions<
    T: CountdownNumberBaseType,
    N: NumberSystem<T>,
    M: MySender<Expression<T>>,
>(
    source_numbers: Vec<T>,
    number_system: &N,
    target_number: T,
    operators: &Operators,
    sender: &mut M,
) {
    let mut _sender =
        sender.filter(move |a| *a.get_value() == target_number);
    let mut sender_ = _sender.blocked();
    for permutation in SubsetPermutationGenerator::new(source_numbers)
    {
        generate_tree(
            &permutation,
            number_system,
            &mut sender_,
            operators,
        );
    }
    sender.set_done();
}

// #[test]
// fn test() {
//     let number_system = ModularNumberSystem::new(11);
//     let numbers = vec![5, 4, 3, 2, 1];

//     let (first_time, last_time, expressions) =
//         find_expressions::<usize, _, _>(numbers, &number_system, 10, &Operators::ALL);
//     if let Some(ft) = first_time {
//         eprintln!(
//             "First item ({}) found in {ft:?}. All items ({:?}) found in {last_time:?}.",
//             expressions.first().unwrap(),
//             expressions.len()
//         )
//     } else {
//         eprintln!("No items found, verified in {last_time:?}.");
//     }
// }
