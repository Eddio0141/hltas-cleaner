use std::num::NonZeroU32;

use hltas::{HLTAS, types::Line};

pub fn no_dupe_framebulks(hltas: &mut HLTAS) {
    if hltas.lines.len() < 1 {
        return;
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
                prev_line = &line;
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

        prev_line = &line;
    }

    framecount_and_index.reverse();

    // remove duplicate framebulks and update frame count
    for (count, mut index) in framecount_and_index {
        let first_index = index[0];

        if let Line::FrameBulk(bulk) = &mut hltas.lines[first_index] {
            bulk.frame_count = count;
        }

        index.reverse();

        for i in index[..index.len() - 1].iter() {
            hltas.lines.remove(*i);
        }
    }
}