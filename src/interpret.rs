use crate::{
    errors::{InterpretError, InterpretResult},
    output::{GraphOutput, latex::ToLatex},
    parse::ast::{Expression, Program, Statement, StyleInfo},
};

pub struct Interpreter {
    ast: Program,
    output: GraphOutput,
}
impl Interpreter {
    pub fn new(ast: Program) -> Self {
        Self {
            ast,
            output: GraphOutput::default(),
        }
    }

    pub fn interpret(&mut self) -> Result<GraphOutput, InterpretError> {
        Self::interpret_program(&mut self.output, &self.ast)?;
        Ok(self.output.clone())
    }

    fn interpret_program(output: &mut GraphOutput, program: &Program) -> InterpretResult {
        for statement in &program.statements {
            Self::interpret_statement(output, statement)?
        }

        Ok(())
    }

    fn interpret_statement(output: &mut GraphOutput, stmt: &Statement) -> InterpretResult {
        match stmt {
            Statement::Comment(c) => {
                output.add_note(c);
                Ok(())
            }
            Statement::Namespace { name, stmts } => {
                if output.current_folder.is_some() {
                    return Err(InterpretError::NestedNamespace);
                }
                let id = output.add_folder(name);
                output.current_folder = Some(id);
                for s in stmts {
                    Self::interpret_statement(output, s)?;
                }
                output.current_folder = None;
                Ok(())
            }
            Statement::Annotated { style, stmt } => {
                Self::interpret_statement(output, stmt)?;

                match style {
                    StyleInfo::Color(c) => output
                        .expressions
                        .list
                        .last_mut()
                        .unwrap()
                        .apply_annotation("color", color_from(c)?.as_str())?,
                };
                Ok(())
            }
            _ => {
                output.add_line();
                output
                    .expressions
                    .list
                    .last_mut()
                    .unwrap()
                    .add_latex(stmt.to_latex());
                Ok(())
            }
        }
    }
}

fn color_from(expr: &Expression) -> Result<String, InterpretError> {
    let Expression::Str(s) = expr else {
        return Err(InterpretError::InvalidSyntax); // e.g. an unresolved $c or a number
    };
    let hex = s.trim_start_matches('#');
    if !hex.chars().all(|c| c.is_ascii_hexdigit()) || !matches!(hex.len(), 3 | 6) {
        return Err(InterpretError::InvalidSyntax);
    }
    Ok(format!("#{hex}"))
}
/*
{
  "version": 11,
  "randomSeed": "6a5f8353f12a137473ad868c3a8973c0",
  "graph": {
    "viewport": {
      "xmin": -10,
      "ymin": -8.673254835996635,
      "xmax": 10,
      "ymax": 8.673254835996635
    },
    "__v12ViewportLatexStash": {
      "xmin": "-10",
      "xmax": "10",
      "ymin": "-8.673254835996635",
      "ymax": "8.673254835996635"
    }
  },
  "expressions": {
    "list": [
      {
        "type": "expression",
        "id": "1",
        "color": "#c74440",
        "latex": "x\\ =\\ 1"
      },
      {
        "type": "expression",
        "id": "3",
        "color": "#388c46",
        "latex": "y\\ =\\ 1"
      },
      {
        "type": "expression",
        "id": "4",
        "color": "#6042a6",
        "latex": "a\\ =2"
      },
      {
        "type": "expression",
        "id": "5",
        "color": "#000000",
        "latex": "b\\ =\\ 3"
      },
      {
        "type": "expression",
        "id": "6",
        "color": "#c74440",
        "latex": "y\\ =\\ b\\cdot\\ x\\ +a"
      },
      {
        "type": "expression",
        "id": "2",
        "color": "#2d70b3"
      }
    ]
  },
  "includeFunctionParametersInRandomSeed": true,
  "doNotMigrateMovablePointStyle": true,
  "backmigrated": 14
} */
