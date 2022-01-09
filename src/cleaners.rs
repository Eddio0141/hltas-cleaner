//! # Cleaners
//!
//! `cleaners` is a collection of utilities for cleaning up a hltas struct.

use hltas::{types::Line, HLTAS};
use std::num::NonZeroU32;

/// Contains information on what lines changed how.
///
/// * `lines_changed` contains indexes on what lines got modified.
/// * `lines_removed` contains indexes on what lines got removed.
#[derive(Default, Debug, PartialEq)]
pub struct CleanerResult {
    pub lines_changed: Vec<usize>,
    pub lines_removed: Vec<usize>,
}

/// Combines duplicate framebulks together.
///
/// * Returns a `CleanerResult` with information of changed lines.
///
/// # Example
///
/// ```
/// use hltas::HLTAS;
/// use hltas_cleaner::no_dupe_framebulks;
/// use hltas_cleaner::CleanerResult;
///
/// let hltas_before = "\
/// version 1
/// frames
/// s03-------|------|------|0.001|0|-|100
/// s03-------|------|------|0.001|0|-|50
/// // I'm in the way!
/// s03-------|------|------|0.001|0|-|1
/// ";
///
/// let hltas_after = "\
/// version 1
/// frames
/// s03-------|------|------|0.001|0|-|150
/// // I'm in the way!
/// s03-------|------|------|0.001|0|-|1
/// ";
///
/// let mut hltas_before = HLTAS::from_str(hltas_before).unwrap();
/// let hltas_after = HLTAS::from_str(hltas_after).unwrap();
///
/// let remove_result_expected = CleanerResult { lines_changed: vec![0], lines_removed: vec![1] };
/// let remove_result = no_dupe_framebulks(&mut hltas_before);
///
/// assert_eq!(hltas_before, hltas_after);
/// assert_eq!(remove_result_expected, remove_result);
/// ```
pub fn no_dupe_framebulks(hltas: &mut HLTAS) -> CleanerResult {
    if hltas.lines.is_empty() {
        return CleanerResult::default();
    }

    let mut prev_line = &hltas.lines[0];

    // contains total frames and all matches
    let mut framecount_and_index: Vec<(NonZeroU32, Vec<usize>)> = Vec::new();
    let mut prev_dupe = false;

    for (mut i, line) in hltas.lines[1..].iter().enumerate() {
        i += 1;

        let prev_framebulk = match prev_line {
            Line::FrameBulk(bulk) => bulk,
            _ => {
                // better way for the double prev_dupe changes?
                prev_dupe = false;
                prev_line = line;
                continue;
            }
        };

        if let Line::FrameBulk(bulk) = line {
            if bulk.auto_actions == prev_framebulk.auto_actions
                && bulk.movement_keys == prev_framebulk.movement_keys
                && bulk.action_keys == prev_framebulk.action_keys
                && bulk.pitch == prev_framebulk.pitch
                && bulk.console_command == prev_framebulk.console_command
                && bulk.frame_time == prev_framebulk.frame_time
            {
                // dupe found

                // nonzerou32 orignally so no need for error check
                let mut total_framecount = bulk.frame_count.get();
                if !prev_dupe {
                    total_framecount += prev_framebulk.frame_count.get();
                }

                if prev_dupe {
                    let last_item_index = framecount_and_index.len() - 1;

                    let prev_total = &mut framecount_and_index[last_item_index].0;
                    total_framecount += prev_total.get();
                    *prev_total = NonZeroU32::new(total_framecount).unwrap();

                    let index_vec = &mut framecount_and_index[last_item_index].1;

                    index_vec.push(i);
                } else {
                    let total_framecount = NonZeroU32::new(total_framecount).unwrap();
                    framecount_and_index.push((total_framecount, vec![i - 1, i]));
                }

                prev_dupe = true;
            } else {
                prev_dupe = false;
            }
        } else {
            prev_dupe = false;
        }

        prev_line = line;
    }

    let mut lines_changed = Vec::new();
    let mut lines_removed = Vec::new();

    // remove duplicate framebulks and update frame count
    for (count, index) in framecount_and_index.into_iter().rev() {
        let first_index = index[0];

        if let Line::FrameBulk(bulk) = &mut hltas.lines[first_index] {
            bulk.frame_count = count;
            lines_changed.push(first_index);
        }

        for i in index[1..].iter().rev() {
            hltas.lines.remove(*i);
            lines_removed.push(*i);
        }
    }

    lines_changed.reverse();
    lines_removed.reverse();

    CleanerResult {
        lines_changed,
        lines_removed,
    }
}

/// Removes all comments.
///
/// # Example
///
/// ```
/// use hltas::HLTAS;
/// use hltas_cleaner::remove_comments;
/// use hltas_cleaner::CleanerResult;
///
/// let hltas_before = "\
/// version 1
/// frames
/// s03-------|------|------|0.001|0|-|100
/// // blah
/// s03-------|------|------|0.001|0|-|50
/// // I dont need this anymore
/// // zzz
/// s03-------|------|------|0.001|0|-|1
/// // hello?
/// ";
///
/// let hltas_after = "\
/// version 1
/// frames
/// s03-------|------|------|0.001|0|-|100
/// s03-------|------|------|0.001|0|-|50
/// s03-------|------|------|0.001|0|-|1
/// ";
///
/// let mut hltas_before = HLTAS::from_str(hltas_before).unwrap();
/// let hltas_after = HLTAS::from_str(hltas_after).unwrap();
///
/// let remove_result_expected = CleanerResult { lines_changed: Vec::new(), lines_removed: vec![1, 3, 4, 6] };
/// let remove_result = remove_comments(&mut hltas_before);
///
/// assert_eq!(hltas_before, hltas_after);
/// assert_eq!(remove_result_expected, remove_result);
/// ```
pub fn remove_comments(hltas: &mut HLTAS) -> CleanerResult {
    let comment_indexes = hltas
        .lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if matches!(line, Line::Comment(_)) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for i in comment_indexes.iter().rev() {
        hltas.lines.remove(*i);
    }

    CleanerResult {
        lines_removed: comment_indexes,
        ..Default::default()
    }
}
