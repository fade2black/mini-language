use crate::code_generator::CodeGenerator;
use crate::operator::Operator;

#[derive(Debug)]
pub enum ExprNode {
    Number(f64),
    Variable(String),
    BinaryExpr {
        op: Operator,
        lhs: Box<ExprNode>,
        rhs: Box<ExprNode>,
    },
    UnaryExpr {
        op: Operator,
        rhs: Box<ExprNode>,
    },
    CallExpr {
        callee: String,
        args: Vec<ExprNode>,
    },
    IfExpr {
        cond: Box<ExprNode>,
        then_branch: Box<ExprNode>,
        else_branch: Box<ExprNode>,
    },
}

impl ExprNode {
    pub fn create_call(callee: String, args: Vec<ExprNode>) -> ExprNode {
        ExprNode::CallExpr { callee, args }
    }

    pub fn create_binary_op(op: Operator, lhs: ExprNode, rhs: ExprNode) -> ExprNode {
        ExprNode::BinaryExpr {
            op: op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    pub fn create_unary_op(op: Operator, rhs: ExprNode) -> ExprNode {
        ExprNode::UnaryExpr {
            op: op,
            rhs: Box::new(rhs),
        }
    }

    pub fn create_if_then_else(
        cond: ExprNode,
        then_branch: ExprNode,
        else_branch: ExprNode,
    ) -> ExprNode {
        ExprNode::IfExpr {
            cond: Box::new(cond),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
    }

    pub fn to_wat(&self) -> Vec<String> {
        let mut wat = vec![];

        match self {
            ExprNode::Number(number) => wat = vec![format!("f32.const {}\n", number)],
            ExprNode::Variable(var) => wat = vec![format!("local.get ${}\n", var)],
            ExprNode::BinaryExpr { op, lhs, rhs } => {
                wat.append(&mut lhs.to_wat());
                wat.append(&mut rhs.to_wat());
                wat.push(op.to_wat());
            }
            ExprNode::UnaryExpr { op, rhs } => {
                wat.append(&mut rhs.to_wat());
                wat.push(op.to_wat());
            }

            ExprNode::CallExpr { callee, args } => {
                let builtin_funcs = CodeGenerator::builtin_funcs();

                for expr in args {
                    wat.append(&mut expr.to_wat());
                }
                if let Some(name) = builtin_funcs.get(callee.as_str()) {
                    wat.append(&mut vec![format!("{}\n", name)]);
                } else {
                    wat.append(&mut vec![format!("call ${}\n", callee)]);
                }
            }
            ExprNode::IfExpr {
                cond,
                then_branch,
                else_branch,
            } => {
                wat.append(&mut cond.to_wat());
                wat.append(&mut vec![String::from("if (result f32)\n")]);
                wat.append(&mut then_branch.to_wat());
                wat.append(&mut vec![String::from("else\n")]);
                wat.append(&mut else_branch.to_wat());
                wat.append(&mut vec![String::from("end\n")]);
            }
        };

        wat
    }
}

#[derive(Debug)]
pub struct Prototype {
    name: String,
    args: Vec<String>,
}

impl Prototype {
    pub fn new(name: String, args: Vec<String>) -> Self {
        Self { name, args }
    }

    pub fn to_wat(&self) -> Vec<String> {
        let mut proto = vec![format!("(func ${}", self.name)];

        for arg in self.args.iter() {
            proto.append(&mut vec![format!(" (param ${} f32)", arg)]);
        }
        proto.append(&mut vec![String::from(" (result f32)\n")]);
        proto
    }
}

#[derive(Debug)]
pub struct Function {
    proto: Prototype,
    body: ExprNode,
}

impl Function {
    pub fn new(proto: Prototype, body: ExprNode) -> Self {
        Self { proto, body }
    }

    pub fn to_wat(&self) -> Vec<String> {
        let mut func = vec![];

        func.append(&mut self.proto.to_wat());
        func.append(&mut self.body.to_wat());
        func.append(&mut vec![String::from(")\n")]);

        func
    }

    pub fn get_function_name(&self) -> &str {
        &self.proto.name
    }
}

#[derive(Debug)]
pub enum Ast {
    Definition(Function),
}
