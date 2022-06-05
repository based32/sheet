#[cfg(test)]
/// Test helper macro that simplifies definition of selections storage state
/// and expectations after executing some actions.
/// For better transparency it has no default selection (`(0, 0)` to `(0, 1)`).
///
/// # Structure
/// - First arg is initial state and represented as array of selections;
/// - selection is pair of two positions: (from_line, from_col) - (to_line,
///   to_col);
/// - second arg is a selection storage binding -> { code block to execture };
///   (must return deltas to compare next)
/// - third arg is an array of deltas;
/// - delta might be Created(selection), Deleted(selection) or Updated { old:
///   selection, new: selection}, where selection has an already known format of
///   pair of two positions;
/// - fourth arg is a final state to expect represented just like initial state.
///
/// ```
/// selections_test! {
///     [
///         (0, 3) - (0, 5),
///         (1, 3) - (3, 7),
///         (4, 3) - (5, 7),
///     ],
///     storage -> { storage.insert(Position::new(0, 10, Position::new(5, 8))) },
///     [
///         Created((0, 10) - (5 ,8)),
///         Deleted((1, 3) - (3, 7)),
///         Deleted((4, 3) - (5, 7))
///     ],
///     [
///         (0, 3) - (0, 5),
///         (0, 10) - (5, 8),
///     ]
/// }
/// ```
macro_rules! selections_test {
    // Macro top-level representation
    (
        [$(($left_from:expr, $left_to:expr) - ($right_from:expr, $right_to:expr)),*$(,)?],
        $storage:ident -> {$($body:tt)*},
        [$($deltas:tt)*],
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

        let deltas = { $($body)* };
        let expected_deltas_selections = selections_test! { @deltas_selections [] $($deltas)* };

        selections_test! { @deltas_start deltas expected_deltas_selections $($deltas)* }

        let mut selections_iter = $storage.iter_all();
        let expected_selections = [
            $(
                $crate::Selection::new(
                    $crate::Position::new($left_from_exp, $left_to_exp),
                    $crate::Position::new($right_from_exp, $right_to_exp)
                ),
            )*
        ];
        for right in expected_selections.iter() {
            assert_eq!(selections_iter.next(), Some(right));
        }
        assert!(selections_iter.next().is_none());
    };

    // Incrementally build a helper array of selections for expected deltas as some of delta
    // variants require borrowed selections
    (@deltas_selections [$($acc:tt)*] $(,)? $_op:ident(
        ($left_from:expr, $left_to:expr) -
        ($right_from:expr, $right_to:expr)
    ) $($rest:tt)*) => {
        selections_test! { @deltas_selections [
            $($acc)*
            $crate::Selection::new(
                $crate::Position::new($left_from, $left_to),
                $crate::Position::new($right_from, $right_to)
            ),
        ] $($rest)* }
    };

    // Finalize selections helper array builder
    (@deltas_selections [$($acc:tt)*] $(,)?) => {
        [$($acc)*]
    };

    // Section of deltas assertions
    (@deltas_start $deltas_ident:ident $deltas_pos:ident $($rest:tt)*) => {
        {
            let mut deltas_iter = $deltas_ident.into_iter();
            let expected_deltas = selections_test! { @deltas_exp $deltas_pos (0) [] $($rest)* };
            for right in expected_deltas.into_iter() {
                assert_eq!(deltas_iter.next(), Some(right));
            }
            assert!(deltas_iter.next().is_none());
        }
    };

    // Incremental builder of array of expected deltas (`Created` variant)
    (@deltas_exp $deltas_pos:ident ($n:expr) [$($acc:tt)*] $(,)? Created(
        ($left_from:expr, $left_to:expr) -
        ($right_from:expr, $right_to:expr)
    ) $($rest:tt)*) => {
        selections_test! { @deltas_exp $deltas_pos ($n + 1) [
            $($acc)*
            $crate::SelectionDelta::Created(&$deltas_pos[$n]),
        ] $($rest)* }
    };

    // Incremental builder of array of expected deltas (`Deleted` variant)
    (@deltas_exp $_deltas_pos:ident ($n:expr) [$($acc:tt)*] $(,)? Deleted(
        ($left_from:expr, $left_to:expr) -
        ($right_from:expr, $right_to:expr)
    ) $($rest:tt)* ) => {
        selections_test! { @deltas_exp $_deltas_pos ($n + 1) [
            $($acc)*
            $crate::SelectionDelta::Deleted(::std::boxed::Box::new($crate::Selection {
                from: $crate::Position::new($left_from, $left_to),
                to: $crate::Position::new($right_from, $right_to),
                ..::std::default::Default::default()
            })),
        ] $($rest)* }
    };

    // Finalize expected deltas array
    (@deltas_exp $deltas_pos:ident ($n:expr) [$($acc:tt)*] $(,)? ) => {
        [$($acc)*]
    };
}

#[cfg(test)]
pub(crate) use selections_test;
