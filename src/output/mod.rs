use core::default;
use std::fmt::format;

use serde::Serialize;

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
}
impl GraphOutput {
    pub fn add_line(&mut self) {
        let id = (self.expressions.list.len() + 1).to_string();

        self.expressions.list.push(Expression {
            expr_type: "expression".into(),
            id,
            color: "#3b8b2e".into(),
            latex: String::new(),
        });
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
        return GraphOutput {
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
        };
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
        return Viewport {
            xmin: -10.0,
            ymin: -8.673254835996635,
            xmax: 10.0,
            ymax: 8.673254835996635,
        };
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
        return Self {
            xmin: value.xmin.to_string(),
            ymin: value.ymin.to_string(),
            xmax: value.xmax.to_string(),
            ymax: value.ymax.to_string(),
        };
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Expression {
    #[serde(rename = "type")]
    expr_type: String,

    id: String,
    color: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    latex: String,
}

impl Expression {
    pub fn add_latex(&mut self, latex: impl ToString) {
        self.latex.push_str(&latex.to_string());
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
