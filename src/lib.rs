mod cleaners;

use hltas::HLTAS;
use std::{error::Error, fs::File};

pub use cleaners::no_dupe_framebulks;

pub fn run(config: Config, hltas: &mut HLTAS) -> Result<(), Box<dyn Error>> {
    if config.remove_dupe_framebulks {
        cleaners::no_dupe_framebulks(hltas);
    }

    let file = File::create(config.output_path)?;
    hltas.to_writer(file)?;

    Ok(())
}

pub struct Config {
    pub file_path: String,
    pub output_path: String,
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
            file_path: filename,
            output_path: output_name,
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

        cleaners::no_dupe_framebulks(&mut content_before);

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

        cleaners::no_dupe_framebulks(&mut content_before);

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

        cleaners::no_dupe_framebulks(&mut content_before);

        assert_eq!(content_before, content_after);
    }

    #[test]
    fn framebulk_dupe_test_4() {
        let content_before = "\
version 1
frames
----------|------|------|0.001|-|-|1
----------|------|------|0.25|-|-|2
----------|------|------|0.001|-|-|3
----------|------|------|0.001|-|-|4
----------|------|------|0.010000001|-|-|5
----------|------|------|0.001|-|-|6
----------|------|------|0.001|-|-|5
";

        let content_after = "\
version 1
frames
----------|------|------|0.001|-|-|1
----------|------|------|0.25|-|-|2
----------|------|------|0.001|-|-|7
----------|------|------|0.010000001|-|-|5
----------|------|------|0.001|-|-|11
";

        let mut content_before = HLTAS::from_str(content_before).unwrap();
        let content_after = HLTAS::from_str(content_after).unwrap();

        cleaners::no_dupe_framebulks(&mut content_before);

        assert_eq!(content_before, content_after);
    }
}
