use projstd::log;
use std::process;
use std::io::{self, BufRead};

const SOURCE_CODE: &'static str = r#"
{
  "type": "Program",
  "start": 0,
  "end": 616,
  "body": [
    {
      "type": "FunctionDeclaration",
      "start": 0,
      "end": 50,
      "id": {
        "type": "Identifier",
        "start": 9,
        "end": 12,
        "name": "abs"
      },
      "expression": false,
      "generator": false,
      "async": false,
      "params": [
        {
          "type": "Identifier",
          "start": 13,
          "end": 14,
          "name": "x"
        }
      ],
      "body": {
        "type": "BlockStatement",
        "start": 16,
        "end": 50,
        "body": [
          {
            "type": "ReturnStatement",
            "start": 22,
            "end": 48,
            "argument": {
              "type": "ConditionalExpression",
              "start": 29,
              "end": 47,
              "test": {
                "type": "BinaryExpression",
                "start": 29,
                "end": 35,
                "left": {
                  "type": "Identifier",
                  "start": 29,
                  "end": 30,
                  "name": "x"
                },
                "operator": ">=",
                "right": {
                  "type": "Literal",
                  "start": 34,
                  "end": 35,
                  "value": 0,
                  "raw": "0"
                }
              },
              "consequent": {
                "type": "Identifier",
                "start": 38,
                "end": 39,
                "name": "x"
              },
              "alternate": {
                "type": "BinaryExpression",
                "start": 42,
                "end": 47,
                "left": {
                  "type": "Literal",
                  "start": 42,
                  "end": 43,
                  "value": 0,
                  "raw": "0"
                },
                "operator": "-",
                "right": {
                  "type": "Identifier",
                  "start": 46,
                  "end": 47,
                  "name": "x"
                }
              }
            }
          }
        ]
      }
    },
    {
      "type": "FunctionDeclaration",
      "start": 51,
      "end": 91,
      "id": {
        "type": "Identifier",
        "start": 60,
        "end": 66,
        "name": "square"
      },
      "expression": false,
      "generator": false,
      "async": false,
      "params": [
        {
          "type": "Identifier",
          "start": 67,
          "end": 68,
          "name": "x"
        }
      ],
      "body": {
        "type": "BlockStatement",
        "start": 70,
        "end": 91,
        "body": [
          {
            "type": "ReturnStatement",
            "start": 76,
            "end": 89,
            "argument": {
              "type": "BinaryExpression",
              "start": 83,
              "end": 88,
              "left": {
                "type": "Identifier",
                "start": 83,
                "end": 84,
                "name": "x"
              },
              "operator": "*",
              "right": {
                "type": "Identifier",
                "start": 87,
                "end": 88,
                "name": "x"
              }
            }
          }
        ]
      }
    },
    {
      "type": "FunctionDeclaration",
      "start": 92,
      "end": 141,
      "id": {
        "type": "Identifier",
        "start": 101,
        "end": 108,
        "name": "average"
      },
      "expression": false,
      "generator": false,
      "async": false,
      "params": [
        {
          "type": "Identifier",
          "start": 109,
          "end": 110,
          "name": "x"
        },
        {
          "type": "Identifier",
          "start": 111,
          "end": 112,
          "name": "y"
        }
      ],
      "body": {
        "type": "BlockStatement",
        "start": 114,
        "end": 141,
        "body": [
          {
            "type": "ReturnStatement",
            "start": 120,
            "end": 139,
            "argument": {
              "type": "BinaryExpression",
              "start": 127,
              "end": 138,
              "left": {
                "type": "BinaryExpression",
                "start": 128,
                "end": 133,
                "left": {
                  "type": "Identifier",
                  "start": 128,
                  "end": 129,
                  "name": "x"
                },
                "operator": "+",
                "right": {
                  "type": "Identifier",
                  "start": 132,
                  "end": 133,
                  "name": "y"
                }
              },
              "operator": "/",
              "right": {
                "type": "Literal",
                "start": 137,
                "end": 138,
                "value": 2,
                "raw": "2"
              }
            }
          }
        ]
      }
    },
    {
      "type": "FunctionDeclaration",
      "start": 142,
      "end": 602,
      "id": {
        "type": "Identifier",
        "start": 151,
        "end": 155,
        "name": "sqrt"
      },
      "expression": false,
      "generator": false,
      "async": false,
      "params": [
        {
          "type": "Identifier",
          "start": 156,
          "end": 157,
          "name": "x"
        }
      ],
      "body": {
        "type": "BlockStatement",
        "start": 159,
        "end": 602,
        "body": [
          {
            "type": "FunctionDeclaration",
            "start": 165,
            "end": 250,
            "id": {
              "type": "Identifier",
              "start": 174,
              "end": 185,
              "name": "good_enough"
            },
            "expression": false,
            "generator": false,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "start": 186,
                "end": 191,
                "name": "guess"
              },
              {
                "type": "Identifier",
                "start": 193,
                "end": 194,
                "name": "x"
              }
            ],
            "body": {
              "type": "BlockStatement",
              "start": 196,
              "end": 250,
              "body": [
                {
                  "type": "ReturnStatement",
                  "start": 206,
                  "end": 244,
                  "argument": {
                    "type": "BinaryExpression",
                    "start": 213,
                    "end": 243,
                    "left": {
                      "type": "CallExpression",
                      "start": 213,
                      "end": 235,
                      "callee": {
                        "type": "Identifier",
                        "start": 213,
                        "end": 216,
                        "name": "abs"
                      },
                      "arguments": [
                        {
                          "type": "BinaryExpression",
                          "start": 217,
                          "end": 234,
                          "left": {
                            "type": "CallExpression",
                            "start": 217,
                            "end": 230,
                            "callee": {
                              "type": "Identifier",
                              "start": 217,
                              "end": 223,
                              "name": "square"
                            },
                            "arguments": [
                              {
                                "type": "Identifier",
                                "start": 224,
                                "end": 229,
                                "name": "guess"
                              }
                            ],
                            "optional": false
                          },
                          "operator": "-",
                          "right": {
                            "type": "Identifier",
                            "start": 233,
                            "end": 234,
                            "name": "x"
                          }
                        }
                      ],
                      "optional": false
                    },
                    "operator": "<",
                    "right": {
                      "type": "Literal",
                      "start": 238,
                      "end": 243,
                      "value": 0.001,
                      "raw": "0.001"
                    }
                  }
                }
              ]
            }
          },
          {
            "type": "FunctionDeclaration",
            "start": 255,
            "end": 331,
            "id": {
              "type": "Identifier",
              "start": 264,
              "end": 271,
              "name": "improve"
            },
            "expression": false,
            "generator": false,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "start": 272,
                "end": 277,
                "name": "guess"
              },
              {
                "type": "Identifier",
                "start": 279,
                "end": 280,
                "name": "x"
              }
            ],
            "body": {
              "type": "BlockStatement",
              "start": 282,
              "end": 331,
              "body": [
                {
                  "type": "ReturnStatement",
                  "start": 292,
                  "end": 325,
                  "argument": {
                    "type": "CallExpression",
                    "start": 299,
                    "end": 324,
                    "callee": {
                      "type": "Identifier",
                      "start": 299,
                      "end": 306,
                      "name": "average"
                    },
                    "arguments": [
                      {
                        "type": "Identifier",
                        "start": 307,
                        "end": 312,
                        "name": "guess"
                      },
                      {
                        "type": "BinaryExpression",
                        "start": 314,
                        "end": 323,
                        "left": {
                          "type": "Identifier",
                          "start": 314,
                          "end": 315,
                          "name": "x"
                        },
                        "operator": "/",
                        "right": {
                          "type": "Identifier",
                          "start": 318,
                          "end": 323,
                          "name": "guess"
                        }
                      }
                    ],
                    "optional": false
                  }
                }
              ]
            }
          },
          {
            "type": "FunctionDeclaration",
            "start": 336,
            "end": 570,
            "id": {
              "type": "Identifier",
              "start": 345,
              "end": 354,
              "name": "sqrt_iter"
            },
            "expression": false,
            "generator": false,
            "async": false,
            "params": [
              {
                "type": "Identifier",
                "start": 355,
                "end": 360,
                "name": "guess"
              },
              {
                "type": "Identifier",
                "start": 362,
                "end": 363,
                "name": "x"
              }
            ],
            "body": {
              "type": "BlockStatement",
              "start": 365,
              "end": 570,
              "body": [
                {
                  "type": "ReturnStatement",
                  "start": 375,
                  "end": 564,
                  "argument": {
                    "type": "ConditionalExpression",
                    "start": 382,
                    "end": 563,
                    "test": {
                      "type": "CallExpression",
                      "start": 382,
                      "end": 403,
                      "callee": {
                        "type": "Identifier",
                        "start": 382,
                        "end": 393,
                        "name": "good_enough"
                      },
                      "arguments": [
                        {
                          "type": "Identifier",
                          "start": 394,
                          "end": 399,
                          "name": "guess"
                        },
                        {
                          "type": "Identifier",
                          "start": 401,
                          "end": 402,
                          "name": "x"
                        }
                      ],
                      "optional": false
                    },
                    "consequent": {
                      "type": "Identifier",
                      "start": 425,
                      "end": 430,
                      "name": "guess"
                    },
                    "alternate": {
                      "type": "CallExpression",
                      "start": 499,
                      "end": 563,
                      "callee": {
                        "type": "Identifier",
                        "start": 499,
                        "end": 508,
                        "name": "sqrt_iter"
                      },
                      "arguments": [
                        {
                          "type": "CallExpression",
                          "start": 509,
                          "end": 559,
                          "callee": {
                            "type": "Identifier",
                            "start": 509,
                            "end": 516,
                            "name": "improve"
                          },
                          "arguments": [
                            {
                              "type": "Identifier",
                              "start": 550,
                              "end": 555,
                              "name": "guess"
                            },
                            {
                              "type": "Identifier",
                              "start": 557,
                              "end": 558,
                              "name": "x"
                            }
                          ],
                          "optional": false
                        },
                        {
                          "type": "Identifier",
                          "start": 561,
                          "end": 562,
                          "name": "x"
                        }
                      ],
                      "optional": false
                    }
                  }
                }
              ]
            }
          },
          {
            "type": "ReturnStatement",
            "start": 575,
            "end": 600,
            "argument": {
              "type": "CallExpression",
              "start": 582,
              "end": 599,
              "callee": {
                "type": "Identifier",
                "start": 582,
                "end": 591,
                "name": "sqrt_iter"
              },
              "arguments": [
                {
                  "type": "Literal",
                  "start": 592,
                  "end": 595,
                  "value": 1,
                  "raw": "1.0"
                },
                {
                  "type": "Identifier",
                  "start": 597,
                  "end": 598,
                  "name": "x"
                }
              ],
              "optional": false
            }
          }
        ]
      }
    },
    {
      "type": "ExpressionStatement",
      "start": 604,
      "end": 612,
      "expression": {
        "type": "CallExpression",
        "start": 604,
        "end": 611,
        "callee": {
          "type": "Identifier",
          "start": 604,
          "end": 608,
          "name": "sqrt"
        },
        "arguments": [
          {
            "type": "Literal",
            "start": 609,
            "end": 610,
            "value": 5,
            "raw": "5"
          }
        ],
        "optional": false
      }
    }
  ],
  "sourceType": "script"
}

"#;

const IMPORT_MATH_FFI: &'static str = r#"@SourceImports
math_sin math sin number number
"#;
const IMPORT_MATH: &'static str = r#"
{"type":"Program","start":0,"end":1617,"loc":{"start":{"line":1,"column":0},"end":{"line":59,"column":0}},"body":[{"type":"ImportDeclaration","start":0,"end":32,"loc":{"start":{"line":1,"column":0},"end":{"line":1,"column":32}},"specifiers":[{"type":"ImportSpecifier","start":9,"end":17,"loc":{"start":{"line":1,"column":9},"end":{"line":1,"column":17}},"imported":{"type":"Identifier","start":9,"end":17,"loc":{"start":{"line":1,"column":9},"end":{"line":1,"column":17}},"name":"Infinity"},"local":{"type":"Identifier","start":9,"end":17,"loc":{"start":{"line":1,"column":9},"end":{"line":1,"column":17}},"name":"Infinity"}}],"source":{"type":"Literal","start":25,"end":31,"loc":{"start":{"line":1,"column":25},"end":{"line":1,"column":31}},"value":"misc","raw":"\"misc\""}},{"type":"ImportDeclaration","start":33,"end":69,"loc":{"start":{"line":2,"column":0},"end":{"line":2,"column":36}},"specifiers":[{"type":"ImportSpecifier","start":42,"end":50,"loc":{"start":{"line":2,"column":9},"end":{"line":2,"column":17}},"imported":{"type":"Identifier","start":42,"end":50,"loc":{"start":{"line":2,"column":9},"end":{"line":2,"column":17}},"name":"math_sin"},"local":{"type":"Identifier","start":42,"end":50,"loc":{"start":{"line":2,"column":9},"end":{"line":2,"column":17}},"name":"math_sin"}}],"source":{"type":"Literal","start":58,"end":68,"loc":{"start":{"line":2,"column":25},"end":{"line":2,"column":35}},"value":"math.ffi","raw":"\"math.ffi\""}},{"type":"VariableDeclaration","start":71,"end":104,"loc":{"start":{"line":4,"column":0},"end":{"line":4,"column":33}},"declarations":[{"type":"VariableDeclarator","start":77,"end":103,"loc":{"start":{"line":4,"column":6},"end":{"line":4,"column":32}},"id":{"type":"Identifier","start":77,"end":83,"loc":{"start":{"line":4,"column":6},"end":{"line":4,"column":12}},"name":"math_E"},"init":{"type":"Literal","start":86,"end":103,"loc":{"start":{"line":4,"column":15},"end":{"line":4,"column":32}},"value":2.718281828459045,"raw":"2.718281828459045"}}],"kind":"const"},{"type":"VariableDeclaration","start":105,"end":141,"loc":{"start":{"line":5,"column":0},"end":{"line":5,"column":36}},"declarations":[{"type":"VariableDeclarator","start":111,"end":140,"loc":{"start":{"line":5,"column":6},"end":{"line":5,"column":35}},"id":{"type":"Identifier","start":111,"end":119,"loc":{"start":{"line":5,"column":6},"end":{"line":5,"column":14}},"name":"math_LN2"},"init":{"type":"Literal","start":122,"end":140,"loc":{"start":{"line":5,"column":17},"end":{"line":5,"column":35}},"value":0.6931471805599453,"raw":"0.6931471805599453"}}],"kind":"const"},{"type":"VariableDeclaration","start":142,"end":178,"loc":{"start":{"line":6,"column":0},"end":{"line":6,"column":36}},"declarations":[{"type":"VariableDeclarator","start":148,"end":177,"loc":{"start":{"line":6,"column":6},"end":{"line":6,"column":35}},"id":{"type":"Identifier","start":148,"end":157,"loc":{"start":{"line":6,"column":6},"end":{"line":6,"column":15}},"name":"math_LN10"},"init":{"type":"Literal","start":160,"end":177,"loc":{"start":{"line":6,"column":18},"end":{"line":6,"column":35}},"value":2.302585092994046,"raw":"2.302585092994046"}}],"kind":"const"},{"type":"VariableDeclaration","start":179,"end":217,"loc":{"start":{"line":7,"column":0},"end":{"line":7,"column":38}},"declarations":[{"type":"VariableDeclarator","start":185,"end":216,"loc":{"start":{"line":7,"column":6},"end":{"line":7,"column":37}},"id":{"type":"Identifier","start":185,"end":195,"loc":{"start":{"line":7,"column":6},"end":{"line":7,"column":16}},"name":"math_LOG2E"},"init":{"type":"Literal","start":198,"end":216,"loc":{"start":{"line":7,"column":19},"end":{"line":7,"column":37}},"value":1.4426950408889634,"raw":"1.4426950408889634"}}],"kind":"const"},{"type":"VariableDeclaration","start":218,"end":257,"loc":{"start":{"line":8,"column":0},"end":{"line":8,"column":39}},"declarations":[{"type":"VariableDeclarator","start":224,"end":256,"loc":{"start":{"line":8,"column":6},"end":{"line":8,"column":38}},"id":{"type":"Identifier","start":224,"end":235,"loc":{"start":{"line":8,"column":6},"end":{"line":8,"column":17}},"name":"math_LOG10E"},"init":{"type":"Literal","start":238,"end":256,"loc":{"start":{"line":8,"column":20},"end":{"line":8,"column":38}},"value":0.4342944819032518,"raw":"0.4342944819032518"}}],"kind":"const"},{"type":"VariableDeclaration","start":258,"end":293,"loc":{"start":{"line":9,"column":0},"end":{"line":9,"column":35}},"declarations":[{"type":"VariableDeclarator","start":264,"end":292,"loc":{"start":{"line":9,"column":6},"end":{"line":9,"column":34}},"id":{"type":"Identifier","start":264,"end":271,"loc":{"start":{"line":9,"column":6},"end":{"line":9,"column":13}},"name":"math_PI"},"init":{"type":"Literal","start":274,"end":292,"loc":{"start":{"line":9,"column":16},"end":{"line":9,"column":34}},"value":3.141592653589793,"raw":"3.1415926535897932"}}],"kind":"const"},{"type":"VariableDeclaration","start":294,"end":334,"loc":{"start":{"line":10,"column":0},"end":{"line":10,"column":40}},"declarations":[{"type":"VariableDeclarator","start":300,"end":333,"loc":{"start":{"line":10,"column":6},"end":{"line":10,"column":39}},"id":{"type":"Identifier","start":300,"end":312,"loc":{"start":{"line":10,"column":6},"end":{"line":10,"column":18}},"name":"math_SQRT1_2"},"init":{"type":"Literal","start":315,"end":333,"loc":{"start":{"line":10,"column":21},"end":{"line":10,"column":39}},"value":0.7071067811865476,"raw":"0.7071067811865476"}}],"kind":"const"},{"type":"VariableDeclaration","start":335,"end":373,"loc":{"start":{"line":11,"column":0},"end":{"line":11,"column":38}},"declarations":[{"type":"VariableDeclarator","start":341,"end":372,"loc":{"start":{"line":11,"column":6},"end":{"line":11,"column":37}},"id":{"type":"Identifier","start":341,"end":351,"loc":{"start":{"line":11,"column":6},"end":{"line":11,"column":16}},"name":"math_SQRT2"},"init":{"type":"Literal","start":354,"end":372,"loc":{"start":{"line":11,"column":19},"end":{"line":11,"column":37}},"value":1.4142135623730951,"raw":"1.4142135623730951"}}],"kind":"const"},{"type":"ExpressionStatement","start":632,"end":676,"loc":{"start":{"line":18,"column":0},"end":{"line":18,"column":44}},"expression":{"type":"AssignmentExpression","start":632,"end":675,"loc":{"start":{"line":18,"column":0},"end":{"line":18,"column":43}},"operator":"=","left":{"type":"Identifier","start":632,"end":644,"loc":{"start":{"line":18,"column":0},"end":{"line":18,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":647,"end":675,"loc":{"start":{"line":18,"column":15},"end":{"line":18,"column":43}},"value":"direct;constraint=x:number","raw":"\"direct;constraint=x:number\""}}},{"type":"FunctionDeclaration","start":677,"end":729,"loc":{"start":{"line":19,"column":0},"end":{"line":21,"column":1}},"id":{"type":"Identifier","start":686,"end":694,"loc":{"start":{"line":19,"column":9},"end":{"line":19,"column":17}},"name":"math_abs"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":695,"end":696,"loc":{"start":{"line":19,"column":18},"end":{"line":19,"column":19}},"name":"x"}],"body":{"type":"BlockStatement","start":698,"end":729,"loc":{"start":{"line":19,"column":21},"end":{"line":21,"column":1}},"body":[{"type":"ReturnStatement","start":704,"end":727,"loc":{"start":{"line":20,"column":4},"end":{"line":20,"column":27}},"argument":{"type":"ConditionalExpression","start":711,"end":726,"loc":{"start":{"line":20,"column":11},"end":{"line":20,"column":26}},"test":{"type":"BinaryExpression","start":711,"end":717,"loc":{"start":{"line":20,"column":11},"end":{"line":20,"column":17}},"left":{"type":"Identifier","start":711,"end":712,"loc":{"start":{"line":20,"column":11},"end":{"line":20,"column":12}},"name":"x"},"operator":">=","right":{"type":"Literal","start":716,"end":717,"loc":{"start":{"line":20,"column":16},"end":{"line":20,"column":17}},"value":0,"raw":"0"}},"consequent":{"type":"Identifier","start":720,"end":721,"loc":{"start":{"line":20,"column":20},"end":{"line":20,"column":21}},"name":"x"},"alternate":{"type":"UnaryExpression","start":724,"end":726,"loc":{"start":{"line":20,"column":24},"end":{"line":20,"column":26}},"operator":"-","prefix":true,"argument":{"type":"Identifier","start":725,"end":726,"loc":{"start":{"line":20,"column":25},"end":{"line":20,"column":26}},"name":"x"}}}}]}},{"type":"ExpressionStatement","start":731,"end":775,"loc":{"start":{"line":23,"column":0},"end":{"line":23,"column":44}},"expression":{"type":"AssignmentExpression","start":731,"end":774,"loc":{"start":{"line":23,"column":0},"end":{"line":23,"column":43}},"operator":"=","left":{"type":"Identifier","start":731,"end":743,"loc":{"start":{"line":23,"column":0},"end":{"line":23,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":746,"end":774,"loc":{"start":{"line":23,"column":15},"end":{"line":23,"column":43}},"value":"direct;constraint=x:number","raw":"\"direct;constraint=x:number\""}}},{"type":"FunctionDeclaration","start":776,"end":840,"loc":{"start":{"line":24,"column":0},"end":{"line":26,"column":1}},"id":{"type":"Identifier","start":785,"end":794,"loc":{"start":{"line":24,"column":9},"end":{"line":24,"column":18}},"name":"math_sign"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":795,"end":796,"loc":{"start":{"line":24,"column":19},"end":{"line":24,"column":20}},"name":"x"}],"body":{"type":"BlockStatement","start":798,"end":840,"loc":{"start":{"line":24,"column":22},"end":{"line":26,"column":1}},"body":[{"type":"ReturnStatement","start":804,"end":838,"loc":{"start":{"line":25,"column":4},"end":{"line":25,"column":38}},"argument":{"type":"ConditionalExpression","start":811,"end":837,"loc":{"start":{"line":25,"column":11},"end":{"line":25,"column":37}},"test":{"type":"BinaryExpression","start":811,"end":816,"loc":{"start":{"line":25,"column":11},"end":{"line":25,"column":16}},"left":{"type":"Identifier","start":811,"end":812,"loc":{"start":{"line":25,"column":11},"end":{"line":25,"column":12}},"name":"x"},"operator":">","right":{"type":"Literal","start":815,"end":816,"loc":{"start":{"line":25,"column":15},"end":{"line":25,"column":16}},"value":0,"raw":"0"}},"consequent":{"type":"Literal","start":819,"end":820,"loc":{"start":{"line":25,"column":19},"end":{"line":25,"column":20}},"value":1,"raw":"1"},"alternate":{"type":"ConditionalExpression","start":823,"end":837,"loc":{"start":{"line":25,"column":23},"end":{"line":25,"column":37}},"test":{"type":"BinaryExpression","start":823,"end":828,"loc":{"start":{"line":25,"column":23},"end":{"line":25,"column":28}},"left":{"type":"Identifier","start":823,"end":824,"loc":{"start":{"line":25,"column":23},"end":{"line":25,"column":24}},"name":"x"},"operator":"<","right":{"type":"Literal","start":827,"end":828,"loc":{"start":{"line":25,"column":27},"end":{"line":25,"column":28}},"value":0,"raw":"0"}},"consequent":{"type":"UnaryExpression","start":831,"end":833,"loc":{"start":{"line":25,"column":31},"end":{"line":25,"column":33}},"operator":"-","prefix":true,"argument":{"type":"Literal","start":832,"end":833,"loc":{"start":{"line":25,"column":32},"end":{"line":25,"column":33}},"value":1,"raw":"1"}},"alternate":{"type":"Literal","start":836,"end":837,"loc":{"start":{"line":25,"column":36},"end":{"line":25,"column":37}},"value":0,"raw":"0"}}}}]}},{"type":"ExpressionStatement","start":941,"end":965,"loc":{"start":{"line":30,"column":0},"end":{"line":30,"column":24}},"expression":{"type":"AssignmentExpression","start":941,"end":964,"loc":{"start":{"line":30,"column":0},"end":{"line":30,"column":23}},"operator":"=","left":{"type":"Identifier","start":941,"end":953,"loc":{"start":{"line":30,"column":0},"end":{"line":30,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":956,"end":964,"loc":{"start":{"line":30,"column":15},"end":{"line":30,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":966,"end":1011,"loc":{"start":{"line":31,"column":0},"end":{"line":33,"column":1}},"id":{"type":"Identifier","start":975,"end":983,"loc":{"start":{"line":31,"column":9},"end":{"line":31,"column":17}},"name":"math_max"},"expression":false,"generator":false,"params":[],"body":{"type":"BlockStatement","start":986,"end":1011,"loc":{"start":{"line":31,"column":20},"end":{"line":33,"column":1}},"body":[{"type":"ReturnStatement","start":992,"end":1009,"loc":{"start":{"line":32,"column":4},"end":{"line":32,"column":21}},"argument":{"type":"UnaryExpression","start":999,"end":1008,"loc":{"start":{"line":32,"column":11},"end":{"line":32,"column":20}},"operator":"-","prefix":true,"argument":{"type":"Identifier","start":1000,"end":1008,"loc":{"start":{"line":32,"column":12},"end":{"line":32,"column":20}},"name":"Infinity"}}}]}},{"type":"ExpressionStatement","start":1012,"end":1056,"loc":{"start":{"line":34,"column":0},"end":{"line":34,"column":44}},"expression":{"type":"AssignmentExpression","start":1012,"end":1055,"loc":{"start":{"line":34,"column":0},"end":{"line":34,"column":43}},"operator":"=","left":{"type":"Identifier","start":1012,"end":1024,"loc":{"start":{"line":34,"column":0},"end":{"line":34,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1027,"end":1055,"loc":{"start":{"line":34,"column":15},"end":{"line":34,"column":43}},"value":"direct;constraint=x:number","raw":"\"direct;constraint=x:number\""}}},{"type":"FunctionDeclaration","start":1057,"end":1095,"loc":{"start":{"line":35,"column":0},"end":{"line":37,"column":1}},"id":{"type":"Identifier","start":1066,"end":1074,"loc":{"start":{"line":35,"column":9},"end":{"line":35,"column":17}},"name":"math_max"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1075,"end":1076,"loc":{"start":{"line":35,"column":18},"end":{"line":35,"column":19}},"name":"x"}],"body":{"type":"BlockStatement","start":1078,"end":1095,"loc":{"start":{"line":35,"column":21},"end":{"line":37,"column":1}},"body":[{"type":"ReturnStatement","start":1084,"end":1093,"loc":{"start":{"line":36,"column":4},"end":{"line":36,"column":13}},"argument":{"type":"Identifier","start":1091,"end":1092,"loc":{"start":{"line":36,"column":11},"end":{"line":36,"column":12}},"name":"x"}}]}},{"type":"ExpressionStatement","start":1096,"end":1149,"loc":{"start":{"line":38,"column":0},"end":{"line":38,"column":53}},"expression":{"type":"AssignmentExpression","start":1096,"end":1148,"loc":{"start":{"line":38,"column":0},"end":{"line":38,"column":52}},"operator":"=","left":{"type":"Identifier","start":1096,"end":1108,"loc":{"start":{"line":38,"column":0},"end":{"line":38,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1111,"end":1148,"loc":{"start":{"line":38,"column":15},"end":{"line":38,"column":52}},"value":"direct;constraint=x:number,y:number","raw":"\"direct;constraint=x:number,y:number\""}}},{"type":"FunctionDeclaration","start":1150,"end":1203,"loc":{"start":{"line":39,"column":0},"end":{"line":41,"column":1}},"id":{"type":"Identifier","start":1159,"end":1167,"loc":{"start":{"line":39,"column":9},"end":{"line":39,"column":17}},"name":"math_max"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1168,"end":1169,"loc":{"start":{"line":39,"column":18},"end":{"line":39,"column":19}},"name":"x"},{"type":"Identifier","start":1171,"end":1172,"loc":{"start":{"line":39,"column":21},"end":{"line":39,"column":22}},"name":"y"}],"body":{"type":"BlockStatement","start":1174,"end":1203,"loc":{"start":{"line":39,"column":24},"end":{"line":41,"column":1}},"body":[{"type":"ReturnStatement","start":1180,"end":1201,"loc":{"start":{"line":40,"column":4},"end":{"line":40,"column":25}},"argument":{"type":"ConditionalExpression","start":1187,"end":1200,"loc":{"start":{"line":40,"column":11},"end":{"line":40,"column":24}},"test":{"type":"BinaryExpression","start":1187,"end":1192,"loc":{"start":{"line":40,"column":11},"end":{"line":40,"column":16}},"left":{"type":"Identifier","start":1187,"end":1188,"loc":{"start":{"line":40,"column":11},"end":{"line":40,"column":12}},"name":"x"},"operator":"<","right":{"type":"Identifier","start":1191,"end":1192,"loc":{"start":{"line":40,"column":15},"end":{"line":40,"column":16}},"name":"y"}},"consequent":{"type":"Identifier","start":1195,"end":1196,"loc":{"start":{"line":40,"column":19},"end":{"line":40,"column":20}},"name":"y"},"alternate":{"type":"Identifier","start":1199,"end":1200,"loc":{"start":{"line":40,"column":23},"end":{"line":40,"column":24}},"name":"x"}}}]}},{"type":"ExpressionStatement","start":1205,"end":1229,"loc":{"start":{"line":43,"column":0},"end":{"line":43,"column":24}},"expression":{"type":"AssignmentExpression","start":1205,"end":1228,"loc":{"start":{"line":43,"column":0},"end":{"line":43,"column":23}},"operator":"=","left":{"type":"Identifier","start":1205,"end":1217,"loc":{"start":{"line":43,"column":0},"end":{"line":43,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1220,"end":1228,"loc":{"start":{"line":43,"column":15},"end":{"line":43,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":1230,"end":1274,"loc":{"start":{"line":44,"column":0},"end":{"line":46,"column":1}},"id":{"type":"Identifier","start":1239,"end":1247,"loc":{"start":{"line":44,"column":9},"end":{"line":44,"column":17}},"name":"math_min"},"expression":false,"generator":false,"params":[],"body":{"type":"BlockStatement","start":1250,"end":1274,"loc":{"start":{"line":44,"column":20},"end":{"line":46,"column":1}},"body":[{"type":"ReturnStatement","start":1256,"end":1272,"loc":{"start":{"line":45,"column":4},"end":{"line":45,"column":20}},"argument":{"type":"Identifier","start":1263,"end":1271,"loc":{"start":{"line":45,"column":11},"end":{"line":45,"column":19}},"name":"Infinity"}}]}},{"type":"ExpressionStatement","start":1275,"end":1319,"loc":{"start":{"line":47,"column":0},"end":{"line":47,"column":44}},"expression":{"type":"AssignmentExpression","start":1275,"end":1318,"loc":{"start":{"line":47,"column":0},"end":{"line":47,"column":43}},"operator":"=","left":{"type":"Identifier","start":1275,"end":1287,"loc":{"start":{"line":47,"column":0},"end":{"line":47,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1290,"end":1318,"loc":{"start":{"line":47,"column":15},"end":{"line":47,"column":43}},"value":"direct;constraint=x:number","raw":"\"direct;constraint=x:number\""}}},{"type":"FunctionDeclaration","start":1320,"end":1358,"loc":{"start":{"line":48,"column":0},"end":{"line":50,"column":1}},"id":{"type":"Identifier","start":1329,"end":1337,"loc":{"start":{"line":48,"column":9},"end":{"line":48,"column":17}},"name":"math_min"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1338,"end":1339,"loc":{"start":{"line":48,"column":18},"end":{"line":48,"column":19}},"name":"x"}],"body":{"type":"BlockStatement","start":1341,"end":1358,"loc":{"start":{"line":48,"column":21},"end":{"line":50,"column":1}},"body":[{"type":"ReturnStatement","start":1347,"end":1356,"loc":{"start":{"line":49,"column":4},"end":{"line":49,"column":13}},"argument":{"type":"Identifier","start":1354,"end":1355,"loc":{"start":{"line":49,"column":11},"end":{"line":49,"column":12}},"name":"x"}}]}},{"type":"ExpressionStatement","start":1359,"end":1412,"loc":{"start":{"line":51,"column":0},"end":{"line":51,"column":53}},"expression":{"type":"AssignmentExpression","start":1359,"end":1411,"loc":{"start":{"line":51,"column":0},"end":{"line":51,"column":52}},"operator":"=","left":{"type":"Identifier","start":1359,"end":1371,"loc":{"start":{"line":51,"column":0},"end":{"line":51,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1374,"end":1411,"loc":{"start":{"line":51,"column":15},"end":{"line":51,"column":52}},"value":"direct;constraint=x:number,y:number","raw":"\"direct;constraint=x:number,y:number\""}}},{"type":"FunctionDeclaration","start":1413,"end":1466,"loc":{"start":{"line":52,"column":0},"end":{"line":54,"column":1}},"id":{"type":"Identifier","start":1422,"end":1430,"loc":{"start":{"line":52,"column":9},"end":{"line":52,"column":17}},"name":"math_min"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1431,"end":1432,"loc":{"start":{"line":52,"column":18},"end":{"line":52,"column":19}},"name":"x"},{"type":"Identifier","start":1434,"end":1435,"loc":{"start":{"line":52,"column":21},"end":{"line":52,"column":22}},"name":"y"}],"body":{"type":"BlockStatement","start":1437,"end":1466,"loc":{"start":{"line":52,"column":24},"end":{"line":54,"column":1}},"body":[{"type":"ReturnStatement","start":1443,"end":1464,"loc":{"start":{"line":53,"column":4},"end":{"line":53,"column":25}},"argument":{"type":"ConditionalExpression","start":1450,"end":1463,"loc":{"start":{"line":53,"column":11},"end":{"line":53,"column":24}},"test":{"type":"BinaryExpression","start":1450,"end":1455,"loc":{"start":{"line":53,"column":11},"end":{"line":53,"column":16}},"left":{"type":"Identifier","start":1450,"end":1451,"loc":{"start":{"line":53,"column":11},"end":{"line":53,"column":12}},"name":"x"},"operator":"<","right":{"type":"Identifier","start":1454,"end":1455,"loc":{"start":{"line":53,"column":15},"end":{"line":53,"column":16}},"name":"y"}},"consequent":{"type":"Identifier","start":1458,"end":1459,"loc":{"start":{"line":53,"column":19},"end":{"line":53,"column":20}},"name":"x"},"alternate":{"type":"Identifier","start":1462,"end":1463,"loc":{"start":{"line":53,"column":23},"end":{"line":53,"column":24}},"name":"y"}}}]}},{"type":"ExportNamedDeclaration","start":1468,"end":1616,"loc":{"start":{"line":56,"column":0},"end":{"line":58,"column":46}},"declaration":null,"specifiers":[{"type":"ExportSpecifier","start":1481,"end":1487,"loc":{"start":{"line":57,"column":4},"end":{"line":57,"column":10}},"local":{"type":"Identifier","start":1481,"end":1487,"loc":{"start":{"line":57,"column":4},"end":{"line":57,"column":10}},"name":"math_E"},"exported":{"type":"Identifier","start":1481,"end":1487,"loc":{"start":{"line":57,"column":4},"end":{"line":57,"column":10}},"name":"math_E"}},{"type":"ExportSpecifier","start":1489,"end":1497,"loc":{"start":{"line":57,"column":12},"end":{"line":57,"column":20}},"local":{"type":"Identifier","start":1489,"end":1497,"loc":{"start":{"line":57,"column":12},"end":{"line":57,"column":20}},"name":"math_LN2"},"exported":{"type":"Identifier","start":1489,"end":1497,"loc":{"start":{"line":57,"column":12},"end":{"line":57,"column":20}},"name":"math_LN2"}},{"type":"ExportSpecifier","start":1499,"end":1508,"loc":{"start":{"line":57,"column":22},"end":{"line":57,"column":31}},"local":{"type":"Identifier","start":1499,"end":1508,"loc":{"start":{"line":57,"column":22},"end":{"line":57,"column":31}},"name":"math_LN10"},"exported":{"type":"Identifier","start":1499,"end":1508,"loc":{"start":{"line":57,"column":22},"end":{"line":57,"column":31}},"name":"math_LN10"}},{"type":"ExportSpecifier","start":1510,"end":1520,"loc":{"start":{"line":57,"column":33},"end":{"line":57,"column":43}},"local":{"type":"Identifier","start":1510,"end":1520,"loc":{"start":{"line":57,"column":33},"end":{"line":57,"column":43}},"name":"math_LOG2E"},"exported":{"type":"Identifier","start":1510,"end":1520,"loc":{"start":{"line":57,"column":33},"end":{"line":57,"column":43}},"name":"math_LOG2E"}},{"type":"ExportSpecifier","start":1522,"end":1533,"loc":{"start":{"line":57,"column":45},"end":{"line":57,"column":56}},"local":{"type":"Identifier","start":1522,"end":1533,"loc":{"start":{"line":57,"column":45},"end":{"line":57,"column":56}},"name":"math_LOG10E"},"exported":{"type":"Identifier","start":1522,"end":1533,"loc":{"start":{"line":57,"column":45},"end":{"line":57,"column":56}},"name":"math_LOG10E"}},{"type":"ExportSpecifier","start":1535,"end":1542,"loc":{"start":{"line":57,"column":58},"end":{"line":57,"column":65}},"local":{"type":"Identifier","start":1535,"end":1542,"loc":{"start":{"line":57,"column":58},"end":{"line":57,"column":65}},"name":"math_PI"},"exported":{"type":"Identifier","start":1535,"end":1542,"loc":{"start":{"line":57,"column":58},"end":{"line":57,"column":65}},"name":"math_PI"}},{"type":"ExportSpecifier","start":1544,"end":1556,"loc":{"start":{"line":57,"column":67},"end":{"line":57,"column":79}},"local":{"type":"Identifier","start":1544,"end":1556,"loc":{"start":{"line":57,"column":67},"end":{"line":57,"column":79}},"name":"math_SQRT1_2"},"exported":{"type":"Identifier","start":1544,"end":1556,"loc":{"start":{"line":57,"column":67},"end":{"line":57,"column":79}},"name":"math_SQRT1_2"}},{"type":"ExportSpecifier","start":1558,"end":1568,"loc":{"start":{"line":57,"column":81},"end":{"line":57,"column":91}},"local":{"type":"Identifier","start":1558,"end":1568,"loc":{"start":{"line":57,"column":81},"end":{"line":57,"column":91}},"name":"math_SQRT2"},"exported":{"type":"Identifier","start":1558,"end":1568,"loc":{"start":{"line":57,"column":81},"end":{"line":57,"column":91}},"name":"math_SQRT2"}},{"type":"ExportSpecifier","start":1574,"end":1582,"loc":{"start":{"line":58,"column":4},"end":{"line":58,"column":12}},"local":{"type":"Identifier","start":1574,"end":1582,"loc":{"start":{"line":58,"column":4},"end":{"line":58,"column":12}},"name":"math_abs"},"exported":{"type":"Identifier","start":1574,"end":1582,"loc":{"start":{"line":58,"column":4},"end":{"line":58,"column":12}},"name":"math_abs"}},{"type":"ExportSpecifier","start":1584,"end":1593,"loc":{"start":{"line":58,"column":14},"end":{"line":58,"column":23}},"local":{"type":"Identifier","start":1584,"end":1593,"loc":{"start":{"line":58,"column":14},"end":{"line":58,"column":23}},"name":"math_sign"},"exported":{"type":"Identifier","start":1584,"end":1593,"loc":{"start":{"line":58,"column":14},"end":{"line":58,"column":23}},"name":"math_sign"}},{"type":"ExportSpecifier","start":1595,"end":1603,"loc":{"start":{"line":58,"column":25},"end":{"line":58,"column":33}},"local":{"type":"Identifier","start":1595,"end":1603,"loc":{"start":{"line":58,"column":25},"end":{"line":58,"column":33}},"name":"math_max"},"exported":{"type":"Identifier","start":1595,"end":1603,"loc":{"start":{"line":58,"column":25},"end":{"line":58,"column":33}},"name":"math_max"}},{"type":"ExportSpecifier","start":1605,"end":1613,"loc":{"start":{"line":58,"column":35},"end":{"line":58,"column":43}},"local":{"type":"Identifier","start":1605,"end":1613,"loc":{"start":{"line":58,"column":35},"end":{"line":58,"column":43}},"name":"math_min"},"exported":{"type":"Identifier","start":1605,"end":1613,"loc":{"start":{"line":58,"column":35},"end":{"line":58,"column":43}},"name":"math_min"}}],"source":null}],"sourceType":"module"}
"#;
/*const IMPORT_MATH: &'static str = r#"
{"type":"Program","start":0,"end":1617,"loc":{"start":{"line":1,"column":0},"end":{"line":59,"column":0}},"body":[{"type":"ImportDeclaration","start":0,"end":32,"loc":{"start":{"line":1,"column":0},"end":{"line":1,"column":32}},"specifiers":[{"type":"ImportSpecifier","start":9,"end":17,"loc":{"start":{"line":1,"column":9},"end":{"line":1,"column":17}},"imported":{"type":"Identifier","start":9,"end":17,"loc":{"start":{"line":1,"column":9},"end":{"line":1,"column":17}},"name":"Infinity"},"local":{"type":"Identifier","start":9,"end":17,"loc":{"start":{"line":1,"column":9},"end":{"line":1,"column":17}},"name":"Infinity"}}],"source":{"type":"Literal","start":25,"end":31,"loc":{"start":{"line":1,"column":25},"end":{"line":1,"column":31}},"value":"misc","raw":"\"misc\""}},{"type":"VariableDeclaration","start":71,"end":104,"loc":{"start":{"line":4,"column":0},"end":{"line":4,"column":33}},"declarations":[{"type":"VariableDeclarator","start":77,"end":103,"loc":{"start":{"line":4,"column":6},"end":{"line":4,"column":32}},"id":{"type":"Identifier","start":77,"end":83,"loc":{"start":{"line":4,"column":6},"end":{"line":4,"column":12}},"name":"math_E"},"init":{"type":"Literal","start":86,"end":103,"loc":{"start":{"line":4,"column":15},"end":{"line":4,"column":32}},"value":2.718281828459045,"raw":"2.718281828459045"}}],"kind":"const"},{"type":"VariableDeclaration","start":105,"end":141,"loc":{"start":{"line":5,"column":0},"end":{"line":5,"column":36}},"declarations":[{"type":"VariableDeclarator","start":111,"end":140,"loc":{"start":{"line":5,"column":6},"end":{"line":5,"column":35}},"id":{"type":"Identifier","start":111,"end":119,"loc":{"start":{"line":5,"column":6},"end":{"line":5,"column":14}},"name":"math_LN2"},"init":{"type":"Literal","start":122,"end":140,"loc":{"start":{"line":5,"column":17},"end":{"line":5,"column":35}},"value":0.6931471805599453,"raw":"0.6931471805599453"}}],"kind":"const"},{"type":"VariableDeclaration","start":142,"end":178,"loc":{"start":{"line":6,"column":0},"end":{"line":6,"column":36}},"declarations":[{"type":"VariableDeclarator","start":148,"end":177,"loc":{"start":{"line":6,"column":6},"end":{"line":6,"column":35}},"id":{"type":"Identifier","start":148,"end":157,"loc":{"start":{"line":6,"column":6},"end":{"line":6,"column":15}},"name":"math_LN10"},"init":{"type":"Literal","start":160,"end":177,"loc":{"start":{"line":6,"column":18},"end":{"line":6,"column":35}},"value":2.302585092994046,"raw":"2.302585092994046"}}],"kind":"const"},{"type":"VariableDeclaration","start":179,"end":217,"loc":{"start":{"line":7,"column":0},"end":{"line":7,"column":38}},"declarations":[{"type":"VariableDeclarator","start":185,"end":216,"loc":{"start":{"line":7,"column":6},"end":{"line":7,"column":37}},"id":{"type":"Identifier","start":185,"end":195,"loc":{"start":{"line":7,"column":6},"end":{"line":7,"column":16}},"name":"math_LOG2E"},"init":{"type":"Literal","start":198,"end":216,"loc":{"start":{"line":7,"column":19},"end":{"line":7,"column":37}},"value":1.4426950408889634,"raw":"1.4426950408889634"}}],"kind":"const"},{"type":"VariableDeclaration","start":218,"end":257,"loc":{"start":{"line":8,"column":0},"end":{"line":8,"column":39}},"declarations":[{"type":"VariableDeclarator","start":224,"end":256,"loc":{"start":{"line":8,"column":6},"end":{"line":8,"column":38}},"id":{"type":"Identifier","start":224,"end":235,"loc":{"start":{"line":8,"column":6},"end":{"line":8,"column":17}},"name":"math_LOG10E"},"init":{"type":"Literal","start":238,"end":256,"loc":{"start":{"line":8,"column":20},"end":{"line":8,"column":38}},"value":0.4342944819032518,"raw":"0.4342944819032518"}}],"kind":"const"},{"type":"VariableDeclaration","start":258,"end":293,"loc":{"start":{"line":9,"column":0},"end":{"line":9,"column":35}},"declarations":[{"type":"VariableDeclarator","start":264,"end":292,"loc":{"start":{"line":9,"column":6},"end":{"line":9,"column":34}},"id":{"type":"Identifier","start":264,"end":271,"loc":{"start":{"line":9,"column":6},"end":{"line":9,"column":13}},"name":"math_PI"},"init":{"type":"Literal","start":274,"end":292,"loc":{"start":{"line":9,"column":16},"end":{"line":9,"column":34}},"value":3.141592653589793,"raw":"3.1415926535897932"}}],"kind":"const"},{"type":"VariableDeclaration","start":294,"end":334,"loc":{"start":{"line":10,"column":0},"end":{"line":10,"column":40}},"declarations":[{"type":"VariableDeclarator","start":300,"end":333,"loc":{"start":{"line":10,"column":6},"end":{"line":10,"column":39}},"id":{"type":"Identifier","start":300,"end":312,"loc":{"start":{"line":10,"column":6},"end":{"line":10,"column":18}},"name":"math_SQRT1_2"},"init":{"type":"Literal","start":315,"end":333,"loc":{"start":{"line":10,"column":21},"end":{"line":10,"column":39}},"value":0.7071067811865476,"raw":"0.7071067811865476"}}],"kind":"const"},{"type":"VariableDeclaration","start":335,"end":373,"loc":{"start":{"line":11,"column":0},"end":{"line":11,"column":38}},"declarations":[{"type":"VariableDeclarator","start":341,"end":372,"loc":{"start":{"line":11,"column":6},"end":{"line":11,"column":37}},"id":{"type":"Identifier","start":341,"end":351,"loc":{"start":{"line":11,"column":6},"end":{"line":11,"column":16}},"name":"math_SQRT2"},"init":{"type":"Literal","start":354,"end":372,"loc":{"start":{"line":11,"column":19},"end":{"line":11,"column":37}},"value":1.4142135623730951,"raw":"1.4142135623730951"}}],"kind":"const"},{"type":"ExpressionStatement","start":632,"end":676,"loc":{"start":{"line":18,"column":0},"end":{"line":18,"column":44}},"expression":{"type":"AssignmentExpression","start":632,"end":675,"loc":{"start":{"line":18,"column":0},"end":{"line":18,"column":43}},"operator":"=","left":{"type":"Identifier","start":632,"end":644,"loc":{"start":{"line":18,"column":0},"end":{"line":18,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":647,"end":675,"loc":{"start":{"line":18,"column":15},"end":{"line":18,"column":43}},"value":"direct;constraint=x:number","raw":"\"direct;constraint=x:number\""}}},{"type":"FunctionDeclaration","start":677,"end":729,"loc":{"start":{"line":19,"column":0},"end":{"line":21,"column":1}},"id":{"type":"Identifier","start":686,"end":694,"loc":{"start":{"line":19,"column":9},"end":{"line":19,"column":17}},"name":"math_abs"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":695,"end":696,"loc":{"start":{"line":19,"column":18},"end":{"line":19,"column":19}},"name":"x"}],"body":{"type":"BlockStatement","start":698,"end":729,"loc":{"start":{"line":19,"column":21},"end":{"line":21,"column":1}},"body":[{"type":"ReturnStatement","start":704,"end":727,"loc":{"start":{"line":20,"column":4},"end":{"line":20,"column":27}},"argument":{"type":"ConditionalExpression","start":711,"end":726,"loc":{"start":{"line":20,"column":11},"end":{"line":20,"column":26}},"test":{"type":"BinaryExpression","start":711,"end":717,"loc":{"start":{"line":20,"column":11},"end":{"line":20,"column":17}},"left":{"type":"Identifier","start":711,"end":712,"loc":{"start":{"line":20,"column":11},"end":{"line":20,"column":12}},"name":"x"},"operator":">=","right":{"type":"Literal","start":716,"end":717,"loc":{"start":{"line":20,"column":16},"end":{"line":20,"column":17}},"value":0,"raw":"0"}},"consequent":{"type":"Identifier","start":720,"end":721,"loc":{"start":{"line":20,"column":20},"end":{"line":20,"column":21}},"name":"x"},"alternate":{"type":"UnaryExpression","start":724,"end":726,"loc":{"start":{"line":20,"column":24},"end":{"line":20,"column":26}},"operator":"-","prefix":true,"argument":{"type":"Identifier","start":725,"end":726,"loc":{"start":{"line":20,"column":25},"end":{"line":20,"column":26}},"name":"x"}}}}]}},{"type":"ExpressionStatement","start":731,"end":775,"loc":{"start":{"line":23,"column":0},"end":{"line":23,"column":44}},"expression":{"type":"AssignmentExpression","start":731,"end":774,"loc":{"start":{"line":23,"column":0},"end":{"line":23,"column":43}},"operator":"=","left":{"type":"Identifier","start":731,"end":743,"loc":{"start":{"line":23,"column":0},"end":{"line":23,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":746,"end":774,"loc":{"start":{"line":23,"column":15},"end":{"line":23,"column":43}},"value":"direct;constraint=x:number","raw":"\"direct;constraint=x:number\""}}},{"type":"FunctionDeclaration","start":776,"end":840,"loc":{"start":{"line":24,"column":0},"end":{"line":26,"column":1}},"id":{"type":"Identifier","start":785,"end":794,"loc":{"start":{"line":24,"column":9},"end":{"line":24,"column":18}},"name":"math_sign"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":795,"end":796,"loc":{"start":{"line":24,"column":19},"end":{"line":24,"column":20}},"name":"x"}],"body":{"type":"BlockStatement","start":798,"end":840,"loc":{"start":{"line":24,"column":22},"end":{"line":26,"column":1}},"body":[{"type":"ReturnStatement","start":804,"end":838,"loc":{"start":{"line":25,"column":4},"end":{"line":25,"column":38}},"argument":{"type":"ConditionalExpression","start":811,"end":837,"loc":{"start":{"line":25,"column":11},"end":{"line":25,"column":37}},"test":{"type":"BinaryExpression","start":811,"end":816,"loc":{"start":{"line":25,"column":11},"end":{"line":25,"column":16}},"left":{"type":"Identifier","start":811,"end":812,"loc":{"start":{"line":25,"column":11},"end":{"line":25,"column":12}},"name":"x"},"operator":">","right":{"type":"Literal","start":815,"end":816,"loc":{"start":{"line":25,"column":15},"end":{"line":25,"column":16}},"value":0,"raw":"0"}},"consequent":{"type":"Literal","start":819,"end":820,"loc":{"start":{"line":25,"column":19},"end":{"line":25,"column":20}},"value":1,"raw":"1"},"alternate":{"type":"ConditionalExpression","start":823,"end":837,"loc":{"start":{"line":25,"column":23},"end":{"line":25,"column":37}},"test":{"type":"BinaryExpression","start":823,"end":828,"loc":{"start":{"line":25,"column":23},"end":{"line":25,"column":28}},"left":{"type":"Identifier","start":823,"end":824,"loc":{"start":{"line":25,"column":23},"end":{"line":25,"column":24}},"name":"x"},"operator":"<","right":{"type":"Literal","start":827,"end":828,"loc":{"start":{"line":25,"column":27},"end":{"line":25,"column":28}},"value":0,"raw":"0"}},"consequent":{"type":"UnaryExpression","start":831,"end":833,"loc":{"start":{"line":25,"column":31},"end":{"line":25,"column":33}},"operator":"-","prefix":true,"argument":{"type":"Literal","start":832,"end":833,"loc":{"start":{"line":25,"column":32},"end":{"line":25,"column":33}},"value":1,"raw":"1"}},"alternate":{"type":"Literal","start":836,"end":837,"loc":{"start":{"line":25,"column":36},"end":{"line":25,"column":37}},"value":0,"raw":"0"}}}}]}},{"type":"ExpressionStatement","start":941,"end":965,"loc":{"start":{"line":30,"column":0},"end":{"line":30,"column":24}},"expression":{"type":"AssignmentExpression","start":941,"end":964,"loc":{"start":{"line":30,"column":0},"end":{"line":30,"column":23}},"operator":"=","left":{"type":"Identifier","start":941,"end":953,"loc":{"start":{"line":30,"column":0},"end":{"line":30,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":956,"end":964,"loc":{"start":{"line":30,"column":15},"end":{"line":30,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":966,"end":1011,"loc":{"start":{"line":31,"column":0},"end":{"line":33,"column":1}},"id":{"type":"Identifier","start":975,"end":983,"loc":{"start":{"line":31,"column":9},"end":{"line":31,"column":17}},"name":"math_max"},"expression":false,"generator":false,"params":[],"body":{"type":"BlockStatement","start":986,"end":1011,"loc":{"start":{"line":31,"column":20},"end":{"line":33,"column":1}},"body":[{"type":"ReturnStatement","start":992,"end":1009,"loc":{"start":{"line":32,"column":4},"end":{"line":32,"column":21}},"argument":{"type":"UnaryExpression","start":999,"end":1008,"loc":{"start":{"line":32,"column":11},"end":{"line":32,"column":20}},"operator":"-","prefix":true,"argument":{"type":"Identifier","start":1000,"end":1008,"loc":{"start":{"line":32,"column":12},"end":{"line":32,"column":20}},"name":"Infinity"}}}]}},{"type":"ExpressionStatement","start":1012,"end":1056,"loc":{"start":{"line":34,"column":0},"end":{"line":34,"column":44}},"expression":{"type":"AssignmentExpression","start":1012,"end":1055,"loc":{"start":{"line":34,"column":0},"end":{"line":34,"column":43}},"operator":"=","left":{"type":"Identifier","start":1012,"end":1024,"loc":{"start":{"line":34,"column":0},"end":{"line":34,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1027,"end":1055,"loc":{"start":{"line":34,"column":15},"end":{"line":34,"column":43}},"value":"direct;constraint=x:number","raw":"\"direct;constraint=x:number\""}}},{"type":"FunctionDeclaration","start":1057,"end":1095,"loc":{"start":{"line":35,"column":0},"end":{"line":37,"column":1}},"id":{"type":"Identifier","start":1066,"end":1074,"loc":{"start":{"line":35,"column":9},"end":{"line":35,"column":17}},"name":"math_max"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1075,"end":1076,"loc":{"start":{"line":35,"column":18},"end":{"line":35,"column":19}},"name":"x"}],"body":{"type":"BlockStatement","start":1078,"end":1095,"loc":{"start":{"line":35,"column":21},"end":{"line":37,"column":1}},"body":[{"type":"ReturnStatement","start":1084,"end":1093,"loc":{"start":{"line":36,"column":4},"end":{"line":36,"column":13}},"argument":{"type":"Identifier","start":1091,"end":1092,"loc":{"start":{"line":36,"column":11},"end":{"line":36,"column":12}},"name":"x"}}]}},{"type":"ExpressionStatement","start":1096,"end":1149,"loc":{"start":{"line":38,"column":0},"end":{"line":38,"column":53}},"expression":{"type":"AssignmentExpression","start":1096,"end":1148,"loc":{"start":{"line":38,"column":0},"end":{"line":38,"column":52}},"operator":"=","left":{"type":"Identifier","start":1096,"end":1108,"loc":{"start":{"line":38,"column":0},"end":{"line":38,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1111,"end":1148,"loc":{"start":{"line":38,"column":15},"end":{"line":38,"column":52}},"value":"direct;constraint=x:number,y:number","raw":"\"direct;constraint=x:number,y:number\""}}},{"type":"FunctionDeclaration","start":1150,"end":1203,"loc":{"start":{"line":39,"column":0},"end":{"line":41,"column":1}},"id":{"type":"Identifier","start":1159,"end":1167,"loc":{"start":{"line":39,"column":9},"end":{"line":39,"column":17}},"name":"math_max"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1168,"end":1169,"loc":{"start":{"line":39,"column":18},"end":{"line":39,"column":19}},"name":"x"},{"type":"Identifier","start":1171,"end":1172,"loc":{"start":{"line":39,"column":21},"end":{"line":39,"column":22}},"name":"y"}],"body":{"type":"BlockStatement","start":1174,"end":1203,"loc":{"start":{"line":39,"column":24},"end":{"line":41,"column":1}},"body":[{"type":"ReturnStatement","start":1180,"end":1201,"loc":{"start":{"line":40,"column":4},"end":{"line":40,"column":25}},"argument":{"type":"ConditionalExpression","start":1187,"end":1200,"loc":{"start":{"line":40,"column":11},"end":{"line":40,"column":24}},"test":{"type":"BinaryExpression","start":1187,"end":1192,"loc":{"start":{"line":40,"column":11},"end":{"line":40,"column":16}},"left":{"type":"Identifier","start":1187,"end":1188,"loc":{"start":{"line":40,"column":11},"end":{"line":40,"column":12}},"name":"x"},"operator":"<","right":{"type":"Identifier","start":1191,"end":1192,"loc":{"start":{"line":40,"column":15},"end":{"line":40,"column":16}},"name":"y"}},"consequent":{"type":"Identifier","start":1195,"end":1196,"loc":{"start":{"line":40,"column":19},"end":{"line":40,"column":20}},"name":"y"},"alternate":{"type":"Identifier","start":1199,"end":1200,"loc":{"start":{"line":40,"column":23},"end":{"line":40,"column":24}},"name":"x"}}}]}},{"type":"ExpressionStatement","start":1205,"end":1229,"loc":{"start":{"line":43,"column":0},"end":{"line":43,"column":24}},"expression":{"type":"AssignmentExpression","start":1205,"end":1228,"loc":{"start":{"line":43,"column":0},"end":{"line":43,"column":23}},"operator":"=","left":{"type":"Identifier","start":1205,"end":1217,"loc":{"start":{"line":43,"column":0},"end":{"line":43,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1220,"end":1228,"loc":{"start":{"line":43,"column":15},"end":{"line":43,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":1230,"end":1274,"loc":{"start":{"line":44,"column":0},"end":{"line":46,"column":1}},"id":{"type":"Identifier","start":1239,"end":1247,"loc":{"start":{"line":44,"column":9},"end":{"line":44,"column":17}},"name":"math_min"},"expression":false,"generator":false,"params":[],"body":{"type":"BlockStatement","start":1250,"end":1274,"loc":{"start":{"line":44,"column":20},"end":{"line":46,"column":1}},"body":[{"type":"ReturnStatement","start":1256,"end":1272,"loc":{"start":{"line":45,"column":4},"end":{"line":45,"column":20}},"argument":{"type":"Identifier","start":1263,"end":1271,"loc":{"start":{"line":45,"column":11},"end":{"line":45,"column":19}},"name":"Infinity"}}]}},{"type":"ExpressionStatement","start":1275,"end":1319,"loc":{"start":{"line":47,"column":0},"end":{"line":47,"column":44}},"expression":{"type":"AssignmentExpression","start":1275,"end":1318,"loc":{"start":{"line":47,"column":0},"end":{"line":47,"column":43}},"operator":"=","left":{"type":"Identifier","start":1275,"end":1287,"loc":{"start":{"line":47,"column":0},"end":{"line":47,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1290,"end":1318,"loc":{"start":{"line":47,"column":15},"end":{"line":47,"column":43}},"value":"direct;constraint=x:number","raw":"\"direct;constraint=x:number\""}}},{"type":"FunctionDeclaration","start":1320,"end":1358,"loc":{"start":{"line":48,"column":0},"end":{"line":50,"column":1}},"id":{"type":"Identifier","start":1329,"end":1337,"loc":{"start":{"line":48,"column":9},"end":{"line":48,"column":17}},"name":"math_min"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1338,"end":1339,"loc":{"start":{"line":48,"column":18},"end":{"line":48,"column":19}},"name":"x"}],"body":{"type":"BlockStatement","start":1341,"end":1358,"loc":{"start":{"line":48,"column":21},"end":{"line":50,"column":1}},"body":[{"type":"ReturnStatement","start":1347,"end":1356,"loc":{"start":{"line":49,"column":4},"end":{"line":49,"column":13}},"argument":{"type":"Identifier","start":1354,"end":1355,"loc":{"start":{"line":49,"column":11},"end":{"line":49,"column":12}},"name":"x"}}]}},{"type":"ExpressionStatement","start":1359,"end":1412,"loc":{"start":{"line":51,"column":0},"end":{"line":51,"column":53}},"expression":{"type":"AssignmentExpression","start":1359,"end":1411,"loc":{"start":{"line":51,"column":0},"end":{"line":51,"column":52}},"operator":"=","left":{"type":"Identifier","start":1359,"end":1371,"loc":{"start":{"line":51,"column":0},"end":{"line":51,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1374,"end":1411,"loc":{"start":{"line":51,"column":15},"end":{"line":51,"column":52}},"value":"direct;constraint=x:number,y:number","raw":"\"direct;constraint=x:number,y:number\""}}},{"type":"FunctionDeclaration","start":1413,"end":1466,"loc":{"start":{"line":52,"column":0},"end":{"line":54,"column":1}},"id":{"type":"Identifier","start":1422,"end":1430,"loc":{"start":{"line":52,"column":9},"end":{"line":52,"column":17}},"name":"math_min"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1431,"end":1432,"loc":{"start":{"line":52,"column":18},"end":{"line":52,"column":19}},"name":"x"},{"type":"Identifier","start":1434,"end":1435,"loc":{"start":{"line":52,"column":21},"end":{"line":52,"column":22}},"name":"y"}],"body":{"type":"BlockStatement","start":1437,"end":1466,"loc":{"start":{"line":52,"column":24},"end":{"line":54,"column":1}},"body":[{"type":"ReturnStatement","start":1443,"end":1464,"loc":{"start":{"line":53,"column":4},"end":{"line":53,"column":25}},"argument":{"type":"ConditionalExpression","start":1450,"end":1463,"loc":{"start":{"line":53,"column":11},"end":{"line":53,"column":24}},"test":{"type":"BinaryExpression","start":1450,"end":1455,"loc":{"start":{"line":53,"column":11},"end":{"line":53,"column":16}},"left":{"type":"Identifier","start":1450,"end":1451,"loc":{"start":{"line":53,"column":11},"end":{"line":53,"column":12}},"name":"x"},"operator":"<","right":{"type":"Identifier","start":1454,"end":1455,"loc":{"start":{"line":53,"column":15},"end":{"line":53,"column":16}},"name":"y"}},"consequent":{"type":"Identifier","start":1458,"end":1459,"loc":{"start":{"line":53,"column":19},"end":{"line":53,"column":20}},"name":"x"},"alternate":{"type":"Identifier","start":1462,"end":1463,"loc":{"start":{"line":53,"column":23},"end":{"line":53,"column":24}},"name":"y"}}}]}},{"type":"ExportNamedDeclaration","start":1468,"end":1616,"loc":{"start":{"line":56,"column":0},"end":{"line":58,"column":46}},"declaration":null,"specifiers":[{"type":"ExportSpecifier","start":1481,"end":1487,"loc":{"start":{"line":57,"column":4},"end":{"line":57,"column":10}},"local":{"type":"Identifier","start":1481,"end":1487,"loc":{"start":{"line":57,"column":4},"end":{"line":57,"column":10}},"name":"math_E"},"exported":{"type":"Identifier","start":1481,"end":1487,"loc":{"start":{"line":57,"column":4},"end":{"line":57,"column":10}},"name":"math_E"}},{"type":"ExportSpecifier","start":1489,"end":1497,"loc":{"start":{"line":57,"column":12},"end":{"line":57,"column":20}},"local":{"type":"Identifier","start":1489,"end":1497,"loc":{"start":{"line":57,"column":12},"end":{"line":57,"column":20}},"name":"math_LN2"},"exported":{"type":"Identifier","start":1489,"end":1497,"loc":{"start":{"line":57,"column":12},"end":{"line":57,"column":20}},"name":"math_LN2"}},{"type":"ExportSpecifier","start":1499,"end":1508,"loc":{"start":{"line":57,"column":22},"end":{"line":57,"column":31}},"local":{"type":"Identifier","start":1499,"end":1508,"loc":{"start":{"line":57,"column":22},"end":{"line":57,"column":31}},"name":"math_LN10"},"exported":{"type":"Identifier","start":1499,"end":1508,"loc":{"start":{"line":57,"column":22},"end":{"line":57,"column":31}},"name":"math_LN10"}},{"type":"ExportSpecifier","start":1510,"end":1520,"loc":{"start":{"line":57,"column":33},"end":{"line":57,"column":43}},"local":{"type":"Identifier","start":1510,"end":1520,"loc":{"start":{"line":57,"column":33},"end":{"line":57,"column":43}},"name":"math_LOG2E"},"exported":{"type":"Identifier","start":1510,"end":1520,"loc":{"start":{"line":57,"column":33},"end":{"line":57,"column":43}},"name":"math_LOG2E"}},{"type":"ExportSpecifier","start":1522,"end":1533,"loc":{"start":{"line":57,"column":45},"end":{"line":57,"column":56}},"local":{"type":"Identifier","start":1522,"end":1533,"loc":{"start":{"line":57,"column":45},"end":{"line":57,"column":56}},"name":"math_LOG10E"},"exported":{"type":"Identifier","start":1522,"end":1533,"loc":{"start":{"line":57,"column":45},"end":{"line":57,"column":56}},"name":"math_LOG10E"}},{"type":"ExportSpecifier","start":1535,"end":1542,"loc":{"start":{"line":57,"column":58},"end":{"line":57,"column":65}},"local":{"type":"Identifier","start":1535,"end":1542,"loc":{"start":{"line":57,"column":58},"end":{"line":57,"column":65}},"name":"math_PI"},"exported":{"type":"Identifier","start":1535,"end":1542,"loc":{"start":{"line":57,"column":58},"end":{"line":57,"column":65}},"name":"math_PI"}},{"type":"ExportSpecifier","start":1544,"end":1556,"loc":{"start":{"line":57,"column":67},"end":{"line":57,"column":79}},"local":{"type":"Identifier","start":1544,"end":1556,"loc":{"start":{"line":57,"column":67},"end":{"line":57,"column":79}},"name":"math_SQRT1_2"},"exported":{"type":"Identifier","start":1544,"end":1556,"loc":{"start":{"line":57,"column":67},"end":{"line":57,"column":79}},"name":"math_SQRT1_2"}},{"type":"ExportSpecifier","start":1558,"end":1568,"loc":{"start":{"line":57,"column":81},"end":{"line":57,"column":91}},"local":{"type":"Identifier","start":1558,"end":1568,"loc":{"start":{"line":57,"column":81},"end":{"line":57,"column":91}},"name":"math_SQRT2"},"exported":{"type":"Identifier","start":1558,"end":1568,"loc":{"start":{"line":57,"column":81},"end":{"line":57,"column":91}},"name":"math_SQRT2"}},{"type":"ExportSpecifier","start":1574,"end":1582,"loc":{"start":{"line":58,"column":4},"end":{"line":58,"column":12}},"local":{"type":"Identifier","start":1574,"end":1582,"loc":{"start":{"line":58,"column":4},"end":{"line":58,"column":12}},"name":"math_abs"},"exported":{"type":"Identifier","start":1574,"end":1582,"loc":{"start":{"line":58,"column":4},"end":{"line":58,"column":12}},"name":"math_abs"}},{"type":"ExportSpecifier","start":1584,"end":1593,"loc":{"start":{"line":58,"column":14},"end":{"line":58,"column":23}},"local":{"type":"Identifier","start":1584,"end":1593,"loc":{"start":{"line":58,"column":14},"end":{"line":58,"column":23}},"name":"math_sign"},"exported":{"type":"Identifier","start":1584,"end":1593,"loc":{"start":{"line":58,"column":14},"end":{"line":58,"column":23}},"name":"math_sign"}},{"type":"ExportSpecifier","start":1595,"end":1603,"loc":{"start":{"line":58,"column":25},"end":{"line":58,"column":33}},"local":{"type":"Identifier","start":1595,"end":1603,"loc":{"start":{"line":58,"column":25},"end":{"line":58,"column":33}},"name":"math_max"},"exported":{"type":"Identifier","start":1595,"end":1603,"loc":{"start":{"line":58,"column":25},"end":{"line":58,"column":33}},"name":"math_max"}},{"type":"ExportSpecifier","start":1605,"end":1613,"loc":{"start":{"line":58,"column":35},"end":{"line":58,"column":43}},"local":{"type":"Identifier","start":1605,"end":1613,"loc":{"start":{"line":58,"column":35},"end":{"line":58,"column":43}},"name":"math_min"},"exported":{"type":"Identifier","start":1605,"end":1613,"loc":{"start":{"line":58,"column":35},"end":{"line":58,"column":43}},"name":"math_min"}}],"source":null}],"sourceType":"module"}
"#;*/
const IMPORT_MISC_FFI: &'static str = r#"@SourceImports
get_time misc get_time number
display misc display undefined string
prompt misc prompt string string
abort core abort undefined
parse_int misc parse_int number string number
parse_float misc parse_float number string
stringify_float misc stringify_float string number
"#;
const IMPORT_MISC: &'static str = r#"
{"type":"Program","start":0,"end":2869,"loc":{"start":{"line":1,"column":0},"end":{"line":128,"column":0}},"body":[{"type":"ImportDeclaration","start":0,"end":117,"loc":{"start":{"line":1,"column":0},"end":{"line":1,"column":117}},"specifiers":[{"type":"ImportSpecifier","start":9,"end":17,"loc":{"start":{"line":1,"column":9},"end":{"line":1,"column":17}},"imported":{"type":"Identifier","start":9,"end":17,"loc":{"start":{"line":1,"column":9},"end":{"line":1,"column":17}},"name":"get_time"},"local":{"type":"Identifier","start":9,"end":17,"loc":{"start":{"line":1,"column":9},"end":{"line":1,"column":17}},"name":"get_time"}},{"type":"ImportSpecifier","start":19,"end":41,"loc":{"start":{"line":1,"column":19},"end":{"line":1,"column":41}},"imported":{"type":"Identifier","start":19,"end":26,"loc":{"start":{"line":1,"column":19},"end":{"line":1,"column":26}},"name":"display"},"local":{"type":"Identifier","start":30,"end":41,"loc":{"start":{"line":1,"column":30},"end":{"line":1,"column":41}},"name":"ffi_display"}},{"type":"ImportSpecifier","start":43,"end":63,"loc":{"start":{"line":1,"column":43},"end":{"line":1,"column":63}},"imported":{"type":"Identifier","start":43,"end":49,"loc":{"start":{"line":1,"column":43},"end":{"line":1,"column":49}},"name":"prompt"},"local":{"type":"Identifier","start":53,"end":63,"loc":{"start":{"line":1,"column":53},"end":{"line":1,"column":63}},"name":"ffi_prompt"}},{"type":"ImportSpecifier","start":65,"end":70,"loc":{"start":{"line":1,"column":65},"end":{"line":1,"column":70}},"imported":{"type":"Identifier","start":65,"end":70,"loc":{"start":{"line":1,"column":65},"end":{"line":1,"column":70}},"name":"abort"},"local":{"type":"Identifier","start":65,"end":70,"loc":{"start":{"line":1,"column":65},"end":{"line":1,"column":70}},"name":"abort"}},{"type":"ImportSpecifier","start":72,"end":81,"loc":{"start":{"line":1,"column":72},"end":{"line":1,"column":81}},"imported":{"type":"Identifier","start":72,"end":81,"loc":{"start":{"line":1,"column":72},"end":{"line":1,"column":81}},"name":"parse_int"},"local":{"type":"Identifier","start":72,"end":81,"loc":{"start":{"line":1,"column":72},"end":{"line":1,"column":81}},"name":"parse_int"}},{"type":"ImportSpecifier","start":83,"end":98,"loc":{"start":{"line":1,"column":83},"end":{"line":1,"column":98}},"imported":{"type":"Identifier","start":83,"end":98,"loc":{"start":{"line":1,"column":83},"end":{"line":1,"column":98}},"name":"stringify_float"},"local":{"type":"Identifier","start":83,"end":98,"loc":{"start":{"line":1,"column":83},"end":{"line":1,"column":98}},"name":"stringify_float"}}],"source":{"type":"Literal","start":106,"end":116,"loc":{"start":{"line":1,"column":106},"end":{"line":1,"column":116}},"value":"misc.ffi","raw":"\"misc.ffi\""}},{"type":"VariableDeclaration","start":119,"end":142,"loc":{"start":{"line":3,"column":0},"end":{"line":3,"column":23}},"declarations":[{"type":"VariableDeclarator","start":125,"end":141,"loc":{"start":{"line":3,"column":6},"end":{"line":3,"column":22}},"id":{"type":"Identifier","start":125,"end":133,"loc":{"start":{"line":3,"column":6},"end":{"line":3,"column":14}},"name":"Infinity"},"init":{"type":"BinaryExpression","start":136,"end":141,"loc":{"start":{"line":3,"column":17},"end":{"line":3,"column":22}},"left":{"type":"Literal","start":136,"end":137,"loc":{"start":{"line":3,"column":17},"end":{"line":3,"column":18}},"value":1,"raw":"1"},"operator":"/","right":{"type":"Literal","start":140,"end":141,"loc":{"start":{"line":3,"column":21},"end":{"line":3,"column":22}},"value":0,"raw":"0"}}}],"kind":"const"},{"type":"VariableDeclaration","start":143,"end":168,"loc":{"start":{"line":4,"column":0},"end":{"line":4,"column":25}},"declarations":[{"type":"VariableDeclarator","start":149,"end":167,"loc":{"start":{"line":4,"column":6},"end":{"line":4,"column":24}},"id":{"type":"Identifier","start":149,"end":152,"loc":{"start":{"line":4,"column":6},"end":{"line":4,"column":9}},"name":"NaN"},"init":{"type":"BinaryExpression","start":155,"end":167,"loc":{"start":{"line":4,"column":12},"end":{"line":4,"column":24}},"left":{"type":"Literal","start":155,"end":156,"loc":{"start":{"line":4,"column":12},"end":{"line":4,"column":13}},"value":0,"raw":"0"},"operator":"*","right":{"type":"Identifier","start":159,"end":167,"loc":{"start":{"line":4,"column":16},"end":{"line":4,"column":24}},"name":"Infinity"}}}],"kind":"const"},{"type":"VariableDeclaration","start":169,"end":200,"loc":{"start":{"line":5,"column":0},"end":{"line":5,"column":31}},"declarations":[{"type":"VariableDeclarator","start":175,"end":199,"loc":{"start":{"line":5,"column":6},"end":{"line":5,"column":30}},"id":{"type":"Identifier","start":175,"end":184,"loc":{"start":{"line":5,"column":6},"end":{"line":5,"column":15}},"name":"undefined"},"init":{"type":"CallExpression","start":187,"end":199,"loc":{"start":{"line":5,"column":18},"end":{"line":5,"column":30}},"callee":{"type":"ArrowFunctionExpression","start":188,"end":196,"loc":{"start":{"line":5,"column":19},"end":{"line":5,"column":27}},"id":null,"expression":false,"generator":false,"params":[],"body":{"type":"BlockStatement","start":194,"end":196,"loc":{"start":{"line":5,"column":25},"end":{"line":5,"column":27}},"body":[]}},"arguments":[]}}],"kind":"const"},{"type":"ExpressionStatement","start":202,"end":226,"loc":{"start":{"line":7,"column":0},"end":{"line":7,"column":24}},"expression":{"type":"AssignmentExpression","start":202,"end":225,"loc":{"start":{"line":7,"column":0},"end":{"line":7,"column":23}},"operator":"=","left":{"type":"Identifier","start":202,"end":214,"loc":{"start":{"line":7,"column":0},"end":{"line":7,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":217,"end":225,"loc":{"start":{"line":7,"column":15},"end":{"line":7,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":227,"end":295,"loc":{"start":{"line":8,"column":0},"end":{"line":11,"column":1}},"id":{"type":"Identifier","start":236,"end":243,"loc":{"start":{"line":8,"column":9},"end":{"line":8,"column":16}},"name":"display"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":244,"end":245,"loc":{"start":{"line":8,"column":17},"end":{"line":8,"column":18}},"name":"v"}],"body":{"type":"BlockStatement","start":247,"end":295,"loc":{"start":{"line":8,"column":20},"end":{"line":11,"column":1}},"body":[{"type":"ExpressionStatement","start":253,"end":279,"loc":{"start":{"line":9,"column":4},"end":{"line":9,"column":30}},"expression":{"type":"CallExpression","start":253,"end":278,"loc":{"start":{"line":9,"column":4},"end":{"line":9,"column":29}},"callee":{"type":"Identifier","start":253,"end":264,"loc":{"start":{"line":9,"column":4},"end":{"line":9,"column":15}},"name":"ffi_display"},"arguments":[{"type":"CallExpression","start":265,"end":277,"loc":{"start":{"line":9,"column":16},"end":{"line":9,"column":28}},"callee":{"type":"Identifier","start":265,"end":274,"loc":{"start":{"line":9,"column":16},"end":{"line":9,"column":25}},"name":"stringify"},"arguments":[{"type":"Identifier","start":275,"end":276,"loc":{"start":{"line":9,"column":26},"end":{"line":9,"column":27}},"name":"v"}]}]}},{"type":"ReturnStatement","start":284,"end":293,"loc":{"start":{"line":10,"column":4},"end":{"line":10,"column":13}},"argument":{"type":"Identifier","start":291,"end":292,"loc":{"start":{"line":10,"column":11},"end":{"line":10,"column":12}},"name":"v"}}]}},{"type":"ExpressionStatement","start":296,"end":340,"loc":{"start":{"line":12,"column":0},"end":{"line":12,"column":44}},"expression":{"type":"AssignmentExpression","start":296,"end":339,"loc":{"start":{"line":12,"column":0},"end":{"line":12,"column":43}},"operator":"=","left":{"type":"Identifier","start":296,"end":308,"loc":{"start":{"line":12,"column":0},"end":{"line":12,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":311,"end":339,"loc":{"start":{"line":12,"column":15},"end":{"line":12,"column":43}},"value":"direct;constraint=s:string","raw":"\"direct;constraint=s:string\""}}},{"type":"FunctionDeclaration","start":341,"end":422,"loc":{"start":{"line":13,"column":0},"end":{"line":16,"column":1}},"id":{"type":"Identifier","start":350,"end":357,"loc":{"start":{"line":13,"column":9},"end":{"line":13,"column":16}},"name":"display"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":358,"end":359,"loc":{"start":{"line":13,"column":17},"end":{"line":13,"column":18}},"name":"v"},{"type":"Identifier","start":361,"end":362,"loc":{"start":{"line":13,"column":20},"end":{"line":13,"column":21}},"name":"s"}],"body":{"type":"BlockStatement","start":364,"end":422,"loc":{"start":{"line":13,"column":23},"end":{"line":16,"column":1}},"body":[{"type":"ExpressionStatement","start":370,"end":406,"loc":{"start":{"line":14,"column":4},"end":{"line":14,"column":40}},"expression":{"type":"CallExpression","start":370,"end":405,"loc":{"start":{"line":14,"column":4},"end":{"line":14,"column":39}},"callee":{"type":"Identifier","start":370,"end":381,"loc":{"start":{"line":14,"column":4},"end":{"line":14,"column":15}},"name":"ffi_display"},"arguments":[{"type":"BinaryExpression","start":382,"end":404,"loc":{"start":{"line":14,"column":16},"end":{"line":14,"column":38}},"left":{"type":"BinaryExpression","start":382,"end":389,"loc":{"start":{"line":14,"column":16},"end":{"line":14,"column":23}},"left":{"type":"Identifier","start":382,"end":383,"loc":{"start":{"line":14,"column":16},"end":{"line":14,"column":17}},"name":"s"},"operator":"+","right":{"type":"Literal","start":386,"end":389,"loc":{"start":{"line":14,"column":20},"end":{"line":14,"column":23}},"value":" ","raw":"\" \""}},"operator":"+","right":{"type":"CallExpression","start":392,"end":404,"loc":{"start":{"line":14,"column":26},"end":{"line":14,"column":38}},"callee":{"type":"Identifier","start":392,"end":401,"loc":{"start":{"line":14,"column":26},"end":{"line":14,"column":35}},"name":"stringify"},"arguments":[{"type":"Identifier","start":402,"end":403,"loc":{"start":{"line":14,"column":36},"end":{"line":14,"column":37}},"name":"v"}]}}]}},{"type":"ReturnStatement","start":411,"end":420,"loc":{"start":{"line":15,"column":4},"end":{"line":15,"column":13}},"argument":{"type":"Identifier","start":418,"end":419,"loc":{"start":{"line":15,"column":11},"end":{"line":15,"column":12}},"name":"v"}}]}},{"type":"ExpressionStatement","start":423,"end":447,"loc":{"start":{"line":17,"column":0},"end":{"line":17,"column":24}},"expression":{"type":"AssignmentExpression","start":423,"end":446,"loc":{"start":{"line":17,"column":0},"end":{"line":17,"column":23}},"operator":"=","left":{"type":"Identifier","start":423,"end":435,"loc":{"start":{"line":17,"column":0},"end":{"line":17,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":438,"end":446,"loc":{"start":{"line":17,"column":15},"end":{"line":17,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":448,"end":513,"loc":{"start":{"line":18,"column":0},"end":{"line":21,"column":1}},"id":{"type":"Identifier","start":457,"end":462,"loc":{"start":{"line":18,"column":9},"end":{"line":18,"column":14}},"name":"error"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":463,"end":464,"loc":{"start":{"line":18,"column":15},"end":{"line":18,"column":16}},"name":"v"}],"body":{"type":"BlockStatement","start":466,"end":513,"loc":{"start":{"line":18,"column":18},"end":{"line":21,"column":1}},"body":[{"type":"ExpressionStatement","start":472,"end":498,"loc":{"start":{"line":19,"column":4},"end":{"line":19,"column":30}},"expression":{"type":"CallExpression","start":472,"end":497,"loc":{"start":{"line":19,"column":4},"end":{"line":19,"column":29}},"callee":{"type":"Identifier","start":472,"end":483,"loc":{"start":{"line":19,"column":4},"end":{"line":19,"column":15}},"name":"ffi_display"},"arguments":[{"type":"CallExpression","start":484,"end":496,"loc":{"start":{"line":19,"column":16},"end":{"line":19,"column":28}},"callee":{"type":"Identifier","start":484,"end":493,"loc":{"start":{"line":19,"column":16},"end":{"line":19,"column":25}},"name":"stringify"},"arguments":[{"type":"Identifier","start":494,"end":495,"loc":{"start":{"line":19,"column":26},"end":{"line":19,"column":27}},"name":"v"}]}]}},{"type":"ExpressionStatement","start":503,"end":511,"loc":{"start":{"line":20,"column":4},"end":{"line":20,"column":12}},"expression":{"type":"CallExpression","start":503,"end":510,"loc":{"start":{"line":20,"column":4},"end":{"line":20,"column":11}},"callee":{"type":"Identifier","start":503,"end":508,"loc":{"start":{"line":20,"column":4},"end":{"line":20,"column":9}},"name":"abort"},"arguments":[]}}]}},{"type":"ExpressionStatement","start":514,"end":558,"loc":{"start":{"line":22,"column":0},"end":{"line":22,"column":44}},"expression":{"type":"AssignmentExpression","start":514,"end":557,"loc":{"start":{"line":22,"column":0},"end":{"line":22,"column":43}},"operator":"=","left":{"type":"Identifier","start":514,"end":526,"loc":{"start":{"line":22,"column":0},"end":{"line":22,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":529,"end":557,"loc":{"start":{"line":22,"column":15},"end":{"line":22,"column":43}},"value":"direct;constraint=s:string","raw":"\"direct;constraint=s:string\""}}},{"type":"FunctionDeclaration","start":559,"end":637,"loc":{"start":{"line":23,"column":0},"end":{"line":26,"column":1}},"id":{"type":"Identifier","start":568,"end":573,"loc":{"start":{"line":23,"column":9},"end":{"line":23,"column":14}},"name":"error"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":574,"end":575,"loc":{"start":{"line":23,"column":15},"end":{"line":23,"column":16}},"name":"v"},{"type":"Identifier","start":577,"end":578,"loc":{"start":{"line":23,"column":18},"end":{"line":23,"column":19}},"name":"s"}],"body":{"type":"BlockStatement","start":580,"end":637,"loc":{"start":{"line":23,"column":21},"end":{"line":26,"column":1}},"body":[{"type":"ExpressionStatement","start":586,"end":622,"loc":{"start":{"line":24,"column":4},"end":{"line":24,"column":40}},"expression":{"type":"CallExpression","start":586,"end":621,"loc":{"start":{"line":24,"column":4},"end":{"line":24,"column":39}},"callee":{"type":"Identifier","start":586,"end":597,"loc":{"start":{"line":24,"column":4},"end":{"line":24,"column":15}},"name":"ffi_display"},"arguments":[{"type":"BinaryExpression","start":598,"end":620,"loc":{"start":{"line":24,"column":16},"end":{"line":24,"column":38}},"left":{"type":"BinaryExpression","start":598,"end":605,"loc":{"start":{"line":24,"column":16},"end":{"line":24,"column":23}},"left":{"type":"Identifier","start":598,"end":599,"loc":{"start":{"line":24,"column":16},"end":{"line":24,"column":17}},"name":"s"},"operator":"+","right":{"type":"Literal","start":602,"end":605,"loc":{"start":{"line":24,"column":20},"end":{"line":24,"column":23}},"value":" ","raw":"\" \""}},"operator":"+","right":{"type":"CallExpression","start":608,"end":620,"loc":{"start":{"line":24,"column":26},"end":{"line":24,"column":38}},"callee":{"type":"Identifier","start":608,"end":617,"loc":{"start":{"line":24,"column":26},"end":{"line":24,"column":35}},"name":"stringify"},"arguments":[{"type":"Identifier","start":618,"end":619,"loc":{"start":{"line":24,"column":36},"end":{"line":24,"column":37}},"name":"v"}]}}]}},{"type":"ExpressionStatement","start":627,"end":635,"loc":{"start":{"line":25,"column":4},"end":{"line":25,"column":12}},"expression":{"type":"CallExpression","start":627,"end":634,"loc":{"start":{"line":25,"column":4},"end":{"line":25,"column":11}},"callee":{"type":"Identifier","start":627,"end":632,"loc":{"start":{"line":25,"column":4},"end":{"line":25,"column":9}},"name":"abort"},"arguments":[]}}]}},{"type":"ExpressionStatement","start":639,"end":663,"loc":{"start":{"line":28,"column":0},"end":{"line":28,"column":24}},"expression":{"type":"AssignmentExpression","start":639,"end":662,"loc":{"start":{"line":28,"column":0},"end":{"line":28,"column":23}},"operator":"=","left":{"type":"Identifier","start":639,"end":651,"loc":{"start":{"line":28,"column":0},"end":{"line":28,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":654,"end":662,"loc":{"start":{"line":28,"column":15},"end":{"line":28,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":664,"end":710,"loc":{"start":{"line":29,"column":0},"end":{"line":31,"column":1}},"id":{"type":"Identifier","start":673,"end":685,"loc":{"start":{"line":29,"column":9},"end":{"line":29,"column":21}},"name":"is_undefined"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":686,"end":687,"loc":{"start":{"line":29,"column":22},"end":{"line":29,"column":23}},"name":"v"}],"body":{"type":"BlockStatement","start":689,"end":710,"loc":{"start":{"line":29,"column":25},"end":{"line":31,"column":1}},"body":[{"type":"ReturnStatement","start":695,"end":708,"loc":{"start":{"line":30,"column":4},"end":{"line":30,"column":17}},"argument":{"type":"Literal","start":702,"end":707,"loc":{"start":{"line":30,"column":11},"end":{"line":30,"column":16}},"value":false,"raw":"false"}}]}},{"type":"ExpressionStatement","start":711,"end":758,"loc":{"start":{"line":32,"column":0},"end":{"line":32,"column":47}},"expression":{"type":"AssignmentExpression","start":711,"end":757,"loc":{"start":{"line":32,"column":0},"end":{"line":32,"column":46}},"operator":"=","left":{"type":"Identifier","start":711,"end":723,"loc":{"start":{"line":32,"column":0},"end":{"line":32,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":726,"end":757,"loc":{"start":{"line":32,"column":15},"end":{"line":32,"column":46}},"value":"direct;constraint=v:undefined","raw":"\"direct;constraint=v:undefined\""}}},{"type":"FunctionDeclaration","start":759,"end":804,"loc":{"start":{"line":33,"column":0},"end":{"line":35,"column":1}},"id":{"type":"Identifier","start":768,"end":780,"loc":{"start":{"line":33,"column":9},"end":{"line":33,"column":21}},"name":"is_undefined"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":781,"end":782,"loc":{"start":{"line":33,"column":22},"end":{"line":33,"column":23}},"name":"v"}],"body":{"type":"BlockStatement","start":784,"end":804,"loc":{"start":{"line":33,"column":25},"end":{"line":35,"column":1}},"body":[{"type":"ReturnStatement","start":790,"end":802,"loc":{"start":{"line":34,"column":4},"end":{"line":34,"column":16}},"argument":{"type":"Literal","start":797,"end":801,"loc":{"start":{"line":34,"column":11},"end":{"line":34,"column":15}},"value":true,"raw":"true"}}]}},{"type":"ExpressionStatement","start":805,"end":829,"loc":{"start":{"line":36,"column":0},"end":{"line":36,"column":24}},"expression":{"type":"AssignmentExpression","start":805,"end":828,"loc":{"start":{"line":36,"column":0},"end":{"line":36,"column":23}},"operator":"=","left":{"type":"Identifier","start":805,"end":817,"loc":{"start":{"line":36,"column":0},"end":{"line":36,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":820,"end":828,"loc":{"start":{"line":36,"column":15},"end":{"line":36,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":830,"end":874,"loc":{"start":{"line":37,"column":0},"end":{"line":39,"column":1}},"id":{"type":"Identifier","start":839,"end":849,"loc":{"start":{"line":37,"column":9},"end":{"line":37,"column":19}},"name":"is_boolean"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":850,"end":851,"loc":{"start":{"line":37,"column":20},"end":{"line":37,"column":21}},"name":"v"}],"body":{"type":"BlockStatement","start":853,"end":874,"loc":{"start":{"line":37,"column":23},"end":{"line":39,"column":1}},"body":[{"type":"ReturnStatement","start":859,"end":872,"loc":{"start":{"line":38,"column":4},"end":{"line":38,"column":17}},"argument":{"type":"Literal","start":866,"end":871,"loc":{"start":{"line":38,"column":11},"end":{"line":38,"column":16}},"value":false,"raw":"false"}}]}},{"type":"ExpressionStatement","start":875,"end":920,"loc":{"start":{"line":40,"column":0},"end":{"line":40,"column":45}},"expression":{"type":"AssignmentExpression","start":875,"end":919,"loc":{"start":{"line":40,"column":0},"end":{"line":40,"column":44}},"operator":"=","left":{"type":"Identifier","start":875,"end":887,"loc":{"start":{"line":40,"column":0},"end":{"line":40,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":890,"end":919,"loc":{"start":{"line":40,"column":15},"end":{"line":40,"column":44}},"value":"direct;constraint=v:boolean","raw":"\"direct;constraint=v:boolean\""}}},{"type":"FunctionDeclaration","start":921,"end":964,"loc":{"start":{"line":41,"column":0},"end":{"line":43,"column":1}},"id":{"type":"Identifier","start":930,"end":940,"loc":{"start":{"line":41,"column":9},"end":{"line":41,"column":19}},"name":"is_boolean"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":941,"end":942,"loc":{"start":{"line":41,"column":20},"end":{"line":41,"column":21}},"name":"v"}],"body":{"type":"BlockStatement","start":944,"end":964,"loc":{"start":{"line":41,"column":23},"end":{"line":43,"column":1}},"body":[{"type":"ReturnStatement","start":950,"end":962,"loc":{"start":{"line":42,"column":4},"end":{"line":42,"column":16}},"argument":{"type":"Literal","start":957,"end":961,"loc":{"start":{"line":42,"column":11},"end":{"line":42,"column":15}},"value":true,"raw":"true"}}]}},{"type":"ExpressionStatement","start":965,"end":989,"loc":{"start":{"line":44,"column":0},"end":{"line":44,"column":24}},"expression":{"type":"AssignmentExpression","start":965,"end":988,"loc":{"start":{"line":44,"column":0},"end":{"line":44,"column":23}},"operator":"=","left":{"type":"Identifier","start":965,"end":977,"loc":{"start":{"line":44,"column":0},"end":{"line":44,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":980,"end":988,"loc":{"start":{"line":44,"column":15},"end":{"line":44,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":990,"end":1033,"loc":{"start":{"line":45,"column":0},"end":{"line":47,"column":1}},"id":{"type":"Identifier","start":999,"end":1008,"loc":{"start":{"line":45,"column":9},"end":{"line":45,"column":18}},"name":"is_number"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1009,"end":1010,"loc":{"start":{"line":45,"column":19},"end":{"line":45,"column":20}},"name":"v"}],"body":{"type":"BlockStatement","start":1012,"end":1033,"loc":{"start":{"line":45,"column":22},"end":{"line":47,"column":1}},"body":[{"type":"ReturnStatement","start":1018,"end":1031,"loc":{"start":{"line":46,"column":4},"end":{"line":46,"column":17}},"argument":{"type":"Literal","start":1025,"end":1030,"loc":{"start":{"line":46,"column":11},"end":{"line":46,"column":16}},"value":false,"raw":"false"}}]}},{"type":"ExpressionStatement","start":1034,"end":1078,"loc":{"start":{"line":48,"column":0},"end":{"line":48,"column":44}},"expression":{"type":"AssignmentExpression","start":1034,"end":1077,"loc":{"start":{"line":48,"column":0},"end":{"line":48,"column":43}},"operator":"=","left":{"type":"Identifier","start":1034,"end":1046,"loc":{"start":{"line":48,"column":0},"end":{"line":48,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1049,"end":1077,"loc":{"start":{"line":48,"column":15},"end":{"line":48,"column":43}},"value":"direct;constraint=v:number","raw":"\"direct;constraint=v:number\""}}},{"type":"FunctionDeclaration","start":1079,"end":1121,"loc":{"start":{"line":49,"column":0},"end":{"line":51,"column":1}},"id":{"type":"Identifier","start":1088,"end":1097,"loc":{"start":{"line":49,"column":9},"end":{"line":49,"column":18}},"name":"is_number"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1098,"end":1099,"loc":{"start":{"line":49,"column":19},"end":{"line":49,"column":20}},"name":"v"}],"body":{"type":"BlockStatement","start":1101,"end":1121,"loc":{"start":{"line":49,"column":22},"end":{"line":51,"column":1}},"body":[{"type":"ReturnStatement","start":1107,"end":1119,"loc":{"start":{"line":50,"column":4},"end":{"line":50,"column":16}},"argument":{"type":"Literal","start":1114,"end":1118,"loc":{"start":{"line":50,"column":11},"end":{"line":50,"column":15}},"value":true,"raw":"true"}}]}},{"type":"ExpressionStatement","start":1122,"end":1146,"loc":{"start":{"line":52,"column":0},"end":{"line":52,"column":24}},"expression":{"type":"AssignmentExpression","start":1122,"end":1145,"loc":{"start":{"line":52,"column":0},"end":{"line":52,"column":23}},"operator":"=","left":{"type":"Identifier","start":1122,"end":1134,"loc":{"start":{"line":52,"column":0},"end":{"line":52,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1137,"end":1145,"loc":{"start":{"line":52,"column":15},"end":{"line":52,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":1147,"end":1190,"loc":{"start":{"line":53,"column":0},"end":{"line":55,"column":1}},"id":{"type":"Identifier","start":1156,"end":1165,"loc":{"start":{"line":53,"column":9},"end":{"line":53,"column":18}},"name":"is_string"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1166,"end":1167,"loc":{"start":{"line":53,"column":19},"end":{"line":53,"column":20}},"name":"v"}],"body":{"type":"BlockStatement","start":1169,"end":1190,"loc":{"start":{"line":53,"column":22},"end":{"line":55,"column":1}},"body":[{"type":"ReturnStatement","start":1175,"end":1188,"loc":{"start":{"line":54,"column":4},"end":{"line":54,"column":17}},"argument":{"type":"Literal","start":1182,"end":1187,"loc":{"start":{"line":54,"column":11},"end":{"line":54,"column":16}},"value":false,"raw":"false"}}]}},{"type":"ExpressionStatement","start":1191,"end":1235,"loc":{"start":{"line":56,"column":0},"end":{"line":56,"column":44}},"expression":{"type":"AssignmentExpression","start":1191,"end":1234,"loc":{"start":{"line":56,"column":0},"end":{"line":56,"column":43}},"operator":"=","left":{"type":"Identifier","start":1191,"end":1203,"loc":{"start":{"line":56,"column":0},"end":{"line":56,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1206,"end":1234,"loc":{"start":{"line":56,"column":15},"end":{"line":56,"column":43}},"value":"direct;constraint=v:string","raw":"\"direct;constraint=v:string\""}}},{"type":"FunctionDeclaration","start":1236,"end":1278,"loc":{"start":{"line":57,"column":0},"end":{"line":59,"column":1}},"id":{"type":"Identifier","start":1245,"end":1254,"loc":{"start":{"line":57,"column":9},"end":{"line":57,"column":18}},"name":"is_string"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1255,"end":1256,"loc":{"start":{"line":57,"column":19},"end":{"line":57,"column":20}},"name":"v"}],"body":{"type":"BlockStatement","start":1258,"end":1278,"loc":{"start":{"line":57,"column":22},"end":{"line":59,"column":1}},"body":[{"type":"ReturnStatement","start":1264,"end":1276,"loc":{"start":{"line":58,"column":4},"end":{"line":58,"column":16}},"argument":{"type":"Literal","start":1271,"end":1275,"loc":{"start":{"line":58,"column":11},"end":{"line":58,"column":15}},"value":true,"raw":"true"}}]}},{"type":"ExpressionStatement","start":1279,"end":1303,"loc":{"start":{"line":60,"column":0},"end":{"line":60,"column":24}},"expression":{"type":"AssignmentExpression","start":1279,"end":1302,"loc":{"start":{"line":60,"column":0},"end":{"line":60,"column":23}},"operator":"=","left":{"type":"Identifier","start":1279,"end":1291,"loc":{"start":{"line":60,"column":0},"end":{"line":60,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1294,"end":1302,"loc":{"start":{"line":60,"column":15},"end":{"line":60,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":1304,"end":1349,"loc":{"start":{"line":61,"column":0},"end":{"line":63,"column":1}},"id":{"type":"Identifier","start":1313,"end":1324,"loc":{"start":{"line":61,"column":9},"end":{"line":61,"column":20}},"name":"is_function"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1325,"end":1326,"loc":{"start":{"line":61,"column":21},"end":{"line":61,"column":22}},"name":"v"}],"body":{"type":"BlockStatement","start":1328,"end":1349,"loc":{"start":{"line":61,"column":24},"end":{"line":63,"column":1}},"body":[{"type":"ReturnStatement","start":1334,"end":1347,"loc":{"start":{"line":62,"column":4},"end":{"line":62,"column":17}},"argument":{"type":"Literal","start":1341,"end":1346,"loc":{"start":{"line":62,"column":11},"end":{"line":62,"column":16}},"value":false,"raw":"false"}}]}},{"type":"ExpressionStatement","start":1350,"end":1396,"loc":{"start":{"line":64,"column":0},"end":{"line":64,"column":46}},"expression":{"type":"AssignmentExpression","start":1350,"end":1395,"loc":{"start":{"line":64,"column":0},"end":{"line":64,"column":45}},"operator":"=","left":{"type":"Identifier","start":1350,"end":1362,"loc":{"start":{"line":64,"column":0},"end":{"line":64,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1365,"end":1395,"loc":{"start":{"line":64,"column":15},"end":{"line":64,"column":45}},"value":"direct;constraint=v:function","raw":"\"direct;constraint=v:function\""}}},{"type":"FunctionDeclaration","start":1397,"end":1441,"loc":{"start":{"line":65,"column":0},"end":{"line":67,"column":1}},"id":{"type":"Identifier","start":1406,"end":1417,"loc":{"start":{"line":65,"column":9},"end":{"line":65,"column":20}},"name":"is_function"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1418,"end":1419,"loc":{"start":{"line":65,"column":21},"end":{"line":65,"column":22}},"name":"v"}],"body":{"type":"BlockStatement","start":1421,"end":1441,"loc":{"start":{"line":65,"column":24},"end":{"line":67,"column":1}},"body":[{"type":"ReturnStatement","start":1427,"end":1439,"loc":{"start":{"line":66,"column":4},"end":{"line":66,"column":16}},"argument":{"type":"Literal","start":1434,"end":1438,"loc":{"start":{"line":66,"column":11},"end":{"line":66,"column":15}},"value":true,"raw":"true"}}]}},{"type":"ExpressionStatement","start":1443,"end":1487,"loc":{"start":{"line":69,"column":0},"end":{"line":69,"column":44}},"expression":{"type":"AssignmentExpression","start":1443,"end":1486,"loc":{"start":{"line":69,"column":0},"end":{"line":69,"column":43}},"operator":"=","left":{"type":"Identifier","start":1443,"end":1455,"loc":{"start":{"line":69,"column":0},"end":{"line":69,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":1458,"end":1486,"loc":{"start":{"line":69,"column":15},"end":{"line":69,"column":43}},"value":"direct;constraint=s:string","raw":"\"direct;constraint=s:string\""}}},{"type":"FunctionDeclaration","start":1488,"end":1582,"loc":{"start":{"line":70,"column":0},"end":{"line":73,"column":1}},"id":{"type":"Identifier","start":1497,"end":1503,"loc":{"start":{"line":70,"column":9},"end":{"line":70,"column":15}},"name":"prompt"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":1504,"end":1505,"loc":{"start":{"line":70,"column":16},"end":{"line":70,"column":17}},"name":"s"}],"body":{"type":"BlockStatement","start":1507,"end":1582,"loc":{"start":{"line":70,"column":19},"end":{"line":73,"column":1}},"body":[{"type":"VariableDeclaration","start":1513,"end":1537,"loc":{"start":{"line":71,"column":4},"end":{"line":71,"column":28}},"declarations":[{"type":"VariableDeclarator","start":1517,"end":1536,"loc":{"start":{"line":71,"column":8},"end":{"line":71,"column":27}},"id":{"type":"Identifier","start":1517,"end":1520,"loc":{"start":{"line":71,"column":8},"end":{"line":71,"column":11}},"name":"ret"},"init":{"type":"CallExpression","start":1523,"end":1536,"loc":{"start":{"line":71,"column":14},"end":{"line":71,"column":27}},"callee":{"type":"Identifier","start":1523,"end":1533,"loc":{"start":{"line":71,"column":14},"end":{"line":71,"column":24}},"name":"ffi_prompt"},"arguments":[{"type":"Identifier","start":1534,"end":1535,"loc":{"start":{"line":71,"column":25},"end":{"line":71,"column":26}},"name":"s"}]}}],"kind":"let"},{"type":"ReturnStatement","start":1542,"end":1580,"loc":{"start":{"line":72,"column":4},"end":{"line":72,"column":42}},"argument":{"type":"ConditionalExpression","start":1549,"end":1579,"loc":{"start":{"line":72,"column":11},"end":{"line":72,"column":41}},"test":{"type":"BinaryExpression","start":1549,"end":1561,"loc":{"start":{"line":72,"column":11},"end":{"line":72,"column":23}},"left":{"type":"Identifier","start":1549,"end":1552,"loc":{"start":{"line":72,"column":11},"end":{"line":72,"column":14}},"name":"ret"},"operator":"===","right":{"type":"Literal","start":1557,"end":1561,"loc":{"start":{"line":72,"column":19},"end":{"line":72,"column":23}},"value":"\u0000","raw":"\"\\0\""}},"consequent":{"type":"Identifier","start":1564,"end":1573,"loc":{"start":{"line":72,"column":26},"end":{"line":72,"column":35}},"name":"undefined"},"alternate":{"type":"Identifier","start":1576,"end":1579,"loc":{"start":{"line":72,"column":38},"end":{"line":72,"column":41}},"name":"ret"}}}]}},{"type":"ExpressionStatement","start":2100,"end":2124,"loc":{"start":{"line":96,"column":0},"end":{"line":96,"column":24}},"expression":{"type":"AssignmentExpression","start":2100,"end":2123,"loc":{"start":{"line":96,"column":0},"end":{"line":96,"column":23}},"operator":"=","left":{"type":"Identifier","start":2100,"end":2112,"loc":{"start":{"line":96,"column":0},"end":{"line":96,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":2115,"end":2123,"loc":{"start":{"line":96,"column":15},"end":{"line":96,"column":23}},"value":"direct","raw":"\"direct\""}}},{"type":"FunctionDeclaration","start":2125,"end":2187,"loc":{"start":{"line":97,"column":0},"end":{"line":99,"column":1}},"id":{"type":"Identifier","start":2134,"end":2143,"loc":{"start":{"line":97,"column":9},"end":{"line":97,"column":18}},"name":"stringify"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":2144,"end":2145,"loc":{"start":{"line":97,"column":19},"end":{"line":97,"column":20}},"name":"v"}],"body":{"type":"BlockStatement","start":2147,"end":2187,"loc":{"start":{"line":97,"column":22},"end":{"line":99,"column":1}},"body":[{"type":"ReturnStatement","start":2153,"end":2185,"loc":{"start":{"line":98,"column":4},"end":{"line":98,"column":36}},"argument":{"type":"Literal","start":2160,"end":2184,"loc":{"start":{"line":98,"column":11},"end":{"line":98,"column":35}},"value":"(unstringifiable type)","raw":"\"(unstringifiable type)\""}}]}},{"type":"ExpressionStatement","start":2188,"end":2235,"loc":{"start":{"line":100,"column":0},"end":{"line":100,"column":47}},"expression":{"type":"AssignmentExpression","start":2188,"end":2234,"loc":{"start":{"line":100,"column":0},"end":{"line":100,"column":46}},"operator":"=","left":{"type":"Identifier","start":2188,"end":2200,"loc":{"start":{"line":100,"column":0},"end":{"line":100,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":2203,"end":2234,"loc":{"start":{"line":100,"column":15},"end":{"line":100,"column":46}},"value":"direct;constraint=v:undefined","raw":"\"direct;constraint=v:undefined\""}}},{"type":"FunctionDeclaration","start":2236,"end":2285,"loc":{"start":{"line":101,"column":0},"end":{"line":103,"column":1}},"id":{"type":"Identifier","start":2245,"end":2254,"loc":{"start":{"line":101,"column":9},"end":{"line":101,"column":18}},"name":"stringify"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":2255,"end":2256,"loc":{"start":{"line":101,"column":19},"end":{"line":101,"column":20}},"name":"v"}],"body":{"type":"BlockStatement","start":2258,"end":2285,"loc":{"start":{"line":101,"column":22},"end":{"line":103,"column":1}},"body":[{"type":"ReturnStatement","start":2264,"end":2283,"loc":{"start":{"line":102,"column":4},"end":{"line":102,"column":23}},"argument":{"type":"Literal","start":2271,"end":2282,"loc":{"start":{"line":102,"column":11},"end":{"line":102,"column":22}},"value":"undefined","raw":"\"undefined\""}}]}},{"type":"ExpressionStatement","start":2286,"end":2331,"loc":{"start":{"line":104,"column":0},"end":{"line":104,"column":45}},"expression":{"type":"AssignmentExpression","start":2286,"end":2330,"loc":{"start":{"line":104,"column":0},"end":{"line":104,"column":44}},"operator":"=","left":{"type":"Identifier","start":2286,"end":2298,"loc":{"start":{"line":104,"column":0},"end":{"line":104,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":2301,"end":2330,"loc":{"start":{"line":104,"column":15},"end":{"line":104,"column":44}},"value":"direct;constraint=v:boolean","raw":"\"direct;constraint=v:boolean\""}}},{"type":"FunctionDeclaration","start":2332,"end":2390,"loc":{"start":{"line":105,"column":0},"end":{"line":107,"column":1}},"id":{"type":"Identifier","start":2341,"end":2350,"loc":{"start":{"line":105,"column":9},"end":{"line":105,"column":18}},"name":"stringify"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":2351,"end":2352,"loc":{"start":{"line":105,"column":19},"end":{"line":105,"column":20}},"name":"v"}],"body":{"type":"BlockStatement","start":2354,"end":2390,"loc":{"start":{"line":105,"column":22},"end":{"line":107,"column":1}},"body":[{"type":"ReturnStatement","start":2360,"end":2388,"loc":{"start":{"line":106,"column":4},"end":{"line":106,"column":32}},"argument":{"type":"ConditionalExpression","start":2367,"end":2387,"loc":{"start":{"line":106,"column":11},"end":{"line":106,"column":31}},"test":{"type":"Identifier","start":2367,"end":2368,"loc":{"start":{"line":106,"column":11},"end":{"line":106,"column":12}},"name":"v"},"consequent":{"type":"Literal","start":2371,"end":2377,"loc":{"start":{"line":106,"column":15},"end":{"line":106,"column":21}},"value":"true","raw":"\"true\""},"alternate":{"type":"Literal","start":2380,"end":2387,"loc":{"start":{"line":106,"column":24},"end":{"line":106,"column":31}},"value":"false","raw":"\"false\""}}}]}},{"type":"ExpressionStatement","start":2391,"end":2435,"loc":{"start":{"line":108,"column":0},"end":{"line":108,"column":44}},"expression":{"type":"AssignmentExpression","start":2391,"end":2434,"loc":{"start":{"line":108,"column":0},"end":{"line":108,"column":43}},"operator":"=","left":{"type":"Identifier","start":2391,"end":2403,"loc":{"start":{"line":108,"column":0},"end":{"line":108,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":2406,"end":2434,"loc":{"start":{"line":108,"column":15},"end":{"line":108,"column":43}},"value":"direct;constraint=v:number","raw":"\"direct;constraint=v:number\""}}},{"type":"FunctionDeclaration","start":2436,"end":2492,"loc":{"start":{"line":109,"column":0},"end":{"line":111,"column":1}},"id":{"type":"Identifier","start":2445,"end":2454,"loc":{"start":{"line":109,"column":9},"end":{"line":109,"column":18}},"name":"stringify"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":2455,"end":2456,"loc":{"start":{"line":109,"column":19},"end":{"line":109,"column":20}},"name":"v"}],"body":{"type":"BlockStatement","start":2458,"end":2492,"loc":{"start":{"line":109,"column":22},"end":{"line":111,"column":1}},"body":[{"type":"ReturnStatement","start":2464,"end":2490,"loc":{"start":{"line":110,"column":4},"end":{"line":110,"column":30}},"argument":{"type":"CallExpression","start":2471,"end":2489,"loc":{"start":{"line":110,"column":11},"end":{"line":110,"column":29}},"callee":{"type":"Identifier","start":2471,"end":2486,"loc":{"start":{"line":110,"column":11},"end":{"line":110,"column":26}},"name":"stringify_float"},"arguments":[{"type":"Identifier","start":2487,"end":2488,"loc":{"start":{"line":110,"column":27},"end":{"line":110,"column":28}},"name":"v"}]}}]}},{"type":"ExpressionStatement","start":2493,"end":2537,"loc":{"start":{"line":112,"column":0},"end":{"line":112,"column":44}},"expression":{"type":"AssignmentExpression","start":2493,"end":2536,"loc":{"start":{"line":112,"column":0},"end":{"line":112,"column":43}},"operator":"=","left":{"type":"Identifier","start":2493,"end":2505,"loc":{"start":{"line":112,"column":0},"end":{"line":112,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":2508,"end":2536,"loc":{"start":{"line":112,"column":15},"end":{"line":112,"column":43}},"value":"direct;constraint=v:string","raw":"\"direct;constraint=v:string\""}}},{"type":"FunctionDeclaration","start":2538,"end":2591,"loc":{"start":{"line":113,"column":0},"end":{"line":115,"column":1}},"id":{"type":"Identifier","start":2547,"end":2556,"loc":{"start":{"line":113,"column":9},"end":{"line":113,"column":18}},"name":"stringify"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":2557,"end":2558,"loc":{"start":{"line":113,"column":19},"end":{"line":113,"column":20}},"name":"v"}],"body":{"type":"BlockStatement","start":2560,"end":2591,"loc":{"start":{"line":113,"column":22},"end":{"line":115,"column":1}},"body":[{"type":"ReturnStatement","start":2566,"end":2589,"loc":{"start":{"line":114,"column":4},"end":{"line":114,"column":27}},"argument":{"type":"BinaryExpression","start":2573,"end":2588,"loc":{"start":{"line":114,"column":11},"end":{"line":114,"column":26}},"left":{"type":"BinaryExpression","start":2573,"end":2581,"loc":{"start":{"line":114,"column":11},"end":{"line":114,"column":19}},"left":{"type":"Literal","start":2573,"end":2577,"loc":{"start":{"line":114,"column":11},"end":{"line":114,"column":15}},"value":"\"","raw":"\"\\\"\""},"operator":"+","right":{"type":"Identifier","start":2580,"end":2581,"loc":{"start":{"line":114,"column":18},"end":{"line":114,"column":19}},"name":"v"}},"operator":"+","right":{"type":"Literal","start":2584,"end":2588,"loc":{"start":{"line":114,"column":22},"end":{"line":114,"column":26}},"value":"\"","raw":"\"\\\"\""}}}]}},{"type":"ExpressionStatement","start":2592,"end":2638,"loc":{"start":{"line":116,"column":0},"end":{"line":116,"column":46}},"expression":{"type":"AssignmentExpression","start":2592,"end":2637,"loc":{"start":{"line":116,"column":0},"end":{"line":116,"column":45}},"operator":"=","left":{"type":"Identifier","start":2592,"end":2604,"loc":{"start":{"line":116,"column":0},"end":{"line":116,"column":12}},"name":"__attributes"},"right":{"type":"Literal","start":2607,"end":2637,"loc":{"start":{"line":116,"column":15},"end":{"line":116,"column":45}},"value":"direct;constraint=v:function","raw":"\"direct;constraint=v:function\""}}},{"type":"FunctionDeclaration","start":2639,"end":2689,"loc":{"start":{"line":117,"column":0},"end":{"line":119,"column":1}},"id":{"type":"Identifier","start":2648,"end":2657,"loc":{"start":{"line":117,"column":9},"end":{"line":117,"column":18}},"name":"stringify"},"expression":false,"generator":false,"params":[{"type":"Identifier","start":2658,"end":2659,"loc":{"start":{"line":117,"column":19},"end":{"line":117,"column":20}},"name":"v"}],"body":{"type":"BlockStatement","start":2661,"end":2689,"loc":{"start":{"line":117,"column":22},"end":{"line":119,"column":1}},"body":[{"type":"ReturnStatement","start":2667,"end":2687,"loc":{"start":{"line":118,"column":4},"end":{"line":118,"column":24}},"argument":{"type":"Literal","start":2674,"end":2686,"loc":{"start":{"line":118,"column":11},"end":{"line":118,"column":23}},"value":"(function)","raw":"\"(function)\""}}]}},{"type":"ExportNamedDeclaration","start":2691,"end":2868,"loc":{"start":{"line":121,"column":0},"end":{"line":127,"column":16}},"declaration":null,"specifiers":[{"type":"ExportSpecifier","start":2704,"end":2712,"loc":{"start":{"line":122,"column":4},"end":{"line":122,"column":12}},"local":{"type":"Identifier","start":2704,"end":2712,"loc":{"start":{"line":122,"column":4},"end":{"line":122,"column":12}},"name":"Infinity"},"exported":{"type":"Identifier","start":2704,"end":2712,"loc":{"start":{"line":122,"column":4},"end":{"line":122,"column":12}},"name":"Infinity"}},{"type":"ExportSpecifier","start":2714,"end":2717,"loc":{"start":{"line":122,"column":14},"end":{"line":122,"column":17}},"local":{"type":"Identifier","start":2714,"end":2717,"loc":{"start":{"line":122,"column":14},"end":{"line":122,"column":17}},"name":"NaN"},"exported":{"type":"Identifier","start":2714,"end":2717,"loc":{"start":{"line":122,"column":14},"end":{"line":122,"column":17}},"name":"NaN"}},{"type":"ExportSpecifier","start":2719,"end":2728,"loc":{"start":{"line":122,"column":19},"end":{"line":122,"column":28}},"local":{"type":"Identifier","start":2719,"end":2728,"loc":{"start":{"line":122,"column":19},"end":{"line":122,"column":28}},"name":"undefined"},"exported":{"type":"Identifier","start":2719,"end":2728,"loc":{"start":{"line":122,"column":19},"end":{"line":122,"column":28}},"name":"undefined"}},{"type":"ExportSpecifier","start":2734,"end":2746,"loc":{"start":{"line":123,"column":4},"end":{"line":123,"column":16}},"local":{"type":"Identifier","start":2734,"end":2746,"loc":{"start":{"line":123,"column":4},"end":{"line":123,"column":16}},"name":"is_undefined"},"exported":{"type":"Identifier","start":2734,"end":2746,"loc":{"start":{"line":123,"column":4},"end":{"line":123,"column":16}},"name":"is_undefined"}},{"type":"ExportSpecifier","start":2748,"end":2758,"loc":{"start":{"line":123,"column":18},"end":{"line":123,"column":28}},"local":{"type":"Identifier","start":2748,"end":2758,"loc":{"start":{"line":123,"column":18},"end":{"line":123,"column":28}},"name":"is_boolean"},"exported":{"type":"Identifier","start":2748,"end":2758,"loc":{"start":{"line":123,"column":18},"end":{"line":123,"column":28}},"name":"is_boolean"}},{"type":"ExportSpecifier","start":2760,"end":2769,"loc":{"start":{"line":123,"column":30},"end":{"line":123,"column":39}},"local":{"type":"Identifier","start":2760,"end":2769,"loc":{"start":{"line":123,"column":30},"end":{"line":123,"column":39}},"name":"is_number"},"exported":{"type":"Identifier","start":2760,"end":2769,"loc":{"start":{"line":123,"column":30},"end":{"line":123,"column":39}},"name":"is_number"}},{"type":"ExportSpecifier","start":2771,"end":2780,"loc":{"start":{"line":123,"column":41},"end":{"line":123,"column":50}},"local":{"type":"Identifier","start":2771,"end":2780,"loc":{"start":{"line":123,"column":41},"end":{"line":123,"column":50}},"name":"is_string"},"exported":{"type":"Identifier","start":2771,"end":2780,"loc":{"start":{"line":123,"column":41},"end":{"line":123,"column":50}},"name":"is_string"}},{"type":"ExportSpecifier","start":2782,"end":2793,"loc":{"start":{"line":123,"column":52},"end":{"line":123,"column":63}},"local":{"type":"Identifier","start":2782,"end":2793,"loc":{"start":{"line":123,"column":52},"end":{"line":123,"column":63}},"name":"is_function"},"exported":{"type":"Identifier","start":2782,"end":2793,"loc":{"start":{"line":123,"column":52},"end":{"line":123,"column":63}},"name":"is_function"}},{"type":"ExportSpecifier","start":2799,"end":2808,"loc":{"start":{"line":124,"column":4},"end":{"line":124,"column":13}},"local":{"type":"Identifier","start":2799,"end":2808,"loc":{"start":{"line":124,"column":4},"end":{"line":124,"column":13}},"name":"stringify"},"exported":{"type":"Identifier","start":2799,"end":2808,"loc":{"start":{"line":124,"column":4},"end":{"line":124,"column":13}},"name":"stringify"}},{"type":"ExportSpecifier","start":2814,"end":2822,"loc":{"start":{"line":125,"column":4},"end":{"line":125,"column":12}},"local":{"type":"Identifier","start":2814,"end":2822,"loc":{"start":{"line":125,"column":4},"end":{"line":125,"column":12}},"name":"get_time"},"exported":{"type":"Identifier","start":2814,"end":2822,"loc":{"start":{"line":125,"column":4},"end":{"line":125,"column":12}},"name":"get_time"}},{"type":"ExportSpecifier","start":2828,"end":2835,"loc":{"start":{"line":126,"column":4},"end":{"line":126,"column":11}},"local":{"type":"Identifier","start":2828,"end":2835,"loc":{"start":{"line":126,"column":4},"end":{"line":126,"column":11}},"name":"display"},"exported":{"type":"Identifier","start":2828,"end":2835,"loc":{"start":{"line":126,"column":4},"end":{"line":126,"column":11}},"name":"display"}},{"type":"ExportSpecifier","start":2837,"end":2842,"loc":{"start":{"line":126,"column":13},"end":{"line":126,"column":18}},"local":{"type":"Identifier","start":2837,"end":2842,"loc":{"start":{"line":126,"column":13},"end":{"line":126,"column":18}},"name":"error"},"exported":{"type":"Identifier","start":2837,"end":2842,"loc":{"start":{"line":126,"column":13},"end":{"line":126,"column":18}},"name":"error"}},{"type":"ExportSpecifier","start":2844,"end":2850,"loc":{"start":{"line":126,"column":20},"end":{"line":126,"column":26}},"local":{"type":"Identifier","start":2844,"end":2850,"loc":{"start":{"line":126,"column":20},"end":{"line":126,"column":26}},"name":"prompt"},"exported":{"type":"Identifier","start":2844,"end":2850,"loc":{"start":{"line":126,"column":20},"end":{"line":126,"column":26}},"name":"prompt"}},{"type":"ExportSpecifier","start":2856,"end":2865,"loc":{"start":{"line":127,"column":4},"end":{"line":127,"column":13}},"local":{"type":"Identifier","start":2856,"end":2865,"loc":{"start":{"line":127,"column":4},"end":{"line":127,"column":13}},"name":"parse_int"},"exported":{"type":"Identifier","start":2856,"end":2865,"loc":{"start":{"line":127,"column":4},"end":{"line":127,"column":13}},"name":"parse_int"}}],"source":null}],"sourceType":"module"}
"#;

#[derive(Copy, Clone)]
pub struct MainLogger {}
impl log::Logger for MainLogger {
    fn log<L: log::Loggable>(&self, content: L) {
        print!(
            "logger: {}: {}: {}",
            content.severity(),
            content.location(),
            content.message()
        );
    }
}

pub async fn fetch_dep_proxy(name: String) -> Option<String> {
    match name.as_str() {
        "https://btzy.github.io/libsourceror/std/misc.source" => Some(IMPORT_MISC.to_owned()),
        "https://btzy.github.io/libsourceror/std/misc.ffi" => Some(IMPORT_MISC_FFI.to_owned()),
        "https://btzy.github.io/libsourceror/std/math.source" => Some(IMPORT_MATH.to_owned()),
        "https://btzy.github.io/libsourceror/std/math.ffi" => Some(IMPORT_MATH_FFI.to_owned()),
        _ => None,
    }
}

// Retreive AST console input until EOF. Allows command: acorn source.js | cargo run
fn input_ast() -> String {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut source_ast = String::new();
    while let Some(line) = lines.next() {
        source_ast = source_ast + " " + line.unwrap().trim();
    }
    return source_ast;
}

fn main() {
    // use std::io;
    println!("Attach debugger to pid: {} [Please type in the AST (Tip: acorn source.js | cargo run)]", 
             process::id());

    let source_ast = SOURCE_CODE;
    // println!("{}", source_ast);


    let _: () = futures::executor::block_on((|| async {
        use wasmgen::WasmSerialize;

        //let ir_imports = frontend_estree::parse_imports(import_spec, MainLogger::new(context))?;
        let ir_program =
            frontend_estree::run_frontend(source_ast.to_owned(), fetch_dep_proxy, MainLogger {})
            .await?;
        {
            
            std::fs::create_dir("dump").unwrap_or_default();
        }

        {
            use std::io::prelude::*;
            let mut file = std::fs::File::create("dump/out-noop.ir").unwrap();
            file.write_all(format!("{:#?}", &ir_program).as_bytes())
                .unwrap();
        }
        let ir_program_opt = ir::opt::optimize_all(ir_program);
        // println!("{:#?}", &ir_program_opt);
        {
            use std::io::prelude::*;
            let mut file = std::fs::File::create("dump/out.ir").unwrap();
            file.write_all(format!("{:#?}", &ir_program_opt).as_bytes())
                .unwrap();
        }
        let wasm_module =
            backend_wasm::run_backend(&ir_program_opt, backend_wasm::Options::default());
        let mut receiver = std::vec::Vec::<u8>::new();
        wasm_module.wasm_serialize(&mut receiver);
        {
            use std::io::prelude::*;
            let mut file = std::fs::File::create("dump/out.wasm").unwrap();
            file.write_all(&receiver).unwrap();
        }
        Ok(())
    })())
    .unwrap_or_else(|_: ()| panic!("Frontend errored out"));
}
