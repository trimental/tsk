use termion::color;

use task;

pub fn list(tasks: &[task::Task]) {
    for (i, task) in tasks.iter().filter(|t| t.id.is_some()).enumerate() {
        let symbol = match task.priority {
            Some(task::Priority::Low) => ". ".to_string(),
            Some(task::Priority::Medium) => {
                format!("{}~{} ", color::Fg(color::Yellow), color::Fg(color::Reset))
            }
            Some(task::Priority::High) => {
                format!("{}!{} ", color::Fg(color::Red), color::Fg(color::Reset))
            }
            None => "  ".to_string(),
        };

        let id = if i % 2 == 0 {
            format!(
                "{}[{}]{}",
                color::Fg(color::Red),
                task.id.unwrap(),
                color::Fg(color::Reset)
            )
        } else {
            format!(
                "{}({}){}",
                color::Fg(color::Magenta),
                task.id.unwrap(),
                color::Fg(color::Reset)
            )
        };

        let title = if i % 2 == 0 {
            format!(
                "{}{}{}",
                color::Fg(color::Blue),
                task.title,
                color::Fg(color::Reset)
            )
        } else {
            format!(
                "{}{}{}",
                color::Fg(color::Cyan),
                task.title,
                color::Fg(color::Reset)
            )
        };

        println!("{} {}{}", id, symbol, title);
    }
}

pub fn list_completed(tasks: &[task::Task]) {
    for task in tasks.iter().filter(|t| t.comp_time.is_some()) {
        let title = format!(
            "{}{}{}",
            color::Fg(color::Yellow),
            task.title,
            color::Fg(color::Reset)
        );
        let comp_time = task.comp_time.unwrap();
        let comp_time = format!(
            "on {color}{}{reset} at {color}{}{reset}",
            comp_time.format("%v"),
            comp_time.format("%I%P:%Mm"),
            color = color::Fg(color::Yellow),
            reset = color::Fg(color::Reset),
        );

        println!("Completed:");
        println!("    {} {}", title, comp_time);
    }
}

pub fn info(task: &task::Task) {
    let title = format!(
        "{}Title:{} {}",
        color::Fg(color::Magenta),
        color::Fg(color::Reset),
        task.title
    );
    println!("{}", title);
    let description = format!(
        "{}Description:{} {}",
        color::Fg(color::Cyan),
        color::Fg(color::Reset),
        task.description
    );
    println!("{}", description);
    if let Some(ref priority) = task.priority {
        let priority = match priority {
            task::Priority::Low => {
                format!("{}low{} ", color::Fg(color::Green), color::Fg(color::Reset))
            }
            task::Priority::Medium => format!(
                "{}medium{} ",
                color::Fg(color::Yellow),
                color::Fg(color::Reset)
            ),
            task::Priority::High => {
                format!("{}high{} ", color::Fg(color::Red), color::Fg(color::Reset))
            }
        };
        println!(
            "{}Priority:{} {}",
            color::Fg(color::Green),
            color::Fg(color::Reset),
            priority
        );
    }
}
