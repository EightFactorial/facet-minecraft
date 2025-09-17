//! [`Facet`] implementation for [`NbtTape`] and [`NbtTapeItem`].

use facet_core::{Def, Facet, Shape, Type, UserType, ValueVTable, value_vtable};

use super::{NbtTape, NbtTapeItem};

unsafe impl<'facet> Facet<'facet> for NbtTape<'facet> {
    #[allow(unused_mut, reason = "Not used if the `facet-minecraft` feature is disabled")]
    const SHAPE: &'static Shape = &const {
        let mut builder = Shape::builder_for_sized::<Self>()
            .def(Def::Undefined)
            .type_identifier("NbtTape")
            .ty(Type::User(UserType::Opaque));

        #[cfg(feature = "facet-minecraft")]
        {
            builder = builder.attributes(&[facet_core::ShapeAttribute::Arbitrary("custom")]);
        }

        builder.build()
    };
    const VTABLE: &'static ValueVTable =
        &const { value_vtable!(NbtTape, |f, _opts| write!(f, "NbtTape<'_>")) };
}

// -------------------------------------------------------------------------------------------------

unsafe impl Facet<'_> for NbtTapeItem {
    const SHAPE: &'static Shape = &const {
        Shape::builder_for_sized::<Self>()
            .def(Def::Undefined)
            .type_identifier("NbtTapeItem")
            .ty(Type::User(UserType::Opaque))
            .build()
    };
    const VTABLE: &'static ValueVTable =
        &const { value_vtable!(NbtTape, |f, _opts| write!(f, "NbtTapeItem")) };
}
