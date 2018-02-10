use sha1::Digest;

pub type Integer = isize;
pub type Identifier = String;
pub type Hash = Digest;
pub type Length = usize;
pub type Offset = isize;
pub type Version = String;

#[derive(Debug)]
pub struct Specification {
    name: String,
    version: Version,
    fingerprint: Hash,
    size: Size,
    depth: Integer,
    type_length: Integer,
    length_tag: Tag,
    // types: Vec<Type>
}

#[derive(Debug)]
pub struct Type {
    pub name: Identifier,
    pub fingerprint: Hash,
    pub size: Size,
    pub depth: Integer,
    pub desc: TypeDesc,
}

#[derive(Debug)]
pub enum TypeDesc {
    Synonym { id: Identifier },
    Range {
        offset: Offset,
        length: Length,
        tag: Tag,
        primitive: Prim,
    },
    Array { id: Identifier, length: Length },
    Vector {
        id: Identifier,
        length: Length,
        tag: Tag,
    },
    Enumeration { values: Vec<EnumVal>, tag: Tag },
    Record { fields: Vec<Field> },
    Combination { fields: Vec<Field>, tag: Tag },
    Union { fields: Vec<Field>, tag: Tag },
}


#[derive(Debug)]
pub enum Field {
    Data {
        name: Identifier,
        index: Integer,
        ref_: Identifier,
    },
    Empty { name: Identifier, index: Integer },
}

#[derive(Debug)]
pub struct EnumVal {
    name: Identifier,
    index: Integer,
}

#[derive(Debug)]
pub enum Tag {
    T1,
    T2,
    T4,
    T8,
}

#[derive(Debug)]
pub struct Size {
    min: Integer,
    max: Integer,
}



#[derive(Debug)]
pub enum Prim {
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    F32,
    F64,
    Bool,
}
