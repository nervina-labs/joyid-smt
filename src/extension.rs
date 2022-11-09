// Generated by Molecule 0.7.3


#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::if_same_then_else)]

use super::ckb_types::prelude::*;
use super::molecule::{self, prelude::*};
extern crate alloc;
pub use alloc::vec::*;
// these lines above are manually added

use super::common::*;
use molecule::prelude::*;
#[derive(Clone)]
pub struct ExtensionVec(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ExtensionVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ExtensionVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ExtensionVec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl ::core::default::Default for ExtensionVec {
    fn default() -> Self {
        let v: Vec<u8> = vec![0, 0, 0, 0];
        ExtensionVec::new_unchecked(v.into())
    }
}
impl ExtensionVec {
    pub const ITEM_SIZE: usize = 32;
    pub fn total_size(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.item_count()
    }
    pub fn item_count(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn len(&self) -> usize {
        self.item_count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Byte32> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Byte32 {
        let start = molecule::NUMBER_SIZE + Self::ITEM_SIZE * idx;
        let end = start + Self::ITEM_SIZE;
        Byte32::new_unchecked(self.0.slice(start..end))
    }
    pub fn as_reader<'r>(&'r self) -> ExtensionVecReader<'r> {
        ExtensionVecReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ExtensionVec {
    type Builder = ExtensionVecBuilder;
    const NAME: &'static str = "ExtensionVec";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ExtensionVec(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ExtensionVecReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ExtensionVecReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder().extend(self.into_iter())
    }
}
#[derive(Clone, Copy)]
pub struct ExtensionVecReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ExtensionVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ExtensionVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ExtensionVecReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} [", Self::NAME)?;
        for i in 0..self.len() {
            if i == 0 {
                write!(f, "{}", self.get_unchecked(i))?;
            } else {
                write!(f, ", {}", self.get_unchecked(i))?;
            }
        }
        write!(f, "]")
    }
}
impl<'r> ExtensionVecReader<'r> {
    pub const ITEM_SIZE: usize = 32;
    pub fn total_size(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.item_count()
    }
    pub fn item_count(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn len(&self) -> usize {
        self.item_count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn get(&self, idx: usize) -> Option<Byte32Reader<'r>> {
        if idx >= self.len() {
            None
        } else {
            Some(self.get_unchecked(idx))
        }
    }
    pub fn get_unchecked(&self, idx: usize) -> Byte32Reader<'r> {
        let start = molecule::NUMBER_SIZE + Self::ITEM_SIZE * idx;
        let end = start + Self::ITEM_SIZE;
        Byte32Reader::new_unchecked(&self.as_slice()[start..end])
    }
}
impl<'r> molecule::prelude::Reader<'r> for ExtensionVecReader<'r> {
    type Entity = ExtensionVec;
    const NAME: &'static str = "ExtensionVecReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ExtensionVecReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], _compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let item_count = molecule::unpack_number(slice) as usize;
        if item_count == 0 {
            if slice_len != molecule::NUMBER_SIZE {
                return ve!(Self, TotalSizeNotMatch, molecule::NUMBER_SIZE, slice_len);
            }
            return Ok(());
        }
        let total_size = molecule::NUMBER_SIZE + Self::ITEM_SIZE * item_count;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct ExtensionVecBuilder(pub(crate) Vec<Byte32>);
impl ExtensionVecBuilder {
    pub const ITEM_SIZE: usize = 32;
    pub fn set(mut self, v: Vec<Byte32>) -> Self {
        self.0 = v;
        self
    }
    pub fn push(mut self, v: Byte32) -> Self {
        self.0.push(v);
        self
    }
    pub fn extend<T: ::core::iter::IntoIterator<Item = Byte32>>(mut self, iter: T) -> Self {
        for elem in iter {
            self.0.push(elem);
        }
        self
    }
    pub fn replace(&mut self, index: usize, v: Byte32) -> Option<Byte32> {
        self.0
            .get_mut(index)
            .map(|item| ::core::mem::replace(item, v))
    }
}
impl molecule::prelude::Builder for ExtensionVecBuilder {
    type Entity = ExtensionVec;
    const NAME: &'static str = "ExtensionVecBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE + Self::ITEM_SIZE * self.0.len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        writer.write_all(&molecule::pack_number(self.0.len() as molecule::Number))?;
        for inner in &self.0[..] {
            writer.write_all(inner.as_slice())?;
        }
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ExtensionVec::new_unchecked(inner.into())
    }
}
pub struct ExtensionVecIterator(ExtensionVec, usize, usize);
impl ::core::iter::Iterator for ExtensionVecIterator {
    type Item = Byte32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl ::core::iter::ExactSizeIterator for ExtensionVecIterator {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
impl ::core::iter::IntoIterator for ExtensionVec {
    type Item = Byte32;
    type IntoIter = ExtensionVecIterator;
    fn into_iter(self) -> Self::IntoIter {
        let len = self.len();
        ExtensionVecIterator(self, 0, len)
    }
}
impl<'r> ExtensionVecReader<'r> {
    pub fn iter<'t>(&'t self) -> ExtensionVecReaderIterator<'t, 'r> {
        ExtensionVecReaderIterator(&self, 0, self.len())
    }
}
pub struct ExtensionVecReaderIterator<'t, 'r>(&'t ExtensionVecReader<'r>, usize, usize);
impl<'t: 'r, 'r> ::core::iter::Iterator for ExtensionVecReaderIterator<'t, 'r> {
    type Item = Byte32Reader<'t>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.2 {
            None
        } else {
            let ret = self.0.get_unchecked(self.1);
            self.1 += 1;
            Some(ret)
        }
    }
}
impl<'t: 'r, 'r> ::core::iter::ExactSizeIterator for ExtensionVecReaderIterator<'t, 'r> {
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
#[derive(Clone)]
pub struct ExtensionLeaves(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ExtensionLeaves {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ExtensionLeaves {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ExtensionLeaves {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "keys", self.keys())?;
        write!(f, ", {}: {}", "values", self.values())?;
        write!(f, ", {}: {}", "old_values", self.old_values())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for ExtensionLeaves {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            36, 0, 0, 0, 20, 0, 0, 0, 24, 0, 0, 0, 28, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        ExtensionLeaves::new_unchecked(v.into())
    }
}
impl ExtensionLeaves {
    pub const FIELD_COUNT: usize = 4;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn keys(&self) -> ExtensionVec {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        ExtensionVec::new_unchecked(self.0.slice(start..end))
    }
    pub fn values(&self) -> ExtensionVec {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        ExtensionVec::new_unchecked(self.0.slice(start..end))
    }
    pub fn old_values(&self) -> ExtensionVec {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        ExtensionVec::new_unchecked(self.0.slice(start..end))
    }
    pub fn proof(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[20..]) as usize;
            Bytes::new_unchecked(self.0.slice(start..end))
        } else {
            Bytes::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> ExtensionLeavesReader<'r> {
        ExtensionLeavesReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ExtensionLeaves {
    type Builder = ExtensionLeavesBuilder;
    const NAME: &'static str = "ExtensionLeaves";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ExtensionLeaves(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ExtensionLeavesReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ExtensionLeavesReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .keys(self.keys())
            .values(self.values())
            .old_values(self.old_values())
            .proof(self.proof())
    }
}
#[derive(Clone, Copy)]
pub struct ExtensionLeavesReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ExtensionLeavesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ExtensionLeavesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ExtensionLeavesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "keys", self.keys())?;
        write!(f, ", {}: {}", "values", self.values())?;
        write!(f, ", {}: {}", "old_values", self.old_values())?;
        write!(f, ", {}: {}", "proof", self.proof())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> ExtensionLeavesReader<'r> {
    pub const FIELD_COUNT: usize = 4;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn keys(&self) -> ExtensionVecReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        ExtensionVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn values(&self) -> ExtensionVecReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        ExtensionVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn old_values(&self) -> ExtensionVecReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        ExtensionVecReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn proof(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[20..]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for ExtensionLeavesReader<'r> {
    type Entity = ExtensionLeaves;
    const NAME: &'static str = "ExtensionLeavesReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ExtensionLeavesReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        ExtensionVecReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        ExtensionVecReader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        ExtensionVecReader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        BytesReader::verify(&slice[offsets[3]..offsets[4]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct ExtensionLeavesBuilder {
    pub(crate) keys: ExtensionVec,
    pub(crate) values: ExtensionVec,
    pub(crate) old_values: ExtensionVec,
    pub(crate) proof: Bytes,
}
impl ExtensionLeavesBuilder {
    pub const FIELD_COUNT: usize = 4;
    pub fn keys(mut self, v: ExtensionVec) -> Self {
        self.keys = v;
        self
    }
    pub fn values(mut self, v: ExtensionVec) -> Self {
        self.values = v;
        self
    }
    pub fn old_values(mut self, v: ExtensionVec) -> Self {
        self.old_values = v;
        self
    }
    pub fn proof(mut self, v: Bytes) -> Self {
        self.proof = v;
        self
    }
}
impl molecule::prelude::Builder for ExtensionLeavesBuilder {
    type Entity = ExtensionLeaves;
    const NAME: &'static str = "ExtensionLeavesBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.keys.as_slice().len()
            + self.values.as_slice().len()
            + self.old_values.as_slice().len()
            + self.proof.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.keys.as_slice().len();
        offsets.push(total_size);
        total_size += self.values.as_slice().len();
        offsets.push(total_size);
        total_size += self.old_values.as_slice().len();
        offsets.push(total_size);
        total_size += self.proof.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.keys.as_slice())?;
        writer.write_all(self.values.as_slice())?;
        writer.write_all(self.old_values.as_slice())?;
        writer.write_all(self.proof.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ExtensionLeaves::new_unchecked(inner.into())
    }
}
#[derive(Clone)]
pub struct ExtensionEntries(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for ExtensionEntries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for ExtensionEntries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for ExtensionEntries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "leaves", self.leaves())?;
        write!(f, ", {}: {}", "sub_type", self.sub_type())?;
        write!(f, ", {}: {}", "raw_data", self.raw_data())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for ExtensionEntries {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            62, 0, 0, 0, 16, 0, 0, 0, 52, 0, 0, 0, 58, 0, 0, 0, 36, 0, 0, 0, 20, 0, 0, 0, 24, 0, 0,
            0, 28, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];
        ExtensionEntries::new_unchecked(v.into())
    }
}
impl ExtensionEntries {
    pub const FIELD_COUNT: usize = 3;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn leaves(&self) -> ExtensionLeaves {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        ExtensionLeaves::new_unchecked(self.0.slice(start..end))
    }
    pub fn sub_type(&self) -> Byte6 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Byte6::new_unchecked(self.0.slice(start..end))
    }
    pub fn raw_data(&self) -> Bytes {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            Bytes::new_unchecked(self.0.slice(start..end))
        } else {
            Bytes::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> ExtensionEntriesReader<'r> {
        ExtensionEntriesReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for ExtensionEntries {
    type Builder = ExtensionEntriesBuilder;
    const NAME: &'static str = "ExtensionEntries";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        ExtensionEntries(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ExtensionEntriesReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        ExtensionEntriesReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .leaves(self.leaves())
            .sub_type(self.sub_type())
            .raw_data(self.raw_data())
    }
}
#[derive(Clone, Copy)]
pub struct ExtensionEntriesReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for ExtensionEntriesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for ExtensionEntriesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for ExtensionEntriesReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "leaves", self.leaves())?;
        write!(f, ", {}: {}", "sub_type", self.sub_type())?;
        write!(f, ", {}: {}", "raw_data", self.raw_data())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> ExtensionEntriesReader<'r> {
    pub const FIELD_COUNT: usize = 3;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn leaves(&self) -> ExtensionLeavesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        ExtensionLeavesReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn sub_type(&self) -> Byte6Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Byte6Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn raw_data(&self) -> BytesReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[16..]) as usize;
            BytesReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            BytesReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for ExtensionEntriesReader<'r> {
    type Entity = ExtensionEntries;
    const NAME: &'static str = "ExtensionEntriesReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        ExtensionEntriesReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        ExtensionLeavesReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        Byte6Reader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        BytesReader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct ExtensionEntriesBuilder {
    pub(crate) leaves: ExtensionLeaves,
    pub(crate) sub_type: Byte6,
    pub(crate) raw_data: Bytes,
}
impl ExtensionEntriesBuilder {
    pub const FIELD_COUNT: usize = 3;
    pub fn leaves(mut self, v: ExtensionLeaves) -> Self {
        self.leaves = v;
        self
    }
    pub fn sub_type(mut self, v: Byte6) -> Self {
        self.sub_type = v;
        self
    }
    pub fn raw_data(mut self, v: Bytes) -> Self {
        self.raw_data = v;
        self
    }
}
impl molecule::prelude::Builder for ExtensionEntriesBuilder {
    type Entity = ExtensionEntries;
    const NAME: &'static str = "ExtensionEntriesBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.leaves.as_slice().len()
            + self.sub_type.as_slice().len()
            + self.raw_data.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.leaves.as_slice().len();
        offsets.push(total_size);
        total_size += self.sub_type.as_slice().len();
        offsets.push(total_size);
        total_size += self.raw_data.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.leaves.as_slice())?;
        writer.write_all(self.sub_type.as_slice())?;
        writer.write_all(self.raw_data.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        ExtensionEntries::new_unchecked(inner.into())
    }
}
