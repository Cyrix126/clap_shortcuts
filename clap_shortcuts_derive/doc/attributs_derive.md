# Derive attributs

## Struct attributs

- ##### `#[shortcut(params = String)]`

Transfer the parameters to method to be able to use functions with args.
Put here every parameters that will be needed for the functions to be executed inside the methods.
If you have two functions with attributs, except if you want the same parameter to be passed, you need to put all the parameters for each function.
Example:
```rust,ignore
#[shortcut(params = "msg: &str, is_true: bool,")]
```

- ##### `#[shortcut(name="String", func="String")]`

Declare a new ShortCut variant. 
The name is the name of the shortcut the user will be able to call as arg with clap.
The func is the function as you would call it in actual code.
If func is using self, the instance of the struct will be passed through.
&self, &mut or self will determine in which of the tree methods the trait ShortCuts offers the shortcut that will execute this function will be present.
Errors from the function can be propagated to the result of the method trait.
If the function has a parameter apart from self, the struct attribut params must be used.


Example:
```rust,ignore
#[shortcut(name="name of shortcut", func="my_function(&self.field_a.field_in_nested_struct, msg, is_true)")]
```
