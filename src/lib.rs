use hltas::{
    types::{Line},
    HLTAS,
};
use std::{error::Error, num::NonZeroU32};

pub fn run(_config: Config, hltas: &mut HLTAS) -> Result<(), Box<dyn Error>> {
    no_dupe_framebulks(hltas);

    Ok(())
}

fn no_dupe_framebulks(hltas: &mut HLTAS) {
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

pub struct Config {
    pub filename: String,
    pub output_name: String,
    pub remove_dupe_framebulks: bool,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, &str> {
        let arg_count = 4;

        if args.len() < arg_count {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();
        let output_name = args[2].clone();
        let remove_dupe_framebulks = match args[3].clone().parse::<bool>() {
            Ok(b) => b,
            Err(_) => {
                return Err("failed to convert arg 4 to boolean");
            }
        };

        Ok(Config {
            filename,
            output_name,
            remove_dupe_framebulks,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn framebulk_dupe_test() {
        let content_before = "\
version 1
frames
----------|------|------|0.001|0|-|1
----------|------|------|0.001|1|-|1
";

        let content_after = "\
version 1
frames
----------|------|------|0.001|-|-|2
        ";

        let mut content_before = HLTAS::from_str(content_before).unwrap();
        let content_after = HLTAS::from_str(content_after).unwrap();

        no_dupe_framebulks(&mut content_before);

        assert_ne!(content_before, content_after);
    }

    #[test]
    fn framebulk_dupe_test_2() {
        let content_before = "\
version 1
frames
s03l-D-c--|------|------|0.001|280.1407|-|1
s03l-D-c--|------|------|0.001|280.1407|-|2
s03l-D-c--|------|------|0.001|280.1407|-|3
s03l-D-c--|------|------|0.001|280.1407|-|4
s03l-D-c--|------|------|0.001|280.1407|-|5
s03l-D-cg-|------|------|0.001|280.1407|-|5
// im in the way!
target_yaw velocity_lock
s03l-D-c--|------|------|0.001|280.1407|-|6
save buffer
-------c--|------|------|0.001|-|-|50|weapon_shotgun
-------c--|------|------|0.001|-|-|50|weapon_shotgun
";

        let content_after = "\
version 1
frames
s03l-D-c--|------|------|0.001|280.1407|-|15
s03l-D-cg-|------|------|0.001|280.1407|-|5
// im in the way!
target_yaw velocity_lock
s03l-D-c--|------|------|0.001|280.1407|-|6
save buffer
-------c--|------|------|0.001|-|-|100|weapon_shotgun
        ";

        let mut content_before = HLTAS::from_str(content_before).unwrap();
        let content_after = HLTAS::from_str(content_after).unwrap();

        no_dupe_framebulks(&mut content_before);

        assert_eq!(content_before, content_after);
    }

    #[test]
    fn framebulk_dupe_test_3() {
        let content_before = "\
version 1
frames
----------|------|------|0.001|0|-|1|a
----------|------|------|0.001|0|-|2|a
";

        let content_after = "\
version 1
frames
----------|------|------|0.001|0|-|3|a
";

        let mut content_before = HLTAS::from_str(content_before).unwrap();
        let content_after = HLTAS::from_str(content_after).unwrap();

        no_dupe_framebulks(&mut content_before);

        assert_eq!(content_before, content_after);
    }
}
