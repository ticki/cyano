use rustc::hir::def_id::{self, DefId};
use rustc::middle::const_val::ConstVal;
use rustc::mir::mir_map::MirMap;
use rustc::mir::repr;
use rustc_data_structures::indexed_vec::Idx;
use std::{mem, fmt};

use codegen;
use cell::MoveCell;

pub struct Compiler<'a> {
    out: MoveCell<Option<fmt::Formatter<'a>>>,
    mir: MirMap<'a>,
    delayed_fns: Vec<DefId>,
}

impl<'a> Compiler<'a> {
    pub fn finish(mut self) -> fmt::Result {
        // Start anonymous environment.
        self.out(|f| write!(f, "function(){{d0_0();"))?;

        self.write_fn(DefId::local(def_id::DefIndex::new(0)))?;

        // FIXME: In some cases, this might loop infinitely due to visiting the same functions in
        // cycle. The result should be cachced and returned on second visit.
        let delayed_fns = mem::replace(&mut self.delayed_fns, Vec::new());
        for i in delayed_fns {
            self.write_fn(i)?;
        }

        // End anonymous environment.
        self.out(|f| write!(f, "}}()"))
    }

    fn out<F: FnOnce(&mut fmt::Formatter) -> fmt::Result>(&self, f: F) -> fmt::Result {
        // Temporarily grab the formatter.
        let mut old = self.out.replace(None).unwrap();
        // Run the closure.
        let res = f(&mut old);
        // Put it back.
        self.out.replace(Some(old));

        res
    }

    fn write_fn(&self, id: DefId) -> fmt::Result {
        self.out(|f| write!(f, "function {}(", codegen::Item(id)))?;

        // Declare the arguments.
        for (arg, _) in self.mir.map[&id].arg_decls.iter_enumerated() {
            self.out(|f| write!(f, "{}", codegen::Arg(arg)))?;
        }

        // We initialize our "goto loop", which is a jump table used to emulate gotos in
        // JavaScript. While it might seem slow at first, it is worth noting that every modern JS
        // engine will optimize this down to gotos making it zero-cost. Even without such an
        // optimization, the performance is still OK (when the cases in a switch statements is
        // above some threshold, it will almost always be transformed to a jump table, which means
        // one lookup per goto).
        self.out(|f| write!(f, "){{var g=0;t:while(true){{switch g{{"))?;

        let body = &self.mir.map[&id];

        // Unimplemented stuff.
        assert!(body.promoted.is_empty(), "Promoted rvalues are unimplemented.");
        assert!(body.upvar_decls.is_empty(), "Upvars are unimplemented.");

        // The return variable.
        self.out(|f| write!(f, "var r"))?;

        // Declare the variables.
        for (var, _) in body.var_decls.iter_enumerated() {
            self.out(|f| write!(f, ",{}", codegen::Var(var)))?;
        }

        // Declare the variables.
        for (var, _) in body.temp_decls.iter_enumerated() {
            self.out(|f| write!(f, "{}", codegen::Tmp(var)))?;
        }

        self.out(|f| write!(f, ";"))?;

        for (id, bb) in body.basic_blocks().iter_enumerated() {
            self.out(|f| write!(f, "case {}:", id.index()))?;
            // FIXME: I'm sure there is a way to avoid this clone.
            self.write_bb(bb.clone())?;
            self.out(|f| write!(f, "break;"))?;
        }

        // End the function body.
        self.out(|f| write!(f, "}}"))
    }

    fn goto(&self, bb: repr::BasicBlock) -> fmt::Result {
        self.out(|f| write!(f, "g={};continue t;", bb.index()))
    }

    fn write_bb(&self, bb: repr::BasicBlockData) -> fmt::Result {
        use rustc::mir::repr::TerminatorKind;

        for i in bb.statements {
            self.out(|f| write!(f, "{}", codegen::Statement(&i)))?;
        }

        match bb.terminator.unwrap().kind {
            TerminatorKind::Goto { target } => self.goto(target),
            TerminatorKind::If { cond, targets: (branch_true, branch_false) } => {
                self.out(|f| write!(f, "if({}){{", codegen::Operand(&cond)))?;
                self.goto(branch_true)?;
                // Else.
                self.out(|f| write!(f, "}}else{{"))?;
                self.goto(branch_false)?;
                // End the if statement.
                self.out(|f| write!(f, "}}"))
            },
            TerminatorKind::Switch { discr: disc, adt_def: def, targets } => {
                // Begin the switch statement.
                self.out(|f| write!(f, "switch({}){{", codegen::Discriminant(&disc)))?;

                // Fill in the cases.
                for (case, bb) in def.variants.iter().zip(targets) {
                    self.out(|f| write!(f, "case {}:", codegen::Literal(&repr::Literal::Value {
                        value: ConstVal::Integral(case.disr_val),
                    })))?;
                    self.goto(bb)?;
                }

                // End the statement.
                self.out(|f| write!(f, "}}"))
            },
            TerminatorKind::SwitchInt { discr: disc, values, targets, .. } => {
                // Begin the switch statement.
                self.out(|f| write!(f, "switch({}){{", codegen::LvalueGet(&disc)))?;

                // Fill in the cases.
                for (case, bb) in values.iter().zip(targets) {
                    self.out(|f| write!(f, "case {}:", codegen::Literal(&repr::Literal::Value {
                        // FIXME: I'm almost certain that there is a way to eliminate this clone,
                        // but it is messy, so it gets to stay for now.
                        value: case.clone(),
                    })))?;
                    self.goto(bb)?;
                }

                // End the statement.
                self.out(|f| write!(f, "}}"))
            },
            TerminatorKind::Resume => Ok(()),
            TerminatorKind::Return => self.out(|f| write!(f, "return r;")),
            TerminatorKind::Unreachable =>
                self.out(|f| write!(f, "alert('Cyano error: Basic block terminated with unreachable.');")),
            TerminatorKind::Drop { location, target, .. } => {
                self.out(|f| write!(f, "delete {};", codegen::LvalueGet(&location)))?;
                self.goto(target)
            },
            TerminatorKind::DropAndReplace { location, value, target, .. } => {
                self.out(|f| write!(f, "{};", codegen::LvalueSet(&location, codegen::Expr::Rvalue(&repr::Rvalue::Use(value)))))?;
                self.goto(target)
            },
            TerminatorKind::Call {
                func,
                args,
                destination,
                ..
            } => {
                if let repr::Operand::Constant(repr::Constant {
                    literal: repr::Literal::Item { def_id: _, .. },
                    ..
                }) = func {
                    // FIXME:
                    // Make sure it is compiled afterwaards.
                    // self.delayed_fns.push(def_id);

                    if let Some((return_value, bb)) = destination {
                        self.out(|f| write!(f, "{}", codegen::Expr::Call(&return_value, &args)))?;

                        // Continue to the next BB.
                        self.goto(bb)
                    } else {
                        // The function is diverging.
                        self.out(|f| write!(f, "{}(", codegen::Operand(&func)))?;

                        // List the argument.
                        for i in args {
                            self.out(|f| write!(f, "{},", codegen::Operand(&i)))?;
                        }

                        // Close the argument list.
                        self.out(|f| write!(f, ")"))
                    }
                } else {
                    unimplemented!();
                }
            }
            _ => unimplemented!(),
        }
    }
}
