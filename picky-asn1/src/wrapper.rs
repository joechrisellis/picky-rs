use crate::{
    bit_string::BitString,
    date::{GeneralizedTime, UTCTime},
    restricted_string::{IA5String, NumericString, PrintableString, Utf8String},
    tag::Tag,
    Asn1Type,
};
use oid::ObjectIdentifier;
use serde::{de, ser, Deserialize, Serialize};
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

/// Generate a thin ASN1 wrapper type with associated tag
/// and name for serialization and deserialization purpose.
macro_rules! asn1_wrapper {
    (struct $wrapper_ty:ident ( $wrapped_ty:ident ), $tag:expr) => {
        /// Wrapper type
        #[derive(Debug, PartialEq, Clone)]
        pub struct $wrapper_ty(pub $wrapped_ty);

        impls! { $wrapper_ty ( $wrapped_ty ), $tag }
    };
    (auto struct $wrapper_ty:ident ( $wrapped_ty:ident ), $tag:expr) => {
        /// Wrapper type
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
        pub struct $wrapper_ty(pub $wrapped_ty);

        impls! { $wrapper_ty ( $wrapped_ty ), $tag }
    };
    (special tag struct $wrapper_ty:ident < $generic:ident >, $tag:expr) => {
        /// Wrapper type for special tag
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
        pub struct $wrapper_ty<$generic>(pub $generic);

        impls! { $wrapper_ty < $generic >, $tag }
    };
    (auto collection struct $wrapper_ty:ident < T >, $tag:expr) => {
        /// Asn1 wrapper around a collection of elements of the same type.
        #[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
        pub struct $wrapper_ty<T>(
            #[serde(
                serialize_with = "serialize_vec",
                deserialize_with = "deserialize_vec",
                bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>")
            )]
            pub Vec<T>,
        );

        impls! { $wrapper_ty ( Vec < T > ), $tag }
    };
}

macro_rules! impls {
    ($wrapper_ty:ident ( $wrapped_ty:ident ), $tag:expr) => {
        impl $crate::wrapper::Asn1Type for $wrapper_ty {
            const TAG: Tag = $tag;
            const NAME: &'static str = stringify!($wrapper_ty);
        }

        impl From<$wrapped_ty> for $wrapper_ty {
            fn from(wrapped: $wrapped_ty) -> Self {
                Self(wrapped)
            }
        }

        impl Into<$wrapped_ty> for $wrapper_ty {
            fn into(self) -> $wrapped_ty {
                self.0
            }
        }

        impl Deref for $wrapper_ty {
            type Target = $wrapped_ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $wrapper_ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl PartialEq<$wrapped_ty> for $wrapper_ty {
            fn eq(&self, other: &$wrapped_ty) -> bool {
                self.0.eq(other)
            }
        }
    };
    ($wrapper_ty:ident < $generic:ident >, $tag:expr) => {
        impl<$generic> $crate::wrapper::Asn1Type for $wrapper_ty<$generic> {
            const TAG: Tag = $tag;
            const NAME: &'static str = stringify!($wrapper_ty);
        }

        impl<$generic> Default for $wrapper_ty<$generic>
        where
            $generic: Default,
        {
            fn default() -> Self {
                Self($generic::default())
            }
        }

        impl<$generic> From<$generic> for $wrapper_ty<$generic> {
            fn from(wrapped: $generic) -> Self {
                Self(wrapped)
            }
        }

        //-- Into cannot be defined to convert into a generic type (E0119) --

        impl<$generic> Deref for $wrapper_ty<$generic> {
            type Target = $generic;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$generic> DerefMut for $wrapper_ty<$generic> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<$generic> PartialEq<$generic> for $wrapper_ty<$generic>
        where
            $generic: PartialEq,
        {
            fn eq(&self, other: &$generic) -> bool {
                self.0.eq(other)
            }
        }
    };
    ($wrapper_ty:ident ( $wrapped_ty:ident < $generic:ident > ), $tag:expr) => {
        impl<$generic> $crate::wrapper::Asn1Type for $wrapper_ty<$generic> {
            const TAG: Tag = $tag;
            const NAME: &'static str = stringify!($wrapper_ty);
        }

        impl<$generic> From<$wrapped_ty<$generic>> for $wrapper_ty<$generic> {
            fn from(wrapped: $wrapped_ty<$generic>) -> Self {
                Self(wrapped)
            }
        }

        impl<$generic> Into<$wrapped_ty<$generic>> for $wrapper_ty<$generic> {
            fn into(self) -> $wrapped_ty<$generic> {
                self.0
            }
        }

        impl<$generic> Deref for $wrapper_ty<$generic> {
            type Target = $wrapped_ty<$generic>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$generic> DerefMut for $wrapper_ty<$generic> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl<$generic> PartialEq<$wrapped_ty<$generic>> for $wrapper_ty<$generic>
        where
            $generic: PartialEq,
        {
            fn eq(&self, other: &$wrapped_ty<$generic>) -> bool {
                self.0.eq(other)
            }
        }
    };
}

macro_rules! define_special_tag {
    ( $name:ident => $tag:expr ) => {
        asn1_wrapper! { special tag struct $name<T>, $tag }
    };
    ( $( $name:ident => $tag:expr , )+ ) => {
        $( define_special_tag! { $name => $tag } )+
    };
}

asn1_wrapper! { auto struct BitStringAsn1(BitString),               Tag::BIT_STRING }
asn1_wrapper! { auto struct ObjectIdentifierAsn1(ObjectIdentifier), Tag::OID }
asn1_wrapper! { auto struct Utf8StringAsn1(Utf8String),             Tag::UTF8_STRING }
asn1_wrapper! { auto struct NumericStringAsn1(NumericString),       Tag::NUMERIC_STRING }
asn1_wrapper! { auto struct PrintableStringAsn1(PrintableString),   Tag::PRINTABLE_STRING }
asn1_wrapper! { auto struct IA5StringAsn1(IA5String),               Tag::IA5_STRING }
asn1_wrapper! { auto struct UTCTimeAsn1(UTCTime),                   Tag::UTC_TIME }
asn1_wrapper! { auto struct GeneralizedTimeAsn1(GeneralizedTime),   Tag::GENERALIZED_TIME }

asn1_wrapper! { auto collection struct Asn1SequenceOf<T>, Tag::SEQUENCE }
asn1_wrapper! { auto collection struct Asn1SetOf<T>,      Tag::SET }

define_special_tag! {
    ApplicationTag0  => Tag::APP_0,
    ApplicationTag1  => Tag::APP_1,
    ApplicationTag2  => Tag::APP_2,
    ApplicationTag3  => Tag::APP_3,
    ApplicationTag4  => Tag::APP_4,
    ApplicationTag5  => Tag::APP_5,
    ApplicationTag6  => Tag::APP_6,
    ApplicationTag7  => Tag::APP_7,
    ApplicationTag8  => Tag::APP_8,
    ApplicationTag9  => Tag::APP_9,
    ApplicationTag10 => Tag::APP_10,
    ApplicationTag11 => Tag::APP_11,
    ApplicationTag12 => Tag::APP_12,
    ApplicationTag13 => Tag::APP_13,
    ApplicationTag14 => Tag::APP_14,
    ApplicationTag15 => Tag::APP_15,
    ContextTag0      => Tag::CTX_0,
    ContextTag1      => Tag::CTX_1,
    ContextTag2      => Tag::CTX_2,
    ContextTag3      => Tag::CTX_3,
    ContextTag4      => Tag::CTX_4,
    ContextTag5      => Tag::CTX_5,
    ContextTag6      => Tag::CTX_6,
    ContextTag7      => Tag::CTX_7,
    ContextTag8      => Tag::CTX_8,
    ContextTag9      => Tag::CTX_9,
    ContextTag10     => Tag::CTX_10,
    ContextTag11     => Tag::CTX_11,
    ContextTag12     => Tag::CTX_12,
    ContextTag13     => Tag::CTX_13,
    ContextTag14     => Tag::CTX_14,
    ContextTag15     => Tag::CTX_15,
}

fn serialize_vec<S, T>(
    elems: &[T],
    serializer: S,
) -> Result<<S as ser::Serializer>::Ok, <S as ser::Serializer>::Error>
where
    S: ser::Serializer,
    T: Serialize,
{
    use serde::ser::SerializeSeq;

    let mut seq = serializer.serialize_seq(Some(elems.len()))?;
    for e in elems {
        seq.serialize_element(e)?;
    }
    seq.end()
}

fn deserialize_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: de::Deserializer<'de>,
    T: Deserialize<'de>,
{
    struct Visitor<T>(std::marker::PhantomData<T>);

    impl<'de, T> de::Visitor<'de> for Visitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid sequence of T")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(e) = seq.next_element()? {
                vec.push(e);
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_seq(Visitor(std::marker::PhantomData))
}

/// A Vec<u8> wrapper for Asn1 encoding as OctetString.
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Hash, Clone)]
pub struct OctetStringAsn1(#[serde(with = "serde_bytes")] pub Vec<u8>);

type VecU8 = Vec<u8>;
impls! { OctetStringAsn1(VecU8), Tag::OCTET_STRING }

/// A BigInt wrapper for Asn1 encoding.
///
/// Simply use primitive integer types if you don't need big integer.
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Hash, Clone)]
pub struct IntegerAsn1(#[serde(with = "serde_bytes")] pub Vec<u8>);

impls! { IntegerAsn1(VecU8), Tag::INTEGER }

impl IntegerAsn1 {
    pub fn is_positive(&self) -> bool {
        if self.0.len() > 1 && self.0[0] == 0x00 || self.0.is_empty() {
            true
        } else {
            self.0[0] & 0x80 == 0
        }
    }

    pub fn is_negative(&self) -> bool {
        if self.0.len() > 1 && self.0[0] == 0x00 {
            false
        } else if self.0.is_empty() {
            true
        } else {
            self.0[0] & 0x80 != 0
        }
    }

    pub fn as_bytes_be(&self) -> &[u8] {
        if self.0.len() > 1 {
            if self.0[0] == 0x00 {
                &self.0[1..]
            } else {
                &self.0
            }
        } else if self.0.is_empty() {
            &[0]
        } else {
            &self.0
        }
    }

    pub fn as_signed_bytes_be(&self) -> &[u8] {
        if self.0.is_empty() {
            &[0]
        } else {
            &self.0
        }
    }

    pub fn from_signed_bytes_be(bytes: Vec<u8>) -> Self {
        Self::from(bytes)
    }
}

/// A wrapper encoding/decoding only the header of the provided Asn1Wrapper with a length of 0.
///
/// Examples:
/// ```
/// use picky_asn1::wrapper::{ApplicationTag0, HeaderOnly};
/// use serde::{Serialize, Deserialize};
///
/// let tag_only = HeaderOnly::<ApplicationTag0<()>>::default();
/// let buffer = [0xA0, 0x00];
///
/// let encoded = picky_asn1_der::to_vec(&tag_only).expect("couldn't serialize");
/// assert_eq!(
///     encoded,
///     buffer,
/// );
///
/// let decoded: HeaderOnly<ApplicationTag0<()>> = picky_asn1_der::from_bytes(&buffer).expect("couldn't deserialize");
/// assert_eq!(
///     decoded,
///     tag_only,
/// );
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Hash, Clone, Default)]
pub struct HeaderOnly<T: Asn1Type>(
    #[serde(
        serialize_with = "serialize_header_only",
        deserialize_with = "deserialize_header_only",
        bound(serialize = "T: Asn1Type", deserialize = "T: Asn1Type")
    )]
    pub std::marker::PhantomData<T>,
);

impl<T: Asn1Type> Asn1Type for HeaderOnly<T> {
    const TAG: Tag = T::TAG;
    const NAME: &'static str = "HeaderOnly";
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_header_only<S, Phantom>(
    _: &std::marker::PhantomData<Phantom>,
    serializer: S,
) -> Result<<S as ser::Serializer>::Ok, <S as ser::Serializer>::Error>
where
    S: ser::Serializer,
    Phantom: Asn1Type,
{
    serializer.serialize_bytes(&[Phantom::TAG.number(), 0x00][..])
}

fn deserialize_header_only<'de, D, Phantom>(
    deserializer: D,
) -> Result<std::marker::PhantomData<Phantom>, D::Error>
where
    D: de::Deserializer<'de>,
    Phantom: Asn1Type,
{
    struct Visitor<T>(std::marker::PhantomData<T>);

    impl<'de, T> de::Visitor<'de> for Visitor<T>
    where
        T: Asn1Type,
    {
        type Value = std::marker::PhantomData<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a valid header for empty payload")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.len() != 2 {
                return Err(E::invalid_value(
                    de::Unexpected::Other("invalid ASN.1 header length"),
                    &"a valid buffer representing an  ASN.1 header with empty payload (two bytes)",
                ));
            }

            if v[0] != T::TAG.number() {
                return Err(E::invalid_value(
                    de::Unexpected::Other("invalid ASN.1 header: wrong tag"),
                    &"a valid buffer representing an empty ASN.1 header (two bytes) with the expected tag",
                ));
            }

            if v[1] != 0 {
                return Err(E::invalid_value(
                    de::Unexpected::Other("invalid ASN.1 header: bad payload length"),
                    &"a valid buffer representing an empty ASN.1 header (two bytes) with no payload",
                ));
            }

            Ok(std::marker::PhantomData)
        }
    }

    deserializer.deserialize_bytes(Visitor(std::marker::PhantomData))
}

/// A BitString encapsulating things.
///
/// Same as `OctetStringAsn1Container` but using a BitString instead.
///
/// Useful to perform a full serialization / deserialization in one pass
/// instead of using `BitStringAsn1` manually.
///
/// Examples
/// ```
/// use picky_asn1::wrapper::BitStringAsn1Container;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize, Debug, PartialEq)]
/// struct MyType {
///     a: u32,
///     b: u16,
///     c: u16,
/// }
///
/// type MyTypeEncapsulated = BitStringAsn1Container<MyType>;
///
/// let encapsulated: MyTypeEncapsulated = MyType {
///     a: 83910,
///     b: 3839,
///     c: 4023,
/// }.into();
///
/// let buffer = [
///     0x03, 0x10, 0x00, // bit string part
///     0x30, 0x0d, // sequence
///     0x02, 0x03, 0x01, 0x47, 0xc6, // integer a
///     0x02, 0x02, 0x0e, 0xff, // integer b
///     0x02, 0x02, 0x0f, 0xb7, // integer c
/// ];
///
/// let encoded = picky_asn1_der::to_vec(&encapsulated).expect("couldn't serialize");
/// assert_eq!(
///     encoded,
///     buffer,
/// );
///
/// let decoded: MyTypeEncapsulated = picky_asn1_der::from_bytes(&buffer).expect("couldn't deserialize");
/// assert_eq!(
///     decoded,
///     encapsulated,
/// );
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Hash, Clone)]
pub struct BitStringAsn1Container<Encapsulated>(pub Encapsulated);

impls! { BitStringAsn1Container<Encapsulated>, Tag::BIT_STRING }

/// An OctetString encapsulating things.
///
/// Same as `BitStringAsn1Container` but using an OctetString instead.
///
/// Useful to perform a full serialization / deserialization in one pass
/// instead of using `OctetStringAsn1` manually.
///
/// Examples
/// ```
/// use picky_asn1::wrapper::OctetStringAsn1Container;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize, Debug, PartialEq)]
/// struct MyType {
///     a: u32,
///     b: u16,
///     c: u16,
/// }
///
/// type MyTypeEncapsulated = OctetStringAsn1Container<MyType>;
///
/// let encapsulated: MyTypeEncapsulated = MyType {
///     a: 83910,
///     b: 3839,
///     c: 4023,
/// }.into();
///
/// let buffer = [
///     0x04, 0x0F, // octet string part
///     0x30, 0x0d, // sequence
///     0x02, 0x03, 0x01, 0x47, 0xc6, // integer a
///     0x02, 0x02, 0x0e, 0xff, // integer b
///     0x02, 0x02, 0x0f, 0xb7, // integer c
/// ];
///
/// let encoded = picky_asn1_der::to_vec(&encapsulated).expect("couldn't serialize");
/// assert_eq!(
///     encoded,
///     buffer,
/// );
///
/// let decoded: MyTypeEncapsulated = picky_asn1_der::from_bytes(&buffer).expect("couldn't deserialize");
/// assert_eq!(
///     decoded,
///     encapsulated,
/// );
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, PartialOrd, Hash, Clone)]
pub struct OctetStringAsn1Container<Encapsulated>(pub Encapsulated);

impls! { OctetStringAsn1Container<Encapsulated>, Tag::OCTET_STRING }

/// Wrapper for ASN.1 implicits (optionals) fields
///
/// Wrapped type has to implement the Default trait to be deserializable (on deserialization failure
/// a default value is returned).
///
/// Examples:
/// ```
/// use picky_asn1::wrapper::{Implicit, ApplicationTag0};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize, Debug, PartialEq)]
/// struct MyWrapper(u8);
///
/// impl Default for MyWrapper {
///     fn default() -> Self {
///         Self(10)
///     }
/// }
///
/// #[derive(Serialize, Deserialize, Debug, PartialEq)]
/// struct ComplexType {
///     // skip if default to reduce encoded size
///     #[serde(skip_serializing_if = "implicit_field_is_default")]
///     optional_field: Implicit<MyWrapper>,
///     // behind application tag 0 to distinguish from optional_field that is a ASN.1 integer too.
///     explicit_field: ApplicationTag0<u8>,
/// }
///
/// fn implicit_field_is_default(wrapper: &Implicit<MyWrapper>) -> bool {
///     wrapper.0 == MyWrapper::default()
/// }
///
/// let complex_type = ComplexType {
///     optional_field: MyWrapper::default().into(),
///     explicit_field: 5.into(),
/// };
///
/// let buffer = [
///     0x30, 0x05, // sequence
///     // optional field isn't present
///     0xA0, 0x03, 0x02, 0x01, 0x05, // explicit field
/// ];
///
/// let encoded = picky_asn1_der::to_vec(&complex_type).expect("couldn't serialize");
/// assert_eq!(
///     encoded,
///     buffer,
/// );
///
/// let decoded: ComplexType = picky_asn1_der::from_bytes(&buffer).expect("couldn't deserialize");
/// assert_eq!(
///     decoded,
///     complex_type,
/// );
/// ```
#[derive(Debug, PartialEq, PartialOrd, Hash, Clone)]
pub struct Implicit<T>(pub T);

impl<T> From<T> for Implicit<T> {
    fn from(wrapped: T) -> Self {
        Self(wrapped)
    }
}

impl<T> Deref for Implicit<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Implicit<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> PartialEq<T> for Implicit<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        self.0.eq(other)
    }
}

impl<T> Serialize for Implicit<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Implicit<T>
where
    T: Deserialize<'de> + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T> de::Visitor<'de> for Visitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Implicit<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "nothing or a valid DER-encoded T")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                T::deserialize(deserializer).map(Implicit::from)
            }
        }

        match deserializer.deserialize_newtype_struct("Implicit", Visitor(std::marker::PhantomData))
        {
            Err(_) => Ok(Self(T::default())),
            result => result,
        }
    }
}
