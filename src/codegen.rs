use rustc::hir::def_id::DefId;
use rustc::mir::mir_map::MirMap;
use rustc::middle::const_val::ConstVal;
use rustc::mir::repr;
use rustc::mir;
use rustc_data_structures::indexed_vec::Idx;
use std::fmt;

pub struct Arg(mir::repr::Arg);

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a{:x}", self.0.index())
    }
}

pub struct Var(mir::repr::Var);

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{:x}", self.0.index())
    }
}

pub struct Tmp(mir::repr::Temp);

impl fmt::Display for Tmp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "t{:x}", self.0.index())
    }
}

pub struct Field(mir::repr::Temp);

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "f{:x}", self.0.index())
    }
}

pub struct Item(DefId);

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d{:x}_{:x}(", self.0.index.as_u32(), self.0.krate)?;
    }
}

pub struct LvalueGet<'a>(repr::Lvalue<'a>);

impl<'a> fmt::Display for LvalueGet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            repr::Lvalue::Var(var) => write!(f, "{}", Var(var)),
            repr::Lvalue::Temp(var) => write!(f, "{}", Tmp(var)),
            repr::Lvalue::Arg(var) => write!(f, "{}", Arg(var)),
            repr::Lvalue::Static(item) => write!(f, "{}", Item(item)),
            repr::Lvalue::ReturnPointer => write!(f, "r"),
            repr::Lvalue::Projection(repr::Projection { base, elem }) =>
                match elem {
                    repr::ProjectionElem::Deref => write!(f, "{}.get()", LvalueGet(base)),
                    repr::ProjectionElem::Field(field, _) => write!(f, "{}.{}", LvalueGet(base), Field(field)),
                    repr::ProjectionElem::Index(idx) => write!(f, "{}[{}]", LvalueGet(base), Operand(idx)),
                    _ => unimplemented!(),
                }
        }
    }
}

pub struct LvalueSet<'a>(repr::Lvalue<'a>, repr::Rvalue<'a>);

impl<'a> fmt::Display for LvalueSet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            repr::Lvalue::Var(var) => write!(f, "{}={}", Var(var), Rvalue(self.1)),
            repr::Lvalue::Temp(var) => write!(f, "{}={}", Tmp(var), Rvalue(self.1)),
            repr::Lvalue::Arg(var) => write!(f, "{}={}", Arg(var), Rvalue(self.1)),
            repr::Lvalue::Static(item) => write!(f, "{}={}", Item(item), Rvalue(self.1)),
            repr::Lvalue::ReturnPointer => write!(f, "r={}", Rvalue(self.1)),
            repr::Lvalue::Projection(repr::Projection { base, elem }) => match elem {
                repr::ProjectionElem::Deref => write!(f, "{}.set({})", LvalueGet(base), Rvalue(self.1)),
                repr::ProjectionElem::Field(field, _) => write!(f, "{}.{}={}", LvalueGet(base), Field(field.index()), Rvalue(self.1)),
                repr::ProjectionElem::Index(idx) => write!(f, "{}[{}]={}", LvalueGet(base), Operand(idx), Rvalue(self.1)),
                _ => unimplemented!(),
            }
        }
    }
}

pub struct Literal<'a>(repr::Literal<'a>);

impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            repr::Literal::Item { def_id, .. } => write!(f, "{}", Item(def_id)),
            repr::Literal::Value { value } => match value {
                ConstVal::Integral(int) => write!(f, "{}", int.to_u64_unchecked()),
                ConstVal::Str(string) =>
                    if string.starts_with("[js?") && string.ends_with("?js]") {
                        // We output the JavaScript without quotes, meaning that we embeded raw JS.
                        // This is used for making bindings with JS libraries etc.
                        write!(f, "{}", string)
                    } else {
                        write!(f, "\"{}\"", string.escape_default())
                    }
                ConstVal::Bool(b) => write!(f, "{}", b),
                _ => unimplemented!(),
            }
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<ConstVal> for Literal<'a> {
    fn from(f: ConstVal) -> Literal<'a> {
        Literal(repr::Literal::Value { value: f })
    }
}

pub struct Operand<'a>(repr::Operand<'a>);

impl<'a> fmt::Display for Operand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            repr::Operand::Consume(lvalue) => write!(f, "{}", LvalueGet(lvalue)),
            repr::Operand::Constant(constant) => write!(f, "{}", Literal(constant)),
        }
    }
}

fn binop_to_js(binop: repr::BinOp) -> &'static str {
    match binop {
        repr::BinOp::Add => "+",
        repr::BinOp::Sub => "-",
        repr::BinOp::Mul => "*",
        // FIXME: Integer division doesn't not round down, but instead coerces to floats,
        // giving results different from Rust's.
        repr::BinOp::Div => "/",
        repr::BinOp::Rem => "%",
        // FIXME: In JavaScript, using these operations on boolean values will convert them
        // into integers. The same is not true for Rust.
        repr::BinOp::BitXor => "^",
        repr::BinOp::BitAnd => "&",
        repr::BinOp::BitOr => "|",
        repr::BinOp::Shl => "<<",
        repr::BinOp::Shr => ">>",
        repr::BinOp::Eq => "===",
        repr::BinOp::Lt => "<",
        repr::BinOp::Le => "<=",
        repr::BinOp::Ne => "!==",
        repr::BinOp::Ge => ">=",
        repr::BinOp::Gt => ">",
    }
}

fn unop_to_js(unop: repr::UnOp) -> char {
    match unop {
        repr::UnOp::Not => '!',
        repr::UnOp::Neg => '-',
    }
}

pub struct Rvalue<'a>(repr::Rvalue<'a>);

impl<'a> fmt::Display for Rvalue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            repr::Rvalue::Use(operand) => write!(f, "{}", Operand(operand)),
            // JavaScript doesn't have first class pointers, however it is possible to emulate them
            // through closures. The basic idea is to let a setter and getter closure capture the
            // lvalue, and then access it as an alias through these methods. It's pretty hacky, but
            // it works.

            // Immutable references.
            repr::Rvalue::Ref(_, repr::BorrowKind::Shared | repr::BorrowKind::Unique, lvalue) =>
                write!(f, "{get: function(){{return {}}}", LvalueGet(lvalue)),
            // Mutable references.
            repr::Rvalue::Ref(_, _, lvalue) =>
                write!(f, "{{get:function(){{return {}}},set:function(x){{{}=x}}}}",
                       LvalueGet(lvalue)),
            repr::Rvalue::Len(lvalue) => write!(f, "{}.length", LvalueGet(lvalue)),
            // FIXME: Here be hacks! JavaScript does coercions literally everywhere. We cross our
            // fingers and hope that these matches the corresponding casts in Rust. Tests shows
            // that they do "most of the time" (read: might not work at all).
            repr::Rvalue::Cast(_, operand, _) => write!(f, "{}", Operand(operand)),
            repr::Rvalue::CheckedBinaryOp(binop, x, y) | repr::Rvalue::BinaryOp(binop, x, y) =>
                write!(f, "({}){}({})", Operand(x), binop_to_js(binop), Operand(y)),
            repr::Rvalue::UnaryOp(unop, x) =>
                write!(f, "{}({})", unop_to_js(unop), x),
            repr::Rvalue::Box(_) => write!(f, "new function(){\
                                                   this.get=function(){return this.x};\
                                                   this.set=function(x){this.x=x}\
                                               }"),
            repr::Rvalue::Aggregate(kind, args) =>
                match kind {
                    repr::AggregateKind::Vec | repr::AggregateKind::Tuple => {
                        // Start the array delimiter.
                        write!(f, "[")?;
                        for i in args {
                            write!(f, "{},", Operand(i))?;
                        }
                        // End the array delimiter.
                        write!(f, "]")
                    },
                    repr::AggregateKind::Adt(def, variant, _) => {
                        let variant = def.variants[variant];
                        // Write the discriminant field.
                        write!(f, "{{d:{}", variant.disr_value.to_u32())?;

                        // Write in all the fields in.
                        for (field, cont) in variant.fields.iter().zip(args) {
                            write!(f, ",{}:{}", Field(repr::Field::new(field.name.0)), Operand(cont))?;
                        }

                        // End the object.
                        write!(f, "}}")
                    },
                    _ => unimplemented!(),
                },
            _ => unimplemented!(),
        }
    }
}

impl<'a> From<repr::Operand<'a>> for Rvalue<'a> {
    fn from(f: repr::Operand) -> Rvalue {
        Rvalue(repr::Rvalue::Use(f))
    }
}

pub struct Discriminant<'a>(Rvalue<'a>);

impl<'a> fmt::Display for Discriminant<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.d", self.0)
    }
}

pub struct Statement<'a>(repr::Statement<'a>);

impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.kind {
            repr::StatementKind::Assign(lvalue, rvalue) => write!(f, "{}", LvalueSet(lvalue, rvalue)),
            repr::StatementKind::SetDiscriminant(rvalue, disc) =>
                write!(f, "{}={}", Discriminant(Rvalue(rvalue)), disc),
            _ => unimplemented!(),
        }
    }
}
