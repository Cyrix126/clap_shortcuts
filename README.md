# CLAP_DERIVE

simple crate and derive macro to generate Clap Structs and implementation for structs of the ShortCuts trait which enable to apply a function with any parameters on a field or nested field of the struct from the command line.

The magic behind the derive trait Shortcuts:
It will generate:

- An enum with the trait EnumValue from clap with a variant for every shortcuts.
- A subcommand enum using the enum with EnumValue.
- An implementation of the trait Shortcut<Struct>, matching the Arg and executing the function passed through on the field given with parameters given.

## Usage

The derive macro takes a struct attributs composent of a tuple of 4 elements.
- name of the shortcut that will appear in args.
- the path of the field. Can be a field of the struct or a nested field.
- the name of the function. to propagate errors, put the ? operator on the end of the name.
- parameters that the function will take less the first parameter. for example like this: "msg: &str, is_true: bool"




The method in which the process is happening depend of the borrow type first parameter of the function. The first parameter is considered to be the type of the field given. Because the proc macro can't read the parameters of the real function, the dev user must include the borrow type on the field path.

## Example

### Simple Example:

```rust,ignore
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
```

clap will help the user to enter

```bash,ignore
binary --shortcut_ref shortcut-a
```
The output for this example will be:

```bash,ignore
ok
```

Behind the scene, the code generated will be:
```rust,ignore
            impl clap_shortcuts::ShortCuts<()> for Test {
      fn shortcut_mut(&mut self, shortcut: &impl clap::ValueEnum, params: ()) -> anyhow::Result<()> {
                match &shortcut {
                    _ => anyhow::bail!("This shortcut variant is not mutable, use another method of the trait Shortcut")
                };
                Ok(())
            }
      fn shortcut_ref(&self, shortcut: &impl clap::ValueEnum, params: ()) -> anyhow::Result<()> {
                match &shortcut {
                    &ShortCutsTest::A => print(self.a.as_str()),
                    _ => anyhow::bail!("This shortcut variant is not mutable, use another method of the trait Shortcut")
                }
                Ok(())
            }
      fn shortcut_owned(self, shortcut: &impl clap::ValueEnum, params: ()) -> anyhow::Result<()> {
                match &shortcut {
                    _ => anyhow::bail!("This shortcut variant is not mutable, use another method of the trait Shortcut")
                }
                Ok(())
            }
        }

#[derive(clap::ValueEnum, Clone)]
enum ShortCutsTest {
  A
}
```
