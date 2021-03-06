// This file is generated by rust-protobuf 3.0.0-alpha.2. Do not edit
// .proto file is parsed by protobuf-codegen=3.0.0-alpha.2
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `protos/example.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_0_0_ALPHA_2;

#[derive(PartialEq,Clone,Default)]
#[derive(::serde::Serialize, ::serde::Deserialize)]
pub struct GetRequest {
    // message fields
    pub name: ::std::string::String,
    pub age: i32,
    pub features: ::std::vec::Vec<::std::string::String>,
    // special fields
    #[serde(skip)]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[serde(skip)]
    pub cached_size: ::protobuf::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a GetRequest {
    fn default() -> &'a GetRequest {
        <GetRequest as ::protobuf::Message>::default_instance()
    }
}

impl GetRequest {
    pub fn new() -> GetRequest {
        ::std::default::Default::default()
    }

    // string name = 1;

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // int32 age = 2;

    pub fn get_age(&self) -> i32 {
        self.age
    }

    pub fn clear_age(&mut self) {
        self.age = 0;
    }

    // Param is passed by value, moved
    pub fn set_age(&mut self, v: i32) {
        self.age = v;
    }

    // repeated string features = 3;

    pub fn get_features(&self) -> &[::std::string::String] {
        &self.features
    }

    pub fn clear_features(&mut self) {
        self.features.clear();
    }

    // Param is passed by value, moved
    pub fn set_features(&mut self, v: ::std::vec::Vec<::std::string::String>) {
        self.features = v;
    }

    // Mutable pointer to the field.
    pub fn mut_features(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
        &mut self.features
    }

    // Take field
    pub fn take_features(&mut self) -> ::std::vec::Vec<::std::string::String> {
        ::std::mem::replace(&mut self.features, ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::new();
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &GetRequest| { &m.name },
            |m: &mut GetRequest| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "age",
            |m: &GetRequest| { &m.age },
            |m: &mut GetRequest| { &mut m.age },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "features",
            |m: &GetRequest| { &m.features },
            |m: &mut GetRequest| { &mut m.features },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetRequest>(
            "GetRequest",
            0,
            fields,
        )
    }
}

impl ::protobuf::Message for GetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.name = is.read_string()?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.age = is.read_int32()?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.features)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if self.age != 0 {
            my_size += ::protobuf::rt::value_size(2, self.age, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.features {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if self.age != 0 {
            os.write_int32(2, self.age)?;
        }
        for v in &self.features {
            os.write_string(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn new() -> GetRequest {
        GetRequest::new()
    }

    fn descriptor_static() -> ::protobuf::reflect::MessageDescriptor {
        ::protobuf::reflect::MessageDescriptor::new_generated_2(file_descriptor(), 0)
    }

    fn default_instance() -> &'static GetRequest {
        static instance: GetRequest = GetRequest {
            name: ::std::string::String::new(),
            age: 0,
            features: ::std::vec::Vec::new(),
            unknown_fields: ::protobuf::UnknownFields::new(),
            cached_size: ::protobuf::rt::CachedSize::new(),
        };
        &instance
    }
}

impl ::protobuf::Clear for GetRequest {
    fn clear(&mut self) {
        self.name.clear();
        self.age = 0;
        self.features.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetRequest {
    type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default)]
#[derive(::serde::Serialize, ::serde::Deserialize)]
pub struct GetResponse {
    // message fields
    pub status: ::protobuf::ProtobufEnumOrUnknown<get_response::Status>,
    pub address: ::std::string::String,
    pub city: ::std::string::String,
    pub zipcode: i32,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    #[serde(skip)]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[serde(skip)]
    pub cached_size: ::protobuf::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a GetResponse {
    fn default() -> &'a GetResponse {
        <GetResponse as ::protobuf::Message>::default_instance()
    }
}

impl GetResponse {
    pub fn new() -> GetResponse {
        ::std::default::Default::default()
    }

    // .GetResponse.Status status = 1;

    pub fn get_status(&self) -> get_response::Status {
        self.status.enum_value_or_default()
    }

    pub fn clear_status(&mut self) {
        self.status = ::protobuf::ProtobufEnumOrUnknown::new(get_response::Status::OK);
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: get_response::Status) {
        self.status = ::protobuf::ProtobufEnumOrUnknown::new(v);
    }

    // string address = 2;

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }

    // string city = 3;

    pub fn get_city(&self) -> &str {
        &self.city
    }

    pub fn clear_city(&mut self) {
        self.city.clear();
    }

    // Param is passed by value, moved
    pub fn set_city(&mut self, v: ::std::string::String) {
        self.city = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_city(&mut self) -> &mut ::std::string::String {
        &mut self.city
    }

    // Take field
    pub fn take_city(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.city, ::std::string::String::new())
    }

    // int32 zipcode = 4;

    pub fn get_zipcode(&self) -> i32 {
        self.zipcode
    }

    pub fn clear_zipcode(&mut self) {
        self.zipcode = 0;
    }

    // Param is passed by value, moved
    pub fn set_zipcode(&mut self, v: i32) {
        self.zipcode = v;
    }

    // bytes data = 5;

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::new();
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &GetResponse| { &m.status },
            |m: &mut GetResponse| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "address",
            |m: &GetResponse| { &m.address },
            |m: &mut GetResponse| { &mut m.address },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "city",
            |m: &GetResponse| { &m.city },
            |m: &mut GetResponse| { &mut m.city },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "zipcode",
            |m: &GetResponse| { &m.zipcode },
            |m: &mut GetResponse| { &mut m.zipcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &GetResponse| { &m.data },
            |m: &mut GetResponse| { &mut m.data },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetResponse>(
            "GetResponse",
            1,
            fields,
        )
    }
}

impl ::protobuf::Message for GetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.status = is.read_enum_or_unknown()?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.address = is.read_string()?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.city = is.read_string()?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.zipcode = is.read_int32()?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.data = is.read_bytes()?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.status != ::protobuf::ProtobufEnumOrUnknown::new(get_response::Status::OK) {
            my_size += ::protobuf::rt::enum_or_unknown_size(1, self.status);
        }
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.address);
        }
        if !self.city.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.city);
        }
        if self.zipcode != 0 {
            my_size += ::protobuf::rt::value_size(4, self.zipcode, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.status != ::protobuf::ProtobufEnumOrUnknown::new(get_response::Status::OK) {
            os.write_enum(1, ::protobuf::ProtobufEnumOrUnknown::value(&self.status))?;
        }
        if !self.address.is_empty() {
            os.write_string(2, &self.address)?;
        }
        if !self.city.is_empty() {
            os.write_string(3, &self.city)?;
        }
        if self.zipcode != 0 {
            os.write_int32(4, self.zipcode)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(5, &self.data)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn new() -> GetResponse {
        GetResponse::new()
    }

    fn descriptor_static() -> ::protobuf::reflect::MessageDescriptor {
        ::protobuf::reflect::MessageDescriptor::new_generated_2(file_descriptor(), 1)
    }

    fn default_instance() -> &'static GetResponse {
        static instance: GetResponse = GetResponse {
            status: ::protobuf::ProtobufEnumOrUnknown::from_i32(0),
            address: ::std::string::String::new(),
            city: ::std::string::String::new(),
            zipcode: 0,
            data: ::std::vec::Vec::new(),
            unknown_fields: ::protobuf::UnknownFields::new(),
            cached_size: ::protobuf::rt::CachedSize::new(),
        };
        &instance
    }
}

impl ::protobuf::Clear for GetResponse {
    fn clear(&mut self) {
        self.status = ::protobuf::ProtobufEnumOrUnknown::new(get_response::Status::OK);
        self.address.clear();
        self.city.clear();
        self.zipcode = 0;
        self.data.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetResponse {
    type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `GetResponse`
pub mod get_response {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    #[derive(::serde::Serialize, ::serde::Deserialize)]
    pub enum Status {
        OK = 0,
        ERR = 1,
        NOT_FOUND = 2,
    }

    impl ::protobuf::ProtobufEnum for Status {
        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Status> {
            match value {
                0 => ::std::option::Option::Some(Status::OK),
                1 => ::std::option::Option::Some(Status::ERR),
                2 => ::std::option::Option::Some(Status::NOT_FOUND),
                _ => ::std::option::Option::None
            }
        }

        fn values() -> &'static [Self] {
            static values: &'static [Status] = &[
                Status::OK,
                Status::ERR,
                Status::NOT_FOUND,
            ];
            values
        }

        fn enum_descriptor_static() -> ::protobuf::reflect::EnumDescriptor {
            ::protobuf::reflect::EnumDescriptor::new_generated_2(super::file_descriptor(), 0)
        }
    }

    impl ::std::default::Default for Status {
        fn default() -> Self {
            Status::OK
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Status {
        type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeEnum<Self>;
    }

    impl Status {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new_2::<Status>("GetResponse.Status", 0)
        }
    }
}

#[derive(PartialEq,Clone,Default)]
#[derive(::serde::Serialize, ::serde::Deserialize)]
pub struct Strings {
    // message fields
    pub item: ::std::vec::Vec<::std::string::String>,
    // special fields
    #[serde(skip)]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[serde(skip)]
    pub cached_size: ::protobuf::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a Strings {
    fn default() -> &'a Strings {
        <Strings as ::protobuf::Message>::default_instance()
    }
}

impl Strings {
    pub fn new() -> Strings {
        ::std::default::Default::default()
    }

    // repeated string item = 1;

    pub fn get_item(&self) -> &[::std::string::String] {
        &self.item
    }

    pub fn clear_item(&mut self) {
        self.item.clear();
    }

    // Param is passed by value, moved
    pub fn set_item(&mut self, v: ::std::vec::Vec<::std::string::String>) {
        self.item = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
        &mut self.item
    }

    // Take field
    pub fn take_item(&mut self) -> ::std::vec::Vec<::std::string::String> {
        ::std::mem::replace(&mut self.item, ::std::vec::Vec::new())
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::new();
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "item",
            |m: &Strings| { &m.item },
            |m: &mut Strings| { &mut m.item },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Strings>(
            "Strings",
            2,
            fields,
        )
    }
}

impl ::protobuf::Message for Strings {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.item)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.item {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.item {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn new() -> Strings {
        Strings::new()
    }

    fn descriptor_static() -> ::protobuf::reflect::MessageDescriptor {
        ::protobuf::reflect::MessageDescriptor::new_generated_2(file_descriptor(), 2)
    }

    fn default_instance() -> &'static Strings {
        static instance: Strings = Strings {
            item: ::std::vec::Vec::new(),
            unknown_fields: ::protobuf::UnknownFields::new(),
            cached_size: ::protobuf::rt::CachedSize::new(),
        };
        &instance
    }
}

impl ::protobuf::Clear for Strings {
    fn clear(&mut self) {
        self.item.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Strings {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Strings {
    type RuntimeType = ::protobuf::reflect::runtime_types::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14protos/example.proto\x1a\x0frustproto.proto\"V\n\nGetRequest\x12\
    \x14\n\x04name\x18\x01\x20\x01(\tR\x04nameB\0\x12\x12\n\x03age\x18\x02\
    \x20\x01(\x05R\x03ageB\0\x12\x1c\n\x08features\x18\x03\x20\x03(\tR\x08fe\
    aturesB\0:\0\"\xd4\x01\n\x0bGetResponse\x12-\n\x06status\x18\x01\x20\x01\
    (\x0e2\x13.GetResponse.StatusR\x06statusB\0\x12\x1a\n\x07address\x18\x02\
    \x20\x01(\tR\x07addressB\0\x12\x14\n\x04city\x18\x03\x20\x01(\tR\x04city\
    B\0\x12\x1a\n\x07zipcode\x18\x04\x20\x01(\x05R\x07zipcodeB\0\x12\x14\n\
    \x04data\x18\x05\x20\x01(\x0cR\x04dataB\0\"0\n\x06Status\x12\x08\n\x02OK\
    \x10\0\x1a\0\x12\t\n\x03ERR\x10\x01\x1a\0\x12\x0f\n\tNOT_FOUND\x10\x02\
    \x1a\0\x1a\0:\0\"!\n\x07Strings\x12\x14\n\x04item\x18\x01\x20\x03(\tR\
    \x04itemB\0:\0B\0b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> ::protobuf::reflect::FileDescriptor {
    static file_descriptor_lazy: ::protobuf::rt::LazyV2<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::LazyV2::INIT;
    let file_descriptor = file_descriptor_lazy.get(|| {
        let mut deps = ::std::vec::Vec::new();
        deps.push(::protobuf::rustproto::file_descriptor());
        let mut messages = ::std::vec::Vec::new();
        messages.push(GetRequest::generated_message_descriptor_data());
        messages.push(GetResponse::generated_message_descriptor_data());
        messages.push(Strings::generated_message_descriptor_data());
        let mut enums = ::std::vec::Vec::new();
        enums.push(get_response::Status::generated_enum_descriptor_data());
        ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
            file_descriptor_proto(),
            deps,
            messages,
            enums,
        )
    });
    ::protobuf::reflect::FileDescriptor::new_generated_2(file_descriptor)
}
