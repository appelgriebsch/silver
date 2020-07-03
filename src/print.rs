use icons;
use modules;
use sh;
use Segment;

pub fn left_prompt(shell: &str, args: Vec<String>) {
    let mut segments = vec![];
    for arg in args {
        let fields = arg.split(':').collect::<Vec<&str>>();
        if fields.len() < 3 {
            panic!("invalid argument, {}", arg);
        }

        let mut segment = Segment {
            background: fields[1].to_owned(),
            foreground: fields[2].to_owned(),
            value: "".to_owned(),
        };
        modules::handle(fields[0], &mut segment, &fields[3..]);
        segments.push(segment);
    }

    let mut first = true;
    let mut last_segment = Segment {
        background: "black".to_owned(),
        foreground: "white".to_owned(),
        value: String::new(),
    };
    for segment in segments {
        if segment.value.is_empty() {
            continue;
        }

        // if this isn't the first segment, before printing the next segment, separate them
        if !first {
            if segment.background == last_segment.background {
                print_segment(
                    &shell,
                    &segment.background,
                    &last_segment.foreground,
                    &icons::thin_left_separator(),
                );
            } else {
                print_segment(
                    &shell,
                    &segment.background,
                    &last_segment.background,
                    &icons::left_separator(),
                );
            }
        }
        first = false;

        print_segment(
            &shell,
            &segment.background,
            &segment.foreground,
            &format!(" {} ", segment.value),
        );
        last_segment = segment;
    }

    // prints final separator
    print_segment(
        &shell,
        "none",
        &last_segment.background,
        &icons::left_separator(),
    );
    sh::reset_colors(&shell);
}

pub fn right_prompt(shell: &str, args: Vec<String>) {
    let segments = args
        .iter()
        .map(|arg| {
            let fields: Vec<_> = arg.split(':').collect();
            if fields.len() < 3 {
                panic!("invalid argument, {}", arg);
            }

            let mut segment = Segment {
                background: fields[1].to_owned(),
                foreground: fields[2].to_owned(),
                value: "".to_owned(),
            };
            modules::handle(fields[0], &mut segment, &fields[3..]);
            segment
        })
        .filter(|seg| !seg.value.is_empty());

    let mut last_segment = Segment {
        background: "none".to_owned(),
        foreground: "none".to_owned(),
        value: String::new(),
    };
    for segment in segments {
        if segment.background == last_segment.background {
            print_segment(
                &shell,
                &segment.background,
                &last_segment.foreground,
                &icons::thin_right_separator(),
            );
        } else {
            print_segment(
                &shell,
                &last_segment.background,
                &segment.background,
                &icons::right_separator(),
            );
        }

        print_segment(
            &shell,
            &segment.background,
            &segment.foreground,
            &format!(" {} ", segment.value),
        );
        last_segment = segment;
    }

    sh::reset_colors(&shell);
}

fn print_segment(shell: &str, background: &str, foreground: &str, value: &str) {
    print!(
        "{}{}{}",
        sh::escape_background(shell, background),
        sh::escape_foreground(shell, foreground),
        value
    );
}
