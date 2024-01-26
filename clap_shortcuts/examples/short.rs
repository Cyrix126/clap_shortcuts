use clap::Parser;
use clap_shortcuts::ShortCuts;
use clap_shortcuts_derive::ShortCuts;
fn print(a: &str) {
    println!("{a}")
}
#[derive(ShortCuts)]
#[shortcut(values(name = "shortcut_a", func = "print(&self.a.as_str())"))]
struct Test {
    a: String,
}
#[derive(Parser)]
struct Cli {
    #[clap(flatten)]
    shortcut: ShortCutArgTest,
}

fn main() {
    let cli = Cli::parse();
    let test = Test {
        a: String::from("ok"),
    };
    if let Some(s) = cli.shortcut.shortcut_ref {
        test.shortcut_ref(&s, ()).unwrap();
    }
}
