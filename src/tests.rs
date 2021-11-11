use super::*;
use hltas::*;

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

#[test]
fn angle_wrap_test() {
    let content_before = "\
version 1
frames
----------|------|------|0.001|360|-|5
target_yaw 361
target_yaw_override 362 363
s03-------|------|------|0.001|364|-|5
";

    let content_after = "\
version 1
frames
----------|------|------|0.001|0|-|5
target_yaw 1
target_yaw_override 2 3
s03-------|------|------|0.001|4|-|5
";

    let mut content_before = HLTAS::from_str(content_before).unwrap();
    let content_after = HLTAS::from_str(content_after).unwrap();

    cleaners::angle_wrap(&mut content_before);

    assert_eq!(content_before, content_after);
}

#[test]
fn angle_wrap_test2() {
    let content_before = "\
version 1
frames
s03----c--|------|------|0.001|846|-|46|v
";

    let content_after = "\
version 1
frames
s03----c--|------|------|0.001|126|-|46|v
";

    let mut content_before = HLTAS::from_str(content_before).unwrap();
    let content_after = HLTAS::from_str(content_after).unwrap();

    cleaners::angle_wrap(&mut content_before);

    assert_eq!(content_before, content_after);
}
