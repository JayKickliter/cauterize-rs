type Integer = isize;
type Identifier = String;
type Hash = sha1::Digest;
type Length = usize;
type Offset = isize;
type Version = String;

#[derive(Debug)]
struct Specification {
    name: String,
    version: Version,
    fingerprint: Hash,
    size: Size,
    depth: Integer,
    type_length: Integer,
    length_tag: Tag,
    types: Vec<Type>
}

#[derive(Debug)]
struct Type {
    pub name: Identifier,
    pub fingerprint: Hash,
    pub size: Size,
    pub depth: Integer,
    pub desc: TypeDesc,
}

#[derive(Debug)]
enum TypeDesc {
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
enum Field {
    Data {
        name: Identifier,
        index: Integer,
        ref_: Identifier,
    },
    Empty { name: Identifier, index: Integer },
}

#[derive(Debug)]
struct EnumVal {
    name: Identifier,
    index: Integer,
}

#[derive(Debug)]
enum Tag {
    T1,
    T2,
    T4,
    T8,
}

#[derive(Debug)]
struct Size {
    min: Integer,
    max: Integer,
}



#[derive(Debug)]
enum Prim {
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
