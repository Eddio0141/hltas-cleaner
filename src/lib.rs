use hltas::{
    types::{self, FrameBulk, Line},
    HLTAS,
};
use std::{error::Error, num::NonZeroU32, str::Lines};

pub fn run(config: Config, hltas: &mut HLTAS) -> Result<(), Box<dyn Error>> {
    let cleaned_up = no_dupe_framebulks(hltas);
    println!("{:#?}", cleaned_up);

    Ok(())
}

fn no_dupe_framebulks(hltas: &mut HLTAS) {
    // better way to do this?
    let prev_framebulk = {
        let mut line_found: Option<(usize, &types::FrameBulk)> = None;

        for (i, line) in hltas.lines.iter().enumerate() {
            if let Line::FrameBulk(bulk) = line {
                line_found = Some((i, &bulk));
                break;
            }
        }

        line_found
    };

    if prev_framebulk.is_none() {
        return;
    }

    let first_framebulk_index = prev_framebulk.unwrap().0;
    let mut prev_framebulk = prev_framebulk.unwrap().1;

    // contains total frames and all matches
    let mut framecount_and_index: Vec<(NonZeroU32, Vec<usize>)> = Vec::new();

    let mut prev_dupe = false;
    
    for (i, line) in hltas.lines[first_framebulk_index..].iter().enumerate() {
        if let Line::FrameBulk(bulk) = line {
            if bulk.action_keys == prev_framebulk.action_keys
                && bulk.movement_keys == prev_framebulk.movement_keys
                && bulk.action_keys == prev_framebulk.action_keys
                && bulk.pitch == prev_framebulk.pitch
                && bulk.console_command == prev_framebulk.console_command
            {
                // dupe found

                // the total came from nonzerou32 orignally so no need for error check
                let total_framecount = bulk.frame_count.get() + prev_framebulk.frame_count.get();
                let total_framecount = NonZeroU32::new(total_framecount).unwrap();

                if prev_dupe {
                    let last_item_index = framecount_and_index.len() - 1;
                    let index_vec = &mut framecount_and_index[last_item_index].1;

                    index_vec.push(i);
                } else {
                    framecount_and_index.push((total_framecount, vec![i]));
                }

                prev_framebulk = &bulk;
                prev_dupe = true;
            } else {
                prev_dupe = false;
            }
        }
    }

    framecount_and_index.reverse();

    // remove duplicate framebulks and update frame count
    for (count, index) in framecount_and_index {
        let first_index = index[0];

        // it will work
        if let Line::FrameBulk(bulk) = &mut hltas.lines[first_index] {
            bulk.frame_count = count;
        }
        
        for i in index[1..].iter() {
            hltas.lines.remove(*i);
        }
    }
}

pub struct Config {
    pub filename: String,
    pub output_name: String,
}

impl Config {
    pub fn from_args(args: &[String]) -> Result<Config, &str> {
        let arg_count = 3;

        if args.len() < arg_count {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();
        let output_name = args[2].clone();

        Ok(Config {
            filename,
            output_name,
        })
    }
}
