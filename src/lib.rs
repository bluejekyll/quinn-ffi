extern crate quinn;

use std::mem;

pub struct EndpointFFI(quinn::Endpoint);
pub struct EndpointBuilderFFI<'a>(quinn::EndpointBuilder<'a>);

#[no_mangle]
pub extern "C" fn endpoint_new_builder<'a>() -> *mut EndpointBuilderFFI<'a> {
    let builder = EndpointBuilderFFI(quinn::Endpoint::new());
    Box::into_raw(Box::new(builder))
}

#[no_mangle]
pub extern "C" fn endpoint_builder_listen<'a>(builder: *mut EndpointBuilderFFI<'a>) {
    assert!(!builder.is_null());
    let builder: &mut EndpointBuilderFFI = unsafe { &mut *builder };

    // builder.listen
}

#[no_mangle]
pub extern "C" fn endpoint_builder_bind<'a>(builder: *mut EndpointBuilderFFI<'a>) {
    assert!(!builder.is_null());
    let builder: &mut EndpointBuilderFFI = unsafe { &mut *builder };

    // builder.bind
}

/// Generally this is not needed, as the endpoint_builder is consumed on build
///
/// This is available for failure cases, were the program is failing, and needs to drop an unbuilt endpoint
#[no_mangle]
pub extern "C" fn endpoint_builder_free<'a>(builder: *mut EndpointBuilderFFI<'a>) {
    assert!(!builder.is_null());
    let builder: Box<EndpointBuilderFFI> = unsafe { Box::from_raw(builder) };
    drop(builder);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_and_drop() {
        let endpoint_builder = endpoint_new_builder();
        endpoint_builder_free(endpoint_builder);
    }
}
