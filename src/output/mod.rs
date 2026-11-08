pub mod latex;

use core::default;
use serde::Serialize;
use std::ops::Not;

use crate::errors::InterpretError;

#[derive(Clone, Debug, Serialize)]
pub struct GraphOutput {
    version: usize,

    #[serde(rename = "randomSeed")]
    seed: String,
    graph: Graph,
    pub expressions: Expressions,
    /// Not sure what these do, but desmos doesn't seem to like the json without them. Left as defaults
    #[serde(rename = "includeFunctionParametersInRandomSeed")]
    include_function_parameters_in_random_seed: bool,

    #[serde(rename = "doNotMigrateMovablePointStyle")]
    do_not_migrate_movable_point_style: bool,

    backmigrated: usize,

    #[serde(skip_serializing)]
    pub current_folder: Option<String>,
}
impl GraphOutput {
    fn push(
        &mut self,
        expr_type: &str,
        color: Option<String>,
        line_style: Option<String>,
        text: Option<String>,
        title: Option<String>,
        in_folder: bool,
    ) -> String {
        let id = (self.expressions.list.len() + 1).to_string();
        self.expressions.list.push(Expression {
            expr_type: expr_type.into(),
            id: id.clone(),
            color,
            line_style,
            latex: String::new(),
            text,
            title,
            folder_id: if in_folder {
                self.current_folder.clone()
            } else {
                None
            },
            hidden: false,
            show_label: false,
            label: None,
        });
        id
    }
    pub fn add_line(&mut self) {
        self.push("expression", Some("#3b8b2e".into()), None, None, None, true);
    }

    pub fn add_note(&mut self, text: impl ToString) {
        self.push("text", None, None, Some(text.to_string()), None, true);
    }

    /// Returns the folder's id
    pub fn add_folder(&mut self, title: impl ToString) -> String {
        self.push("folder", None, None, None, Some(title.to_string()), false)
    }
    pub fn last(&mut self) -> &mut Expression {
        self.expressions.list.last_mut().unwrap()
    }
}
impl default::Default for GraphOutput {
    fn default() -> Self {
        let viewport = Viewport {
            xmin: -10.0,
            ymin: -8.673254835996635,
            xmax: 10.0,
            ymax: 8.673254835996635,
        };
        GraphOutput {
            version: 11,
            seed: "70c9df4ab4ea8c69e13a1890891a5eaf".into(),
            graph: Graph {
                viewport,
                viewport_latex_stash: ViewportLatexStash::from(viewport),
            },
            expressions: Expressions { list: Vec::new() },

            include_function_parameters_in_random_seed: true,
            do_not_migrate_movable_point_style: true,
            backmigrated: 14,
            current_folder: None,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Expressions {
    pub list: Vec<Expression>,
}

#[derive(Serialize, Clone, Debug)]
struct Graph {
    viewport: Viewport,

    // Again, not sure what this is. Maybe when you use variables as graph bounds
    #[serde(rename = "__v12ViewportLatexStash")]
    viewport_latex_stash: ViewportLatexStash,
}

#[derive(Clone, Debug, Serialize, Copy)]
struct Viewport {
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
}

impl core::default::Default for Viewport {
    fn default() -> Self {
        Viewport {
            xmin: -10.0,
            ymin: -8.673254835996635,
            xmax: 10.0,
            ymax: 8.673254835996635,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
struct ViewportLatexStash {
    xmin: String,
    ymin: String,
    xmax: String,
    ymax: String,
}
impl From<Viewport> for ViewportLatexStash {
    fn from(value: Viewport) -> Self {
        Self {
            xmin: value.xmin.to_string(),
            ymin: value.ymin.to_string(),
            xmax: value.xmax.to_string(),
            ymax: value.ymax.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Expression {
    #[serde(rename = "type")]
    expr_type: String,

    id: String,

    #[serde(rename = "folderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lineStyle")]
    line_style: Option<String>,

    #[serde(skip_serializing_if = "String::is_empty")]
    latex: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(skip_serializing_if = "<&bool>::not")]
    hidden: bool,

    #[serde(rename = "showLabel")]
    #[serde(skip_serializing_if = "<&bool>::not")]
    show_label: bool,

    label: Option<String>,
}

impl Expression {
    pub fn add_latex(&mut self, latex: impl ToString) {
        self.latex.push_str(&latex.to_string());
    }
    pub fn add_text(&mut self, text: impl ToString) {
        let Some(t) = self.text.as_mut() else {
            return;
        };
        t.push_str(&text.to_string());
    }
    pub fn apply_annotation(&mut self, name: &str, arg: &str) -> Result<(), InterpretError> {
        match name {
            "color" => self.color = Some(arg.to_string()),
            "lineStyle" => self.line_style = Some(arg.to_string()),
            "label" => self.label = Some(arg.to_string()),
            _ => return Err(InterpretError::InvalidSyntax),
        }
        Ok(())
    }
    pub fn hide(&mut self) {
        self.hidden = true;
    }
    pub fn show_label(&mut self) {
        self.show_label = true;
    }
}

/* Example output
{
  "version": 11,
  "randomSeed": "70c9df4ab4ea8c69e13a1890891a5eaf",
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
        "color": "#c74440"
      }
    ]
  },
  "includeFunctionParametersInRandomSeed": true,
  "doNotMigrateMovablePointStyle": true,
  "backmigrated": 14
}


*/
