use intrusive_collections::{Bound, RBTreeLink};

use super::{Position, SelectionStorage};
use crate::Selection;

impl SelectionStorage {
    /// Insert a selection bounded by `from` and `to` positions. If inserted
    /// selection overlaps with an existing one(s) it either will be replaced
    /// (`replace == true`) or merged (`replace == false`).
    pub fn insert(&mut self, from: Position, to: Position, replace: bool) {
        // Search for a possible collision
        let (mut new_from, mut new_to) = (None, None);
        // Check left neighbor
        let mut left_collision_cursor = self.tree.upper_bound_mut(Bound::Included(&from));
        if let Some(left) = left_collision_cursor.get() {
            if left.to >= from {
                // Collision with left neighbor
                if !replace {
                    new_from = Some(left.from.clone());
                }
                left_collision_cursor.remove();
            }
        }
        // Check right neighbor
        let mut right_collision_cursor = self.tree.lower_bound_mut(Bound::Included(&to));
        if let Some(right) = right_collision_cursor.get() {
            if right.from <= to {
                // Collision with right neighbor
                if !replace {
                    new_to = Some(right.to.clone());
                }
                right_collision_cursor.remove();
            }
        }
        if replace {
            self.tree.insert(Box::new(Selection {
                from,
                to,
                link: RBTreeLink::new(),
            }));
        } else {
            self.tree.insert(Box::new(Selection {
                from: new_from.unwrap_or(from),
                to: new_to.unwrap_or(to),
                link: RBTreeLink::new(),
            }));
        }
    }
}
