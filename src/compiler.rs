use rustc::hir::def_id::DefId;
use rustc::mir::mir_map::MirMap;
use rustc::mir::repr;
use rustc::mir;
use rustc_data_structures::indexed_vec::Idx;
use std::fmt;

use codegen;

struct Compiler<'a> {
    out: fmt::Formatter<'a>,
    mir: MirMap<'a>,
    delayed_fns: Vec<DefId>,
}

impl<'a> Compiler<'a> {
    pub fn finish(&mut self) -> fmt::Result {
        // Start anonymous environment.
        write!(self.out, "function(){{d0_0();")?;

        self.write_fn(DefId::local(0).unwrap())?;

        for i in self.delayed_fns {
            self.write_fn(id)?;
        }

        // End anonymous environment.
        write!(self.out, "}}()")
    }

    fn write_fn(&mut self, id: DefId) -> fmt::Result {
        write!(self.out, "function {}(", codegen::Item(id))?;

        // Declare the arguments.
        for (arg, _) in body.arg_decls {
            write!(self.out, "{}", codegen::Arg(arg))?;
        }

        // We initialize our "goto loop", which is a jump table used to emulate gotos in
        // JavaScript. While it might seem slow at first, it is worth noting that every modern JS
        // engine will optimize this down to gotos making it zero-cost. Even without such an
        // optimization, the performance is still OK (when the cases in a switch statements is
        // above some threshold, it will almost always be transformed to a jump table, which means
        // one lookup per goto).
        write!(self.out, "){{var g=0;t:while(true){{switch g{{")?;

        let body = self.mir[id];

        // Unimplemented stuff.
        assert!(body.promoted.is_empty(), "Promoted rvalues are unimplemented.");
        assert!(body.upvar_decls.is_empty(), "Upvars are unimplemented.");

        // The return variable.
        write!(self.out, "var {},", codegen::Var(var))?;

        // Declare the variables.
        for (var, _) in body.var_decls {
            write!(self.out, "var {},", codegen::Var(var))?;
        }

        // Declare the variables.
        for (var, _) in body.temp_decls {
            write!(self.out, "{}", codegen::Tmp(var))?;
        }

        write!(self.out, ";")?;

        for (id, bb) in body.basic_blocks().iter_enumerated() {
            write!(self.out, "case {}:", id.index());
            self.write_bb(bb);
            write!(self.out, "break;");
        }

        // End the function body.
        write!(self.out, "}}")
    }

    fn goto(&mut self, bb: repr::BasicBlock) -> fmt::Result {
        write!(self.out, "g={};continue t;", bb.index())
    }

    fn write_bb(&mut self, bb: &repr::BasicBlockData) -> fmt::Result {
        use repr::Terminator;

        for i in bb.statements {
            write!(self.out, "{}", codegen::BasicBlock(i))?;
        }

        match bb.terminator.unwrap() {
            Terminator::Goto { target } => self.goto(target),
            Terminator::If { cond, (branch_true, branch_false): target } => {
                write!(self.out, "if({}){{", codegen::Operand(cond))?;
                self.goto(branch_true)?;
                // Else.
                write!(self.out, "}}else{{")?;
                self.goto(branch_false)?;
                // End the if statement.
                write!(self.out, "}}")?;
            },
            Terminator::Switch { disc: discr, def: adt_defs targets } => {
                // Begin the switch statement.
                write!(self.out, "switch({}){{", codegen::Discriminant(disc))?;

                // Fill in the cases.
                for (case, bb) in def.variants.iter().zip(targets) {
                    write!(self.out, "case {}:", case.disr_value.to_u32())?;
                    self.goto(bb)?;
                }

                // End the statement.
                write!(self.out, "}}")
            },
            Terminator::IntSwitch { disc: discr, values, targets, .. } => {
                // Begin the switch statement.
                write!(self.out, "switch({}){{", codegen::LvalueGet(disc))?;

                // Fill in the cases.
                for (case, bb) in values.iter().zip(targets) {
                    write!(self.out, "case {}:", codegen::Literal::from(case))?;
                    self.goto(bb)?;
                }

                // End the statement.
                write!(self.out, "}}")
            },
            Terminator::Resume => Ok(()),
            Terminator::Return => write!(self.out, "return r;"),
            Terminator::Unreachable =>
                write!(self.out, "alert('Cyano error: Basic block terminated with unreachable.');"),
            Terminator::Drop { location, target, .. } => {
                write!(self.out, "delete {};", codegen::LvalueGet(location))?;
                self.goto(target)
            },
            Terminator::DropAndReplace { location, value, target, .. } => {
                write!(self.out, "{};", codegen::LvalueSet(location, codegen::Rvalue::from(value)))?;
                self.goto(target)?;
            },
            Terminator::Call { func: func @ repr::Operand::Constant(repr::Literal::Item { def_id }, args, destination, .. } => {
                // Make sure it is compiled afterwaards.
                self.delayed_fns.push(def_id);

                if let Some((return_value, bb)) = destination {
                    // Asign the result to some lvalue.
                    write!(self.out, "{}={}(", codegen::Lvalue(return_value), codegen::Operand(func))?;

                    // List the argument.
                    for i in args {
                        write!(self.out, "{},", codegen::Operand(i))?;
                    }

                    // Close the argument list.
                    write!(self.out, ")")?;

                    // Continue to the next BB.
                    self.goto(bb)
                } else {
                    // The function is diverging.
                    write!(self.out, "{}(", codegen::Operand(func))?;

                    // List the argument.
                    for i in args {
                        write!(self.out, "{},", codegen::Operand(i))?;
                    }

                    // Close the argument list.
                    write!(self.out, ")")
                }
            }
            _ => unimplemented!(),
        }
    }
}
