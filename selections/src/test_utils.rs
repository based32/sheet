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
        [$($exp_deltas:tt)*],
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
        let expected_deltas_selections = selections_test! { @deltas_selections [] $($exp_deltas)* };

        selections_test! { @deltas_start deltas expected_deltas_selections $($exp_deltas)* }

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
            ::pretty_assertions::assert_eq!(selections_iter.next(), Some(right));
        }
        assert!(selections_iter.next().is_none());
    };

    // Incrementally build a helper array of selections for expected deltas as some of delta
    // variants require borrowed selections (for `Created` variant).
    (@deltas_selections [$($acc:tt)*] $(,)? Created(
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

    // Incrementally build a helper array of selections for expected deltas as some of delta
    // variants require borrowed selections (for `Updated` variant).
    (@deltas_selections [$($acc:tt)*] $(,)? Updated {
        old: ($old_left_from:expr, $old_left_to:expr) - ($old_right_from:expr, $old_right_to:expr),
        new: ($new_left_from:expr, $new_left_to:expr) - ($new_right_from:expr, $new_right_to:expr)
            $(,)?
    } $($rest:tt)*) => {
        selections_test! { @deltas_selections [
            $($acc)*
            $crate::Selection::new(
                $crate::Position::new($new_left_from, $new_left_to),
                $crate::Position::new($new_right_from, $new_right_to)
            ),
        ] $($rest)* }
    };

    // Incrementally build a helper array of selections for expected deltas as some of delta
    // variants require borrowed selections (for `Deleted` variant we do nothing).
    (@deltas_selections [$($acc:tt)*] $(,)? Deleted(
        ($left_from:expr, $left_to:expr) -
        ($right_from:expr, $right_to:expr)
    ) $($rest:tt)*) => {
        selections_test! { @deltas_selections [
            $($acc)*
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
                ::pretty_assertions::assert_eq!(deltas_iter.next(), Some(right));
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
        selections_test! { @deltas_exp $_deltas_pos ($n) [
            $($acc)*
            $crate::SelectionDelta::Deleted(::std::boxed::Box::new($crate::Selection {
                from: $crate::Position::new($left_from, $left_to),
                to: $crate::Position::new($right_from, $right_to),
                ..::std::default::Default::default()
            })),
        ] $($rest)* }
    };

    // Incremental builder of array of expected deltas (`Updated` variant)
    (@deltas_exp $deltas_pos:ident ($n:expr) [$($acc:tt)*] $(,)? Updated {
        old: ($old_left_from:expr, $old_left_to:expr) - ($old_right_from:expr, $old_right_to:expr),
        new: ($new_left_from:expr, $new_left_to:expr) - ($new_right_from:expr, $new_right_to:expr)
        $(,)?
    } $($rest:tt)* ) => {
        selections_test! { @deltas_exp $deltas_pos ($n + 1) [
            $($acc)*
            $crate::SelectionDelta::Updated{
                old: ::std::boxed::Box::new($crate::Selection {
                    from: $crate::Position::new($old_left_from, $old_left_to),
                    to: $crate::Position::new($old_right_from, $old_right_to),
                    ..::std::default::Default::default()
                }),
                new: &$deltas_pos[$n],
            },
        ] $($rest)* }
    };

    // Finalize expected deltas array
    (@deltas_exp $deltas_pos:ident ($n:expr) [$($acc:tt)*] $(,)? ) => {
        [$($acc)*]
    };
}

use std::collections::BTreeMap;

pub(crate) use selections_test;

use crate::LineLength;

#[derive(Debug)]
pub(crate) struct TestLineLengths {
    line_length: BTreeMap<usize, usize>,
}

impl TestLineLengths {
    pub(crate) fn new() -> Self {
        let mut line_lengths = TestLineLengths {
            line_length: Default::default(),
        };
        line_lengths.set(0, 0);
        line_lengths
    }

    pub(crate) fn set(&mut self, line: usize, length: usize) {
        self.line_length.insert(line, length);
    }
}

impl LineLength for TestLineLengths {
    fn get_len(&self, line: usize) -> Option<usize> {
        self.line_length.get(&line).map(|x| *x)
    }

    fn lines_count(&self) -> usize {
        self.line_length.len()
    }
}
