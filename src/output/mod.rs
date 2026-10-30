use core::default;

pub struct GraphOutput {
    version: usize,
    seed: String,
    viewport: Viewport,
    expressions: Vec<Expression>,
}
impl default::Default for GraphOutput {
    fn default() -> Self {
        return GraphOutput {
            version: 11,
            seed: "70c9df4ab4ea8c69e13a1890891a5eaf".into(),
            viewport: Viewport::default(),
            expressions: Vec::new(),
        };
    }
}
struct Expression {
    expr_type: String,
    id: usize,
    color: String,
}
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
