#[derive(Debug, Clone)]
enum TypeKind {
    Bool,
    Char,
    Int(IntKind),
    Uint(UintKind),
    Float(FloatKind),
    Struct {
        name: &'static str,
        fields: Vec<(&'static str, TypeKind)>,
    },
    // ...
}

#[derive(Debug, Clone)]
enum IntKind {
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
}

#[derive(Debug, Clone)]
enum UintKind {
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Debug, Clone)]
enum FloatKind {
    F32,
    F64,
}

pub fn main() {
    let my_bool = TypeKind::Bool;
    let my_i32 = TypeKind::Int(IntKind::I32);

    let my_struct = TypeKind::Struct {
        name: "MyStruct",
        fields: vec![
            ("field1", TypeKind::Char),
            ("field2", TypeKind::Float(FloatKind::F32)),
        ],
    };

    println!("my_struct: {:#?}", my_struct);

    // XXX: show `Option` and `Result` documentation

    print!("match => ");
    match &my_bool {
        TypeKind::Bool => println!("bool!"),
        TypeKind::Char => println!("char!"),
        TypeKind::Int(int_kind) => println!("int({:?})!", int_kind),
        TypeKind::Uint(uint_kind) => println!("uint({:?})!", uint_kind),
        TypeKind::Float(float_kind) => println!("float({:?})!", float_kind),
        TypeKind::Struct { .. } => println!("struct"),
    };

    print!("partial match => ");
    match &my_i32 {
        TypeKind::Int(int_kind) => println!("This is an int: {:?}!", int_kind),
        something_else => println!("This is not an int: {:?}", something_else),
    };

    print!("if let => ");
    if let TypeKind::Struct { name, .. } = &my_struct {
        println!("This a struct with name: {:?}", name);
    }
}
