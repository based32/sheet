/// Test helper macro that simplifies definition of selections storage state
/// and expectations after executing some actions.
/// For better transparency it has no default selection.
///
/// # Structure
/// - First arg is initial state and represented as an array of selections;
/// - selection is pair of two positions:
///
///   `(anchor_line, anchor_col)` - `(cursor_line, cursor_col)` OR
///   `(anchor_line, anchor_col)` - `(cursor_line, cursor_col) sticky s_col`
///
///   if the cursor is before the anchor it'll mark
///   it as `backwards`.
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
    // Selection constructor matcher (without sticky)
    (
	@selection
	$anchor_line:expr, $anchor_col:expr,
	    $cursor_line:expr, $cursor_col:expr,
    ) => {
	$crate::Selection::new(
	    $crate::Position::new($anchor_line, $anchor_col),
	    $crate::Position::new($cursor_line, $cursor_col)
	)
    };

    // Selection constructor matcher (with sticky)
    (
	@selection
	$anchor_line:expr, $anchor_col:expr,
	    $cursor_line:expr, $cursor_col:expr,
	sticky $sticky:expr
    ) => {
	$crate::Selection::new(
	    $crate::Position::new($anchor_line, $anchor_col),
	    $crate::Position::new_with_sticky($cursor_line, $cursor_col, $sticky)
	)
    };

    // Macro top-level representation
    (
	[$(
	    ($anchor_line:expr, $anchor_col:expr)
		- ($cursor_line:expr, $cursor_col:expr)
	    $(sticky $sticky:expr)?
	),*$(,)?],
	$storage:ident -> {$($body:tt)*},
	[$($exp_deltas:tt)*],
	[$(
	    ($anchor_line_exp:expr, $anchor_col_exp:expr) -
		($cursor_line_exp:expr, $cursor_col_exp:expr)
	    $(sticky $sticky_exp:expr)?
	),*$(,)?]$(,)?
    ) => {
	let mut $storage = $crate::SelectionStorage::new_empty();

	$($storage.insert(
	    selections_test! {
		@selection
		$anchor_line, $anchor_col,
		$cursor_line, $cursor_col,
		$(sticky $sticky)?
	    });
	)*

	let deltas = { $($body)* };
	let expected_deltas_selections = selections_test! {
	    @deltas_selections[] $($exp_deltas)*
	};

	selections_test! { @deltas_start deltas expected_deltas_selections $($exp_deltas)* }

	let mut selections_iter = $storage.iter_all();
	let expected_selections = [
	    $(
		selections_test! {
		    @selection
		    $anchor_line_exp, $anchor_col_exp,
		    $cursor_line_exp, $cursor_col_exp,
		    $(sticky $sticky_exp)?
		},
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
	($anchor_line:expr, $anchor_col:expr) -
	($cursor_line:expr, $cursor_col:expr)
    ) $($rest:tt)*) => {
	selections_test! { @deltas_selections [
	    $($acc)*
	    $crate::Selection::new(
		$crate::Position::new($anchor_line, $anchor_col),
		$crate::Position::new($cursor_line, $cursor_col)
	    ),
	] $($rest)* }
    };

    // Incrementally build a helper array of selections for expected deltas as some of delta
    // variants require borrowed selections (for `Updated` variant).
    (@deltas_selections [$($acc:tt)*] $(,)? Updated {
	old: ($old_anchor_line:expr, $old_anchor_col:expr)
	    - ($old_cursor_line:expr, $old_cursor_col:expr),
	new: ($new_anchor_line:expr, $new_anchor_col:expr)
	    - ($new_cursor_line:expr, $new_cursor_col:expr)
	    $(,)?
    } $($rest:tt)*) => {
	selections_test! { @deltas_selections [
	    $($acc)*
	    $crate::Selection::new(
		$crate::Position::new($new_anchor_line, $new_anchor_col),
		$crate::Position::new($new_cursor_line, $new_cursor_col)
	    ),
	] $($rest)* }
    };

    // Incrementally build a helper array of selections for expected deltas as some of delta
    // variants require borrowed selections (for `Deleted` variant we do nothing).
    (@deltas_selections [$($acc:tt)*] $(,)? Deleted(
	($anchor_line:expr, $anchor_col:expr) -
	($cursor_line:expr, $cursor_col:expr)
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
		::pretty_assertions::assert_eq!(
		    deltas_iter.next().map($crate::deltas::test_utils::DeltaWeakEq),
		    Some($crate::deltas::test_utils::DeltaWeakEq(right))
		);
	    }
	    assert!(deltas_iter.next().is_none());
	}
    };

    // Incremental builder of array of expected deltas (`Created` variant)
    (@deltas_exp $deltas_pos:ident ($n:expr) [$($acc:tt)*] $(,)? Created(
	($anchor_line:expr, $anchor_col:expr) -
	($cursor_line:expr, $cursor_col:expr)
    ) $($rest:tt)*) => {
	selections_test! { @deltas_exp $deltas_pos ($n + 1) [
	    $($acc)*
	    $crate::SelectionDelta::Created(&$deltas_pos[$n]),
	] $($rest)* }
    };

    // Incremental builder of array of expected deltas (`Deleted` variant)
    (@deltas_exp $_deltas_pos:ident ($n:expr) [$($acc:tt)*] $(,)? Deleted(
	($anchor_line:expr, $anchor_col:expr) -
	($cursor_line:expr, $cursor_col:expr)
    ) $($rest:tt)* ) => {
	selections_test! { @deltas_exp $_deltas_pos ($n) [
	    $($acc)*
	    $crate::SelectionDelta::Deleted($crate::Selection::new(
		$crate::Position::new($anchor_line, $anchor_col),
		$crate::Position::new($cursor_line, $cursor_col),
	    )),
	] $($rest)* }
    };

    // Incremental builder of array of expected deltas (`Updated` variant)
    (@deltas_exp $deltas_pos:ident ($n:expr) [$($acc:tt)*] $(,)? Updated {
	old: ($old_anchor_line:expr, $old_anchor_col:expr)
	    - ($old_cursor_line:expr, $old_cursor_col:expr),
	new: ($new_anchor_line:expr, $new_anchor_col:expr)
	    - ($new_cursor_line:expr, $new_cursor_col:expr)
	$(,)?
    } $($rest:tt)* ) => {
	selections_test! { @deltas_exp $deltas_pos ($n + 1) [
	    $($acc)*
	    $crate::SelectionDelta::Updated{
		old: $crate::Selection::new(
		    $crate::Position::new($old_anchor_line, $old_anchor_col),
		    $crate::Position::new($old_cursor_line, $old_cursor_col),
		),
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
