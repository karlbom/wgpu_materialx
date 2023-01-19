use std::{borrow::BorrowMut, ops::Deref};

use autocxx::prelude::*;

const _MATERIAL_X_NAMESPACE: &str = "MaterialX_v1_38_7";

include_cpp! {
    #include "MaterialXGenGlsl/GlslSyntax.h"
    #include "MaterialXGenGlsl/GlslShaderGenerator.h"
    safety!(unsafe_ffi)

    generate!("MaterialX_v1_38_7::createDocument")

}

use ffi::MaterialX_v1_38_7 as MaterialX;

fn main() {
    let _document = MaterialX::createDocument();
    // MaterialX::readFromXmlFile(document)
}
