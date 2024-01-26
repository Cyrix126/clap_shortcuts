use clap::Parser;
use clap_shortcuts::ShortCuts;
use clap_shortcuts_derive::ShortCuts;

fn add_one_pillow(pillows: &mut u8, msg: &str, is_true: bool) {
    if is_true {
        *pillows += 1;
    }
    println!("{}", msg)
}

#[derive(ShortCuts, Default)]
#[shortcut(params = "msg: &str, is_true: bool")]
#[shortcut(values(
    name = "number of pillows",
    func = "println!(\"{}\", &self.bed.pillow)"
))]
#[shortcut(values(
    name = "add a pillow",
    func = "add_one_pillow(&mut self.bed.pillow, msg, is_true)"
))]
struct Bedroom {
    bed: Bed,
}
#[derive(Default)]
struct Bed {
    pillow: u8,
}
#[derive(Parser)]
struct Cli {
    #[clap(flatten)]
    shortcut: ShortCutArgBedroom,
}
fn main() {
    let cli = Cli::parse();
    // if we don't want bedroom to be consumed, we use Rc. And because we want to allow modification, we also wrap it into RefCell.
    // let bedroom = Rc::new(RefCell::new(Bedroom::default()));
    let mut bedroom = Bedroom::default();
    if let Some(s) = cli.shortcut.shortcut_ref {
        bedroom
            .shortcut_ref(&s, ("Number of pillow", true))
            .unwrap();
    }
    if let Some(s) = cli.shortcut.shortcut_mut {
        bedroom.shortcut_mut(&s, ("Add a pillow", true)).unwrap();
    }
}
