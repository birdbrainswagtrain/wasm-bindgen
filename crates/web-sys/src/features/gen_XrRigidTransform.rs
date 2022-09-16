#![allow(unused_imports)]
use super::*;
use wasm_bindgen::prelude::*;
#[cfg(web_sys_unstable_apis)]
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = XRRigidTransform , typescript_type = "XRRigidTransform")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `XrRigidTransform` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRigidTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub type XrRigidTransform;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomPointReadOnly")]
    # [wasm_bindgen (structural , method , getter , js_class = "XRRigidTransform" , js_name = position)]
    #[doc = "Getter for the `position` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform/position)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`, `XrRigidTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn position(this: &XrRigidTransform) -> DomPointReadOnly;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomPointReadOnly")]
    # [wasm_bindgen (structural , method , getter , js_class = "XRRigidTransform" , js_name = orientation)]
    #[doc = "Getter for the `orientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform/orientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointReadOnly`, `XrRigidTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn orientation(this: &XrRigidTransform) -> DomPointReadOnly;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "XRRigidTransform" , js_name = matrix)]
    #[doc = "Getter for the `matrix` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform/matrix)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRigidTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn matrix(this: &XrRigidTransform) -> Vec<f32>;
    #[cfg(web_sys_unstable_apis)]
    # [wasm_bindgen (structural , method , getter , js_class = "XRRigidTransform" , js_name = inverse)]
    #[doc = "Getter for the `inverse` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform/inverse)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRigidTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn inverse(this: &XrRigidTransform) -> XrRigidTransform;
    #[cfg(web_sys_unstable_apis)]
    #[wasm_bindgen(catch, constructor, js_class = "XRRigidTransform")]
    #[doc = "The `new XrRigidTransform(..)` constructor, creating a new instance of `XrRigidTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform/XRRigidTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `XrRigidTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new() -> Result<XrRigidTransform, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, constructor, js_class = "XRRigidTransform")]
    #[doc = "The `new XrRigidTransform(..)` constructor, creating a new instance of `XrRigidTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform/XRRigidTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `XrRigidTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_position(position: &DomPointInit) -> Result<XrRigidTransform, JsValue>;
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomPointInit")]
    #[wasm_bindgen(catch, constructor, js_class = "XRRigidTransform")]
    #[doc = "The `new XrRigidTransform(..)` constructor, creating a new instance of `XrRigidTransform`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/XRRigidTransform/XRRigidTransform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `XrRigidTransform`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/web-sys/unstable-apis.html)*"]
    pub fn new_with_position_and_orientation(
        position: &DomPointInit,
        orientation: &DomPointInit,
    ) -> Result<XrRigidTransform, JsValue>;
}
