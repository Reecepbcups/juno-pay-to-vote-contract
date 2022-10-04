// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc 3.21.5
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

//! Generated file from `protos/CosmosGovV1beta1Query.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:WeightedVoteOption)
pub struct WeightedVoteOption {
    // message fields
    // @@protoc_insertion_point(field:WeightedVoteOption.option)
    pub option: ::protobuf::EnumOrUnknown<VoteOption>,
    // @@protoc_insertion_point(field:WeightedVoteOption.weight)
    pub weight: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:WeightedVoteOption.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a WeightedVoteOption {
    fn default() -> &'a WeightedVoteOption {
        <WeightedVoteOption as ::protobuf::Message>::default_instance()
    }
}

impl WeightedVoteOption {
    pub fn new() -> WeightedVoteOption {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "option",
            |m: &WeightedVoteOption| { &m.option },
            |m: &mut WeightedVoteOption| { &mut m.option },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "weight",
            |m: &WeightedVoteOption| { &m.weight },
            |m: &mut WeightedVoteOption| { &mut m.weight },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<WeightedVoteOption>(
            "WeightedVoteOption",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for WeightedVoteOption {
    const NAME: &'static str = "WeightedVoteOption";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.option = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.weight = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.option != ::protobuf::EnumOrUnknown::new(VoteOption::VOTE_OPTION_UNSPECIFIED) {
            my_size += ::protobuf::rt::int32_size(1, self.option.value());
        }
        if !self.weight.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.weight);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.option != ::protobuf::EnumOrUnknown::new(VoteOption::VOTE_OPTION_UNSPECIFIED) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.option))?;
        }
        if !self.weight.is_empty() {
            os.write_string(2, &self.weight)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> WeightedVoteOption {
        WeightedVoteOption::new()
    }

    fn clear(&mut self) {
        self.option = ::protobuf::EnumOrUnknown::new(VoteOption::VOTE_OPTION_UNSPECIFIED);
        self.weight.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static WeightedVoteOption {
        static instance: WeightedVoteOption = WeightedVoteOption {
            option: ::protobuf::EnumOrUnknown::from_i32(0),
            weight: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for WeightedVoteOption {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("WeightedVoteOption").unwrap()).clone()
    }
}

impl ::std::fmt::Display for WeightedVoteOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WeightedVoteOption {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:Vote)
pub struct Vote {
    // message fields
    // @@protoc_insertion_point(field:Vote.proposal_id)
    pub proposal_id: u64,
    // @@protoc_insertion_point(field:Vote.voter)
    pub voter: ::std::string::String,
    // @@protoc_insertion_point(field:Vote.option)
    pub option: ::protobuf::EnumOrUnknown<VoteOption>,
    // @@protoc_insertion_point(field:Vote.options)
    pub options: ::std::vec::Vec<WeightedVoteOption>,
    // special fields
    // @@protoc_insertion_point(special_field:Vote.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Vote {
    fn default() -> &'a Vote {
        <Vote as ::protobuf::Message>::default_instance()
    }
}

impl Vote {
    pub fn new() -> Vote {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "proposal_id",
            |m: &Vote| { &m.proposal_id },
            |m: &mut Vote| { &mut m.proposal_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "voter",
            |m: &Vote| { &m.voter },
            |m: &mut Vote| { &mut m.voter },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "option",
            |m: &Vote| { &m.option },
            |m: &mut Vote| { &mut m.option },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "options",
            |m: &Vote| { &m.options },
            |m: &mut Vote| { &mut m.options },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Vote>(
            "Vote",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Vote {
    const NAME: &'static str = "Vote";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.proposal_id = is.read_uint64()?;
                },
                18 => {
                    self.voter = is.read_string()?;
                },
                24 => {
                    self.option = is.read_enum_or_unknown()?;
                },
                34 => {
                    self.options.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.proposal_id != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.proposal_id);
        }
        if !self.voter.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.voter);
        }
        if self.option != ::protobuf::EnumOrUnknown::new(VoteOption::VOTE_OPTION_UNSPECIFIED) {
            my_size += ::protobuf::rt::int32_size(3, self.option.value());
        }
        for value in &self.options {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.proposal_id != 0 {
            os.write_uint64(1, self.proposal_id)?;
        }
        if !self.voter.is_empty() {
            os.write_string(2, &self.voter)?;
        }
        if self.option != ::protobuf::EnumOrUnknown::new(VoteOption::VOTE_OPTION_UNSPECIFIED) {
            os.write_enum(3, ::protobuf::EnumOrUnknown::value(&self.option))?;
        }
        for v in &self.options {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Vote {
        Vote::new()
    }

    fn clear(&mut self) {
        self.proposal_id = 0;
        self.voter.clear();
        self.option = ::protobuf::EnumOrUnknown::new(VoteOption::VOTE_OPTION_UNSPECIFIED);
        self.options.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Vote {
        static instance: Vote = Vote {
            proposal_id: 0,
            voter: ::std::string::String::new(),
            option: ::protobuf::EnumOrUnknown::from_i32(0),
            options: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Vote {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Vote").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Vote {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Vote {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:QueryVoteRequest)
pub struct QueryVoteRequest {
    // message fields
    // @@protoc_insertion_point(field:QueryVoteRequest.proposal_id)
    pub proposal_id: u64,
    // @@protoc_insertion_point(field:QueryVoteRequest.voter)
    pub voter: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:QueryVoteRequest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QueryVoteRequest {
    fn default() -> &'a QueryVoteRequest {
        <QueryVoteRequest as ::protobuf::Message>::default_instance()
    }
}

impl QueryVoteRequest {
    pub fn new() -> QueryVoteRequest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "proposal_id",
            |m: &QueryVoteRequest| { &m.proposal_id },
            |m: &mut QueryVoteRequest| { &mut m.proposal_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "voter",
            |m: &QueryVoteRequest| { &m.voter },
            |m: &mut QueryVoteRequest| { &mut m.voter },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QueryVoteRequest>(
            "QueryVoteRequest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QueryVoteRequest {
    const NAME: &'static str = "QueryVoteRequest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.proposal_id = is.read_uint64()?;
                },
                18 => {
                    self.voter = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.proposal_id != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.proposal_id);
        }
        if !self.voter.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.voter);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.proposal_id != 0 {
            os.write_uint64(1, self.proposal_id)?;
        }
        if !self.voter.is_empty() {
            os.write_string(2, &self.voter)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> QueryVoteRequest {
        QueryVoteRequest::new()
    }

    fn clear(&mut self) {
        self.proposal_id = 0;
        self.voter.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QueryVoteRequest {
        static instance: QueryVoteRequest = QueryVoteRequest {
            proposal_id: 0,
            voter: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QueryVoteRequest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QueryVoteRequest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QueryVoteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryVoteRequest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:QueryVoteResponse)
pub struct QueryVoteResponse {
    // message fields
    // @@protoc_insertion_point(field:QueryVoteResponse.vote)
    pub vote: ::protobuf::MessageField<Vote>,
    // special fields
    // @@protoc_insertion_point(special_field:QueryVoteResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QueryVoteResponse {
    fn default() -> &'a QueryVoteResponse {
        <QueryVoteResponse as ::protobuf::Message>::default_instance()
    }
}

impl QueryVoteResponse {
    pub fn new() -> QueryVoteResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, Vote>(
            "vote",
            |m: &QueryVoteResponse| { &m.vote },
            |m: &mut QueryVoteResponse| { &mut m.vote },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QueryVoteResponse>(
            "QueryVoteResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QueryVoteResponse {
    const NAME: &'static str = "QueryVoteResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.vote)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.vote.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.vote.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> QueryVoteResponse {
        QueryVoteResponse::new()
    }

    fn clear(&mut self) {
        self.vote.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QueryVoteResponse {
        static instance: QueryVoteResponse = QueryVoteResponse {
            vote: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QueryVoteResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QueryVoteResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QueryVoteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QueryVoteResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:VoteOption)
pub enum VoteOption {
    // @@protoc_insertion_point(enum_value:VoteOption.VOTE_OPTION_UNSPECIFIED)
    VOTE_OPTION_UNSPECIFIED = 0,
    // @@protoc_insertion_point(enum_value:VoteOption.VOTE_OPTION_YES)
    VOTE_OPTION_YES = 1,
    // @@protoc_insertion_point(enum_value:VoteOption.VOTE_OPTION_ABSTAIN)
    VOTE_OPTION_ABSTAIN = 2,
    // @@protoc_insertion_point(enum_value:VoteOption.VOTE_OPTION_NO)
    VOTE_OPTION_NO = 3,
    // @@protoc_insertion_point(enum_value:VoteOption.VOTE_OPTION_NO_WITH_VETO)
    VOTE_OPTION_NO_WITH_VETO = 4,
}

impl ::protobuf::Enum for VoteOption {
    const NAME: &'static str = "VoteOption";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<VoteOption> {
        match value {
            0 => ::std::option::Option::Some(VoteOption::VOTE_OPTION_UNSPECIFIED),
            1 => ::std::option::Option::Some(VoteOption::VOTE_OPTION_YES),
            2 => ::std::option::Option::Some(VoteOption::VOTE_OPTION_ABSTAIN),
            3 => ::std::option::Option::Some(VoteOption::VOTE_OPTION_NO),
            4 => ::std::option::Option::Some(VoteOption::VOTE_OPTION_NO_WITH_VETO),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [VoteOption] = &[
        VoteOption::VOTE_OPTION_UNSPECIFIED,
        VoteOption::VOTE_OPTION_YES,
        VoteOption::VOTE_OPTION_ABSTAIN,
        VoteOption::VOTE_OPTION_NO,
        VoteOption::VOTE_OPTION_NO_WITH_VETO,
    ];
}

impl ::protobuf::EnumFull for VoteOption {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("VoteOption").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for VoteOption {
    fn default() -> Self {
        VoteOption::VOTE_OPTION_UNSPECIFIED
    }
}

impl VoteOption {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<VoteOption>("VoteOption")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"protos/CosmosGovV1beta1Query.proto\"Q\n\x12WeightedVoteOption\x12#\n\
    \x06option\x18\x01\x20\x01(\x0e2\x0b.VoteOptionR\x06option\x12\x16\n\x06\
    weight\x18\x02\x20\x01(\tR\x06weight\"\x91\x01\n\x04Vote\x12\x1f\n\x0bpr\
    oposal_id\x18\x01\x20\x01(\x04R\nproposalId\x12\x14\n\x05voter\x18\x02\
    \x20\x01(\tR\x05voter\x12#\n\x06option\x18\x03\x20\x01(\x0e2\x0b.VoteOpt\
    ionR\x06option\x12-\n\x07options\x18\x04\x20\x03(\x0b2\x13.WeightedVoteO\
    ptionR\x07options\"I\n\x10QueryVoteRequest\x12\x1f\n\x0bproposal_id\x18\
    \x01\x20\x01(\x04R\nproposalId\x12\x14\n\x05voter\x18\x02\x20\x01(\tR\
    \x05voter\".\n\x11QueryVoteResponse\x12\x19\n\x04vote\x18\x01\x20\x01(\
    \x0b2\x05.VoteR\x04vote*\x89\x01\n\nVoteOption\x12\x1b\n\x17VOTE_OPTION_\
    UNSPECIFIED\x10\0\x12\x13\n\x0fVOTE_OPTION_YES\x10\x01\x12\x17\n\x13VOTE\
    _OPTION_ABSTAIN\x10\x02\x12\x12\n\x0eVOTE_OPTION_NO\x10\x03\x12\x1c\n\
    \x18VOTE_OPTION_NO_WITH_VETO\x10\x04b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(WeightedVoteOption::generated_message_descriptor_data());
            messages.push(Vote::generated_message_descriptor_data());
            messages.push(QueryVoteRequest::generated_message_descriptor_data());
            messages.push(QueryVoteResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(VoteOption::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
