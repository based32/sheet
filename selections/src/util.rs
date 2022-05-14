#[cfg(test)]
/// Test helper macro that simplifies definition of selections storage state
/// and expectations after executing some actions.
/// For better transparency it has no default selection (`(0, 0)` to `(0, 1)`).
///
/// ```
/// selections_test! {
///     [
///         (0, 3) - (0, 5),
///         (1, 3) - (3, 7),
///         (4, 3) - (5, 7),
///     ],
///     storage -> { storage.insert(Position::new(0, 10, Position::new(5, 8))); },
///     [
///         (0, 3) - (0, 5),
///         (0, 10) - (5, 8),
///     ]
/// }
/// ```
macro_rules! selections_test {
    (
        [$(($left_from:expr, $left_to:expr) - ($right_from:expr, $right_to:expr)),*$(,)?],
        $storage:ident -> {$($body:tt)*},
        [$(
            ($left_from_exp:expr, $left_to_exp:expr) -
            ($right_from_exp:expr, $right_to_exp:expr)
        ),*$(,)?]$(,)?
    ) => {
        let mut $storage = $crate::SelectionStorage::new_empty();

        $($storage.insert(
            $crate::Position::new($left_from, $left_to),
            $crate::Position::new($right_from, $right_to));
        )*

        $($body)*

        let mut iter = $storage.iter_all();
        let expected = [
            $(
                $crate::Selection {
                    from: $crate::Position::new($left_from_exp, $left_to_exp),
                    to: $crate::Position::new($right_from_exp, $right_to_exp),
                    ..::std::default::Default::default()
                },
            )*
        ];
        for right in expected.iter() {
            assert_eq!(iter.next(), Some(right));
        }
        assert!(iter.next().is_none());
    };

    (($left_from:expr, $left_to:expr) - ($right_from:expr, $right_to:expr)) => {};
}

#[cfg(test)]
pub(crate) use selections_test;
