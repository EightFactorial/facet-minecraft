//! [`Facet`] implementation for [`NbtTape`] and [`NbtTapeItem`].

use facet_core::{Def, Facet, Shape, Type, UserType, ValueVTable, value_vtable};

use super::{NbtItem, NbtListItem, NbtMap};

unsafe impl<'facet> Facet<'facet> for NbtMap<'facet> {
    #[allow(unused_mut, reason = "Not used if the `facet-minecraft` feature is disabled")]
    const SHAPE: &'static Shape = &const {
        let mut builder = Shape::builder_for_sized::<Self>()
            .def(Def::Undefined)
            .type_identifier("NbtMap")
            .ty(Type::User(UserType::Opaque));

        #[cfg(feature = "facet-minecraft")]
        {
            builder = builder.attributes(&[facet_core::ShapeAttribute::Arbitrary("custom")]);
        }

        builder.build()
    };
    const VTABLE: &'static ValueVTable =
        &const { value_vtable!(NbtMap, |f, _opts| write!(f, "NbtMap<'_>")) };
}

// -------------------------------------------------------------------------------------------------

unsafe impl<'facet> Facet<'facet> for NbtItem<'facet> {
    const SHAPE: &'static Shape = &const {
        Shape::builder_for_sized::<Self>()
            .def(Def::Undefined)
            .type_identifier("NbtItem")
            .ty(Type::User(UserType::Opaque))
            .build()
    };
    const VTABLE: &'static ValueVTable =
        &const { value_vtable!(NbtItem, |f, _opts| write!(f, "NbtItem<'_>")) };
}

// -------------------------------------------------------------------------------------------------

unsafe impl<'facet> Facet<'facet> for NbtListItem<'facet> {
    const SHAPE: &'static Shape = &const {
        Shape::builder_for_sized::<Self>()
            .def(Def::Undefined)
            .type_identifier("NbtListItem")
            .ty(Type::User(UserType::Opaque))
            .build()
    };
    const VTABLE: &'static ValueVTable =
        &const { value_vtable!(NbtItem, |f, _opts| write!(f, "NbtListItem<'_>")) };
}
