use hltas::{HLTAS, types::{self, FrameBulk, Line}};
use std::{error::Error, num::NonZeroU32};

pub fn run(config: Config, hltas: HLTAS) -> Result<(), Box<dyn Error>> {
    let cleaned_up = no_dupe_framebulks(hltas);
    println!("{:#?}", cleaned_up);

    Ok(())
}

fn no_dupe_framebulks(hltas: HLTAS) -> HLTAS {
    // better way to do this?
    let prev_framebulk = {
        let mut line_found: Option<(usize, &types::FrameBulk)> = None;
        
        for (i, line) in hltas.lines.iter().enumerate() {
            if let Line::FrameBulk(bulk) = line {
                line_found = Some((i, bulk));
                break;
            }
        }

        line_found
    };

    if prev_framebulk.is_none() {
        return hltas;
    }
    
    let first_framebulk_index = prev_framebulk.unwrap().0;
    let prev_framebulk = prev_framebulk.unwrap().1;

    let mut dupe_free_hltas = HLTAS::default();
    dupe_free_hltas.properties = hltas.properties.clone();

    for line in &hltas.lines[first_framebulk_index..] {
        if let Line::FrameBulk(bulk) = line {
            if bulk.action_keys == prev_framebulk.action_keys &&
                bulk.movement_keys == prev_framebulk.movement_keys &&
                bulk.action_keys == prev_framebulk.action_keys &&
                bulk.pitch == prev_framebulk.pitch &&
                bulk.console_command == prev_framebulk.console_command
             {
                // dupe found
                // TODO, better way of doing this
                let mut bulk_copy = bulk.clone();
                let total_framecount = bulk.frame_count.get() + prev_framebulk.frame_count.get();
                // the total came from nonzerou32 orignally so its fine
                bulk_copy.frame_count = NonZeroU32::new(total_framecount).unwrap();
                dupe_free_hltas.lines.push(Line::FrameBulk(bulk_copy));
            }
        } else {
            dupe_free_hltas.lines.push(line.clone());
        }
    }

    dupe_free_hltas
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
    
        Ok(Config { filename, output_name })
    }
}
