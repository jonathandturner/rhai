use crate::def_package;
use crate::fn_native::FnPtr;
use crate::plugin::*;

#[cfg(not(feature = "no_object"))]
use crate::engine::make_getter;

def_package!(crate:BasicFnPackage:"Basic Fn functions.", lib, {
    set_exported_fn!(lib, "name", get_fn_name);

    #[cfg(not(feature = "no_object"))]
    set_exported_fn!(lib, make_getter("name"), get_fn_name);
});

#[export_fn]
fn get_fn_name(f: &mut FnPtr) -> ImmutableString {
    f.get_fn_name().clone()
}
