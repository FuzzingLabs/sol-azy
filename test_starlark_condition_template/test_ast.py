AST = {
  "raw_node": {
    "ident": "SplTokenAccount",
    "position": {
      "end_column": 35,
      "end_line": 12,
      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
      "start_column": 20,
      "start_line": 12
    }
  },
  "access_path": "[4].mod.content[1].fn.stmts[0].let.init.expr.try.expr.call.func.path.segments[0]",
  "metadata": {
    "position": {
      "end_column": 35,
      "end_line": 12,
      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
      "start_column": 20,
      "start_line": 12
    }
  },
  "children": [],
  "parent": {
    "raw_node": {
      "ident": "log_message",
      "inputs": [
        {
          "typed": {
            "pat": {
              "ident": {
                "ident": "ctx",
                "position": {
                  "end_column": 14,
                  "end_line": 13,
                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                  "start_column": 11,
                  "start_line": 13
                }
              }
            },
            "ty": {
              "path": {
                "segments": [
                  {
                    "arguments": {
                      "angle_bracketed": {
                        "args": [
                          {
                            "type": {
                              "path": {
                                "segments": [
                                  {
                                    "ident": "LogMessage",
                                    "position": {
                                      "end_column": 21,
                                      "end_line": 22,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                      "start_column": 11,
                                      "start_line": 22
                                    }
                                  }
                                ]
                              }
                            }
                          }
                        ]
                      }
                    },
                    "ident": "Context",
                    "position": {
                      "end_column": 35,
                      "end_line": 11,
                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                      "start_column": 28,
                      "start_line": 11
                    }
                  }
                ]
              }
            }
          }
        }
      ],
      "output": {
        "path": {
          "segments": [
            {
              "ident": "ProgramResult",
              "position": {
                "end_column": 65,
                "end_line": 11,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 52,
                "start_line": 11
              }
            }
          ]
        }
      },
      "position": {
        "end_column": 22,
        "end_line": 11,
        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
        "start_column": 11,
        "start_line": 11
      },
      "stmts": [
        {
          "let": {
            "init": {
              "expr": {
                "try": {
                  "expr": {
                    "call": {
                      "args": [
                        {
                          "reference": {
                            "expr": {
                              "method_call": {
                                "args": [],
                                "method": "borrow",
                                "receiver": {
                                  "field": {
                                    "base": {
                                      "field": {
                                        "base": {
                                          "field": {
                                            "base": {
                                              "path": {
                                                "segments": [
                                                  {
                                                    "ident": "ctx",
                                                    "position": {
                                                      "end_column": 14,
                                                      "end_line": 13,
                                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                                      "start_column": 11,
                                                      "start_line": 13
                                                    }
                                                  }
                                                ]
                                              }
                                            },
                                            "ident": "accounts",
                                            "position": {
                                              "end_column": 23,
                                              "end_line": 13,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                              "start_column": 15,
                                              "start_line": 13
                                            }
                                          }
                                        },
                                        "ident": "token",
                                        "position": {
                                          "end_column": 9,
                                          "end_line": 23,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                          "start_column": 4,
                                          "start_line": 23
                                        }
                                      }
                                    },
                                    "ident": "data",
                                    "position": {
                                      "end_column": 68,
                                      "end_line": 12,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                      "start_column": 64,
                                      "start_line": 12
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                      ],
                      "func": {
                        "path": {
                          "segments": [
                            {
                              "ident": "SplTokenAccount",
                              "position": {
                                "end_column": 35,
                                "end_line": 12,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                "start_column": 20,
                                "start_line": 12
                              }
                            },
                            {
                              "ident": "unpack",
                              "position": {
                                "end_column": 43,
                                "end_line": 12,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                "start_column": 37,
                                "start_line": 12
                              }
                            }
                          ]
                        }
                      }
                    }
                  }
                }
              }
            },
            "pat": {
              "ident": {
                "ident": "token",
                "position": {
                  "end_column": 9,
                  "end_line": 23,
                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                  "start_column": 4,
                  "start_line": 23
                }
              }
            }
          }
        },
        {
          "expr": [
            {
              "if": {
                "cond": {
                  "binary": {
                    "left": {
                      "field": {
                        "base": {
                          "field": {
                            "base": {
                              "field": {
                                "base": {
                                  "path": {
                                    "segments": [
                                      {
                                        "ident": "ctx",
                                        "position": {
                                          "end_column": 14,
                                          "end_line": 13,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                          "start_column": 11,
                                          "start_line": 13
                                        }
                                      }
                                    ]
                                  }
                                },
                                "ident": "accounts",
                                "position": {
                                  "end_column": 23,
                                  "end_line": 13,
                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                  "start_column": 15,
                                  "start_line": 13
                                }
                              }
                            },
                            "ident": "authority",
                            "position": {
                              "end_column": 13,
                              "end_line": 24,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                              "start_column": 4,
                              "start_line": 24
                            }
                          }
                        },
                        "ident": "key",
                        "position": {
                          "end_column": 37,
                          "end_line": 13,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                          "start_column": 34,
                          "start_line": 13
                        }
                      }
                    },
                    "op": "!=",
                    "right": {
                      "reference": {
                        "expr": {
                          "field": {
                            "base": {
                              "path": {
                                "segments": [
                                  {
                                    "ident": "token",
                                    "position": {
                                      "end_column": 9,
                                      "end_line": 23,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                      "start_column": 4,
                                      "start_line": 23
                                    }
                                  }
                                ]
                              }
                            },
                            "ident": "owner",
                            "position": {
                              "end_column": 53,
                              "end_line": 13,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                              "start_column": 48,
                              "start_line": 13
                            }
                          }
                        }
                      }
                    }
                  }
                },
                "then_branch": [
                  {
                    "expr": [
                      {
                        "return": {
                          "expr": {
                            "call": {
                              "args": [
                                {
                                  "path": {
                                    "segments": [
                                      {
                                        "ident": "ProgramError",
                                        "position": {
                                          "end_column": 35,
                                          "end_line": 14,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                          "start_column": 23,
                                          "start_line": 14
                                        }
                                      },
                                      {
                                        "ident": "InvalidAccountData",
                                        "position": {
                                          "end_column": 55,
                                          "end_line": 14,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                          "start_column": 37,
                                          "start_line": 14
                                        }
                                      }
                                    ]
                                  }
                                }
                              ],
                              "func": {
                                "path": {
                                  "segments": [
                                    {
                                      "ident": "Err",
                                      "position": {
                                        "end_column": 22,
                                        "end_line": 14,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                        "start_column": 19,
                                        "start_line": 14
                                      }
                                    }
                                  ]
                                }
                              }
                            }
                          }
                        }
                      },
                      "True"
                    ]
                  }
                ]
              }
            },
            "False"
          ]
        },
        {
          "macro": {
            "delimiter": "paren",
            "path": {
              "segments": [
                {
                  "ident": "msg",
                  "position": {
                    "end_column": 11,
                    "end_line": 16,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 8,
                    "start_line": 16
                  }
                }
              ]
            },
            "semi_token": "True",
            "tokens": [
              {
                "lit": "\"Your account balance is: {}\""
              },
              {
                "punct": {
                  "op": ",",
                  "spacing": "alone"
                }
              },
              {
                "ident": "token",
                "position": {
                  "end_column": 9,
                  "end_line": 23,
                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                  "start_column": 4,
                  "start_line": 23
                }
              },
              {
                "punct": {
                  "op": ".",
                  "spacing": "alone"
                }
              },
              {
                "ident": "amount"
              }
            ]
          }
        },
        {
          "expr": [
            {
              "call": {
                "args": [
                  {
                    "tuple": {
                      "elems": []
                    }
                  }
                ],
                "func": {
                  "path": {
                    "segments": [
                      {
                        "ident": "Ok",
                        "position": {
                          "end_column": 10,
                          "end_line": 17,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                          "start_column": 8,
                          "start_line": 17
                        }
                      }
                    ]
                  }
                }
              }
            },
            "False"
          ]
        }
      ],
      "vis": "pub"
    },
    "access_path": "[4].mod.content[1].fn",
    "metadata": {
      "position": {
        "end_column": 22,
        "end_line": 11,
        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
        "start_column": 11,
        "start_line": 11
      }
    },
    "children": [
      {
        "raw_node": {
          "ident": {
            "ident": "ctx",
            "position": {
              "end_column": 14,
              "end_line": 13,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
              "start_column": 11,
              "start_line": 13
            }
          }
        },
        "access_path": "[4].mod.content[1].fn.inputs[0].typed.pat",
        "metadata": {},
        "children": [
          {
            "raw_node": {
              "ident": "ctx",
              "position": {
                "end_column": 14,
                "end_line": 13,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 11,
                "start_line": 13
              }
            },
            "access_path": "[4].mod.content[1].fn.inputs[0].typed.pat.ident",
            "metadata": {
              "position": {
                "end_column": 14,
                "end_line": 13,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 11,
                "start_line": 13
              }
            },
            "children": [],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "ctx"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "ctx"
      },
      {
        "raw_node": {
          "arguments": {
            "angle_bracketed": {
              "args": [
                {
                  "type": {
                    "path": {
                      "segments": [
                        {
                          "ident": "LogMessage",
                          "position": {
                            "end_column": 21,
                            "end_line": 22,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 22
                          }
                        }
                      ]
                    }
                  }
                }
              ]
            }
          },
          "ident": "Context",
          "position": {
            "end_column": 35,
            "end_line": 11,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 28,
            "start_line": 11
          }
        },
        "access_path": "[4].mod.content[1].fn.inputs[0].typed.ty.path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 35,
            "end_line": 11,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 28,
            "start_line": 11
          }
        },
        "children": [
          {
            "raw_node": {
              "ident": "LogMessage",
              "position": {
                "end_column": 21,
                "end_line": 22,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 11,
                "start_line": 22
              }
            },
            "access_path": "[4].mod.content[1].fn.inputs[0].typed.ty.path.segments[0].arguments.angle_bracketed.args[0].type.path.segments[0]",
            "metadata": {
              "position": {
                "end_column": 21,
                "end_line": 22,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 11,
                "start_line": 22
              }
            },
            "children": [],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "LogMessage"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "Context"
      },
      {
        "raw_node": {
          "ident": "ProgramResult",
          "position": {
            "end_column": 65,
            "end_line": 11,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 52,
            "start_line": 11
          }
        },
        "access_path": "[4].mod.content[1].fn.output.path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 65,
            "end_line": 11,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 52,
            "start_line": 11
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "ProgramResult"
      },
      {
        "raw_node": {
          "args": [],
          "method": "borrow",
          "receiver": {
            "field": {
              "base": {
                "field": {
                  "base": {
                    "field": {
                      "base": {
                        "path": {
                          "segments": [
                            {
                              "ident": "ctx",
                              "position": {
                                "end_column": 14,
                                "end_line": 13,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                "start_column": 11,
                                "start_line": 13
                              }
                            }
                          ]
                        }
                      },
                      "ident": "accounts",
                      "position": {
                        "end_column": 23,
                        "end_line": 13,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 15,
                        "start_line": 13
                      }
                    }
                  },
                  "ident": "token",
                  "position": {
                    "end_column": 9,
                    "end_line": 23,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 23
                  }
                }
              },
              "ident": "data",
              "position": {
                "end_column": 68,
                "end_line": 12,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 64,
                "start_line": 12
              }
            }
          }
        },
        "access_path": "[4].mod.content[1].fn.stmts[0].let.init.expr.try.expr.call.args[0].reference.expr.method_call",
        "metadata": {},
        "children": [
          {
            "raw_node": {
              "base": {
                "field": {
                  "base": {
                    "field": {
                      "base": {
                        "path": {
                          "segments": [
                            {
                              "ident": "ctx",
                              "position": {
                                "end_column": 14,
                                "end_line": 13,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                "start_column": 11,
                                "start_line": 13
                              }
                            }
                          ]
                        }
                      },
                      "ident": "accounts",
                      "position": {
                        "end_column": 23,
                        "end_line": 13,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 15,
                        "start_line": 13
                      }
                    }
                  },
                  "ident": "token",
                  "position": {
                    "end_column": 9,
                    "end_line": 23,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 23
                  }
                }
              },
              "ident": "data",
              "position": {
                "end_column": 68,
                "end_line": 12,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 64,
                "start_line": 12
              }
            },
            "access_path": "[4].mod.content[1].fn.stmts[0].let.init.expr.try.expr.call.args[0].reference.expr.method_call.receiver.field",
            "metadata": {
              "position": {
                "end_column": 68,
                "end_line": 12,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 64,
                "start_line": 12
              }
            },
            "children": [
              {
                "raw_node": {
                  "base": {
                    "field": {
                      "base": {
                        "path": {
                          "segments": [
                            {
                              "ident": "ctx",
                              "position": {
                                "end_column": 14,
                                "end_line": 13,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                "start_column": 11,
                                "start_line": 13
                              }
                            }
                          ]
                        }
                      },
                      "ident": "accounts",
                      "position": {
                        "end_column": 23,
                        "end_line": 13,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 15,
                        "start_line": 13
                      }
                    }
                  },
                  "ident": "token",
                  "position": {
                    "end_column": 9,
                    "end_line": 23,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 23
                  }
                },
                "access_path": "[4].mod.content[1].fn.stmts[0].let.init.expr.try.expr.call.args[0].reference.expr.method_call.receiver.field.base.field",
                "metadata": {
                  "position": {
                    "end_column": 9,
                    "end_line": 23,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 23
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "base": {
                        "path": {
                          "segments": [
                            {
                              "ident": "ctx",
                              "position": {
                                "end_column": 14,
                                "end_line": 13,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                "start_column": 11,
                                "start_line": 13
                              }
                            }
                          ]
                        }
                      },
                      "ident": "accounts",
                      "position": {
                        "end_column": 23,
                        "end_line": 13,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 15,
                        "start_line": 13
                      }
                    },
                    "access_path": "[4].mod.content[1].fn.stmts[0].let.init.expr.try.expr.call.args[0].reference.expr.method_call.receiver.field.base.field.base.field",
                    "metadata": {
                      "position": {
                        "end_column": 23,
                        "end_line": 13,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 15,
                        "start_line": 13
                      }
                    },
                    "children": [
                      {
                        "raw_node": {
                          "ident": "ctx",
                          "position": {
                            "end_column": 14,
                            "end_line": 13,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 13
                          }
                        },
                        "access_path": "[4].mod.content[1].fn.stmts[0].let.init.expr.try.expr.call.args[0].reference.expr.method_call.receiver.field.base.field.base.field.base.path.segments[0]",
                        "metadata": {
                          "position": {
                            "end_column": 14,
                            "end_line": 13,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 13
                          }
                        },
                        "children": [],
                        "parent": "{...}",
                        "root": "False",
                        "args": [],
                        "ident": "ctx"
                      }
                    ],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "accounts"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "token"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "data"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "borrow"
      },
      "{...}",
      {
        "raw_node": {
          "ident": "unpack",
          "position": {
            "end_column": 43,
            "end_line": 12,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 37,
            "start_line": 12
          }
        },
        "access_path": "[4].mod.content[1].fn.stmts[0].let.init.expr.try.expr.call.func.path.segments[1]",
        "metadata": {
          "position": {
            "end_column": 43,
            "end_line": 12,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 37,
            "start_line": 12
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "unpack"
      },
      {
        "raw_node": {
          "ident": {
            "ident": "token",
            "position": {
              "end_column": 9,
              "end_line": 23,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
              "start_column": 4,
              "start_line": 23
            }
          }
        },
        "access_path": "[4].mod.content[1].fn.stmts[0].let.pat",
        "metadata": {},
        "children": [
          {
            "raw_node": {
              "ident": "token",
              "position": {
                "end_column": 9,
                "end_line": 23,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 23
              }
            },
            "access_path": "[4].mod.content[1].fn.stmts[0].let.pat.ident",
            "metadata": {
              "position": {
                "end_column": 9,
                "end_line": 23,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 23
              }
            },
            "children": [],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "token"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "token"
      },
      {
        "raw_node": {
          "base": {
            "field": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 14,
                            "end_line": 13,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 13
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 23,
                    "end_line": 13,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 15,
                    "start_line": 13
                  }
                }
              },
              "ident": "authority",
              "position": {
                "end_column": 13,
                "end_line": 24,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 24
              }
            }
          },
          "ident": "key",
          "position": {
            "end_column": 37,
            "end_line": 13,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 34,
            "start_line": 13
          }
        },
        "access_path": "[4].mod.content[1].fn.stmts[1].expr[0].if.cond.binary.left.field",
        "metadata": {
          "position": {
            "end_column": 37,
            "end_line": 13,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 34,
            "start_line": 13
          }
        },
        "children": [
          {
            "raw_node": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 14,
                            "end_line": 13,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 13
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 23,
                    "end_line": 13,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 15,
                    "start_line": 13
                  }
                }
              },
              "ident": "authority",
              "position": {
                "end_column": 13,
                "end_line": 24,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 24
              }
            },
            "access_path": "[4].mod.content[1].fn.stmts[1].expr[0].if.cond.binary.left.field.base.field",
            "metadata": {
              "position": {
                "end_column": 13,
                "end_line": 24,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 24
              }
            },
            "children": [
              {
                "raw_node": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 14,
                            "end_line": 13,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 13
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 23,
                    "end_line": 13,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 15,
                    "start_line": 13
                  }
                },
                "access_path": "[4].mod.content[1].fn.stmts[1].expr[0].if.cond.binary.left.field.base.field.base.field",
                "metadata": {
                  "position": {
                    "end_column": 23,
                    "end_line": 13,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 15,
                    "start_line": 13
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "ident": "ctx",
                      "position": {
                        "end_column": 14,
                        "end_line": 13,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 11,
                        "start_line": 13
                      }
                    },
                    "access_path": "[4].mod.content[1].fn.stmts[1].expr[0].if.cond.binary.left.field.base.field.base.field.base.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 14,
                        "end_line": 13,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 11,
                        "start_line": 13
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "ctx"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "accounts"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "authority"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "key"
      },
      {
        "raw_node": {
          "base": {
            "path": {
              "segments": [
                {
                  "ident": "token",
                  "position": {
                    "end_column": 9,
                    "end_line": 23,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 23
                  }
                }
              ]
            }
          },
          "ident": "owner",
          "position": {
            "end_column": 53,
            "end_line": 13,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 48,
            "start_line": 13
          }
        },
        "access_path": "[4].mod.content[1].fn.stmts[1].expr[0].if.cond.binary.right.reference.expr.field",
        "metadata": {
          "position": {
            "end_column": 53,
            "end_line": 13,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 48,
            "start_line": 13
          }
        },
        "children": [
          {
            "raw_node": {
              "ident": "token",
              "position": {
                "end_column": 9,
                "end_line": 23,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 23
              }
            },
            "access_path": "[4].mod.content[1].fn.stmts[1].expr[0].if.cond.binary.right.reference.expr.field.base.path.segments[0]",
            "metadata": {
              "position": {
                "end_column": 9,
                "end_line": 23,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 23
              }
            },
            "children": [],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "token"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "owner"
      },
      {
        "raw_node": {
          "ident": "ProgramError",
          "position": {
            "end_column": 35,
            "end_line": 14,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 23,
            "start_line": 14
          }
        },
        "access_path": "[4].mod.content[1].fn.stmts[1].expr[0].if.then_branch[0].expr[0].return.expr.call.args[0].path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 35,
            "end_line": 14,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 23,
            "start_line": 14
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "ProgramError"
      },
      {
        "raw_node": {
          "ident": "InvalidAccountData",
          "position": {
            "end_column": 55,
            "end_line": 14,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 37,
            "start_line": 14
          }
        },
        "access_path": "[4].mod.content[1].fn.stmts[1].expr[0].if.then_branch[0].expr[0].return.expr.call.args[0].path.segments[1]",
        "metadata": {
          "position": {
            "end_column": 55,
            "end_line": 14,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 37,
            "start_line": 14
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "InvalidAccountData"
      },
      {
        "raw_node": {
          "ident": "Err",
          "position": {
            "end_column": 22,
            "end_line": 14,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 19,
            "start_line": 14
          }
        },
        "access_path": "[4].mod.content[1].fn.stmts[1].expr[0].if.then_branch[0].expr[0].return.expr.call.func.path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 22,
            "end_line": 14,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 19,
            "start_line": 14
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "Err"
      },
      {
        "raw_node": {
          "ident": "msg",
          "position": {
            "end_column": 11,
            "end_line": 16,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 8,
            "start_line": 16
          }
        },
        "access_path": "[4].mod.content[1].fn.stmts[2].macro.path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 11,
            "end_line": 16,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 8,
            "start_line": 16
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "msg"
      },
      {
        "raw_node": {
          "ident": "token",
          "position": {
            "end_column": 9,
            "end_line": 23,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 4,
            "start_line": 23
          }
        },
        "access_path": "[4].mod.content[1].fn.stmts[2].macro.tokens[2]",
        "metadata": {
          "position": {
            "end_column": 9,
            "end_line": 23,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 4,
            "start_line": 23
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "token"
      },
      {
        "raw_node": {
          "ident": "amount"
        },
        "access_path": "[4].mod.content[1].fn.stmts[2].macro.tokens[4]",
        "metadata": {},
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "amount"
      },
      {
        "raw_node": {
          "ident": "Ok",
          "position": {
            "end_column": 10,
            "end_line": 17,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 8,
            "start_line": 17
          }
        },
        "access_path": "[4].mod.content[1].fn.stmts[3].expr[0].call.func.path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 10,
            "end_line": 17,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
            "start_column": 8,
            "start_line": 17
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "Ok"
      }
    ],
    "parent": {
      "raw_node": {
        "attrs": [
          {
            "meta": {
              "path": {
                "segments": [
                  {
                    "ident": "program",
                    "position": {
                      "end_column": 9,
                      "end_line": 7,
                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                      "start_column": 2,
                      "start_line": 7
                    }
                  }
                ]
              }
            },
            "style": "outer"
          }
        ],
        "content": [
          {
            "use": {
              "tree": {
                "path": {
                  "ident": "super",
                  "position": {
                    "end_column": 13,
                    "end_line": 9,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 8,
                    "start_line": 9
                  },
                  "tree": "*"
                }
              }
            }
          },
          {
            "fn": {
              "ident": "log_message",
              "inputs": [
                {
                  "typed": {
                    "pat": {
                      "ident": {
                        "ident": "ctx",
                        "position": {
                          "end_column": 14,
                          "end_line": 13,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                          "start_column": 11,
                          "start_line": 13
                        }
                      }
                    },
                    "ty": {
                      "path": {
                        "segments": [
                          {
                            "arguments": {
                              "angle_bracketed": {
                                "args": [
                                  {
                                    "type": {
                                      "path": {
                                        "segments": [
                                          {
                                            "ident": "LogMessage",
                                            "position": {
                                              "end_column": 21,
                                              "end_line": 22,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                              "start_column": 11,
                                              "start_line": 22
                                            }
                                          }
                                        ]
                                      }
                                    }
                                  }
                                ]
                              }
                            },
                            "ident": "Context",
                            "position": {
                              "end_column": 35,
                              "end_line": 11,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                              "start_column": 28,
                              "start_line": 11
                            }
                          }
                        ]
                      }
                    }
                  }
                }
              ],
              "output": {
                "path": {
                  "segments": [
                    {
                      "ident": "ProgramResult",
                      "position": {
                        "end_column": 65,
                        "end_line": 11,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 52,
                        "start_line": 11
                      }
                    }
                  ]
                }
              },
              "position": {
                "end_column": 22,
                "end_line": 11,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 11,
                "start_line": 11
              },
              "stmts": [
                {
                  "let": {
                    "init": {
                      "expr": {
                        "try": {
                          "expr": {
                            "call": {
                              "args": [
                                {
                                  "reference": {
                                    "expr": {
                                      "method_call": {
                                        "args": [],
                                        "method": "borrow",
                                        "receiver": {
                                          "field": {
                                            "base": {
                                              "field": {
                                                "base": {
                                                  "field": {
                                                    "base": {
                                                      "path": {
                                                        "segments": [
                                                          {
                                                            "ident": "ctx",
                                                            "position": {
                                                              "end_column": 14,
                                                              "end_line": 13,
                                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                                              "start_column": 11,
                                                              "start_line": 13
                                                            }
                                                          }
                                                        ]
                                                      }
                                                    },
                                                    "ident": "accounts",
                                                    "position": {
                                                      "end_column": 23,
                                                      "end_line": 13,
                                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                                      "start_column": 15,
                                                      "start_line": 13
                                                    }
                                                  }
                                                },
                                                "ident": "token",
                                                "position": {
                                                  "end_column": 9,
                                                  "end_line": 23,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                                  "start_column": 4,
                                                  "start_line": 23
                                                }
                                              }
                                            },
                                            "ident": "data",
                                            "position": {
                                              "end_column": 68,
                                              "end_line": 12,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                              "start_column": 64,
                                              "start_line": 12
                                            }
                                          }
                                        }
                                      }
                                    }
                                  }
                                }
                              ],
                              "func": {
                                "path": {
                                  "segments": [
                                    {
                                      "ident": "SplTokenAccount",
                                      "position": {
                                        "end_column": 35,
                                        "end_line": 12,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                        "start_column": 20,
                                        "start_line": 12
                                      }
                                    },
                                    {
                                      "ident": "unpack",
                                      "position": {
                                        "end_column": 43,
                                        "end_line": 12,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                        "start_column": 37,
                                        "start_line": 12
                                      }
                                    }
                                  ]
                                }
                              }
                            }
                          }
                        }
                      }
                    },
                    "pat": {
                      "ident": {
                        "ident": "token",
                        "position": {
                          "end_column": 9,
                          "end_line": 23,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                          "start_column": 4,
                          "start_line": 23
                        }
                      }
                    }
                  }
                },
                {
                  "expr": [
                    {
                      "if": {
                        "cond": {
                          "binary": {
                            "left": {
                              "field": {
                                "base": {
                                  "field": {
                                    "base": {
                                      "field": {
                                        "base": {
                                          "path": {
                                            "segments": [
                                              {
                                                "ident": "ctx",
                                                "position": {
                                                  "end_column": 14,
                                                  "end_line": 13,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                                  "start_column": 11,
                                                  "start_line": 13
                                                }
                                              }
                                            ]
                                          }
                                        },
                                        "ident": "accounts",
                                        "position": {
                                          "end_column": 23,
                                          "end_line": 13,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                          "start_column": 15,
                                          "start_line": 13
                                        }
                                      }
                                    },
                                    "ident": "authority",
                                    "position": {
                                      "end_column": 13,
                                      "end_line": 24,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                      "start_column": 4,
                                      "start_line": 24
                                    }
                                  }
                                },
                                "ident": "key",
                                "position": {
                                  "end_column": 37,
                                  "end_line": 13,
                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                  "start_column": 34,
                                  "start_line": 13
                                }
                              }
                            },
                            "op": "!=",
                            "right": {
                              "reference": {
                                "expr": {
                                  "field": {
                                    "base": {
                                      "path": {
                                        "segments": [
                                          {
                                            "ident": "token",
                                            "position": {
                                              "end_column": 9,
                                              "end_line": 23,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                              "start_column": 4,
                                              "start_line": 23
                                            }
                                          }
                                        ]
                                      }
                                    },
                                    "ident": "owner",
                                    "position": {
                                      "end_column": 53,
                                      "end_line": 13,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                      "start_column": 48,
                                      "start_line": 13
                                    }
                                  }
                                }
                              }
                            }
                          }
                        },
                        "then_branch": [
                          {
                            "expr": [
                              {
                                "return": {
                                  "expr": {
                                    "call": {
                                      "args": [
                                        {
                                          "path": {
                                            "segments": [
                                              {
                                                "ident": "ProgramError",
                                                "position": {
                                                  "end_column": 35,
                                                  "end_line": 14,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                                  "start_column": 23,
                                                  "start_line": 14
                                                }
                                              },
                                              {
                                                "ident": "InvalidAccountData",
                                                "position": {
                                                  "end_column": 55,
                                                  "end_line": 14,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                                  "start_column": 37,
                                                  "start_line": 14
                                                }
                                              }
                                            ]
                                          }
                                        }
                                      ],
                                      "func": {
                                        "path": {
                                          "segments": [
                                            {
                                              "ident": "Err",
                                              "position": {
                                                "end_column": 22,
                                                "end_line": 14,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                                "start_column": 19,
                                                "start_line": 14
                                              }
                                            }
                                          ]
                                        }
                                      }
                                    }
                                  }
                                }
                              },
                              "True"
                            ]
                          }
                        ]
                      }
                    },
                    "False"
                  ]
                },
                {
                  "macro": {
                    "delimiter": "paren",
                    "path": {
                      "segments": [
                        {
                          "ident": "msg",
                          "position": {
                            "end_column": 11,
                            "end_line": 16,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                            "start_column": 8,
                            "start_line": 16
                          }
                        }
                      ]
                    },
                    "semi_token": "True",
                    "tokens": [
                      {
                        "lit": "\"Your account balance is: {}\""
                      },
                      {
                        "punct": {
                          "op": ",",
                          "spacing": "alone"
                        }
                      },
                      {
                        "ident": "token",
                        "position": {
                          "end_column": 9,
                          "end_line": 23,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                          "start_column": 4,
                          "start_line": 23
                        }
                      },
                      {
                        "punct": {
                          "op": ".",
                          "spacing": "alone"
                        }
                      },
                      {
                        "ident": "amount"
                      }
                    ]
                  }
                },
                {
                  "expr": [
                    {
                      "call": {
                        "args": [
                          {
                            "tuple": {
                              "elems": []
                            }
                          }
                        ],
                        "func": {
                          "path": {
                            "segments": [
                              {
                                "ident": "Ok",
                                "position": {
                                  "end_column": 10,
                                  "end_line": 17,
                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                                  "start_column": 8,
                                  "start_line": 17
                                }
                              }
                            ]
                          }
                        }
                      }
                    },
                    "False"
                  ]
                }
              ],
              "vis": "pub"
            }
          }
        ],
        "ident": "account_data_matching_insecure",
        "position": {
          "end_column": 38,
          "end_line": 8,
          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
          "start_column": 8,
          "start_line": 8
        },
        "vis": "pub"
      },
      "access_path": "[4].mod",
      "metadata": {
        "position": {
          "end_column": 38,
          "end_line": 8,
          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
          "start_column": 8,
          "start_line": 8
        }
      },
      "children": [
        {
          "raw_node": {
            "ident": "program",
            "position": {
              "end_column": 9,
              "end_line": 7,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
              "start_column": 2,
              "start_line": 7
            }
          },
          "access_path": "[4].mod.attrs[0].meta.path.segments[0]",
          "metadata": {
            "position": {
              "end_column": 9,
              "end_line": 7,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
              "start_column": 2,
              "start_line": 7
            }
          },
          "children": [],
          "parent": "{...}",
          "root": "False",
          "args": [],
          "ident": "program"
        },
        {
          "raw_node": {
            "ident": "super",
            "position": {
              "end_column": 13,
              "end_line": 9,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
              "start_column": 8,
              "start_line": 9
            },
            "tree": "*"
          },
          "access_path": "[4].mod.content[0].use.tree.path",
          "metadata": {
            "position": {
              "end_column": 13,
              "end_line": 9,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
              "start_column": 8,
              "start_line": 9
            }
          },
          "children": [],
          "parent": "{...}",
          "root": "False",
          "args": [],
          "ident": "super"
        },
        "{...}"
      ],
      "parent": {
        "raw_node": {},
        "access_path": "root",
        "metadata": {},
        "children": [
          {
            "raw_node": {
              "ident": "anchor_lang",
              "position": {
                "end_column": 15,
                "end_line": 2,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 2
              },
              "tree": {
                "path": {
                  "ident": "prelude",
                  "position": {
                    "end_column": 24,
                    "end_line": 1,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 17,
                    "start_line": 1
                  },
                  "tree": "*"
                }
              }
            },
            "access_path": "[0].use.tree.path",
            "metadata": {
              "position": {
                "end_column": 15,
                "end_line": 2,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 2
              }
            },
            "children": [
              {
                "raw_node": {
                  "ident": "prelude",
                  "position": {
                    "end_column": 24,
                    "end_line": 1,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 17,
                    "start_line": 1
                  },
                  "tree": "*"
                },
                "access_path": "[0].use.tree.path.tree.path",
                "metadata": {
                  "position": {
                    "end_column": 24,
                    "end_line": 1,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 17,
                    "start_line": 1
                  }
                },
                "children": [],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "prelude"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "anchor_lang"
          },
          {
            "raw_node": {
              "ident": "anchor_lang",
              "position": {
                "end_column": 15,
                "end_line": 2,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 2
              },
              "tree": {
                "path": {
                  "ident": "solana_program",
                  "position": {
                    "end_column": 31,
                    "end_line": 2,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 17,
                    "start_line": 2
                  },
                  "tree": {
                    "path": {
                      "ident": "program_pack",
                      "position": {
                        "end_column": 45,
                        "end_line": 2,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 33,
                        "start_line": 2
                      },
                      "tree": {
                        "ident": "Pack",
                        "position": {
                          "end_column": 51,
                          "end_line": 2,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                          "start_column": 47,
                          "start_line": 2
                        }
                      }
                    }
                  }
                }
              }
            },
            "access_path": "[1].use.tree.path",
            "metadata": {
              "position": {
                "end_column": 15,
                "end_line": 2,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 2
              }
            },
            "children": [
              {
                "raw_node": {
                  "ident": "solana_program",
                  "position": {
                    "end_column": 31,
                    "end_line": 2,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 17,
                    "start_line": 2
                  },
                  "tree": {
                    "path": {
                      "ident": "program_pack",
                      "position": {
                        "end_column": 45,
                        "end_line": 2,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 33,
                        "start_line": 2
                      },
                      "tree": {
                        "ident": "Pack",
                        "position": {
                          "end_column": 51,
                          "end_line": 2,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                          "start_column": 47,
                          "start_line": 2
                        }
                      }
                    }
                  }
                },
                "access_path": "[1].use.tree.path.tree.path",
                "metadata": {
                  "position": {
                    "end_column": 31,
                    "end_line": 2,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 17,
                    "start_line": 2
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "ident": "program_pack",
                      "position": {
                        "end_column": 45,
                        "end_line": 2,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 33,
                        "start_line": 2
                      },
                      "tree": {
                        "ident": "Pack",
                        "position": {
                          "end_column": 51,
                          "end_line": 2,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                          "start_column": 47,
                          "start_line": 2
                        }
                      }
                    },
                    "access_path": "[1].use.tree.path.tree.path.tree.path",
                    "metadata": {
                      "position": {
                        "end_column": 45,
                        "end_line": 2,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 33,
                        "start_line": 2
                      }
                    },
                    "children": [
                      {
                        "raw_node": {
                          "ident": "Pack",
                          "position": {
                            "end_column": 51,
                            "end_line": 2,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                            "start_column": 47,
                            "start_line": 2
                          }
                        },
                        "access_path": "[1].use.tree.path.tree.path.tree.path.tree",
                        "metadata": {
                          "position": {
                            "end_column": 51,
                            "end_line": 2,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                            "start_column": 47,
                            "start_line": 2
                          }
                        },
                        "children": [],
                        "parent": "{...}",
                        "root": "False",
                        "args": [],
                        "ident": "Pack"
                      }
                    ],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "program_pack"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "solana_program"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "anchor_lang"
          },
          {
            "raw_node": {
              "ident": "spl_token",
              "position": {
                "end_column": 13,
                "end_line": 3,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 3
              },
              "tree": {
                "path": {
                  "ident": "state",
                  "position": {
                    "end_column": 20,
                    "end_line": 3,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 15,
                    "start_line": 3
                  },
                  "tree": {
                    "rename": {
                      "ident": "Account",
                      "position": {
                        "end_column": 29,
                        "end_line": 3,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 22,
                        "start_line": 3
                      },
                      "rename": "SplTokenAccount"
                    }
                  }
                }
              }
            },
            "access_path": "[2].use.tree.path",
            "metadata": {
              "position": {
                "end_column": 13,
                "end_line": 3,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 4,
                "start_line": 3
              }
            },
            "children": [
              {
                "raw_node": {
                  "ident": "state",
                  "position": {
                    "end_column": 20,
                    "end_line": 3,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 15,
                    "start_line": 3
                  },
                  "tree": {
                    "rename": {
                      "ident": "Account",
                      "position": {
                        "end_column": 29,
                        "end_line": 3,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 22,
                        "start_line": 3
                      },
                      "rename": "SplTokenAccount"
                    }
                  }
                },
                "access_path": "[2].use.tree.path.tree.path",
                "metadata": {
                  "position": {
                    "end_column": 20,
                    "end_line": 3,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 15,
                    "start_line": 3
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "ident": "Account",
                      "position": {
                        "end_column": 29,
                        "end_line": 3,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 22,
                        "start_line": 3
                      },
                      "rename": "SplTokenAccount"
                    },
                    "access_path": "[2].use.tree.path.tree.path.tree.rename",
                    "metadata": {
                      "position": {
                        "end_column": 29,
                        "end_line": 3,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 22,
                        "start_line": 3
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "Account"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "state"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "spl_token"
          },
          {
            "raw_node": {
              "ident": "declare_id",
              "position": {
                "end_column": 10,
                "end_line": 5,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 0,
                "start_line": 5
              }
            },
            "access_path": "[3].macro.path.segments[0]",
            "metadata": {
              "position": {
                "end_column": 10,
                "end_line": 5,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 0,
                "start_line": 5
              }
            },
            "children": [],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "declare_id"
          },
          "{...}",
          {
            "raw_node": {
              "attrs": [
                {
                  "meta": {
                    "list": {
                      "delimiter": "paren",
                      "path": {
                        "segments": [
                          {
                            "ident": "derive",
                            "position": {
                              "end_column": 8,
                              "end_line": 21,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                              "start_column": 2,
                              "start_line": 21
                            }
                          }
                        ]
                      },
                      "tokens": [
                        {
                          "ident": "Accounts"
                        }
                      ]
                    }
                  },
                  "style": "outer"
                }
              ],
              "fields": {
                "named": [
                  {
                    "colon_token": "True",
                    "ident": "token",
                    "position": {
                      "end_column": 9,
                      "end_line": 23,
                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                      "start_column": 4,
                      "start_line": 23
                    },
                    "ty": {
                      "path": {
                        "segments": [
                          {
                            "arguments": {
                              "angle_bracketed": {
                                "args": [
                                  {
                                    "lifetime": "info"
                                  }
                                ]
                              }
                            },
                            "ident": "AccountInfo",
                            "position": {
                              "end_column": 22,
                              "end_line": 23,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                              "start_column": 11,
                              "start_line": 23
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "colon_token": "True",
                    "ident": "authority",
                    "position": {
                      "end_column": 13,
                      "end_line": 24,
                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                      "start_column": 4,
                      "start_line": 24
                    },
                    "ty": {
                      "path": {
                        "segments": [
                          {
                            "arguments": {
                              "angle_bracketed": {
                                "args": [
                                  {
                                    "lifetime": "info"
                                  }
                                ]
                              }
                            },
                            "ident": "Signer",
                            "position": {
                              "end_column": 21,
                              "end_line": 24,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                              "start_column": 15,
                              "start_line": 24
                            }
                          }
                        ]
                      }
                    }
                  }
                ]
              },
              "generics": {
                "params": [
                  {
                    "lifetime": {
                      "bounds": [],
                      "lifetime": "info"
                    }
                  }
                ]
              },
              "ident": "LogMessage",
              "position": {
                "end_column": 21,
                "end_line": 22,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 11,
                "start_line": 22
              },
              "vis": "pub"
            },
            "access_path": "[5].struct",
            "metadata": {
              "position": {
                "end_column": 21,
                "end_line": 22,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                "start_column": 11,
                "start_line": 22
              }
            },
            "children": [
              {
                "raw_node": {
                  "ident": "derive",
                  "position": {
                    "end_column": 8,
                    "end_line": 21,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 2,
                    "start_line": 21
                  }
                },
                "access_path": "[5].struct.attrs[0].meta.list.path.segments[0]",
                "metadata": {
                  "position": {
                    "end_column": 8,
                    "end_line": 21,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 2,
                    "start_line": 21
                  }
                },
                "children": [],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "derive"
              },
              {
                "raw_node": {
                  "ident": "Accounts"
                },
                "access_path": "[5].struct.attrs[0].meta.list.tokens[0]",
                "metadata": {},
                "children": [],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "Accounts"
              },
              {
                "raw_node": {
                  "colon_token": "True",
                  "ident": "token",
                  "position": {
                    "end_column": 9,
                    "end_line": 23,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 23
                  },
                  "ty": {
                    "path": {
                      "segments": [
                        {
                          "arguments": {
                            "angle_bracketed": {
                              "args": [
                                {
                                  "lifetime": "info"
                                }
                              ]
                            }
                          },
                          "ident": "AccountInfo",
                          "position": {
                            "end_column": 22,
                            "end_line": 23,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 23
                          }
                        }
                      ]
                    }
                  }
                },
                "access_path": "[5].struct.fields.named[0]",
                "metadata": {
                  "position": {
                    "end_column": 9,
                    "end_line": 23,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 23
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "arguments": {
                        "angle_bracketed": {
                          "args": [
                            {
                              "lifetime": "info"
                            }
                          ]
                        }
                      },
                      "ident": "AccountInfo",
                      "position": {
                        "end_column": 22,
                        "end_line": 23,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 11,
                        "start_line": 23
                      }
                    },
                    "access_path": "[5].struct.fields.named[0].ty.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 22,
                        "end_line": 23,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 11,
                        "start_line": 23
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "AccountInfo"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "token"
              },
              {
                "raw_node": {
                  "colon_token": "True",
                  "ident": "authority",
                  "position": {
                    "end_column": 13,
                    "end_line": 24,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 24
                  },
                  "ty": {
                    "path": {
                      "segments": [
                        {
                          "arguments": {
                            "angle_bracketed": {
                              "args": [
                                {
                                  "lifetime": "info"
                                }
                              ]
                            }
                          },
                          "ident": "Signer",
                          "position": {
                            "end_column": 21,
                            "end_line": 24,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                            "start_column": 15,
                            "start_line": 24
                          }
                        }
                      ]
                    }
                  }
                },
                "access_path": "[5].struct.fields.named[1]",
                "metadata": {
                  "position": {
                    "end_column": 13,
                    "end_line": 24,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 24
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "arguments": {
                        "angle_bracketed": {
                          "args": [
                            {
                              "lifetime": "info"
                            }
                          ]
                        }
                      },
                      "ident": "Signer",
                      "position": {
                        "end_column": 21,
                        "end_line": 24,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 15,
                        "start_line": 24
                      }
                    },
                    "access_path": "[5].struct.fields.named[1].ty.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 21,
                        "end_line": 24,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/1-account-data-matching/insecure/src/lib.rs",
                        "start_column": 15,
                        "start_line": 24
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "Signer"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "authority"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "LogMessage"
          }
        ],
        "parent": {
          "raw_node": {},
          "access_path": "EMPTY_ACCESS_PATH",
          "metadata": {},
          "children": [],
          "parent": {},
          "root": "False",
          "args": []
        },
        "root": "False",
        "args": [],
        "ident": "EMPTY_IDENT"
      },
      "root": "False",
      "args": [],
      "ident": "account_data_matching_insecure"
    },
    "root": "False",
    "args": [],
    "ident": "log_message"
  },
  "root": "False",
  "args": [],
  "ident": "SplTokenAccount"
}

AST2 = {
  "raw_node": {
    "ident": "solana_program",
    "position": {
      "end_column": 22,
      "end_line": 14,
      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
      "start_column": 8,
      "start_line": 14
    }
  },
  "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.func.path.segments[0]",
  "metadata": {
    "position": {
      "end_column": 22,
      "end_line": 14,
      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
      "start_column": 8,
      "start_line": 14
    }
  },
  "children": [],
  "parent": {
    "raw_node": {
      "ident": "cpi_secure",
      "inputs": [
        {
          "typed": {
            "pat": {
              "ident": {
                "ident": "ctx",
                "position": {
                  "end_column": 19,
                  "end_line": 26,
                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                  "start_column": 16,
                  "start_line": 26
                }
              }
            },
            "ty": {
              "path": {
                "segments": [
                  {
                    "arguments": {
                      "angle_bracketed": {
                        "args": [
                          {
                            "type": {
                              "path": {
                                "segments": [
                                  {
                                    "ident": "Cpi",
                                    "position": {
                                      "end_column": 14,
                                      "end_line": 51,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                      "start_column": 11,
                                      "start_line": 51
                                    }
                                  }
                                ]
                              }
                            }
                          }
                        ]
                      }
                    },
                    "ident": "Context",
                    "position": {
                      "end_column": 34,
                      "end_line": 10,
                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                      "start_column": 27,
                      "start_line": 10
                    }
                  }
                ]
              }
            }
          }
        },
        {
          "typed": {
            "pat": {
              "ident": {
                "ident": "amount",
                "position": {
                  "end_column": 22,
                  "end_line": 21,
                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                  "start_column": 16,
                  "start_line": 21
                }
              }
            },
            "ty": {
              "path": {
                "segments": [
                  {
                    "ident": "u64",
                    "position": {
                      "end_column": 52,
                      "end_line": 10,
                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                      "start_column": 49,
                      "start_line": 10
                    }
                  }
                ]
              }
            }
          }
        }
      ],
      "output": {
        "path": {
          "segments": [
            {
              "ident": "ProgramResult",
              "position": {
                "end_column": 70,
                "end_line": 10,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 57,
                "start_line": 10
              }
            }
          ]
        }
      },
      "position": {
        "end_column": 21,
        "end_line": 10,
        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
        "start_column": 11,
        "start_line": 10
      },
      "stmts": [
        {
          "expr": [
            {
              "if": {
                "cond": {
                  "binary": {
                    "left": {
                      "reference": {
                        "expr": {
                          "path": {
                            "segments": [
                              {
                                "ident": "spl_token",
                                "position": {
                                  "end_column": 22,
                                  "end_line": 15,
                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                  "start_column": 13,
                                  "start_line": 15
                                }
                              },
                              {
                                "ident": "ID",
                                "position": {
                                  "end_column": 25,
                                  "end_line": 11,
                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                  "start_column": 23,
                                  "start_line": 11
                                }
                              }
                            ]
                          }
                        }
                      }
                    },
                    "op": "!=",
                    "right": {
                      "field": {
                        "base": {
                          "field": {
                            "base": {
                              "field": {
                                "base": {
                                  "path": {
                                    "segments": [
                                      {
                                        "ident": "ctx",
                                        "position": {
                                          "end_column": 19,
                                          "end_line": 26,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 16,
                                          "start_line": 26
                                        }
                                      }
                                    ]
                                  }
                                },
                                "ident": "accounts",
                                "position": {
                                  "end_column": 28,
                                  "end_line": 26,
                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                  "start_column": 20,
                                  "start_line": 26
                                }
                              }
                            },
                            "ident": "token_program",
                            "position": {
                              "end_column": 17,
                              "end_line": 55,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                              "start_column": 4,
                              "start_line": 55
                            }
                          }
                        },
                        "ident": "key",
                        "position": {
                          "end_column": 42,
                          "end_line": 19,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                          "start_column": 39,
                          "start_line": 19
                        }
                      }
                    }
                  }
                },
                "then_branch": [
                  {
                    "expr": [
                      {
                        "return": {
                          "expr": {
                            "call": {
                              "args": [
                                {
                                  "path": {
                                    "segments": [
                                      {
                                        "ident": "ProgramError",
                                        "position": {
                                          "end_column": 35,
                                          "end_line": 12,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 23,
                                          "start_line": 12
                                        }
                                      },
                                      {
                                        "ident": "IncorrectProgramId",
                                        "position": {
                                          "end_column": 55,
                                          "end_line": 12,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 37,
                                          "start_line": 12
                                        }
                                      }
                                    ]
                                  }
                                }
                              ],
                              "func": {
                                "path": {
                                  "segments": [
                                    {
                                      "ident": "Err",
                                      "position": {
                                        "end_column": 22,
                                        "end_line": 12,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                        "start_column": 19,
                                        "start_line": 12
                                      }
                                    }
                                  ]
                                }
                              }
                            }
                          }
                        }
                      },
                      "True"
                    ]
                  }
                ]
              }
            },
            "False"
          ]
        },
        {
          "expr": [
            {
              "call": {
                "args": [
                  {
                    "reference": {
                      "expr": {
                        "try": {
                          "expr": {
                            "call": {
                              "args": [
                                {
                                  "field": {
                                    "base": {
                                      "field": {
                                        "base": {
                                          "field": {
                                            "base": {
                                              "path": {
                                                "segments": [
                                                  {
                                                    "ident": "ctx",
                                                    "position": {
                                                      "end_column": 19,
                                                      "end_line": 26,
                                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                      "start_column": 16,
                                                      "start_line": 26
                                                    }
                                                  }
                                                ]
                                              }
                                            },
                                            "ident": "accounts",
                                            "position": {
                                              "end_column": 28,
                                              "end_line": 26,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 20,
                                              "start_line": 26
                                            }
                                          }
                                        },
                                        "ident": "token_program",
                                        "position": {
                                          "end_column": 17,
                                          "end_line": 55,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 4,
                                          "start_line": 55
                                        }
                                      }
                                    },
                                    "ident": "key",
                                    "position": {
                                      "end_column": 42,
                                      "end_line": 19,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                      "start_column": 39,
                                      "start_line": 19
                                    }
                                  }
                                },
                                {
                                  "field": {
                                    "base": {
                                      "field": {
                                        "base": {
                                          "field": {
                                            "base": {
                                              "path": {
                                                "segments": [
                                                  {
                                                    "ident": "ctx",
                                                    "position": {
                                                      "end_column": 19,
                                                      "end_line": 26,
                                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                      "start_column": 16,
                                                      "start_line": 26
                                                    }
                                                  }
                                                ]
                                              }
                                            },
                                            "ident": "accounts",
                                            "position": {
                                              "end_column": 28,
                                              "end_line": 26,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 20,
                                              "start_line": 26
                                            }
                                          }
                                        },
                                        "ident": "source",
                                        "position": {
                                          "end_column": 10,
                                          "end_line": 52,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 4,
                                          "start_line": 52
                                        }
                                      }
                                    },
                                    "ident": "key",
                                    "position": {
                                      "end_column": 42,
                                      "end_line": 19,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                      "start_column": 39,
                                      "start_line": 19
                                    }
                                  }
                                },
                                {
                                  "field": {
                                    "base": {
                                      "field": {
                                        "base": {
                                          "field": {
                                            "base": {
                                              "path": {
                                                "segments": [
                                                  {
                                                    "ident": "ctx",
                                                    "position": {
                                                      "end_column": 19,
                                                      "end_line": 26,
                                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                      "start_column": 16,
                                                      "start_line": 26
                                                    }
                                                  }
                                                ]
                                              }
                                            },
                                            "ident": "accounts",
                                            "position": {
                                              "end_column": 28,
                                              "end_line": 26,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 20,
                                              "start_line": 26
                                            }
                                          }
                                        },
                                        "ident": "destination",
                                        "position": {
                                          "end_column": 15,
                                          "end_line": 53,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 4,
                                          "start_line": 53
                                        }
                                      }
                                    },
                                    "ident": "key",
                                    "position": {
                                      "end_column": 42,
                                      "end_line": 19,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                      "start_column": 39,
                                      "start_line": 19
                                    }
                                  }
                                },
                                {
                                  "field": {
                                    "base": {
                                      "field": {
                                        "base": {
                                          "field": {
                                            "base": {
                                              "path": {
                                                "segments": [
                                                  {
                                                    "ident": "ctx",
                                                    "position": {
                                                      "end_column": 19,
                                                      "end_line": 26,
                                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                      "start_column": 16,
                                                      "start_line": 26
                                                    }
                                                  }
                                                ]
                                              }
                                            },
                                            "ident": "accounts",
                                            "position": {
                                              "end_column": 28,
                                              "end_line": 26,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 20,
                                              "start_line": 26
                                            }
                                          }
                                        },
                                        "ident": "authority",
                                        "position": {
                                          "end_column": 13,
                                          "end_line": 54,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 4,
                                          "start_line": 54
                                        }
                                      }
                                    },
                                    "ident": "key",
                                    "position": {
                                      "end_column": 42,
                                      "end_line": 19,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                      "start_column": 39,
                                      "start_line": 19
                                    }
                                  }
                                },
                                {
                                  "reference": {
                                    "expr": {
                                      "array": {
                                        "elems": []
                                      }
                                    }
                                  }
                                },
                                {
                                  "path": {
                                    "segments": [
                                      {
                                        "ident": "amount",
                                        "position": {
                                          "end_column": 22,
                                          "end_line": 21,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 16,
                                          "start_line": 21
                                        }
                                      }
                                    ]
                                  }
                                }
                              ],
                              "func": {
                                "path": {
                                  "segments": [
                                    {
                                      "ident": "spl_token",
                                      "position": {
                                        "end_column": 22,
                                        "end_line": 15,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                        "start_column": 13,
                                        "start_line": 15
                                      }
                                    },
                                    {
                                      "ident": "instruction",
                                      "position": {
                                        "end_column": 35,
                                        "end_line": 15,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                        "start_column": 24,
                                        "start_line": 15
                                      }
                                    },
                                    {
                                      "ident": "transfer",
                                      "position": {
                                        "end_column": 45,
                                        "end_line": 15,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                        "start_column": 37,
                                        "start_line": 15
                                      }
                                    }
                                  ]
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  },
                  {
                    "reference": {
                      "expr": {
                        "array": {
                          "elems": [
                            {
                              "method_call": {
                                "args": [],
                                "method": "clone",
                                "receiver": {
                                  "field": {
                                    "base": {
                                      "field": {
                                        "base": {
                                          "path": {
                                            "segments": [
                                              {
                                                "ident": "ctx",
                                                "position": {
                                                  "end_column": 19,
                                                  "end_line": 26,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 16,
                                                  "start_line": 26
                                                }
                                              }
                                            ]
                                          }
                                        },
                                        "ident": "accounts",
                                        "position": {
                                          "end_column": 28,
                                          "end_line": 26,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 20,
                                          "start_line": 26
                                        }
                                      }
                                    },
                                    "ident": "source",
                                    "position": {
                                      "end_column": 10,
                                      "end_line": 52,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                      "start_column": 4,
                                      "start_line": 52
                                    }
                                  }
                                }
                              }
                            },
                            {
                              "method_call": {
                                "args": [],
                                "method": "clone",
                                "receiver": {
                                  "field": {
                                    "base": {
                                      "field": {
                                        "base": {
                                          "path": {
                                            "segments": [
                                              {
                                                "ident": "ctx",
                                                "position": {
                                                  "end_column": 19,
                                                  "end_line": 26,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 16,
                                                  "start_line": 26
                                                }
                                              }
                                            ]
                                          }
                                        },
                                        "ident": "accounts",
                                        "position": {
                                          "end_column": 28,
                                          "end_line": 26,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 20,
                                          "start_line": 26
                                        }
                                      }
                                    },
                                    "ident": "destination",
                                    "position": {
                                      "end_column": 15,
                                      "end_line": 53,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                      "start_column": 4,
                                      "start_line": 53
                                    }
                                  }
                                }
                              }
                            },
                            {
                              "method_call": {
                                "args": [],
                                "method": "clone",
                                "receiver": {
                                  "field": {
                                    "base": {
                                      "field": {
                                        "base": {
                                          "path": {
                                            "segments": [
                                              {
                                                "ident": "ctx",
                                                "position": {
                                                  "end_column": 19,
                                                  "end_line": 26,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 16,
                                                  "start_line": 26
                                                }
                                              }
                                            ]
                                          }
                                        },
                                        "ident": "accounts",
                                        "position": {
                                          "end_column": 28,
                                          "end_line": 26,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 20,
                                          "start_line": 26
                                        }
                                      }
                                    },
                                    "ident": "authority",
                                    "position": {
                                      "end_column": 13,
                                      "end_line": 54,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                      "start_column": 4,
                                      "start_line": 54
                                    }
                                  }
                                }
                              }
                            }
                          ]
                        }
                      }
                    }
                  }
                ],
                "func": {
                  "path": {
                    "segments": [
                      {
                        "ident": "solana_program",
                        "position": {
                          "end_column": 22,
                          "end_line": 14,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                          "start_column": 8,
                          "start_line": 14
                        }
                      },
                      {
                        "ident": "program",
                        "position": {
                          "end_column": 31,
                          "end_line": 14,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                          "start_column": 24,
                          "start_line": 14
                        }
                      },
                      {
                        "ident": "invoke",
                        "position": {
                          "end_column": 39,
                          "end_line": 14,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                          "start_column": 33,
                          "start_line": 14
                        }
                      }
                    ]
                  }
                }
              }
            },
            "False"
          ]
        }
      ],
      "vis": "pub"
    },
    "access_path": "[3].mod.content[1].fn",
    "metadata": {
      "position": {
        "end_column": 21,
        "end_line": 10,
        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
        "start_column": 11,
        "start_line": 10
      }
    },
    "children": [
      {
        "raw_node": {
          "ident": {
            "ident": "ctx",
            "position": {
              "end_column": 19,
              "end_line": 26,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
              "start_column": 16,
              "start_line": 26
            }
          }
        },
        "access_path": "[3].mod.content[1].fn.inputs[0].typed.pat",
        "metadata": {},
        "children": [
          {
            "raw_node": {
              "ident": "ctx",
              "position": {
                "end_column": 19,
                "end_line": 26,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 16,
                "start_line": 26
              }
            },
            "access_path": "[3].mod.content[1].fn.inputs[0].typed.pat.ident",
            "metadata": {
              "position": {
                "end_column": 19,
                "end_line": 26,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 16,
                "start_line": 26
              }
            },
            "children": [],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "ctx"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "ctx"
      },
      {
        "raw_node": {
          "arguments": {
            "angle_bracketed": {
              "args": [
                {
                  "type": {
                    "path": {
                      "segments": [
                        {
                          "ident": "Cpi",
                          "position": {
                            "end_column": 14,
                            "end_line": 51,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 51
                          }
                        }
                      ]
                    }
                  }
                }
              ]
            }
          },
          "ident": "Context",
          "position": {
            "end_column": 34,
            "end_line": 10,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 27,
            "start_line": 10
          }
        },
        "access_path": "[3].mod.content[1].fn.inputs[0].typed.ty.path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 34,
            "end_line": 10,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 27,
            "start_line": 10
          }
        },
        "children": [
          {
            "raw_node": {
              "ident": "Cpi",
              "position": {
                "end_column": 14,
                "end_line": 51,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 11,
                "start_line": 51
              }
            },
            "access_path": "[3].mod.content[1].fn.inputs[0].typed.ty.path.segments[0].arguments.angle_bracketed.args[0].type.path.segments[0]",
            "metadata": {
              "position": {
                "end_column": 14,
                "end_line": 51,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 11,
                "start_line": 51
              }
            },
            "children": [],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "Cpi"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "Context"
      },
      {
        "raw_node": {
          "ident": {
            "ident": "amount",
            "position": {
              "end_column": 22,
              "end_line": 21,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
              "start_column": 16,
              "start_line": 21
            }
          }
        },
        "access_path": "[3].mod.content[1].fn.inputs[1].typed.pat",
        "metadata": {},
        "children": [
          {
            "raw_node": {
              "ident": "amount",
              "position": {
                "end_column": 22,
                "end_line": 21,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 16,
                "start_line": 21
              }
            },
            "access_path": "[3].mod.content[1].fn.inputs[1].typed.pat.ident",
            "metadata": {
              "position": {
                "end_column": 22,
                "end_line": 21,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 16,
                "start_line": 21
              }
            },
            "children": [],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "amount"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "amount"
      },
      {
        "raw_node": {
          "ident": "u64",
          "position": {
            "end_column": 52,
            "end_line": 10,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 49,
            "start_line": 10
          }
        },
        "access_path": "[3].mod.content[1].fn.inputs[1].typed.ty.path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 52,
            "end_line": 10,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 49,
            "start_line": 10
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "u64"
      },
      {
        "raw_node": {
          "ident": "ProgramResult",
          "position": {
            "end_column": 70,
            "end_line": 10,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 57,
            "start_line": 10
          }
        },
        "access_path": "[3].mod.content[1].fn.output.path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 70,
            "end_line": 10,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 57,
            "start_line": 10
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "ProgramResult"
      },
      {
        "raw_node": {
          "ident": "spl_token",
          "position": {
            "end_column": 22,
            "end_line": 15,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 13,
            "start_line": 15
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.left.reference.expr.path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 22,
            "end_line": 15,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 13,
            "start_line": 15
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "spl_token"
      },
      {
        "raw_node": {
          "ident": "ID",
          "position": {
            "end_column": 25,
            "end_line": 11,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 23,
            "start_line": 11
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.left.reference.expr.path.segments[1]",
        "metadata": {
          "position": {
            "end_column": 25,
            "end_line": 11,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 23,
            "start_line": 11
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "ID"
      },
      {
        "raw_node": {
          "base": {
            "field": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "token_program",
              "position": {
                "end_column": 17,
                "end_line": 55,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 55
              }
            }
          },
          "ident": "key",
          "position": {
            "end_column": 42,
            "end_line": 19,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 39,
            "start_line": 19
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.right.field",
        "metadata": {
          "position": {
            "end_column": 42,
            "end_line": 19,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 39,
            "start_line": 19
          }
        },
        "children": [
          {
            "raw_node": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "token_program",
              "position": {
                "end_column": 17,
                "end_line": 55,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 55
              }
            },
            "access_path": "[3].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.right.field.base.field",
            "metadata": {
              "position": {
                "end_column": 17,
                "end_line": 55,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 55
              }
            },
            "children": [
              {
                "raw_node": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "access_path": "[3].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.right.field.base.field.base.field",
                "metadata": {
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "ident": "ctx",
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "access_path": "[3].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.right.field.base.field.base.field.base.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "ctx"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "accounts"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "token_program"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "key"
      },
      {
        "raw_node": {
          "ident": "ProgramError",
          "position": {
            "end_column": 35,
            "end_line": 12,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 23,
            "start_line": 12
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[0].expr[0].if.then_branch[0].expr[0].return.expr.call.args[0].path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 35,
            "end_line": 12,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 23,
            "start_line": 12
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "ProgramError"
      },
      {
        "raw_node": {
          "ident": "IncorrectProgramId",
          "position": {
            "end_column": 55,
            "end_line": 12,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 37,
            "start_line": 12
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[0].expr[0].if.then_branch[0].expr[0].return.expr.call.args[0].path.segments[1]",
        "metadata": {
          "position": {
            "end_column": 55,
            "end_line": 12,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 37,
            "start_line": 12
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "IncorrectProgramId"
      },
      {
        "raw_node": {
          "ident": "Err",
          "position": {
            "end_column": 22,
            "end_line": 12,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 19,
            "start_line": 12
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[0].expr[0].if.then_branch[0].expr[0].return.expr.call.func.path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 22,
            "end_line": 12,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 19,
            "start_line": 12
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "Err"
      },
      {
        "raw_node": {
          "base": {
            "field": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "token_program",
              "position": {
                "end_column": 17,
                "end_line": 55,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 55
              }
            }
          },
          "ident": "key",
          "position": {
            "end_column": 42,
            "end_line": 19,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 39,
            "start_line": 19
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[0].field",
        "metadata": {
          "position": {
            "end_column": 42,
            "end_line": 19,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 39,
            "start_line": 19
          }
        },
        "children": [
          {
            "raw_node": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "token_program",
              "position": {
                "end_column": 17,
                "end_line": 55,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 55
              }
            },
            "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[0].field.base.field",
            "metadata": {
              "position": {
                "end_column": 17,
                "end_line": 55,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 55
              }
            },
            "children": [
              {
                "raw_node": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[0].field.base.field.base.field",
                "metadata": {
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "ident": "ctx",
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[0].field.base.field.base.field.base.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "ctx"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "accounts"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "token_program"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "key"
      },
      {
        "raw_node": {
          "base": {
            "field": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "source",
              "position": {
                "end_column": 10,
                "end_line": 52,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 52
              }
            }
          },
          "ident": "key",
          "position": {
            "end_column": 42,
            "end_line": 19,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 39,
            "start_line": 19
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[1].field",
        "metadata": {
          "position": {
            "end_column": 42,
            "end_line": 19,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 39,
            "start_line": 19
          }
        },
        "children": [
          {
            "raw_node": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "source",
              "position": {
                "end_column": 10,
                "end_line": 52,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 52
              }
            },
            "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[1].field.base.field",
            "metadata": {
              "position": {
                "end_column": 10,
                "end_line": 52,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 52
              }
            },
            "children": [
              {
                "raw_node": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[1].field.base.field.base.field",
                "metadata": {
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "ident": "ctx",
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[1].field.base.field.base.field.base.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "ctx"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "accounts"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "source"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "key"
      },
      {
        "raw_node": {
          "base": {
            "field": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "destination",
              "position": {
                "end_column": 15,
                "end_line": 53,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 53
              }
            }
          },
          "ident": "key",
          "position": {
            "end_column": 42,
            "end_line": 19,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 39,
            "start_line": 19
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[2].field",
        "metadata": {
          "position": {
            "end_column": 42,
            "end_line": 19,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 39,
            "start_line": 19
          }
        },
        "children": [
          {
            "raw_node": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "destination",
              "position": {
                "end_column": 15,
                "end_line": 53,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 53
              }
            },
            "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[2].field.base.field",
            "metadata": {
              "position": {
                "end_column": 15,
                "end_line": 53,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 53
              }
            },
            "children": [
              {
                "raw_node": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[2].field.base.field.base.field",
                "metadata": {
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "ident": "ctx",
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[2].field.base.field.base.field.base.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "ctx"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "accounts"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "destination"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "key"
      },
      {
        "raw_node": {
          "base": {
            "field": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "authority",
              "position": {
                "end_column": 13,
                "end_line": 54,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 54
              }
            }
          },
          "ident": "key",
          "position": {
            "end_column": 42,
            "end_line": 19,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 39,
            "start_line": 19
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[3].field",
        "metadata": {
          "position": {
            "end_column": 42,
            "end_line": 19,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 39,
            "start_line": 19
          }
        },
        "children": [
          {
            "raw_node": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "authority",
              "position": {
                "end_column": 13,
                "end_line": 54,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 54
              }
            },
            "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[3].field.base.field",
            "metadata": {
              "position": {
                "end_column": 13,
                "end_line": 54,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 54
              }
            },
            "children": [
              {
                "raw_node": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[3].field.base.field.base.field",
                "metadata": {
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "ident": "ctx",
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[3].field.base.field.base.field.base.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "ctx"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "accounts"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "authority"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "key"
      },
      {
        "raw_node": {
          "ident": "amount",
          "position": {
            "end_column": 22,
            "end_line": 21,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 16,
            "start_line": 21
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.args[5].path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 22,
            "end_line": 21,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 16,
            "start_line": 21
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "amount"
      },
      {
        "raw_node": {
          "ident": "spl_token",
          "position": {
            "end_column": 22,
            "end_line": 15,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 13,
            "start_line": 15
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.func.path.segments[0]",
        "metadata": {
          "position": {
            "end_column": 22,
            "end_line": 15,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 13,
            "start_line": 15
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "spl_token"
      },
      {
        "raw_node": {
          "ident": "instruction",
          "position": {
            "end_column": 35,
            "end_line": 15,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 24,
            "start_line": 15
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.func.path.segments[1]",
        "metadata": {
          "position": {
            "end_column": 35,
            "end_line": 15,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 24,
            "start_line": 15
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "instruction"
      },
      {
        "raw_node": {
          "ident": "transfer",
          "position": {
            "end_column": 45,
            "end_line": 15,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 37,
            "start_line": 15
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[0].reference.expr.try.expr.call.func.path.segments[2]",
        "metadata": {
          "position": {
            "end_column": 45,
            "end_line": 15,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 37,
            "start_line": 15
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "transfer"
      },
      {
        "raw_node": {
          "args": [],
          "method": "clone",
          "receiver": {
            "field": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "source",
              "position": {
                "end_column": 10,
                "end_line": 52,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 52
              }
            }
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[0].method_call",
        "metadata": {},
        "children": [
          {
            "raw_node": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "source",
              "position": {
                "end_column": 10,
                "end_line": 52,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 52
              }
            },
            "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[0].method_call.receiver.field",
            "metadata": {
              "position": {
                "end_column": 10,
                "end_line": 52,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 52
              }
            },
            "children": [
              {
                "raw_node": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[0].method_call.receiver.field.base.field",
                "metadata": {
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "ident": "ctx",
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[0].method_call.receiver.field.base.field.base.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "ctx"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "accounts"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "source"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "clone"
      },
      {
        "raw_node": {
          "args": [],
          "method": "clone",
          "receiver": {
            "field": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "destination",
              "position": {
                "end_column": 15,
                "end_line": 53,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 53
              }
            }
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[1].method_call",
        "metadata": {},
        "children": [
          {
            "raw_node": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "destination",
              "position": {
                "end_column": 15,
                "end_line": 53,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 53
              }
            },
            "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[1].method_call.receiver.field",
            "metadata": {
              "position": {
                "end_column": 15,
                "end_line": 53,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 53
              }
            },
            "children": [
              {
                "raw_node": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[1].method_call.receiver.field.base.field",
                "metadata": {
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "ident": "ctx",
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[1].method_call.receiver.field.base.field.base.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "ctx"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "accounts"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "destination"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "clone"
      },
      {
        "raw_node": {
          "args": [],
          "method": "clone",
          "receiver": {
            "field": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "authority",
              "position": {
                "end_column": 13,
                "end_line": 54,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 54
              }
            }
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[2].method_call",
        "metadata": {},
        "children": [
          {
            "raw_node": {
              "base": {
                "field": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                }
              },
              "ident": "authority",
              "position": {
                "end_column": 13,
                "end_line": 54,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 54
              }
            },
            "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[2].method_call.receiver.field",
            "metadata": {
              "position": {
                "end_column": 13,
                "end_line": 54,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 54
              }
            },
            "children": [
              {
                "raw_node": {
                  "base": {
                    "path": {
                      "segments": [
                        {
                          "ident": "ctx",
                          "position": {
                            "end_column": 19,
                            "end_line": 26,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 16,
                            "start_line": 26
                          }
                        }
                      ]
                    }
                  },
                  "ident": "accounts",
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[2].method_call.receiver.field.base.field",
                "metadata": {
                  "position": {
                    "end_column": 28,
                    "end_line": 26,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 20,
                    "start_line": 26
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "ident": "ctx",
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.args[1].reference.expr.array.elems[2].method_call.receiver.field.base.field.base.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 19,
                        "end_line": 26,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 16,
                        "start_line": 26
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "ctx"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "accounts"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "authority"
          }
        ],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "clone"
      },
      "{...}",
      {
        "raw_node": {
          "ident": "program",
          "position": {
            "end_column": 31,
            "end_line": 14,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 24,
            "start_line": 14
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.func.path.segments[1]",
        "metadata": {
          "position": {
            "end_column": 31,
            "end_line": 14,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 24,
            "start_line": 14
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "program"
      },
      {
        "raw_node": {
          "ident": "invoke",
          "position": {
            "end_column": 39,
            "end_line": 14,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 33,
            "start_line": 14
          }
        },
        "access_path": "[3].mod.content[1].fn.stmts[1].expr[0].call.func.path.segments[2]",
        "metadata": {
          "position": {
            "end_column": 39,
            "end_line": 14,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
            "start_column": 33,
            "start_line": 14
          }
        },
        "children": [],
        "parent": "{...}",
        "root": "False",
        "args": [],
        "ident": "invoke"
      }
    ],
    "parent": {
      "raw_node": {
        "attrs": [
          {
            "meta": {
              "path": {
                "segments": [
                  {
                    "ident": "program",
                    "position": {
                      "end_column": 31,
                      "end_line": 14,
                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                      "start_column": 24,
                      "start_line": 14
                    }
                  }
                ]
              }
            },
            "style": "outer"
          }
        ],
        "content": [
          {
            "use": {
              "tree": {
                "path": {
                  "ident": "super",
                  "position": {
                    "end_column": 13,
                    "end_line": 8,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 8,
                    "start_line": 8
                  },
                  "tree": "*"
                }
              }
            }
          },
          {
            "fn": {
              "ident": "cpi_secure",
              "inputs": [
                {
                  "typed": {
                    "pat": {
                      "ident": {
                        "ident": "ctx",
                        "position": {
                          "end_column": 19,
                          "end_line": 26,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                          "start_column": 16,
                          "start_line": 26
                        }
                      }
                    },
                    "ty": {
                      "path": {
                        "segments": [
                          {
                            "arguments": {
                              "angle_bracketed": {
                                "args": [
                                  {
                                    "type": {
                                      "path": {
                                        "segments": [
                                          {
                                            "ident": "Cpi",
                                            "position": {
                                              "end_column": 14,
                                              "end_line": 51,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 11,
                                              "start_line": 51
                                            }
                                          }
                                        ]
                                      }
                                    }
                                  }
                                ]
                              }
                            },
                            "ident": "Context",
                            "position": {
                              "end_column": 34,
                              "end_line": 10,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                              "start_column": 27,
                              "start_line": 10
                            }
                          }
                        ]
                      }
                    }
                  }
                },
                {
                  "typed": {
                    "pat": {
                      "ident": {
                        "ident": "amount",
                        "position": {
                          "end_column": 22,
                          "end_line": 21,
                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                          "start_column": 16,
                          "start_line": 21
                        }
                      }
                    },
                    "ty": {
                      "path": {
                        "segments": [
                          {
                            "ident": "u64",
                            "position": {
                              "end_column": 52,
                              "end_line": 10,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                              "start_column": 49,
                              "start_line": 10
                            }
                          }
                        ]
                      }
                    }
                  }
                }
              ],
              "output": {
                "path": {
                  "segments": [
                    {
                      "ident": "ProgramResult",
                      "position": {
                        "end_column": 70,
                        "end_line": 10,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 57,
                        "start_line": 10
                      }
                    }
                  ]
                }
              },
              "position": {
                "end_column": 21,
                "end_line": 10,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 11,
                "start_line": 10
              },
              "stmts": [
                {
                  "expr": [
                    {
                      "if": {
                        "cond": {
                          "binary": {
                            "left": {
                              "reference": {
                                "expr": {
                                  "path": {
                                    "segments": [
                                      {
                                        "ident": "spl_token",
                                        "position": {
                                          "end_column": 22,
                                          "end_line": 15,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 13,
                                          "start_line": 15
                                        }
                                      },
                                      {
                                        "ident": "ID",
                                        "position": {
                                          "end_column": 25,
                                          "end_line": 11,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 23,
                                          "start_line": 11
                                        }
                                      }
                                    ]
                                  }
                                }
                              }
                            },
                            "op": "!=",
                            "right": {
                              "field": {
                                "base": {
                                  "field": {
                                    "base": {
                                      "field": {
                                        "base": {
                                          "path": {
                                            "segments": [
                                              {
                                                "ident": "ctx",
                                                "position": {
                                                  "end_column": 19,
                                                  "end_line": 26,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 16,
                                                  "start_line": 26
                                                }
                                              }
                                            ]
                                          }
                                        },
                                        "ident": "accounts",
                                        "position": {
                                          "end_column": 28,
                                          "end_line": 26,
                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                          "start_column": 20,
                                          "start_line": 26
                                        }
                                      }
                                    },
                                    "ident": "token_program",
                                    "position": {
                                      "end_column": 17,
                                      "end_line": 55,
                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                      "start_column": 4,
                                      "start_line": 55
                                    }
                                  }
                                },
                                "ident": "key",
                                "position": {
                                  "end_column": 42,
                                  "end_line": 19,
                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                  "start_column": 39,
                                  "start_line": 19
                                }
                              }
                            }
                          }
                        },
                        "then_branch": [
                          {
                            "expr": [
                              {
                                "return": {
                                  "expr": {
                                    "call": {
                                      "args": [
                                        {
                                          "path": {
                                            "segments": [
                                              {
                                                "ident": "ProgramError",
                                                "position": {
                                                  "end_column": 35,
                                                  "end_line": 12,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 23,
                                                  "start_line": 12
                                                }
                                              },
                                              {
                                                "ident": "IncorrectProgramId",
                                                "position": {
                                                  "end_column": 55,
                                                  "end_line": 12,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 37,
                                                  "start_line": 12
                                                }
                                              }
                                            ]
                                          }
                                        }
                                      ],
                                      "func": {
                                        "path": {
                                          "segments": [
                                            {
                                              "ident": "Err",
                                              "position": {
                                                "end_column": 22,
                                                "end_line": 12,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                "start_column": 19,
                                                "start_line": 12
                                              }
                                            }
                                          ]
                                        }
                                      }
                                    }
                                  }
                                }
                              },
                              "True"
                            ]
                          }
                        ]
                      }
                    },
                    "False"
                  ]
                },
                {
                  "expr": [
                    {
                      "call": {
                        "args": [
                          {
                            "reference": {
                              "expr": {
                                "try": {
                                  "expr": {
                                    "call": {
                                      "args": [
                                        {
                                          "field": {
                                            "base": {
                                              "field": {
                                                "base": {
                                                  "field": {
                                                    "base": {
                                                      "path": {
                                                        "segments": [
                                                          {
                                                            "ident": "ctx",
                                                            "position": {
                                                              "end_column": 19,
                                                              "end_line": 26,
                                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                              "start_column": 16,
                                                              "start_line": 26
                                                            }
                                                          }
                                                        ]
                                                      }
                                                    },
                                                    "ident": "accounts",
                                                    "position": {
                                                      "end_column": 28,
                                                      "end_line": 26,
                                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                      "start_column": 20,
                                                      "start_line": 26
                                                    }
                                                  }
                                                },
                                                "ident": "token_program",
                                                "position": {
                                                  "end_column": 17,
                                                  "end_line": 55,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 4,
                                                  "start_line": 55
                                                }
                                              }
                                            },
                                            "ident": "key",
                                            "position": {
                                              "end_column": 42,
                                              "end_line": 19,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 39,
                                              "start_line": 19
                                            }
                                          }
                                        },
                                        {
                                          "field": {
                                            "base": {
                                              "field": {
                                                "base": {
                                                  "field": {
                                                    "base": {
                                                      "path": {
                                                        "segments": [
                                                          {
                                                            "ident": "ctx",
                                                            "position": {
                                                              "end_column": 19,
                                                              "end_line": 26,
                                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                              "start_column": 16,
                                                              "start_line": 26
                                                            }
                                                          }
                                                        ]
                                                      }
                                                    },
                                                    "ident": "accounts",
                                                    "position": {
                                                      "end_column": 28,
                                                      "end_line": 26,
                                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                      "start_column": 20,
                                                      "start_line": 26
                                                    }
                                                  }
                                                },
                                                "ident": "source",
                                                "position": {
                                                  "end_column": 10,
                                                  "end_line": 52,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 4,
                                                  "start_line": 52
                                                }
                                              }
                                            },
                                            "ident": "key",
                                            "position": {
                                              "end_column": 42,
                                              "end_line": 19,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 39,
                                              "start_line": 19
                                            }
                                          }
                                        },
                                        {
                                          "field": {
                                            "base": {
                                              "field": {
                                                "base": {
                                                  "field": {
                                                    "base": {
                                                      "path": {
                                                        "segments": [
                                                          {
                                                            "ident": "ctx",
                                                            "position": {
                                                              "end_column": 19,
                                                              "end_line": 26,
                                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                              "start_column": 16,
                                                              "start_line": 26
                                                            }
                                                          }
                                                        ]
                                                      }
                                                    },
                                                    "ident": "accounts",
                                                    "position": {
                                                      "end_column": 28,
                                                      "end_line": 26,
                                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                      "start_column": 20,
                                                      "start_line": 26
                                                    }
                                                  }
                                                },
                                                "ident": "destination",
                                                "position": {
                                                  "end_column": 15,
                                                  "end_line": 53,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 4,
                                                  "start_line": 53
                                                }
                                              }
                                            },
                                            "ident": "key",
                                            "position": {
                                              "end_column": 42,
                                              "end_line": 19,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 39,
                                              "start_line": 19
                                            }
                                          }
                                        },
                                        {
                                          "field": {
                                            "base": {
                                              "field": {
                                                "base": {
                                                  "field": {
                                                    "base": {
                                                      "path": {
                                                        "segments": [
                                                          {
                                                            "ident": "ctx",
                                                            "position": {
                                                              "end_column": 19,
                                                              "end_line": 26,
                                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                              "start_column": 16,
                                                              "start_line": 26
                                                            }
                                                          }
                                                        ]
                                                      }
                                                    },
                                                    "ident": "accounts",
                                                    "position": {
                                                      "end_column": 28,
                                                      "end_line": 26,
                                                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                      "start_column": 20,
                                                      "start_line": 26
                                                    }
                                                  }
                                                },
                                                "ident": "authority",
                                                "position": {
                                                  "end_column": 13,
                                                  "end_line": 54,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 4,
                                                  "start_line": 54
                                                }
                                              }
                                            },
                                            "ident": "key",
                                            "position": {
                                              "end_column": 42,
                                              "end_line": 19,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 39,
                                              "start_line": 19
                                            }
                                          }
                                        },
                                        {
                                          "reference": {
                                            "expr": {
                                              "array": {
                                                "elems": []
                                              }
                                            }
                                          }
                                        },
                                        {
                                          "path": {
                                            "segments": [
                                              {
                                                "ident": "amount",
                                                "position": {
                                                  "end_column": 22,
                                                  "end_line": 21,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 16,
                                                  "start_line": 21
                                                }
                                              }
                                            ]
                                          }
                                        }
                                      ],
                                      "func": {
                                        "path": {
                                          "segments": [
                                            {
                                              "ident": "spl_token",
                                              "position": {
                                                "end_column": 22,
                                                "end_line": 15,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                "start_column": 13,
                                                "start_line": 15
                                              }
                                            },
                                            {
                                              "ident": "instruction",
                                              "position": {
                                                "end_column": 35,
                                                "end_line": 15,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                "start_column": 24,
                                                "start_line": 15
                                              }
                                            },
                                            {
                                              "ident": "transfer",
                                              "position": {
                                                "end_column": 45,
                                                "end_line": 15,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                "start_column": 37,
                                                "start_line": 15
                                              }
                                            }
                                          ]
                                        }
                                      }
                                    }
                                  }
                                }
                              }
                            }
                          },
                          {
                            "reference": {
                              "expr": {
                                "array": {
                                  "elems": [
                                    {
                                      "method_call": {
                                        "args": [],
                                        "method": "clone",
                                        "receiver": {
                                          "field": {
                                            "base": {
                                              "field": {
                                                "base": {
                                                  "path": {
                                                    "segments": [
                                                      {
                                                        "ident": "ctx",
                                                        "position": {
                                                          "end_column": 19,
                                                          "end_line": 26,
                                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                          "start_column": 16,
                                                          "start_line": 26
                                                        }
                                                      }
                                                    ]
                                                  }
                                                },
                                                "ident": "accounts",
                                                "position": {
                                                  "end_column": 28,
                                                  "end_line": 26,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 20,
                                                  "start_line": 26
                                                }
                                              }
                                            },
                                            "ident": "source",
                                            "position": {
                                              "end_column": 10,
                                              "end_line": 52,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 4,
                                              "start_line": 52
                                            }
                                          }
                                        }
                                      }
                                    },
                                    {
                                      "method_call": {
                                        "args": [],
                                        "method": "clone",
                                        "receiver": {
                                          "field": {
                                            "base": {
                                              "field": {
                                                "base": {
                                                  "path": {
                                                    "segments": [
                                                      {
                                                        "ident": "ctx",
                                                        "position": {
                                                          "end_column": 19,
                                                          "end_line": 26,
                                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                          "start_column": 16,
                                                          "start_line": 26
                                                        }
                                                      }
                                                    ]
                                                  }
                                                },
                                                "ident": "accounts",
                                                "position": {
                                                  "end_column": 28,
                                                  "end_line": 26,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 20,
                                                  "start_line": 26
                                                }
                                              }
                                            },
                                            "ident": "destination",
                                            "position": {
                                              "end_column": 15,
                                              "end_line": 53,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 4,
                                              "start_line": 53
                                            }
                                          }
                                        }
                                      }
                                    },
                                    {
                                      "method_call": {
                                        "args": [],
                                        "method": "clone",
                                        "receiver": {
                                          "field": {
                                            "base": {
                                              "field": {
                                                "base": {
                                                  "path": {
                                                    "segments": [
                                                      {
                                                        "ident": "ctx",
                                                        "position": {
                                                          "end_column": 19,
                                                          "end_line": 26,
                                                          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                          "start_column": 16,
                                                          "start_line": 26
                                                        }
                                                      }
                                                    ]
                                                  }
                                                },
                                                "ident": "accounts",
                                                "position": {
                                                  "end_column": 28,
                                                  "end_line": 26,
                                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                                  "start_column": 20,
                                                  "start_line": 26
                                                }
                                              }
                                            },
                                            "ident": "authority",
                                            "position": {
                                              "end_column": 13,
                                              "end_line": 54,
                                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                              "start_column": 4,
                                              "start_line": 54
                                            }
                                          }
                                        }
                                      }
                                    }
                                  ]
                                }
                              }
                            }
                          }
                        ],
                        "func": {
                          "path": {
                            "segments": [
                              {
                                "ident": "solana_program",
                                "position": {
                                  "end_column": 22,
                                  "end_line": 14,
                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                  "start_column": 8,
                                  "start_line": 14
                                }
                              },
                              {
                                "ident": "program",
                                "position": {
                                  "end_column": 31,
                                  "end_line": 14,
                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                  "start_column": 24,
                                  "start_line": 14
                                }
                              },
                              {
                                "ident": "invoke",
                                "position": {
                                  "end_column": 39,
                                  "end_line": 14,
                                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                                  "start_column": 33,
                                  "start_line": 14
                                }
                              }
                            ]
                          }
                        }
                      }
                    },
                    "False"
                  ]
                }
              ],
              "vis": "pub"
            }
          }
        ],
        "ident": "arbitrary_cpi_secure",
        "position": {
          "end_column": 28,
          "end_line": 7,
          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
          "start_column": 8,
          "start_line": 7
        },
        "vis": "pub"
      },
      "access_path": "[3].mod",
      "metadata": {
        "position": {
          "end_column": 28,
          "end_line": 7,
          "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
          "start_column": 8,
          "start_line": 7
        }
      },
      "children": [
        {
          "raw_node": {
            "ident": "program",
            "position": {
              "end_column": 31,
              "end_line": 14,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
              "start_column": 24,
              "start_line": 14
            }
          },
          "access_path": "[3].mod.attrs[0].meta.path.segments[0]",
          "metadata": {
            "position": {
              "end_column": 31,
              "end_line": 14,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
              "start_column": 24,
              "start_line": 14
            }
          },
          "children": [],
          "parent": "{...}",
          "root": "False",
          "args": [],
          "ident": "program"
        },
        {
          "raw_node": {
            "ident": "super",
            "position": {
              "end_column": 13,
              "end_line": 8,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
              "start_column": 8,
              "start_line": 8
            },
            "tree": "*"
          },
          "access_path": "[3].mod.content[0].use.tree.path",
          "metadata": {
            "position": {
              "end_column": 13,
              "end_line": 8,
              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
              "start_column": 8,
              "start_line": 8
            }
          },
          "children": [],
          "parent": "{...}",
          "root": "False",
          "args": [],
          "ident": "super"
        },
        "{...}"
      ],
      "parent": {
        "raw_node": {},
        "access_path": "root",
        "metadata": {},
        "children": [
          {
            "raw_node": {
              "ident": "anchor_lang",
              "position": {
                "end_column": 15,
                "end_line": 2,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 2
              },
              "tree": {
                "path": {
                  "ident": "prelude",
                  "position": {
                    "end_column": 24,
                    "end_line": 1,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 17,
                    "start_line": 1
                  },
                  "tree": "*"
                }
              }
            },
            "access_path": "[0].use.tree.path",
            "metadata": {
              "position": {
                "end_column": 15,
                "end_line": 2,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 2
              }
            },
            "children": [
              {
                "raw_node": {
                  "ident": "prelude",
                  "position": {
                    "end_column": 24,
                    "end_line": 1,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 17,
                    "start_line": 1
                  },
                  "tree": "*"
                },
                "access_path": "[0].use.tree.path.tree.path",
                "metadata": {
                  "position": {
                    "end_column": 24,
                    "end_line": 1,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 17,
                    "start_line": 1
                  }
                },
                "children": [],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "prelude"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "anchor_lang"
          },
          {
            "raw_node": {
              "ident": "anchor_lang",
              "position": {
                "end_column": 15,
                "end_line": 2,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 2
              },
              "tree": {
                "ident": "solana_program",
                "position": {
                  "end_column": 22,
                  "end_line": 14,
                  "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                  "start_column": 8,
                  "start_line": 14
                }
              }
            },
            "access_path": "[1].use.tree.path",
            "metadata": {
              "position": {
                "end_column": 15,
                "end_line": 2,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 4,
                "start_line": 2
              }
            },
            "children": [
              {
                "raw_node": {
                  "ident": "solana_program",
                  "position": {
                    "end_column": 22,
                    "end_line": 14,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 8,
                    "start_line": 14
                  }
                },
                "access_path": "[1].use.tree.path.tree",
                "metadata": {
                  "position": {
                    "end_column": 22,
                    "end_line": 14,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 8,
                    "start_line": 14
                  }
                },
                "children": [],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "solana_program"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "anchor_lang"
          },
          {
            "raw_node": {
              "ident": "declare_id",
              "position": {
                "end_column": 10,
                "end_line": 4,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 0,
                "start_line": 4
              }
            },
            "access_path": "[2].macro.path.segments[0]",
            "metadata": {
              "position": {
                "end_column": 10,
                "end_line": 4,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 0,
                "start_line": 4
              }
            },
            "children": [],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "declare_id"
          },
          "{...}",
          {
            "raw_node": {
              "attrs": [
                {
                  "meta": {
                    "list": {
                      "delimiter": "paren",
                      "path": {
                        "segments": [
                          {
                            "ident": "derive",
                            "position": {
                              "end_column": 8,
                              "end_line": 50,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                              "start_column": 2,
                              "start_line": 50
                            }
                          }
                        ]
                      },
                      "tokens": [
                        {
                          "ident": "Accounts"
                        }
                      ]
                    }
                  },
                  "style": "outer"
                }
              ],
              "fields": {
                "named": [
                  {
                    "colon_token": "True",
                    "ident": "source",
                    "position": {
                      "end_column": 10,
                      "end_line": 52,
                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                      "start_column": 4,
                      "start_line": 52
                    },
                    "ty": {
                      "path": {
                        "segments": [
                          {
                            "arguments": {
                              "angle_bracketed": {
                                "args": [
                                  {
                                    "lifetime": "info"
                                  }
                                ]
                              }
                            },
                            "ident": "AccountInfo",
                            "position": {
                              "end_column": 30,
                              "end_line": 55,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                              "start_column": 19,
                              "start_line": 55
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "colon_token": "True",
                    "ident": "destination",
                    "position": {
                      "end_column": 15,
                      "end_line": 53,
                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                      "start_column": 4,
                      "start_line": 53
                    },
                    "ty": {
                      "path": {
                        "segments": [
                          {
                            "arguments": {
                              "angle_bracketed": {
                                "args": [
                                  {
                                    "lifetime": "info"
                                  }
                                ]
                              }
                            },
                            "ident": "AccountInfo",
                            "position": {
                              "end_column": 30,
                              "end_line": 55,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                              "start_column": 19,
                              "start_line": 55
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "colon_token": "True",
                    "ident": "authority",
                    "position": {
                      "end_column": 13,
                      "end_line": 54,
                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                      "start_column": 4,
                      "start_line": 54
                    },
                    "ty": {
                      "path": {
                        "segments": [
                          {
                            "arguments": {
                              "angle_bracketed": {
                                "args": [
                                  {
                                    "lifetime": "info"
                                  }
                                ]
                              }
                            },
                            "ident": "AccountInfo",
                            "position": {
                              "end_column": 30,
                              "end_line": 55,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                              "start_column": 19,
                              "start_line": 55
                            }
                          }
                        ]
                      }
                    }
                  },
                  {
                    "colon_token": "True",
                    "ident": "token_program",
                    "position": {
                      "end_column": 17,
                      "end_line": 55,
                      "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                      "start_column": 4,
                      "start_line": 55
                    },
                    "ty": {
                      "path": {
                        "segments": [
                          {
                            "arguments": {
                              "angle_bracketed": {
                                "args": [
                                  {
                                    "lifetime": "info"
                                  }
                                ]
                              }
                            },
                            "ident": "AccountInfo",
                            "position": {
                              "end_column": 30,
                              "end_line": 55,
                              "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                              "start_column": 19,
                              "start_line": 55
                            }
                          }
                        ]
                      }
                    }
                  }
                ]
              },
              "generics": {
                "params": [
                  {
                    "lifetime": {
                      "bounds": [],
                      "lifetime": "info"
                    }
                  }
                ]
              },
              "ident": "Cpi",
              "position": {
                "end_column": 14,
                "end_line": 51,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 11,
                "start_line": 51
              },
              "vis": "pub"
            },
            "access_path": "[4].struct",
            "metadata": {
              "position": {
                "end_column": 14,
                "end_line": 51,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                "start_column": 11,
                "start_line": 51
              }
            },
            "children": [
              {
                "raw_node": {
                  "ident": "derive",
                  "position": {
                    "end_column": 8,
                    "end_line": 50,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 2,
                    "start_line": 50
                  }
                },
                "access_path": "[4].struct.attrs[0].meta.list.path.segments[0]",
                "metadata": {
                  "position": {
                    "end_column": 8,
                    "end_line": 50,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 2,
                    "start_line": 50
                  }
                },
                "children": [],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "derive"
              },
              {
                "raw_node": {
                  "ident": "Accounts"
                },
                "access_path": "[4].struct.attrs[0].meta.list.tokens[0]",
                "metadata": {},
                "children": [],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "Accounts"
              },
              {
                "raw_node": {
                  "colon_token": "True",
                  "ident": "source",
                  "position": {
                    "end_column": 10,
                    "end_line": 52,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 52
                  },
                  "ty": {
                    "path": {
                      "segments": [
                        {
                          "arguments": {
                            "angle_bracketed": {
                              "args": [
                                {
                                  "lifetime": "info"
                                }
                              ]
                            }
                          },
                          "ident": "AccountInfo",
                          "position": {
                            "end_column": 30,
                            "end_line": 55,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 19,
                            "start_line": 55
                          }
                        }
                      ]
                    }
                  }
                },
                "access_path": "[4].struct.fields.named[0]",
                "metadata": {
                  "position": {
                    "end_column": 10,
                    "end_line": 52,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 52
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "arguments": {
                        "angle_bracketed": {
                          "args": [
                            {
                              "lifetime": "info"
                            }
                          ]
                        }
                      },
                      "ident": "AccountInfo",
                      "position": {
                        "end_column": 30,
                        "end_line": 55,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 19,
                        "start_line": 55
                      }
                    },
                    "access_path": "[4].struct.fields.named[0].ty.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 30,
                        "end_line": 55,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 19,
                        "start_line": 55
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "AccountInfo"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "source"
              },
              {
                "raw_node": {
                  "colon_token": "True",
                  "ident": "destination",
                  "position": {
                    "end_column": 15,
                    "end_line": 53,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 53
                  },
                  "ty": {
                    "path": {
                      "segments": [
                        {
                          "arguments": {
                            "angle_bracketed": {
                              "args": [
                                {
                                  "lifetime": "info"
                                }
                              ]
                            }
                          },
                          "ident": "AccountInfo",
                          "position": {
                            "end_column": 30,
                            "end_line": 55,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 19,
                            "start_line": 55
                          }
                        }
                      ]
                    }
                  }
                },
                "access_path": "[4].struct.fields.named[1]",
                "metadata": {
                  "position": {
                    "end_column": 15,
                    "end_line": 53,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 53
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "arguments": {
                        "angle_bracketed": {
                          "args": [
                            {
                              "lifetime": "info"
                            }
                          ]
                        }
                      },
                      "ident": "AccountInfo",
                      "position": {
                        "end_column": 30,
                        "end_line": 55,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 19,
                        "start_line": 55
                      }
                    },
                    "access_path": "[4].struct.fields.named[1].ty.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 30,
                        "end_line": 55,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 19,
                        "start_line": 55
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "AccountInfo"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "destination"
              },
              {
                "raw_node": {
                  "colon_token": "True",
                  "ident": "authority",
                  "position": {
                    "end_column": 13,
                    "end_line": 54,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 54
                  },
                  "ty": {
                    "path": {
                      "segments": [
                        {
                          "arguments": {
                            "angle_bracketed": {
                              "args": [
                                {
                                  "lifetime": "info"
                                }
                              ]
                            }
                          },
                          "ident": "AccountInfo",
                          "position": {
                            "end_column": 30,
                            "end_line": 55,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 19,
                            "start_line": 55
                          }
                        }
                      ]
                    }
                  }
                },
                "access_path": "[4].struct.fields.named[2]",
                "metadata": {
                  "position": {
                    "end_column": 13,
                    "end_line": 54,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 54
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "arguments": {
                        "angle_bracketed": {
                          "args": [
                            {
                              "lifetime": "info"
                            }
                          ]
                        }
                      },
                      "ident": "AccountInfo",
                      "position": {
                        "end_column": 30,
                        "end_line": 55,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 19,
                        "start_line": 55
                      }
                    },
                    "access_path": "[4].struct.fields.named[2].ty.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 30,
                        "end_line": 55,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 19,
                        "start_line": 55
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "AccountInfo"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "authority"
              },
              {
                "raw_node": {
                  "colon_token": "True",
                  "ident": "token_program",
                  "position": {
                    "end_column": 17,
                    "end_line": 55,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 55
                  },
                  "ty": {
                    "path": {
                      "segments": [
                        {
                          "arguments": {
                            "angle_bracketed": {
                              "args": [
                                {
                                  "lifetime": "info"
                                }
                              ]
                            }
                          },
                          "ident": "AccountInfo",
                          "position": {
                            "end_column": 30,
                            "end_line": 55,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                            "start_column": 19,
                            "start_line": 55
                          }
                        }
                      ]
                    }
                  }
                },
                "access_path": "[4].struct.fields.named[3]",
                "metadata": {
                  "position": {
                    "end_column": 17,
                    "end_line": 55,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 55
                  }
                },
                "children": [
                  {
                    "raw_node": {
                      "arguments": {
                        "angle_bracketed": {
                          "args": [
                            {
                              "lifetime": "info"
                            }
                          ]
                        }
                      },
                      "ident": "AccountInfo",
                      "position": {
                        "end_column": 30,
                        "end_line": 55,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 19,
                        "start_line": 55
                      }
                    },
                    "access_path": "[4].struct.fields.named[3].ty.path.segments[0]",
                    "metadata": {
                      "position": {
                        "end_column": 30,
                        "end_line": 55,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/5-arbitrary-cpi/secure/src/lib.rs",
                        "start_column": 19,
                        "start_line": 55
                      }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "AccountInfo"
                  }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "token_program"
              }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "Cpi"
          }
        ],
        "parent": {
          "raw_node": {},
          "access_path": "EMPTY_ACCESS_PATH",
          "metadata": {},
          "children": [],
          "parent": {},
          "root": "False",
          "args": []
        },
        "root": "False",
        "args": [],
        "ident": "EMPTY_IDENT"
      },
      "root": "False",
      "args": [],
      "ident": "arbitrary_cpi_secure"
    },
    "root": "False",
    "args": [],
    "ident": "cpi_secure"
  },
  "root": "False",
  "args": [],
  "ident": "solana_program"
}

AST3 = {
    "raw_node": {
        "ident": "derive",
        "position": {
            "end_column": 8,
            "end_line": 18,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
            "start_column": 2,
            "start_line": 18
        }
    },
    "access_path": "[3].struct.attrs[0].meta.list.path.segments[0]",
    "metadata": {
        "position": {
            "end_column": 8,
            "end_line": 18,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
            "start_column": 2,
            "start_line": 18
        }
    },
    "children": [],
    "parent": {
        "raw_node": {
            "attrs": [
                {
                    "meta": {
                        "list": {
                            "delimiter": "paren",
                            "path": {
                                "segments": [
                                    {
                                        "ident": "derive",
                                        "position": {
                                            "end_column": 8,
                                            "end_line": 18,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 2,
                                            "start_line": 18
                                        }
                                    }
                                ]
                            },
                            "tokens": [
                                {
                                    "ident": "Accounts"
                                }
                            ]
                        }
                    },
                    "style": "outer"
                }
            ],
            "fields": {
                "named": [
                    {
                        "colon_token": "True",
                        "ident": "authority",
                        "position": {
                            "end_column": 13,
                            "end_line": 20,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                            "start_column": 4,
                            "start_line": 20
                        },
                        "ty": {
                            "path": {
                                "segments": [
                                    {
                                        "arguments": {
                                            "angle_bracketed": {
                                                "args": [
                                                    {
                                                        "lifetime": "info"
                                                    }
                                                ]
                                            }
                                        },
                                        "ident": "AccountInfo",
                                        "position": {
                                            "end_column": 26,
                                            "end_line": 20,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 15,
                                            "start_line": 20
                                        }
                                    }
                                ]
                            }
                        }
                    }
                ]
            },
            "generics": {
                "params": [
                    {
                        "lifetime": {
                            "bounds": [],
                            "lifetime": "info"
                        }
                    }
                ]
            },
            "ident": "LogMessage",
            "position": {
                "end_column": 21,
                "end_line": 19,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                "start_column": 11,
                "start_line": 19
            },
            "vis": "pub"
        },
        "access_path": "[3].struct",
        "metadata": {
            "position": {
                "end_column": 21,
                "end_line": 19,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                "start_column": 11,
                "start_line": 19
            }
        },
        "children": [
            "{...}",
            {
                "raw_node": {
                    "ident": "Accounts"
                },
                "access_path": "[3].struct.attrs[0].meta.list.tokens[0]",
                "metadata": {},
                "children": [],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "Accounts"
            },
            {
                "raw_node": {
                    "colon_token": "True",
                    "ident": "authority",
                    "position": {
                        "end_column": 13,
                        "end_line": 20,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                        "start_column": 4,
                        "start_line": 20
                    },
                    "ty": {
                        "path": {
                            "segments": [
                                {
                                    "arguments": {
                                        "angle_bracketed": {
                                            "args": [
                                                {
                                                    "lifetime": "info"
                                                }
                                            ]
                                        }
                                    },
                                    "ident": "AccountInfo",
                                    "position": {
                                        "end_column": 26,
                                        "end_line": 20,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                        "start_column": 15,
                                        "start_line": 20
                                    }
                                }
                            ]
                        }
                    }
                },
                "access_path": "[3].struct.fields.named[0]",
                "metadata": {
                    "position": {
                        "end_column": 13,
                        "end_line": 20,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                        "start_column": 4,
                        "start_line": 20
                    }
                },
                "children": [
                    {
                        "raw_node": {
                            "arguments": {
                                "angle_bracketed": {
                                    "args": [
                                        {
                                            "lifetime": "info"
                                        }
                                    ]
                                }
                            },
                            "ident": "AccountInfo",
                            "position": {
                                "end_column": 26,
                                "end_line": 20,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                "start_column": 15,
                                "start_line": 20
                            }
                        },
                        "access_path": "[3].struct.fields.named[0].ty.path.segments[0]",
                        "metadata": {
                            "position": {
                                "end_column": 26,
                                "end_line": 20,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                "start_column": 15,
                                "start_line": 20
                            }
                        },
                        "children": [],
                        "parent": "{...}",
                        "root": "False",
                        "args": [],
                        "ident": "AccountInfo"
                    }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "authority"
            }
        ],
        "parent": {
            "raw_node": {},
            "access_path": "root",
            "metadata": {},
            "children": [
                {
                    "raw_node": {
                        "ident": "anchor_lang",
                        "position": {
                            "end_column": 15,
                            "end_line": 1,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                            "start_column": 4,
                            "start_line": 1
                        },
                        "tree": {
                            "path": {
                                "ident": "prelude",
                                "position": {
                                    "end_column": 24,
                                    "end_line": 1,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                    "start_column": 17,
                                    "start_line": 1
                                },
                                "tree": "*"
                            }
                        }
                    },
                    "access_path": "[0].use.tree.path",
                    "metadata": {
                        "position": {
                            "end_column": 15,
                            "end_line": 1,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                            "start_column": 4,
                            "start_line": 1
                        }
                    },
                    "children": [
                        {
                            "raw_node": {
                                "ident": "prelude",
                                "position": {
                                    "end_column": 24,
                                    "end_line": 1,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                    "start_column": 17,
                                    "start_line": 1
                                },
                                "tree": "*"
                            },
                            "access_path": "[0].use.tree.path.tree.path",
                            "metadata": {
                                "position": {
                                    "end_column": 24,
                                    "end_line": 1,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                    "start_column": 17,
                                    "start_line": 1
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "prelude"
                        }
                    ],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "anchor_lang"
                },
                {
                    "raw_node": {
                        "ident": "declare_id",
                        "position": {
                            "end_column": 10,
                            "end_line": 3,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                            "start_column": 0,
                            "start_line": 3
                        }
                    },
                    "access_path": "[1].macro.path.segments[0]",
                    "metadata": {
                        "position": {
                            "end_column": 10,
                            "end_line": 3,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                            "start_column": 0,
                            "start_line": 3
                        }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "declare_id"
                },
                {
                    "raw_node": {
                        "attrs": [
                            {
                                "meta": {
                                    "path": {
                                        "segments": [
                                            {
                                                "ident": "program",
                                                "position": {
                                                    "end_column": 9,
                                                    "end_line": 5,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                    "start_column": 2,
                                                    "start_line": 5
                                                }
                                            }
                                        ]
                                    }
                                },
                                "style": "outer"
                            }
                        ],
                        "content": [
                            {
                                "use": {
                                    "tree": {
                                        "path": {
                                            "ident": "super",
                                            "position": {
                                                "end_column": 13,
                                                "end_line": 7,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                "start_column": 8,
                                                "start_line": 7
                                            },
                                            "tree": "*"
                                        }
                                    }
                                }
                            },
                            {
                                "fn": {
                                    "ident": "log_message",
                                    "inputs": [
                                        {
                                            "typed": {
                                                "pat": {
                                                    "ident": {
                                                        "ident": "ctx",
                                                        "position": {
                                                            "end_column": 15,
                                                            "end_line": 10,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                            "start_column": 12,
                                                            "start_line": 10
                                                        }
                                                    }
                                                },
                                                "ty": {
                                                    "path": {
                                                        "segments": [
                                                            {
                                                                "arguments": {
                                                                    "angle_bracketed": {
                                                                        "args": [
                                                                            {
                                                                                "type": {
                                                                                    "path": {
                                                                                        "segments": [
                                                                                            {
                                                                                                "ident": "LogMessage",
                                                                                                "position": {
                                                                                                    "end_column": 21,
                                                                                                    "end_line": 19,
                                                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                                    "start_column": 11,
                                                                                                    "start_line": 19
                                                                                                }
                                                                                            }
                                                                                        ]
                                                                                    }
                                                                                }
                                                                            }
                                                                        ]
                                                                    }
                                                                },
                                                                "ident": "Context",
                                                                "position": {
                                                                    "end_column": 35,
                                                                    "end_line": 9,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                    "start_column": 28,
                                                                    "start_line": 9
                                                                }
                                                            }
                                                        ]
                                                    }
                                                }
                                            }
                                        }
                                    ],
                                    "output": {
                                        "path": {
                                            "segments": [
                                                {
                                                    "ident": "ProgramResult",
                                                    "position": {
                                                        "end_column": 65,
                                                        "end_line": 9,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                        "start_column": 52,
                                                        "start_line": 9
                                                    }
                                                }
                                            ]
                                        }
                                    },
                                    "position": {
                                        "end_column": 22,
                                        "end_line": 9,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                        "start_column": 11,
                                        "start_line": 9
                                    },
                                    "stmts": [
                                        {
                                            "expr": [
                                                {
                                                    "if": {
                                                        "cond": {
                                                            "unary": {
                                                                "expr": {
                                                                    "field": {
                                                                        "base": {
                                                                            "field": {
                                                                                "base": {
                                                                                    "field": {
                                                                                        "base": {
                                                                                            "path": {
                                                                                                "segments": [
                                                                                                    {
                                                                                                        "ident": "ctx",
                                                                                                        "position": {
                                                                                                            "end_column": 15,
                                                                                                            "end_line": 10,
                                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                                            "start_column": 12,
                                                                                                            "start_line": 10
                                                                                                        }
                                                                                                    }
                                                                                                ]
                                                                                            }
                                                                                        },
                                                                                        "ident": "accounts",
                                                                                        "position": {
                                                                                            "end_column": 24,
                                                                                            "end_line": 10,
                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                            "start_column": 16,
                                                                                            "start_line": 10
                                                                                        }
                                                                                    }
                                                                                },
                                                                                "ident": "authority",
                                                                                "position": {
                                                                                    "end_column": 13,
                                                                                    "end_line": 20,
                                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                    "start_column": 4,
                                                                                    "start_line": 20
                                                                                }
                                                                            }
                                                                        },
                                                                        "ident": "is_signer",
                                                                        "position": {
                                                                            "end_column": 44,
                                                                            "end_line": 10,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                            "start_column": 35,
                                                                            "start_line": 10
                                                                        }
                                                                    }
                                                                },
                                                                "op": "!"
                                                            }
                                                        },
                                                        "then_branch": [
                                                            {
                                                                "expr": [
                                                                    {
                                                                        "return": {
                                                                            "expr": {
                                                                                "call": {
                                                                                    "args": [
                                                                                        {
                                                                                            "path": {
                                                                                                "segments": [
                                                                                                    {
                                                                                                        "ident": "ProgramError",
                                                                                                        "position": {
                                                                                                            "end_column": 35,
                                                                                                            "end_line": 11,
                                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                                            "start_column": 23,
                                                                                                            "start_line": 11
                                                                                                        }
                                                                                                    },
                                                                                                    {
                                                                                                        "ident": "MissingRequiredSignature",
                                                                                                        "position": {
                                                                                                            "end_column": 61,
                                                                                                            "end_line": 11,
                                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                                            "start_column": 37,
                                                                                                            "start_line": 11
                                                                                                        }
                                                                                                    }
                                                                                                ]
                                                                                            }
                                                                                        }
                                                                                    ],
                                                                                    "func": {
                                                                                        "path": {
                                                                                            "segments": [
                                                                                                {
                                                                                                    "ident": "Err",
                                                                                                    "position": {
                                                                                                        "end_column": 22,
                                                                                                        "end_line": 11,
                                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                                        "start_column": 19,
                                                                                                        "start_line": 11
                                                                                                    }
                                                                                                }
                                                                                            ]
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    },
                                                                    "True"
                                                                ]
                                                            }
                                                        ]
                                                    }
                                                }, "False"
                                            ]
                                        },
                                        {
                                            "macro": {
                                                "delimiter": "paren",
                                                "path": {
                                                    "segments": [
                                                        {
                                                            "ident": "msg",
                                                            "position": {
                                                                "end_column": 11,
                                                                "end_line": 13,
                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                "start_column": 8,
                                                                "start_line": 13
                                                            }
                                                        }
                                                    ]
                                                },
                                                "semi_token": "True",
                                                "tokens": [
                                                    {
                                                        "lit": "\"GM {}\""
                                                    },
                                                    {
                                                        "punct": {
                                                            "op": ",",
                                                            "spacing": "alone"
                                                        }
                                                    },
                                                    {
                                                        "ident": "ctx",
                                                        "position": {
                                                            "end_column": 15,
                                                            "end_line": 10,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                            "start_column": 12,
                                                            "start_line": 10
                                                        }
                                                    },
                                                    {
                                                        "punct": {
                                                            "op": ".",
                                                            "spacing": "alone"
                                                        }
                                                    },
                                                    {
                                                        "ident": "accounts",
                                                        "position": {
                                                            "end_column": 24,
                                                            "end_line": 10,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                            "start_column": 16,
                                                            "start_line": 10
                                                        }
                                                    },
                                                    {
                                                        "punct": {
                                                            "op": ".",
                                                            "spacing": "alone"
                                                        }
                                                    },
                                                    {
                                                        "ident": "authority",
                                                        "position": {
                                                            "end_column": 13,
                                                            "end_line": 20,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                            "start_column": 4,
                                                            "start_line": 20
                                                        }
                                                    },
                                                    {
                                                        "punct": {
                                                            "op": ".",
                                                            "spacing": "alone"
                                                        }
                                                    },
                                                    {
                                                        "ident": "key"
                                                    },
                                                    {
                                                        "group": {
                                                            "delimiter": "parenthesis",
                                                            "stream": []
                                                        }
                                                    },
                                                    {
                                                        "punct": {
                                                            "op": ".",
                                                            "spacing": "alone"
                                                        }
                                                    },
                                                    {
                                                        "ident": "to_string"
                                                    },
                                                    {
                                                        "group": {
                                                            "delimiter": "parenthesis",
                                                            "stream": []
                                                        }
                                                    }
                                                ]
                                            }
                                        },
                                        {
                                            "expr": [
                                                {
                                                    "call": {
                                                        "args": [
                                                            {
                                                                "tuple": {
                                                                    "elems": []
                                                                }
                                                            }
                                                        ],
                                                        "func": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "Ok",
                                                                        "position": {
                                                                            "end_column": 10,
                                                                            "end_line": 14,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                            "start_column": 8,
                                                                            "start_line": 14
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        }
                                                    }
                                                }, "False"
                                            ]
                                        }
                                    ],
                                    "vis": "pub"
                                }
                            }
                        ],
                        "ident": "signer_authorization_secure",
                        "position": {
                            "end_column": 35,
                            "end_line": 6,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                            "start_column": 8,
                            "start_line": 6
                        },
                        "vis": "pub"
                    },
                    "access_path": "[2].mod",
                    "metadata": {
                        "position": {
                            "end_column": 35,
                            "end_line": 6,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                            "start_column": 8,
                            "start_line": 6
                        }
                    },
                    "children": [
                        {
                            "raw_node": {
                                "ident": "program",
                                "position": {
                                    "end_column": 9,
                                    "end_line": 5,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                    "start_column": 2,
                                    "start_line": 5
                                }
                            },
                            "access_path": "[2].mod.attrs[0].meta.path.segments[0]",
                            "metadata": {
                                "position": {
                                    "end_column": 9,
                                    "end_line": 5,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                    "start_column": 2,
                                    "start_line": 5
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "program"
                        },
                        {
                            "raw_node": {
                                "ident": "super",
                                "position": {
                                    "end_column": 13,
                                    "end_line": 7,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                    "start_column": 8,
                                    "start_line": 7
                                },
                                "tree": "*"
                            },
                            "access_path": "[2].mod.content[0].use.tree.path",
                            "metadata": {
                                "position": {
                                    "end_column": 13,
                                    "end_line": 7,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                    "start_column": 8,
                                    "start_line": 7
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "super"
                        },
                        {
                            "raw_node": {
                                "ident": "log_message",
                                "inputs": [
                                    {
                                        "typed": {
                                            "pat": {
                                                "ident": {
                                                    "ident": "ctx",
                                                    "position": {
                                                        "end_column": 15,
                                                        "end_line": 10,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                        "start_column": 12,
                                                        "start_line": 10
                                                    }
                                                }
                                            },
                                            "ty": {
                                                "path": {
                                                    "segments": [
                                                        {
                                                            "arguments": {
                                                                "angle_bracketed": {
                                                                    "args": [
                                                                        {
                                                                            "type": {
                                                                                "path": {
                                                                                    "segments": [
                                                                                        {
                                                                                            "ident": "LogMessage",
                                                                                            "position": {
                                                                                                "end_column": 21,
                                                                                                "end_line": 19,
                                                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                                "start_column": 11,
                                                                                                "start_line": 19
                                                                                            }
                                                                                        }
                                                                                    ]
                                                                                }
                                                                            }
                                                                        }
                                                                    ]
                                                                }
                                                            },
                                                            "ident": "Context",
                                                            "position": {
                                                                "end_column": 35,
                                                                "end_line": 9,
                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                "start_column": 28,
                                                                "start_line": 9
                                                            }
                                                        }
                                                    ]
                                                }
                                            }
                                        }
                                    }
                                ],
                                "output": {
                                    "path": {
                                        "segments": [
                                            {
                                                "ident": "ProgramResult",
                                                "position": {
                                                    "end_column": 65,
                                                    "end_line": 9,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                    "start_column": 52,
                                                    "start_line": 9
                                                }
                                            }
                                        ]
                                    }
                                },
                                "position": {
                                    "end_column": 22,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                    "start_column": 11,
                                    "start_line": 9
                                },
                                "stmts": [
                                    {
                                        "expr": [
                                            {
                                                "if": {
                                                    "cond": {
                                                        "unary": {
                                                            "expr": {
                                                                "field": {
                                                                    "base": {
                                                                        "field": {
                                                                            "base": {
                                                                                "field": {
                                                                                    "base": {
                                                                                        "path": {
                                                                                            "segments": [
                                                                                                {
                                                                                                    "ident": "ctx",
                                                                                                    "position": {
                                                                                                        "end_column": 15,
                                                                                                        "end_line": 10,
                                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                                        "start_column": 12,
                                                                                                        "start_line": 10
                                                                                                    }
                                                                                                }
                                                                                            ]
                                                                                        }
                                                                                    },
                                                                                    "ident": "accounts",
                                                                                    "position": {
                                                                                        "end_column": 24,
                                                                                        "end_line": 10,
                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                        "start_column": 16,
                                                                                        "start_line": 10
                                                                                    }
                                                                                }
                                                                            },
                                                                            "ident": "authority",
                                                                            "position": {
                                                                                "end_column": 13,
                                                                                "end_line": 20,
                                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                "start_column": 4,
                                                                                "start_line": 20
                                                                            }
                                                                        }
                                                                    },
                                                                    "ident": "is_signer",
                                                                    "position": {
                                                                        "end_column": 44,
                                                                        "end_line": 10,
                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                        "start_column": 35,
                                                                        "start_line": 10
                                                                    }
                                                                }
                                                            },
                                                            "op": "!"
                                                        }
                                                    },
                                                    "then_branch": [
                                                        {
                                                            "expr": [
                                                                {
                                                                    "return": {
                                                                        "expr": {
                                                                            "call": {
                                                                                "args": [
                                                                                    {
                                                                                        "path": {
                                                                                            "segments": [
                                                                                                {
                                                                                                    "ident": "ProgramError",
                                                                                                    "position": {
                                                                                                        "end_column": 35,
                                                                                                        "end_line": 11,
                                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                                        "start_column": 23,
                                                                                                        "start_line": 11
                                                                                                    }
                                                                                                },
                                                                                                {
                                                                                                    "ident": "MissingRequiredSignature",
                                                                                                    "position": {
                                                                                                        "end_column": 61,
                                                                                                        "end_line": 11,
                                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                                        "start_column": 37,
                                                                                                        "start_line": 11
                                                                                                    }
                                                                                                }
                                                                                            ]
                                                                                        }
                                                                                    }
                                                                                ],
                                                                                "func": {
                                                                                    "path": {
                                                                                        "segments": [
                                                                                            {
                                                                                                "ident": "Err",
                                                                                                "position": {
                                                                                                    "end_column": 22,
                                                                                                    "end_line": 11,
                                                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                                                    "start_column": 19,
                                                                                                    "start_line": 11
                                                                                                }
                                                                                            }
                                                                                        ]
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                "True"
                                                            ]
                                                        }
                                                    ]
                                                }
                                            }, "False"
                                        ]
                                    },
                                    {
                                        "macro": {
                                            "delimiter": "paren",
                                            "path": {
                                                "segments": [
                                                    {
                                                        "ident": "msg",
                                                        "position": {
                                                            "end_column": 11,
                                                            "end_line": 13,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                            "start_column": 8,
                                                            "start_line": 13
                                                        }
                                                    }
                                                ]
                                            },
                                            "semi_token": "True",
                                            "tokens": [
                                                {
                                                    "lit": "\"GM {}\""
                                                },
                                                {
                                                    "punct": {
                                                        "op": ",",
                                                        "spacing": "alone"
                                                    }
                                                },
                                                {
                                                    "ident": "ctx",
                                                    "position": {
                                                        "end_column": 15,
                                                        "end_line": 10,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                        "start_column": 12,
                                                        "start_line": 10
                                                    }
                                                },
                                                {
                                                    "punct": {
                                                        "op": ".",
                                                        "spacing": "alone"
                                                    }
                                                },
                                                {
                                                    "ident": "accounts",
                                                    "position": {
                                                        "end_column": 24,
                                                        "end_line": 10,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                        "start_column": 16,
                                                        "start_line": 10
                                                    }
                                                },
                                                {
                                                    "punct": {
                                                        "op": ".",
                                                        "spacing": "alone"
                                                    }
                                                },
                                                {
                                                    "ident": "authority",
                                                    "position": {
                                                        "end_column": 13,
                                                        "end_line": 20,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                        "start_column": 4,
                                                        "start_line": 20
                                                    }
                                                },
                                                {
                                                    "punct": {
                                                        "op": ".",
                                                        "spacing": "alone"
                                                    }
                                                },
                                                {
                                                    "ident": "key"
                                                },
                                                {
                                                    "group": {
                                                        "delimiter": "parenthesis",
                                                        "stream": []
                                                    }
                                                },
                                                {
                                                    "punct": {
                                                        "op": ".",
                                                        "spacing": "alone"
                                                    }
                                                },
                                                {
                                                    "ident": "to_string"
                                                },
                                                {
                                                    "group": {
                                                        "delimiter": "parenthesis",
                                                        "stream": []
                                                    }
                                                }
                                            ]
                                        }
                                    },
                                    {
                                        "expr": [
                                            {
                                                "call": {
                                                    "args": [
                                                        {
                                                            "tuple": {
                                                                "elems": []
                                                            }
                                                        }
                                                    ],
                                                    "func": {
                                                        "path": {
                                                            "segments": [
                                                                {
                                                                    "ident": "Ok",
                                                                    "position": {
                                                                        "end_column": 10,
                                                                        "end_line": 14,
                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                        "start_column": 8,
                                                                        "start_line": 14
                                                                    }
                                                                }
                                                            ]
                                                        }
                                                    }
                                                }
                                            }, "False"
                                        ]
                                    }
                                ],
                                "vis": "pub"
                            },
                            "access_path": "[2].mod.content[1].fn",
                            "metadata": {
                                "position": {
                                    "end_column": 22,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                    "start_column": 11,
                                    "start_line": 9
                                }
                            },
                            "children": [
                                {
                                    "raw_node": {
                                        "ident": {
                                            "ident": "ctx",
                                            "position": {
                                                "end_column": 15,
                                                "end_line": 10,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                "start_column": 12,
                                                "start_line": 10
                                            }
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.inputs[0].typed.pat",
                                    "metadata": {},
                                    "children": [
                                        {
                                            "raw_node": {
                                                "ident": "ctx",
                                                "position": {
                                                    "end_column": 15,
                                                    "end_line": 10,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                    "start_column": 12,
                                                    "start_line": 10
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.inputs[0].typed.pat.ident",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 15,
                                                    "end_line": 10,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                    "start_column": 12,
                                                    "start_line": 10
                                                }
                                            },
                                            "children": [],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "ctx"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "ctx"
                                },
                                {
                                    "raw_node": {
                                        "arguments": {
                                            "angle_bracketed": {
                                                "args": [
                                                    {
                                                        "type": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "LogMessage",
                                                                        "position": {
                                                                            "end_column": 21,
                                                                            "end_line": 19,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                            "start_column": 11,
                                                                            "start_line": 19
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        }
                                                    }
                                                ]
                                            }
                                        },
                                        "ident": "Context",
                                        "position": {
                                            "end_column": 35,
                                            "end_line": 9,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 28,
                                            "start_line": 9
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.inputs[0].typed.ty.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 35,
                                            "end_line": 9,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 28,
                                            "start_line": 9
                                        }
                                    },
                                    "children": [
                                        {
                                            "raw_node": {
                                                "ident": "LogMessage",
                                                "position": {
                                                    "end_column": 21,
                                                    "end_line": 19,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                    "start_column": 11,
                                                    "start_line": 19
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.inputs[0].typed.ty.path.segments[0].arguments.angle_bracketed.args[0].type.path.segments[0]",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 21,
                                                    "end_line": 19,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                    "start_column": 11,
                                                    "start_line": 19
                                                }
                                            },
                                            "children": [],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "LogMessage"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "Context"
                                },
                                {
                                    "raw_node": {
                                        "ident": "ProgramResult",
                                        "position": {
                                            "end_column": 65,
                                            "end_line": 9,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 52,
                                            "start_line": 9
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.output.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 65,
                                            "end_line": 9,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 52,
                                            "start_line": 9
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "ProgramResult"
                                },
                                {
                                    "raw_node": {
                                        "base": {
                                            "field": {
                                                "base": {
                                                    "field": {
                                                        "base": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "ctx",
                                                                        "position": {
                                                                            "end_column": 15,
                                                                            "end_line": 10,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                            "start_column": 12,
                                                                            "start_line": 10
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        },
                                                        "ident": "accounts",
                                                        "position": {
                                                            "end_column": 24,
                                                            "end_line": 10,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                            "start_column": 16,
                                                            "start_line": 10
                                                        }
                                                    }
                                                },
                                                "ident": "authority",
                                                "position": {
                                                    "end_column": 13,
                                                    "end_line": 20,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 20
                                                }
                                            }
                                        },
                                        "ident": "is_signer",
                                        "position": {
                                            "end_column": 44,
                                            "end_line": 10,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 35,
                                            "start_line": 10
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.unary.expr.field",
                                    "metadata": {
                                        "position": {
                                            "end_column": 44,
                                            "end_line": 10,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 35,
                                            "start_line": 10
                                        }
                                    },
                                    "children": [
                                        {
                                            "raw_node": {
                                                "base": {
                                                    "field": {
                                                        "base": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "ctx",
                                                                        "position": {
                                                                            "end_column": 15,
                                                                            "end_line": 10,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                            "start_column": 12,
                                                                            "start_line": 10
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        },
                                                        "ident": "accounts",
                                                        "position": {
                                                            "end_column": 24,
                                                            "end_line": 10,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                            "start_column": 16,
                                                            "start_line": 10
                                                        }
                                                    }
                                                },
                                                "ident": "authority",
                                                "position": {
                                                    "end_column": 13,
                                                    "end_line": 20,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 20
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.unary.expr.field.base.field",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 13,
                                                    "end_line": 20,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 20
                                                }
                                            },
                                            "children": [
                                                {
                                                    "raw_node": {
                                                        "base": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "ctx",
                                                                        "position": {
                                                                            "end_column": 15,
                                                                            "end_line": 10,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                            "start_column": 12,
                                                                            "start_line": 10
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        },
                                                        "ident": "accounts",
                                                        "position": {
                                                            "end_column": 24,
                                                            "end_line": 10,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                            "start_column": 16,
                                                            "start_line": 10
                                                        }
                                                    },
                                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.unary.expr.field.base.field.base.field",
                                                    "metadata": {
                                                        "position": {
                                                            "end_column": 24,
                                                            "end_line": 10,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                            "start_column": 16,
                                                            "start_line": 10
                                                        }
                                                    },
                                                    "children": [
                                                        {
                                                            "raw_node": {
                                                                "ident": "ctx",
                                                                "position": {
                                                                    "end_column": 15,
                                                                    "end_line": 10,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                    "start_column": 12,
                                                                    "start_line": 10
                                                                }
                                                            },
                                                            "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.unary.expr.field.base.field.base.field.base.path.segments[0]",
                                                            "metadata": {
                                                                "position": {
                                                                    "end_column": 15,
                                                                    "end_line": 10,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                                                    "start_column": 12,
                                                                    "start_line": 10
                                                                }
                                                            },
                                                            "children": [],
                                                            "parent": "{...}",
                                                            "root": "False",
                                                            "args": [],
                                                            "ident": "ctx"
                                                        }
                                                    ],
                                                    "parent": "{...}",
                                                    "root": "False",
                                                    "args": [],
                                                    "ident": "accounts"
                                                }
                                            ],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "authority"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "is_signer"
                                },
                                {
                                    "raw_node": {
                                        "ident": "ProgramError",
                                        "position": {
                                            "end_column": 35,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 23,
                                            "start_line": 11
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.then_branch[0].expr[0].return.expr.call.args[0].path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 35,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 23,
                                            "start_line": 11
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "ProgramError"
                                },
                                {
                                    "raw_node": {
                                        "ident": "MissingRequiredSignature",
                                        "position": {
                                            "end_column": 61,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 37,
                                            "start_line": 11
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.then_branch[0].expr[0].return.expr.call.args[0].path.segments[1]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 61,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 37,
                                            "start_line": 11
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "MissingRequiredSignature"
                                },
                                {
                                    "raw_node": {
                                        "ident": "Err",
                                        "position": {
                                            "end_column": 22,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 19,
                                            "start_line": 11
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.then_branch[0].expr[0].return.expr.call.func.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 22,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 19,
                                            "start_line": 11
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "Err"
                                },
                                {
                                    "raw_node": {
                                        "ident": "msg",
                                        "position": {
                                            "end_column": 11,
                                            "end_line": 13,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 8,
                                            "start_line": 13
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[1].macro.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 11,
                                            "end_line": 13,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 8,
                                            "start_line": 13
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "msg"
                                },
                                {
                                    "raw_node": {
                                        "ident": "ctx",
                                        "position": {
                                            "end_column": 15,
                                            "end_line": 10,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 12,
                                            "start_line": 10
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[1].macro.tokens[2]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 15,
                                            "end_line": 10,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 12,
                                            "start_line": 10
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "ctx"
                                },
                                {
                                    "raw_node": {
                                        "ident": "accounts",
                                        "position": {
                                            "end_column": 24,
                                            "end_line": 10,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 16,
                                            "start_line": 10
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[1].macro.tokens[4]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 24,
                                            "end_line": 10,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 16,
                                            "start_line": 10
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "accounts"
                                },
                                {
                                    "raw_node": {
                                        "ident": "authority",
                                        "position": {
                                            "end_column": 13,
                                            "end_line": 20,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 4,
                                            "start_line": 20
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[1].macro.tokens[6]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 13,
                                            "end_line": 20,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 4,
                                            "start_line": 20
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "authority"
                                },
                                {
                                    "raw_node": {
                                        "ident": "key"
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[1].macro.tokens[8]",
                                    "metadata": {},
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "key"
                                },
                                {
                                    "raw_node": {
                                        "ident": "to_string"
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[1].macro.tokens[11]",
                                    "metadata": {},
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "to_string"
                                },
                                {
                                    "raw_node": {
                                        "ident": "Ok",
                                        "position": {
                                            "end_column": 10,
                                            "end_line": 14,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 8,
                                            "start_line": 14
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[2].expr[0].call.func.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 10,
                                            "end_line": 14,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/0-signer-authorization/secure/src/lib.rs",
                                            "start_column": 8,
                                            "start_line": 14
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "Ok"
                                }
                            ],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "log_message"
                        }
                    ],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "signer_authorization_secure"
                },
                "{...}"
            ],
            "parent": {
                "raw_node": {},
                "access_path": "EMPTY_ACCESS_PATH",
                "metadata": {},
                "children": [],
                "parent": {},
                "root": "False",
                "args": []
            },
            "root": "False",
            "args": [],
            "ident": "EMPTY_IDENT"
        },
        "root": "False",
        "args": [],
        "ident": "LogMessage"
    },
    "root": "False",
    "args": [],
    "ident": "derive"
}

AST4 = {
    "raw_node": {
        "ident": "derive",
        "position": {
            "end_column": 8,
            "end_line": 22,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
            "start_column": 2,
            "start_line": 22
        }
    },
    "access_path": "[3].struct.attrs[0].meta.list.path.segments[0]",
    "metadata": {
        "position": {
            "end_column": 8,
            "end_line": 22,
            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
            "start_column": 2,
            "start_line": 22
        }
    },
    "children": [],
    "parent": {
        "raw_node": {
            "attrs": [
                {
                    "meta": {
                        "list": {
                            "delimiter": "paren",
                            "path": {
                                "segments": [
                                    {
                                        "ident": "derive",
                                        "position": {
                                            "end_column": 8,
                                            "end_line": 22,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 2,
                                            "start_line": 22
                                        }
                                    }
                                ]
                            },
                            "tokens": [
                                {
                                    "ident": "Accounts"
                                }
                            ]
                        }
                    },
                    "style": "outer"
                }
            ],
            "fields": {
                "named": [
                    {
                        "colon_token": "True",
                        "ident": "user_a",
                        "position": {
                            "end_column": 10,
                            "end_line": 24,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                            "start_column": 4,
                            "start_line": 24
                        },
                        "ty": {
                            "path": {
                                "segments": [
                                    {
                                        "arguments": {
                                            "angle_bracketed": {
                                                "args": [
                                                    {
                                                        "lifetime": "info"
                                                    },
                                                    {
                                                        "type": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "User",
                                                                        "position": {
                                                                            "end_column": 15,
                                                                            "end_line": 29,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 11,
                                                                            "start_line": 29
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        }
                                                    }
                                                ]
                                            }
                                        },
                                        "ident": "Account",
                                        "position": {
                                            "end_column": 19,
                                            "end_line": 25,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 12,
                                            "start_line": 25
                                        }
                                    }
                                ]
                            }
                        }
                    },
                    {
                        "colon_token": "True",
                        "ident": "user_b",
                        "position": {
                            "end_column": 10,
                            "end_line": 25,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                            "start_column": 4,
                            "start_line": 25
                        },
                        "ty": {
                            "path": {
                                "segments": [
                                    {
                                        "arguments": {
                                            "angle_bracketed": {
                                                "args": [
                                                    {
                                                        "lifetime": "info"
                                                    },
                                                    {
                                                        "type": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "User",
                                                                        "position": {
                                                                            "end_column": 15,
                                                                            "end_line": 29,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 11,
                                                                            "start_line": 29
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        }
                                                    }
                                                ]
                                            }
                                        },
                                        "ident": "Account",
                                        "position": {
                                            "end_column": 19,
                                            "end_line": 25,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 12,
                                            "start_line": 25
                                        }
                                    }
                                ]
                            }
                        }
                    }
                ]
            },
            "generics": {
                "params": [
                    {
                        "lifetime": {
                            "bounds": [],
                            "lifetime": "info"
                        }
                    }
                ]
            },
            "ident": "Update",
            "position": {
                "end_column": 17,
                "end_line": 23,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                "start_column": 11,
                "start_line": 23
            },
            "vis": "pub"
        },
        "access_path": "[3].struct",
        "metadata": {
            "position": {
                "end_column": 17,
                "end_line": 23,
                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                "start_column": 11,
                "start_line": 23
            }
        },
        "children": [
            "{...}",
            {
                "raw_node": {
                    "ident": "Accounts"
                },
                "access_path": "[3].struct.attrs[0].meta.list.tokens[0]",
                "metadata": {},
                "children": [],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "Accounts"
            },
            {
                "raw_node": {
                    "colon_token": "True",
                    "ident": "user_a",
                    "position": {
                        "end_column": 10,
                        "end_line": 24,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                        "start_column": 4,
                        "start_line": 24
                    },
                    "ty": {
                        "path": {
                            "segments": [
                                {
                                    "arguments": {
                                        "angle_bracketed": {
                                            "args": [
                                                {
                                                    "lifetime": "info"
                                                },
                                                {
                                                    "type": {
                                                        "path": {
                                                            "segments": [
                                                                {
                                                                    "ident": "User",
                                                                    "position": {
                                                                        "end_column": 15,
                                                                        "end_line": 29,
                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                        "start_column": 11,
                                                                        "start_line": 29
                                                                    }
                                                                }
                                                            ]
                                                        }
                                                    }
                                                }
                                            ]
                                        }
                                    },
                                    "ident": "Account",
                                    "position": {
                                        "end_column": 19,
                                        "end_line": 25,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                        "start_column": 12,
                                        "start_line": 25
                                    }
                                }
                            ]
                        }
                    }
                },
                "access_path": "[3].struct.fields.named[0]",
                "metadata": {
                    "position": {
                        "end_column": 10,
                        "end_line": 24,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                        "start_column": 4,
                        "start_line": 24
                    }
                },
                "children": [
                    {
                        "raw_node": {
                            "arguments": {
                                "angle_bracketed": {
                                    "args": [
                                        {
                                            "lifetime": "info"
                                        },
                                        {
                                            "type": {
                                                "path": {
                                                    "segments": [
                                                        {
                                                            "ident": "User",
                                                            "position": {
                                                                "end_column": 15,
                                                                "end_line": 29,
                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                "start_column": 11,
                                                                "start_line": 29
                                                            }
                                                        }
                                                    ]
                                                }
                                            }
                                        }
                                    ]
                                }
                            },
                            "ident": "Account",
                            "position": {
                                "end_column": 19,
                                "end_line": 25,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                "start_column": 12,
                                "start_line": 25
                            }
                        },
                        "access_path": "[3].struct.fields.named[0].ty.path.segments[0]",
                        "metadata": {
                            "position": {
                                "end_column": 19,
                                "end_line": 25,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                "start_column": 12,
                                "start_line": 25
                            }
                        },
                        "children": [
                            {
                                "raw_node": {
                                    "ident": "User",
                                    "position": {
                                        "end_column": 15,
                                        "end_line": 29,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                        "start_column": 11,
                                        "start_line": 29
                                    }
                                },
                                "access_path": "[3].struct.fields.named[0].ty.path.segments[0].arguments.angle_bracketed.args[1].type.path.segments[0]",
                                "metadata": {
                                    "position": {
                                        "end_column": 15,
                                        "end_line": 29,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                        "start_column": 11,
                                        "start_line": 29
                                    }
                                },
                                "children": [],
                                "parent": "{...}",
                                "root": "False",
                                "args": [],
                                "ident": "User"
                            }
                        ],
                        "parent": "{...}",
                        "root": "False",
                        "args": [],
                        "ident": "Account"
                    }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "user_a"
            },
            {
                "raw_node": {
                    "colon_token": "True",
                    "ident": "user_b",
                    "position": {
                        "end_column": 10,
                        "end_line": 25,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                        "start_column": 4,
                        "start_line": 25
                    },
                    "ty": {
                        "path": {
                            "segments": [
                                {
                                    "arguments": {
                                        "angle_bracketed": {
                                            "args": [
                                                {
                                                    "lifetime": "info"
                                                },
                                                {
                                                    "type": {
                                                        "path": {
                                                            "segments": [
                                                                {
                                                                    "ident": "User",
                                                                    "position": {
                                                                        "end_column": 15,
                                                                        "end_line": 29,
                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                        "start_column": 11,
                                                                        "start_line": 29
                                                                    }
                                                                }
                                                            ]
                                                        }
                                                    }
                                                }
                                            ]
                                        }
                                    },
                                    "ident": "Account",
                                    "position": {
                                        "end_column": 19,
                                        "end_line": 25,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                        "start_column": 12,
                                        "start_line": 25
                                    }
                                }
                            ]
                        }
                    }
                },
                "access_path": "[3].struct.fields.named[1]",
                "metadata": {
                    "position": {
                        "end_column": 10,
                        "end_line": 25,
                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                        "start_column": 4,
                        "start_line": 25
                    }
                },
                "children": [
                    {
                        "raw_node": {
                            "arguments": {
                                "angle_bracketed": {
                                    "args": [
                                        {
                                            "lifetime": "info"
                                        },
                                        {
                                            "type": {
                                                "path": {
                                                    "segments": [
                                                        {
                                                            "ident": "User",
                                                            "position": {
                                                                "end_column": 15,
                                                                "end_line": 29,
                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                "start_column": 11,
                                                                "start_line": 29
                                                            }
                                                        }
                                                    ]
                                                }
                                            }
                                        }
                                    ]
                                }
                            },
                            "ident": "Account",
                            "position": {
                                "end_column": 19,
                                "end_line": 25,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                "start_column": 12,
                                "start_line": 25
                            }
                        },
                        "access_path": "[3].struct.fields.named[1].ty.path.segments[0]",
                        "metadata": {
                            "position": {
                                "end_column": 19,
                                "end_line": 25,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                "start_column": 12,
                                "start_line": 25
                            }
                        },
                        "children": [
                            {
                                "raw_node": {
                                    "ident": "User",
                                    "position": {
                                        "end_column": 15,
                                        "end_line": 29,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                        "start_column": 11,
                                        "start_line": 29
                                    }
                                },
                                "access_path": "[3].struct.fields.named[1].ty.path.segments[0].arguments.angle_bracketed.args[1].type.path.segments[0]",
                                "metadata": {
                                    "position": {
                                        "end_column": 15,
                                        "end_line": 29,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                        "start_column": 11,
                                        "start_line": 29
                                    }
                                },
                                "children": [],
                                "parent": "{...}",
                                "root": "False",
                                "args": [],
                                "ident": "User"
                            }
                        ],
                        "parent": "{...}",
                        "root": "False",
                        "args": [],
                        "ident": "Account"
                    }
                ],
                "parent": "{...}",
                "root": "False",
                "args": [],
                "ident": "user_b"
            }
        ],
        "parent": {
            "raw_node": {},
            "access_path": "root",
            "metadata": {},
            "children": [
                {
                    "raw_node": {
                        "ident": "anchor_lang",
                        "position": {
                            "end_column": 15,
                            "end_line": 1,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                            "start_column": 4,
                            "start_line": 1
                        },
                        "tree": {
                            "path": {
                                "ident": "prelude",
                                "position": {
                                    "end_column": 24,
                                    "end_line": 1,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 17,
                                    "start_line": 1
                                },
                                "tree": "*"
                            }
                        }
                    },
                    "access_path": "[0].use.tree.path",
                    "metadata": {
                        "position": {
                            "end_column": 15,
                            "end_line": 1,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                            "start_column": 4,
                            "start_line": 1
                        }
                    },
                    "children": [
                        {
                            "raw_node": {
                                "ident": "prelude",
                                "position": {
                                    "end_column": 24,
                                    "end_line": 1,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 17,
                                    "start_line": 1
                                },
                                "tree": "*"
                            },
                            "access_path": "[0].use.tree.path.tree.path",
                            "metadata": {
                                "position": {
                                    "end_column": 24,
                                    "end_line": 1,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 17,
                                    "start_line": 1
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "prelude"
                        }
                    ],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "anchor_lang"
                },
                {
                    "raw_node": {
                        "ident": "declare_id",
                        "position": {
                            "end_column": 10,
                            "end_line": 3,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                            "start_column": 0,
                            "start_line": 3
                        }
                    },
                    "access_path": "[1].macro.path.segments[0]",
                    "metadata": {
                        "position": {
                            "end_column": 10,
                            "end_line": 3,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                            "start_column": 0,
                            "start_line": 3
                        }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "declare_id"
                },
                {
                    "raw_node": {
                        "attrs": [
                            {
                                "meta": {
                                    "path": {
                                        "segments": [
                                            {
                                                "ident": "program",
                                                "position": {
                                                    "end_column": 9,
                                                    "end_line": 5,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 2,
                                                    "start_line": 5
                                                }
                                            }
                                        ]
                                    }
                                },
                                "style": "outer"
                            }
                        ],
                        "content": [
                            {
                                "use": {
                                    "tree": {
                                        "path": {
                                            "ident": "super",
                                            "position": {
                                                "end_column": 13,
                                                "end_line": 7,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                "start_column": 8,
                                                "start_line": 7
                                            },
                                            "tree": "*"
                                        }
                                    }
                                }
                            },
                            {
                                "fn": {
                                    "ident": "update",
                                    "inputs": [
                                        {
                                            "typed": {
                                                "pat": {
                                                    "ident": {
                                                        "ident": "ctx",
                                                        "position": {
                                                            "end_column": 29,
                                                            "end_line": 14,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 26,
                                                            "start_line": 14
                                                        }
                                                    }
                                                },
                                                "ty": {
                                                    "path": {
                                                        "segments": [
                                                            {
                                                                "arguments": {
                                                                    "angle_bracketed": {
                                                                        "args": [
                                                                            {
                                                                                "type": {
                                                                                    "path": {
                                                                                        "segments": [
                                                                                            {
                                                                                                "ident": "Update",
                                                                                                "position": {
                                                                                                    "end_column": 17,
                                                                                                    "end_line": 23,
                                                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                    "start_column": 11,
                                                                                                    "start_line": 23
                                                                                                }
                                                                                            }
                                                                                        ]
                                                                                    }
                                                                                }
                                                                            }
                                                                        ]
                                                                    }
                                                                },
                                                                "ident": "Context",
                                                                "position": {
                                                                    "end_column": 30,
                                                                    "end_line": 9,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 23,
                                                                    "start_line": 9
                                                                }
                                                            }
                                                        ]
                                                    }
                                                }
                                            }
                                        },
                                        {
                                            "typed": {
                                                "pat": {
                                                    "ident": {
                                                        "ident": "a",
                                                        "position": {
                                                            "end_column": 23,
                                                            "end_line": 16,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 22,
                                                            "start_line": 16
                                                        }
                                                    }
                                                },
                                                "ty": {
                                                    "path": {
                                                        "segments": [
                                                            {
                                                                "ident": "u64",
                                                                "position": {
                                                                    "end_column": 13,
                                                                    "end_line": 30,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 10,
                                                                    "start_line": 30
                                                                }
                                                            }
                                                        ]
                                                    }
                                                }
                                            }
                                        },
                                        {
                                            "typed": {
                                                "pat": {
                                                    "ident": {
                                                        "ident": "b",
                                                        "position": {
                                                            "end_column": 23,
                                                            "end_line": 17,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 22,
                                                            "start_line": 17
                                                        }
                                                    }
                                                },
                                                "ty": {
                                                    "path": {
                                                        "segments": [
                                                            {
                                                                "ident": "u64",
                                                                "position": {
                                                                    "end_column": 13,
                                                                    "end_line": 30,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 10,
                                                                    "start_line": 30
                                                                }
                                                            }
                                                        ]
                                                    }
                                                }
                                            }
                                        }
                                    ],
                                    "output": {
                                        "path": {
                                            "segments": [
                                                {
                                                    "ident": "ProgramResult",
                                                    "position": {
                                                        "end_column": 72,
                                                        "end_line": 9,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                        "start_column": 59,
                                                        "start_line": 9
                                                    }
                                                }
                                            ]
                                        }
                                    },
                                    "position": {
                                        "end_column": 17,
                                        "end_line": 9,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                        "start_column": 11,
                                        "start_line": 9
                                    },
                                    "stmts": [
                                        {
                                            "expr": [
                                                {
                                                    "if": {
                                                        "cond": {
                                                            "binary": {
                                                                "left": {
                                                                    "method_call": {
                                                                        "args": [],
                                                                        "method": "key",
                                                                        "receiver": {
                                                                            "field": {
                                                                                "base": {
                                                                                    "field": {
                                                                                        "base": {
                                                                                            "path": {
                                                                                                "segments": [
                                                                                                    {
                                                                                                        "ident": "ctx",
                                                                                                        "position": {
                                                                                                            "end_column": 29,
                                                                                                            "end_line": 14,
                                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                            "start_column": 26,
                                                                                                            "start_line": 14
                                                                                                        }
                                                                                                    }
                                                                                                ]
                                                                                            }
                                                                                        },
                                                                                        "ident": "accounts",
                                                                                        "position": {
                                                                                            "end_column": 38,
                                                                                            "end_line": 14,
                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                            "start_column": 30,
                                                                                            "start_line": 14
                                                                                        }
                                                                                    }
                                                                                },
                                                                                "ident": "user_a",
                                                                                "position": {
                                                                                    "end_column": 10,
                                                                                    "end_line": 24,
                                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                    "start_column": 4,
                                                                                    "start_line": 24
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                },
                                                                "op": "==",
                                                                "right": {
                                                                    "method_call": {
                                                                        "args": [],
                                                                        "method": "key",
                                                                        "receiver": {
                                                                            "field": {
                                                                                "base": {
                                                                                    "field": {
                                                                                        "base": {
                                                                                            "path": {
                                                                                                "segments": [
                                                                                                    {
                                                                                                        "ident": "ctx",
                                                                                                        "position": {
                                                                                                            "end_column": 29,
                                                                                                            "end_line": 14,
                                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                            "start_column": 26,
                                                                                                            "start_line": 14
                                                                                                        }
                                                                                                    }
                                                                                                ]
                                                                                            }
                                                                                        },
                                                                                        "ident": "accounts",
                                                                                        "position": {
                                                                                            "end_column": 38,
                                                                                            "end_line": 14,
                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                            "start_column": 30,
                                                                                            "start_line": 14
                                                                                        }
                                                                                    }
                                                                                },
                                                                                "ident": "user_b",
                                                                                "position": {
                                                                                    "end_column": 10,
                                                                                    "end_line": 25,
                                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                    "start_column": 4,
                                                                                    "start_line": 25
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        },
                                                        "then_branch": [
                                                            {
                                                                "expr": [
                                                                    {
                                                                        "return": {
                                                                            "expr": {
                                                                                "call": {
                                                                                    "args": [
                                                                                        {
                                                                                            "path": {
                                                                                                "segments": [
                                                                                                    {
                                                                                                        "ident": "ProgramError",
                                                                                                        "position": {
                                                                                                            "end_column": 35,
                                                                                                            "end_line": 11,
                                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                            "start_column": 23,
                                                                                                            "start_line": 11
                                                                                                        }
                                                                                                    },
                                                                                                    {
                                                                                                        "ident": "InvalidArgument",
                                                                                                        "position": {
                                                                                                            "end_column": 52,
                                                                                                            "end_line": 11,
                                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                            "start_column": 37,
                                                                                                            "start_line": 11
                                                                                                        }
                                                                                                    }
                                                                                                ]
                                                                                            }
                                                                                        }
                                                                                    ],
                                                                                    "func": {
                                                                                        "path": {
                                                                                            "segments": [
                                                                                                {
                                                                                                    "ident": "Err",
                                                                                                    "position": {
                                                                                                        "end_column": 22,
                                                                                                        "end_line": 11,
                                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                        "start_column": 19,
                                                                                                        "start_line": 11
                                                                                                    }
                                                                                                }
                                                                                            ]
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }, "False"
                                                                ]
                                                            }
                                                        ]
                                                    }
                                                }, "False"
                                            ]
                                        },
                                        {
                                            "let": {
                                                "init": {
                                                    "expr": {
                                                        "reference": {
                                                            "expr": {
                                                                "field": {
                                                                    "base": {
                                                                        "field": {
                                                                            "base": {
                                                                                "path": {
                                                                                    "segments": [
                                                                                        {
                                                                                            "ident": "ctx",
                                                                                            "position": {
                                                                                                "end_column": 29,
                                                                                                "end_line": 14,
                                                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                "start_column": 26,
                                                                                                "start_line": 14
                                                                                            }
                                                                                        }
                                                                                    ]
                                                                                }
                                                                            },
                                                                            "ident": "accounts",
                                                                            "position": {
                                                                                "end_column": 38,
                                                                                "end_line": 14,
                                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                "start_column": 30,
                                                                                "start_line": 14
                                                                            }
                                                                        }
                                                                    },
                                                                    "ident": "user_a",
                                                                    "position": {
                                                                        "end_column": 10,
                                                                        "end_line": 24,
                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                        "start_column": 4,
                                                                        "start_line": 24
                                                                    }
                                                                }
                                                            },
                                                            "mut": "True"
                                                        }
                                                    }
                                                },
                                                "pat": {
                                                    "ident": {
                                                        "ident": "user_a",
                                                        "position": {
                                                            "end_column": 10,
                                                            "end_line": 24,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 4,
                                                            "start_line": 24
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        {
                                            "let": {
                                                "init": {
                                                    "expr": {
                                                        "reference": {
                                                            "expr": {
                                                                "field": {
                                                                    "base": {
                                                                        "field": {
                                                                            "base": {
                                                                                "path": {
                                                                                    "segments": [
                                                                                        {
                                                                                            "ident": "ctx",
                                                                                            "position": {
                                                                                                "end_column": 29,
                                                                                                "end_line": 14,
                                                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                "start_column": 26,
                                                                                                "start_line": 14
                                                                                            }
                                                                                        }
                                                                                    ]
                                                                                }
                                                                            },
                                                                            "ident": "accounts",
                                                                            "position": {
                                                                                "end_column": 38,
                                                                                "end_line": 14,
                                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                "start_column": 30,
                                                                                "start_line": 14
                                                                            }
                                                                        }
                                                                    },
                                                                    "ident": "user_b",
                                                                    "position": {
                                                                        "end_column": 10,
                                                                        "end_line": 25,
                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                        "start_column": 4,
                                                                        "start_line": 25
                                                                    }
                                                                }
                                                            },
                                                            "mut": "True"
                                                        }
                                                    }
                                                },
                                                "pat": {
                                                    "ident": {
                                                        "ident": "user_b",
                                                        "position": {
                                                            "end_column": 10,
                                                            "end_line": 25,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 4,
                                                            "start_line": 25
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        {
                                            "expr": [
                                                {
                                                    "assign": {
                                                        "left": {
                                                            "field": {
                                                                "base": {
                                                                    "path": {
                                                                        "segments": [
                                                                            {
                                                                                "ident": "user_a",
                                                                                "position": {
                                                                                    "end_column": 10,
                                                                                    "end_line": 24,
                                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                    "start_column": 4,
                                                                                    "start_line": 24
                                                                                }
                                                                            }
                                                                        ]
                                                                    }
                                                                },
                                                                "ident": "data",
                                                                "position": {
                                                                    "end_column": 8,
                                                                    "end_line": 30,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 4,
                                                                    "start_line": 30
                                                                }
                                                            }
                                                        },
                                                        "right": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "a",
                                                                        "position": {
                                                                            "end_column": 23,
                                                                            "end_line": 16,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 22,
                                                                            "start_line": 16
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        }
                                                    }
                                                }, "True"
                                            ]
                                        },
                                        {
                                            "expr": [
                                                {
                                                    "assign": {
                                                        "left": {
                                                            "field": {
                                                                "base": {
                                                                    "path": {
                                                                        "segments": [
                                                                            {
                                                                                "ident": "user_b",
                                                                                "position": {
                                                                                    "end_column": 10,
                                                                                    "end_line": 25,
                                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                    "start_column": 4,
                                                                                    "start_line": 25
                                                                                }
                                                                            }
                                                                        ]
                                                                    }
                                                                },
                                                                "ident": "data",
                                                                "position": {
                                                                    "end_column": 8,
                                                                    "end_line": 30,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 4,
                                                                    "start_line": 30
                                                                }
                                                            }
                                                        },
                                                        "right": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "b",
                                                                        "position": {
                                                                            "end_column": 23,
                                                                            "end_line": 17,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 22,
                                                                            "start_line": 17
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        }
                                                    }
                                                }, "True"
                                            ]
                                        },
                                        {
                                            "expr": [
                                                {
                                                    "call": {
                                                        "args": [
                                                            {
                                                                "tuple": {
                                                                    "elems": []
                                                                }
                                                            }
                                                        ],
                                                        "func": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "Ok",
                                                                        "position": {
                                                                            "end_column": 10,
                                                                            "end_line": 18,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 8,
                                                                            "start_line": 18
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        }
                                                    }
                                                }, "False"
                                            ]
                                        }
                                    ],
                                    "vis": "pub"
                                }
                            }
                        ],
                        "ident": "duplicate_mutable_accounts_secure",
                        "position": {
                            "end_column": 41,
                            "end_line": 6,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                            "start_column": 8,
                            "start_line": 6
                        },
                        "vis": "pub"
                    },
                    "access_path": "[2].mod",
                    "metadata": {
                        "position": {
                            "end_column": 41,
                            "end_line": 6,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                            "start_column": 8,
                            "start_line": 6
                        }
                    },
                    "children": [
                        {
                            "raw_node": {
                                "ident": "program",
                                "position": {
                                    "end_column": 9,
                                    "end_line": 5,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 2,
                                    "start_line": 5
                                }
                            },
                            "access_path": "[2].mod.attrs[0].meta.path.segments[0]",
                            "metadata": {
                                "position": {
                                    "end_column": 9,
                                    "end_line": 5,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 2,
                                    "start_line": 5
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "program"
                        },
                        {
                            "raw_node": {
                                "ident": "super",
                                "position": {
                                    "end_column": 13,
                                    "end_line": 7,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 8,
                                    "start_line": 7
                                },
                                "tree": "*"
                            },
                            "access_path": "[2].mod.content[0].use.tree.path",
                            "metadata": {
                                "position": {
                                    "end_column": 13,
                                    "end_line": 7,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 8,
                                    "start_line": 7
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "super"
                        },
                        {
                            "raw_node": {
                                "ident": "update",
                                "inputs": [
                                    {
                                        "typed": {
                                            "pat": {
                                                "ident": {
                                                    "ident": "ctx",
                                                    "position": {
                                                        "end_column": 29,
                                                        "end_line": 14,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                        "start_column": 26,
                                                        "start_line": 14
                                                    }
                                                }
                                            },
                                            "ty": {
                                                "path": {
                                                    "segments": [
                                                        {
                                                            "arguments": {
                                                                "angle_bracketed": {
                                                                    "args": [
                                                                        {
                                                                            "type": {
                                                                                "path": {
                                                                                    "segments": [
                                                                                        {
                                                                                            "ident": "Update",
                                                                                            "position": {
                                                                                                "end_column": 17,
                                                                                                "end_line": 23,
                                                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                "start_column": 11,
                                                                                                "start_line": 23
                                                                                            }
                                                                                        }
                                                                                    ]
                                                                                }
                                                                            }
                                                                        }
                                                                    ]
                                                                }
                                                            },
                                                            "ident": "Context",
                                                            "position": {
                                                                "end_column": 30,
                                                                "end_line": 9,
                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                "start_column": 23,
                                                                "start_line": 9
                                                            }
                                                        }
                                                    ]
                                                }
                                            }
                                        }
                                    },
                                    {
                                        "typed": {
                                            "pat": {
                                                "ident": {
                                                    "ident": "a",
                                                    "position": {
                                                        "end_column": 23,
                                                        "end_line": 16,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                        "start_column": 22,
                                                        "start_line": 16
                                                    }
                                                }
                                            },
                                            "ty": {
                                                "path": {
                                                    "segments": [
                                                        {
                                                            "ident": "u64",
                                                            "position": {
                                                                "end_column": 13,
                                                                "end_line": 30,
                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                "start_column": 10,
                                                                "start_line": 30
                                                            }
                                                        }
                                                    ]
                                                }
                                            }
                                        }
                                    },
                                    {
                                        "typed": {
                                            "pat": {
                                                "ident": {
                                                    "ident": "b",
                                                    "position": {
                                                        "end_column": 23,
                                                        "end_line": 17,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                        "start_column": 22,
                                                        "start_line": 17
                                                    }
                                                }
                                            },
                                            "ty": {
                                                "path": {
                                                    "segments": [
                                                        {
                                                            "ident": "u64",
                                                            "position": {
                                                                "end_column": 13,
                                                                "end_line": 30,
                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                "start_column": 10,
                                                                "start_line": 30
                                                            }
                                                        }
                                                    ]
                                                }
                                            }
                                        }
                                    }
                                ],
                                "output": {
                                    "path": {
                                        "segments": [
                                            {
                                                "ident": "ProgramResult",
                                                "position": {
                                                    "end_column": 72,
                                                    "end_line": 9,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 59,
                                                    "start_line": 9
                                                }
                                            }
                                        ]
                                    }
                                },
                                "position": {
                                    "end_column": 17,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 11,
                                    "start_line": 9
                                },
                                "stmts": [
                                    {
                                        "expr": [
                                            {
                                                "if": {
                                                    "cond": {
                                                        "binary": {
                                                            "left": {
                                                                "method_call": {
                                                                    "args": [],
                                                                    "method": "key",
                                                                    "receiver": {
                                                                        "field": {
                                                                            "base": {
                                                                                "field": {
                                                                                    "base": {
                                                                                        "path": {
                                                                                            "segments": [
                                                                                                {
                                                                                                    "ident": "ctx",
                                                                                                    "position": {
                                                                                                        "end_column": 29,
                                                                                                        "end_line": 14,
                                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                        "start_column": 26,
                                                                                                        "start_line": 14
                                                                                                    }
                                                                                                }
                                                                                            ]
                                                                                        }
                                                                                    },
                                                                                    "ident": "accounts",
                                                                                    "position": {
                                                                                        "end_column": 38,
                                                                                        "end_line": 14,
                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                        "start_column": 30,
                                                                                        "start_line": 14
                                                                                    }
                                                                                }
                                                                            },
                                                                            "ident": "user_a",
                                                                            "position": {
                                                                                "end_column": 10,
                                                                                "end_line": 24,
                                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                "start_column": 4,
                                                                                "start_line": 24
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            },
                                                            "op": "==",
                                                            "right": {
                                                                "method_call": {
                                                                    "args": [],
                                                                    "method": "key",
                                                                    "receiver": {
                                                                        "field": {
                                                                            "base": {
                                                                                "field": {
                                                                                    "base": {
                                                                                        "path": {
                                                                                            "segments": [
                                                                                                {
                                                                                                    "ident": "ctx",
                                                                                                    "position": {
                                                                                                        "end_column": 29,
                                                                                                        "end_line": 14,
                                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                        "start_column": 26,
                                                                                                        "start_line": 14
                                                                                                    }
                                                                                                }
                                                                                            ]
                                                                                        }
                                                                                    },
                                                                                    "ident": "accounts",
                                                                                    "position": {
                                                                                        "end_column": 38,
                                                                                        "end_line": 14,
                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                        "start_column": 30,
                                                                                        "start_line": 14
                                                                                    }
                                                                                }
                                                                            },
                                                                            "ident": "user_b",
                                                                            "position": {
                                                                                "end_column": 10,
                                                                                "end_line": 25,
                                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                "start_column": 4,
                                                                                "start_line": 25
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    "then_branch": [
                                                        {
                                                            "expr": [
                                                                {
                                                                    "return": {
                                                                        "expr": {
                                                                            "call": {
                                                                                "args": [
                                                                                    {
                                                                                        "path": {
                                                                                            "segments": [
                                                                                                {
                                                                                                    "ident": "ProgramError",
                                                                                                    "position": {
                                                                                                        "end_column": 35,
                                                                                                        "end_line": 11,
                                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                        "start_column": 23,
                                                                                                        "start_line": 11
                                                                                                    }
                                                                                                },
                                                                                                {
                                                                                                    "ident": "InvalidArgument",
                                                                                                    "position": {
                                                                                                        "end_column": 52,
                                                                                                        "end_line": 11,
                                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                        "start_column": 37,
                                                                                                        "start_line": 11
                                                                                                    }
                                                                                                }
                                                                                            ]
                                                                                        }
                                                                                    }
                                                                                ],
                                                                                "func": {
                                                                                    "path": {
                                                                                        "segments": [
                                                                                            {
                                                                                                "ident": "Err",
                                                                                                "position": {
                                                                                                    "end_column": 22,
                                                                                                    "end_line": 11,
                                                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                                    "start_column": 19,
                                                                                                    "start_line": 11
                                                                                                }
                                                                                            }
                                                                                        ]
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }, "False"
                                                            ]
                                                        }
                                                    ]
                                                }
                                            }, "False"
                                        ]
                                    },
                                    {
                                        "let": {
                                            "init": {
                                                "expr": {
                                                    "reference": {
                                                        "expr": {
                                                            "field": {
                                                                "base": {
                                                                    "field": {
                                                                        "base": {
                                                                            "path": {
                                                                                "segments": [
                                                                                    {
                                                                                        "ident": "ctx",
                                                                                        "position": {
                                                                                            "end_column": 29,
                                                                                            "end_line": 14,
                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                            "start_column": 26,
                                                                                            "start_line": 14
                                                                                        }
                                                                                    }
                                                                                ]
                                                                            }
                                                                        },
                                                                        "ident": "accounts",
                                                                        "position": {
                                                                            "end_column": 38,
                                                                            "end_line": 14,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 30,
                                                                            "start_line": 14
                                                                        }
                                                                    }
                                                                },
                                                                "ident": "user_a",
                                                                "position": {
                                                                    "end_column": 10,
                                                                    "end_line": 24,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 4,
                                                                    "start_line": 24
                                                                }
                                                            }
                                                        },
                                                        "mut": "True"
                                                    }
                                                }
                                            },
                                            "pat": {
                                                "ident": {
                                                    "ident": "user_a",
                                                    "position": {
                                                        "end_column": 10,
                                                        "end_line": 24,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                        "start_column": 4,
                                                        "start_line": 24
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    {
                                        "let": {
                                            "init": {
                                                "expr": {
                                                    "reference": {
                                                        "expr": {
                                                            "field": {
                                                                "base": {
                                                                    "field": {
                                                                        "base": {
                                                                            "path": {
                                                                                "segments": [
                                                                                    {
                                                                                        "ident": "ctx",
                                                                                        "position": {
                                                                                            "end_column": 29,
                                                                                            "end_line": 14,
                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                            "start_column": 26,
                                                                                            "start_line": 14
                                                                                        }
                                                                                    }
                                                                                ]
                                                                            }
                                                                        },
                                                                        "ident": "accounts",
                                                                        "position": {
                                                                            "end_column": 38,
                                                                            "end_line": 14,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 30,
                                                                            "start_line": 14
                                                                        }
                                                                    }
                                                                },
                                                                "ident": "user_b",
                                                                "position": {
                                                                    "end_column": 10,
                                                                    "end_line": 25,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 4,
                                                                    "start_line": 25
                                                                }
                                                            }
                                                        },
                                                        "mut": "True"
                                                    }
                                                }
                                            },
                                            "pat": {
                                                "ident": {
                                                    "ident": "user_b",
                                                    "position": {
                                                        "end_column": 10,
                                                        "end_line": 25,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                        "start_column": 4,
                                                        "start_line": 25
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    {
                                        "expr": [
                                            {
                                                "assign": {
                                                    "left": {
                                                        "field": {
                                                            "base": {
                                                                "path": {
                                                                    "segments": [
                                                                        {
                                                                            "ident": "user_a",
                                                                            "position": {
                                                                                "end_column": 10,
                                                                                "end_line": 24,
                                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                "start_column": 4,
                                                                                "start_line": 24
                                                                            }
                                                                        }
                                                                    ]
                                                                }
                                                            },
                                                            "ident": "data",
                                                            "position": {
                                                                "end_column": 8,
                                                                "end_line": 30,
                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                "start_column": 4,
                                                                "start_line": 30
                                                            }
                                                        }
                                                    },
                                                    "right": {
                                                        "path": {
                                                            "segments": [
                                                                {
                                                                    "ident": "a",
                                                                    "position": {
                                                                        "end_column": 23,
                                                                        "end_line": 16,
                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                        "start_column": 22,
                                                                        "start_line": 16
                                                                    }
                                                                }
                                                            ]
                                                        }
                                                    }
                                                }
                                            }, "True"
                                        ]
                                    },
                                    {
                                        "expr": [
                                            {
                                                "assign": {
                                                    "left": {
                                                        "field": {
                                                            "base": {
                                                                "path": {
                                                                    "segments": [
                                                                        {
                                                                            "ident": "user_b",
                                                                            "position": {
                                                                                "end_column": 10,
                                                                                "end_line": 25,
                                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                                "start_column": 4,
                                                                                "start_line": 25
                                                                            }
                                                                        }
                                                                    ]
                                                                }
                                                            },
                                                            "ident": "data",
                                                            "position": {
                                                                "end_column": 8,
                                                                "end_line": 30,
                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                "start_column": 4,
                                                                "start_line": 30
                                                            }
                                                        }
                                                    },
                                                    "right": {
                                                        "path": {
                                                            "segments": [
                                                                {
                                                                    "ident": "b",
                                                                    "position": {
                                                                        "end_column": 23,
                                                                        "end_line": 17,
                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                        "start_column": 22,
                                                                        "start_line": 17
                                                                    }
                                                                }
                                                            ]
                                                        }
                                                    }
                                                }
                                            }, "True"
                                        ]
                                    },
                                    {
                                        "expr": [
                                            {
                                                "call": {
                                                    "args": [
                                                        {
                                                            "tuple": {
                                                                "elems": []
                                                            }
                                                        }
                                                    ],
                                                    "func": {
                                                        "path": {
                                                            "segments": [
                                                                {
                                                                    "ident": "Ok",
                                                                    "position": {
                                                                        "end_column": 10,
                                                                        "end_line": 18,
                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                        "start_column": 8,
                                                                        "start_line": 18
                                                                    }
                                                                }
                                                            ]
                                                        }
                                                    }
                                                }
                                            }, "False"
                                        ]
                                    }
                                ],
                                "vis": "pub"
                            },
                            "access_path": "[2].mod.content[1].fn",
                            "metadata": {
                                "position": {
                                    "end_column": 17,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 11,
                                    "start_line": 9
                                }
                            },
                            "children": [
                                {
                                    "raw_node": {
                                        "base": {
                                            "field": {
                                                "base": {
                                                    "path": {
                                                        "segments": [
                                                            {
                                                                "ident": "ctx",
                                                                "position": {
                                                                    "end_column": 29,
                                                                    "end_line": 14,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 26,
                                                                    "start_line": 14
                                                                }
                                                            }
                                                        ]
                                                    }
                                                },
                                                "ident": "accounts",
                                                "position": {
                                                    "end_column": 38,
                                                    "end_line": 14,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 30,
                                                    "start_line": 14
                                                }
                                            }
                                        },
                                        "ident": "user_a",
                                        "position": {
                                            "end_column": 10,
                                            "end_line": 24,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 4,
                                            "start_line": 24
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[1].let.init.expr.reference.expr.field",
                                    "metadata": {
                                        "mut": "True"
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "user_a"
                                },
                                {
                                    "raw_node": {
                                        "base": {
                                            "field": {
                                                "base": {
                                                    "path": {
                                                        "segments": [
                                                            {
                                                                "ident": "ctx",
                                                                "position": {
                                                                    "end_column": 29,
                                                                    "end_line": 14,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 26,
                                                                    "start_line": 14
                                                                }
                                                            }
                                                        ]
                                                    }
                                                },
                                                "ident": "accounts",
                                                "position": {
                                                    "end_column": 38,
                                                    "end_line": 14,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 30,
                                                    "start_line": 14
                                                }
                                            }
                                        },
                                        "ident": "user_b",
                                        "position": {
                                            "end_column": 10,
                                            "end_line": 25,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 4,
                                            "start_line": 25
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[2].let.init.expr.reference.expr.field",
                                    "metadata": {
                                        "mut": "True"
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "user_b"
                                },
                                {
                                    "raw_node": {
                                        "ident": {
                                            "ident": "ctx",
                                            "position": {
                                                "end_column": 29,
                                                "end_line": 14,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                "start_column": 26,
                                                "start_line": 14
                                            }
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.inputs[0].typed.pat",
                                    "metadata": {},
                                    "children": [
                                        {
                                            "raw_node": {
                                                "ident": "ctx",
                                                "position": {
                                                    "end_column": 29,
                                                    "end_line": 14,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 26,
                                                    "start_line": 14
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.inputs[0].typed.pat.ident",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 29,
                                                    "end_line": 14,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 26,
                                                    "start_line": 14
                                                }
                                            },
                                            "children": [],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "ctx"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "ctx"
                                },
                                {
                                    "raw_node": {
                                        "arguments": {
                                            "angle_bracketed": {
                                                "args": [
                                                    {
                                                        "type": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "Update",
                                                                        "position": {
                                                                            "end_column": 17,
                                                                            "end_line": 23,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 11,
                                                                            "start_line": 23
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        }
                                                    }
                                                ]
                                            }
                                        },
                                        "ident": "Context",
                                        "position": {
                                            "end_column": 30,
                                            "end_line": 9,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 23,
                                            "start_line": 9
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.inputs[0].typed.ty.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 30,
                                            "end_line": 9,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 23,
                                            "start_line": 9
                                        }
                                    },
                                    "children": [
                                        {
                                            "raw_node": {
                                                "ident": "Update",
                                                "position": {
                                                    "end_column": 17,
                                                    "end_line": 23,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 11,
                                                    "start_line": 23
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.inputs[0].typed.ty.path.segments[0].arguments.angle_bracketed.args[0].type.path.segments[0]",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 17,
                                                    "end_line": 23,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 11,
                                                    "start_line": 23
                                                }
                                            },
                                            "children": [],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "Update"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "Context"
                                },
                                {
                                    "raw_node": {
                                        "ident": {
                                            "ident": "a",
                                            "position": {
                                                "end_column": 23,
                                                "end_line": 16,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                "start_column": 22,
                                                "start_line": 16
                                            }
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.inputs[1].typed.pat",
                                    "metadata": {},
                                    "children": [
                                        {
                                            "raw_node": {
                                                "ident": "a",
                                                "position": {
                                                    "end_column": 23,
                                                    "end_line": 16,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 22,
                                                    "start_line": 16
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.inputs[1].typed.pat.ident",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 23,
                                                    "end_line": 16,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 22,
                                                    "start_line": 16
                                                }
                                            },
                                            "children": [],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "a"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "a"
                                },
                                {
                                    "raw_node": {
                                        "ident": "u64",
                                        "position": {
                                            "end_column": 13,
                                            "end_line": 30,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 10,
                                            "start_line": 30
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.inputs[1].typed.ty.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 13,
                                            "end_line": 30,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 10,
                                            "start_line": 30
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "u64"
                                },
                                {
                                    "raw_node": {
                                        "ident": {
                                            "ident": "b",
                                            "position": {
                                                "end_column": 23,
                                                "end_line": 17,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                "start_column": 22,
                                                "start_line": 17
                                            }
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.inputs[2].typed.pat",
                                    "metadata": {},
                                    "children": [
                                        {
                                            "raw_node": {
                                                "ident": "b",
                                                "position": {
                                                    "end_column": 23,
                                                    "end_line": 17,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 22,
                                                    "start_line": 17
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.inputs[2].typed.pat.ident",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 23,
                                                    "end_line": 17,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 22,
                                                    "start_line": 17
                                                }
                                            },
                                            "children": [],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "b"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "b"
                                },
                                {
                                    "raw_node": {
                                        "ident": "u64",
                                        "position": {
                                            "end_column": 13,
                                            "end_line": 30,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 10,
                                            "start_line": 30
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.inputs[2].typed.ty.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 13,
                                            "end_line": 30,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 10,
                                            "start_line": 30
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "u64"
                                },
                                {
                                    "raw_node": {
                                        "ident": "ProgramResult",
                                        "position": {
                                            "end_column": 72,
                                            "end_line": 9,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 59,
                                            "start_line": 9
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.output.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 72,
                                            "end_line": 9,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 59,
                                            "start_line": 9
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "ProgramResult"
                                },
                                {
                                    "raw_node": {
                                        "args": [],
                                        "method": "key",
                                        "receiver": {
                                            "field": {
                                                "base": {
                                                    "field": {
                                                        "base": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "ctx",
                                                                        "position": {
                                                                            "end_column": 29,
                                                                            "end_line": 14,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 26,
                                                                            "start_line": 14
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        },
                                                        "ident": "accounts",
                                                        "position": {
                                                            "end_column": 38,
                                                            "end_line": 14,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 30,
                                                            "start_line": 14
                                                        }
                                                    }
                                                },
                                                "ident": "user_a",
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 24,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 24
                                                }
                                            }
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.left.method_call",
                                    "metadata": {},
                                    "children": [
                                        {
                                            "raw_node": {
                                                "base": {
                                                    "field": {
                                                        "base": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "ctx",
                                                                        "position": {
                                                                            "end_column": 29,
                                                                            "end_line": 14,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 26,
                                                                            "start_line": 14
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        },
                                                        "ident": "accounts",
                                                        "position": {
                                                            "end_column": 38,
                                                            "end_line": 14,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 30,
                                                            "start_line": 14
                                                        }
                                                    }
                                                },
                                                "ident": "user_a",
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 24,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 24
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.left.method_call.receiver.field",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 24,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 24
                                                }
                                            },
                                            "children": [
                                                {
                                                    "raw_node": {
                                                        "base": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "ctx",
                                                                        "position": {
                                                                            "end_column": 29,
                                                                            "end_line": 14,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 26,
                                                                            "start_line": 14
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        },
                                                        "ident": "accounts",
                                                        "position": {
                                                            "end_column": 38,
                                                            "end_line": 14,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 30,
                                                            "start_line": 14
                                                        }
                                                    },
                                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.left.method_call.receiver.field.base.field",
                                                    "metadata": {
                                                        "position": {
                                                            "end_column": 38,
                                                            "end_line": 14,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 30,
                                                            "start_line": 14
                                                        }
                                                    },
                                                    "children": [
                                                        {
                                                            "raw_node": {
                                                                "ident": "ctx",
                                                                "position": {
                                                                    "end_column": 29,
                                                                    "end_line": 14,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 26,
                                                                    "start_line": 14
                                                                }
                                                            },
                                                            "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.left.method_call.receiver.field.base.field.base.path.segments[0]",
                                                            "metadata": {
                                                                "position": {
                                                                    "end_column": 29,
                                                                    "end_line": 14,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 26,
                                                                    "start_line": 14
                                                                }
                                                            },
                                                            "children": [],
                                                            "parent": "{...}",
                                                            "root": "False",
                                                            "args": [],
                                                            "ident": "ctx"
                                                        }
                                                    ],
                                                    "parent": "{...}",
                                                    "root": "False",
                                                    "args": [],
                                                    "ident": "accounts"
                                                }
                                            ],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "user_a"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "key"
                                },
                                {
                                    "raw_node": {
                                        "args": [],
                                        "method": "key",
                                        "receiver": {
                                            "field": {
                                                "base": {
                                                    "field": {
                                                        "base": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "ctx",
                                                                        "position": {
                                                                            "end_column": 29,
                                                                            "end_line": 14,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 26,
                                                                            "start_line": 14
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        },
                                                        "ident": "accounts",
                                                        "position": {
                                                            "end_column": 38,
                                                            "end_line": 14,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 30,
                                                            "start_line": 14
                                                        }
                                                    }
                                                },
                                                "ident": "user_b",
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 25,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 25
                                                }
                                            }
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.right.method_call",
                                    "metadata": {},
                                    "children": [
                                        {
                                            "raw_node": {
                                                "base": {
                                                    "field": {
                                                        "base": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "ctx",
                                                                        "position": {
                                                                            "end_column": 29,
                                                                            "end_line": 14,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 26,
                                                                            "start_line": 14
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        },
                                                        "ident": "accounts",
                                                        "position": {
                                                            "end_column": 38,
                                                            "end_line": 14,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 30,
                                                            "start_line": 14
                                                        }
                                                    }
                                                },
                                                "ident": "user_b",
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 25,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 25
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.right.method_call.receiver.field",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 25,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 25
                                                }
                                            },
                                            "children": [
                                                {
                                                    "raw_node": {
                                                        "base": {
                                                            "path": {
                                                                "segments": [
                                                                    {
                                                                        "ident": "ctx",
                                                                        "position": {
                                                                            "end_column": 29,
                                                                            "end_line": 14,
                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                            "start_column": 26,
                                                                            "start_line": 14
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        },
                                                        "ident": "accounts",
                                                        "position": {
                                                            "end_column": 38,
                                                            "end_line": 14,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 30,
                                                            "start_line": 14
                                                        }
                                                    },
                                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.right.method_call.receiver.field.base.field",
                                                    "metadata": {
                                                        "position": {
                                                            "end_column": 38,
                                                            "end_line": 14,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 30,
                                                            "start_line": 14
                                                        }
                                                    },
                                                    "children": [
                                                        {
                                                            "raw_node": {
                                                                "ident": "ctx",
                                                                "position": {
                                                                    "end_column": 29,
                                                                    "end_line": 14,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 26,
                                                                    "start_line": 14
                                                                }
                                                            },
                                                            "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.cond.binary.right.method_call.receiver.field.base.field.base.path.segments[0]",
                                                            "metadata": {
                                                                "position": {
                                                                    "end_column": 29,
                                                                    "end_line": 14,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 26,
                                                                    "start_line": 14
                                                                }
                                                            },
                                                            "children": [],
                                                            "parent": "{...}",
                                                            "root": "False",
                                                            "args": [],
                                                            "ident": "ctx"
                                                        }
                                                    ],
                                                    "parent": "{...}",
                                                    "root": "False",
                                                    "args": [],
                                                    "ident": "accounts"
                                                }
                                            ],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "user_b"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "key"
                                },
                                {
                                    "raw_node": {
                                        "ident": "ProgramError",
                                        "position": {
                                            "end_column": 35,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 23,
                                            "start_line": 11
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.then_branch[0].expr[0].return.expr.call.args[0].path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 35,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 23,
                                            "start_line": 11
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "ProgramError"
                                },
                                {
                                    "raw_node": {
                                        "ident": "InvalidArgument",
                                        "position": {
                                            "end_column": 52,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 37,
                                            "start_line": 11
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.then_branch[0].expr[0].return.expr.call.args[0].path.segments[1]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 52,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 37,
                                            "start_line": 11
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "InvalidArgument"
                                },
                                {
                                    "raw_node": {
                                        "ident": "Err",
                                        "position": {
                                            "end_column": 22,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 19,
                                            "start_line": 11
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[0].expr[0].if.then_branch[0].expr[0].return.expr.call.func.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 22,
                                            "end_line": 11,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 19,
                                            "start_line": 11
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "Err"
                                },
                                {
                                    "raw_node": {
                                        "base": {
                                            "field": {
                                                "base": {
                                                    "path": {
                                                        "segments": [
                                                            {
                                                                "ident": "ctx",
                                                                "position": {
                                                                    "end_column": 29,
                                                                    "end_line": 14,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 26,
                                                                    "start_line": 14
                                                                }
                                                            }
                                                        ]
                                                    }
                                                },
                                                "ident": "accounts",
                                                "position": {
                                                    "end_column": 38,
                                                    "end_line": 14,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 30,
                                                    "start_line": 14
                                                }
                                            }
                                        },
                                        "ident": "user_a",
                                        "position": {
                                            "end_column": 10,
                                            "end_line": 24,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 4,
                                            "start_line": 24
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[1].let.init.expr.reference.expr.field",
                                    "metadata": {
                                        "mut": "True"
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "user_a"
                                },
                                {
                                    "raw_node": {
                                        "ident": {
                                            "ident": "user_a",
                                            "position": {
                                                "end_column": 10,
                                                "end_line": 24,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                "start_column": 4,
                                                "start_line": 24
                                            }
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[1].let.pat",
                                    "metadata": {},
                                    "children": [
                                        {
                                            "raw_node": {
                                                "ident": "user_a",
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 24,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 24
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.stmts[1].let.pat.ident",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 24,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 24
                                                }
                                            },
                                            "children": [],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "user_a"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "user_a"
                                },
                                {
                                    "raw_node": {
                                        "base": {
                                            "field": {
                                                "base": {
                                                    "path": {
                                                        "segments": [
                                                            {
                                                                "ident": "ctx",
                                                                "position": {
                                                                    "end_column": 29,
                                                                    "end_line": 14,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                                    "start_column": 26,
                                                                    "start_line": 14
                                                                }
                                                            }
                                                        ]
                                                    }
                                                },
                                                "ident": "accounts",
                                                "position": {
                                                    "end_column": 38,
                                                    "end_line": 14,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 30,
                                                    "start_line": 14
                                                }
                                            }
                                        },
                                        "ident": "user_b",
                                        "position": {
                                            "end_column": 10,
                                            "end_line": 25,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 4,
                                            "start_line": 25
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[2].let.init.expr.reference.expr.field",
                                    "metadata": {
                                        "mut": "True"
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "user_b"
                                },
                                {
                                    "raw_node": {
                                        "ident": {
                                            "ident": "user_b",
                                            "position": {
                                                "end_column": 10,
                                                "end_line": 25,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                "start_column": 4,
                                                "start_line": 25
                                            }
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[2].let.pat",
                                    "metadata": {},
                                    "children": [
                                        {
                                            "raw_node": {
                                                "ident": "user_b",
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 25,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 25
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.stmts[2].let.pat.ident",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 25,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 25
                                                }
                                            },
                                            "children": [],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "user_b"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "user_b"
                                },
                                {
                                    "raw_node": {
                                        "base": {
                                            "path": {
                                                "segments": [
                                                    {
                                                        "ident": "user_a",
                                                        "position": {
                                                            "end_column": 10,
                                                            "end_line": 24,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 4,
                                                            "start_line": 24
                                                        }
                                                    }
                                                ]
                                            }
                                        },
                                        "ident": "data",
                                        "position": {
                                            "end_column": 8,
                                            "end_line": 30,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 4,
                                            "start_line": 30
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[3].expr[0].assign.left.field",
                                    "metadata": {
                                        "position": {
                                            "end_column": 8,
                                            "end_line": 30,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 4,
                                            "start_line": 30
                                        }
                                    },
                                    "children": [
                                        {
                                            "raw_node": {
                                                "ident": "user_a",
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 24,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 24
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.stmts[3].expr[0].assign.left.field.base.path.segments[0]",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 24,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 24
                                                }
                                            },
                                            "children": [],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "user_a"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "data"
                                },
                                {
                                    "raw_node": {
                                        "ident": "a",
                                        "position": {
                                            "end_column": 23,
                                            "end_line": 16,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 22,
                                            "start_line": 16
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[3].expr[0].assign.right.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 23,
                                            "end_line": 16,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 22,
                                            "start_line": 16
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "a"
                                },
                                {
                                    "raw_node": {
                                        "base": {
                                            "path": {
                                                "segments": [
                                                    {
                                                        "ident": "user_b",
                                                        "position": {
                                                            "end_column": 10,
                                                            "end_line": 25,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                            "start_column": 4,
                                                            "start_line": 25
                                                        }
                                                    }
                                                ]
                                            }
                                        },
                                        "ident": "data",
                                        "position": {
                                            "end_column": 8,
                                            "end_line": 30,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 4,
                                            "start_line": 30
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[4].expr[0].assign.left.field",
                                    "metadata": {
                                        "position": {
                                            "end_column": 8,
                                            "end_line": 30,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 4,
                                            "start_line": 30
                                        }
                                    },
                                    "children": [
                                        {
                                            "raw_node": {
                                                "ident": "user_b",
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 25,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 25
                                                }
                                            },
                                            "access_path": "[2].mod.content[1].fn.stmts[4].expr[0].assign.left.field.base.path.segments[0]",
                                            "metadata": {
                                                "position": {
                                                    "end_column": 10,
                                                    "end_line": 25,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 25
                                                }
                                            },
                                            "children": [],
                                            "parent": "{...}",
                                            "root": "False",
                                            "args": [],
                                            "ident": "user_b"
                                        }
                                    ],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "data"
                                },
                                {
                                    "raw_node": {
                                        "ident": "b",
                                        "position": {
                                            "end_column": 23,
                                            "end_line": 17,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 22,
                                            "start_line": 17
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[4].expr[0].assign.right.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 23,
                                            "end_line": 17,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 22,
                                            "start_line": 17
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "b"
                                },
                                {
                                    "raw_node": {
                                        "ident": "Ok",
                                        "position": {
                                            "end_column": 10,
                                            "end_line": 18,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 8,
                                            "start_line": 18
                                        }
                                    },
                                    "access_path": "[2].mod.content[1].fn.stmts[5].expr[0].call.func.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 10,
                                            "end_line": 18,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 8,
                                            "start_line": 18
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "Ok"
                                }
                            ],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "update"
                        }
                    ],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "duplicate_mutable_accounts_secure"
                },
                "{...}",
                {
                    "raw_node": {
                        "attrs": [
                            {
                                "meta": {
                                    "path": {
                                        "segments": [
                                            {
                                                "ident": "account",
                                                "position": {
                                                    "end_column": 9,
                                                    "end_line": 28,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 2,
                                                    "start_line": 28
                                                }
                                            }
                                        ]
                                    }
                                },
                                "style": "outer"
                            }
                        ],
                        "fields": {
                            "named": [
                                {
                                    "colon_token": "True",
                                    "ident": "data",
                                    "position": {
                                        "end_column": 8,
                                        "end_line": 30,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                        "start_column": 4,
                                        "start_line": 30
                                    },
                                    "ty": {
                                        "path": {
                                            "segments": [
                                                {
                                                    "ident": "u64",
                                                    "position": {
                                                        "end_column": 13,
                                                        "end_line": 30,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                        "start_column": 10,
                                                        "start_line": 30
                                                    }
                                                }
                                            ]
                                        }
                                    }
                                }
                            ]
                        },
                        "ident": "User",
                        "position": {
                            "end_column": 15,
                            "end_line": 29,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 29
                        },
                        "vis": "pub"
                    },
                    "access_path": "[4].struct",
                    "metadata": {
                        "position": {
                            "end_column": 15,
                            "end_line": 29,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 29
                        }
                    },
                    "children": [
                        {
                            "raw_node": {
                                "ident": "account",
                                "position": {
                                    "end_column": 9,
                                    "end_line": 28,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 2,
                                    "start_line": 28
                                }
                            },
                            "access_path": "[4].struct.attrs[0].meta.path.segments[0]",
                            "metadata": {
                                "position": {
                                    "end_column": 9,
                                    "end_line": 28,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 2,
                                    "start_line": 28
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "account"
                        },
                        {
                            "raw_node": {
                                "colon_token": "True",
                                "ident": "data",
                                "position": {
                                    "end_column": 8,
                                    "end_line": 30,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 4,
                                    "start_line": 30
                                },
                                "ty": {
                                    "path": {
                                        "segments": [
                                            {
                                                "ident": "u64",
                                                "position": {
                                                    "end_column": 13,
                                                    "end_line": 30,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                                    "start_column": 10,
                                                    "start_line": 30
                                                }
                                            }
                                        ]
                                    }
                                }
                            },
                            "access_path": "[4].struct.fields.named[0]",
                            "metadata": {
                                "position": {
                                    "end_column": 8,
                                    "end_line": 30,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                    "start_column": 4,
                                    "start_line": 30
                                }
                            },
                            "children": [
                                {
                                    "raw_node": {
                                        "ident": "u64",
                                        "position": {
                                            "end_column": 13,
                                            "end_line": 30,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 10,
                                            "start_line": 30
                                        }
                                    },
                                    "access_path": "[4].struct.fields.named[0].ty.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 13,
                                            "end_line": 30,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/6-duplicate-mutable-accounts/secure/src/lib.rs",
                                            "start_column": 10,
                                            "start_line": 30
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "u64"
                                }
                            ],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "data"
                        }
                    ],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "User"
                }
            ],
            "parent": {
                "raw_node": {},
                "access_path": "EMPTY_ACCESS_PATH",
                "metadata": {},
                "children": [],
                "parent": {},
                "root": "False",
                "args": []
            },
            "root": "False",
            "args": [],
            "ident": "EMPTY_IDENT"
        },
        "root": "False",
        "args": [],
        "ident": "Update"
    },
    "root": "False",
    "args": [],
    "ident": "derive"
}

AST5 = {
    "raw_node": {},
    "access_path": "root",
    "metadata": {},
    "children": [
        {
            "raw_node": {
                "ident": "anchor_lang",
                "position": {
                    "end_column": 15,
                    "end_line": 2,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 2
                },
                "tree": {
                    "path": {
                        "ident": "prelude",
                        "position": {
                            "end_column": 24,
                            "end_line": 1,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 17,
                            "start_line": 1
                        },
                        "tree": "*"
                    }
                }
            },
            "access_path": "[0].use.tree.path",
            "metadata": {
                "position": {
                    "end_column": 15,
                    "end_line": 2,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 2
                }
            },
            "children": [
                {
                    "raw_node": {
                        "ident": "prelude",
                        "position": {
                            "end_column": 24,
                            "end_line": 1,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 17,
                            "start_line": 1
                        },
                        "tree": "*"
                    },
                    "access_path": "[0].use.tree.path.tree.path",
                    "metadata": {
                        "position": {
                            "end_column": 24,
                            "end_line": 1,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 17,
                            "start_line": 1
                        }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "prelude"
                }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "anchor_lang"
        },
        {
            "raw_node": {
                "ident": "anchor_lang",
                "position": {
                    "end_column": 15,
                    "end_line": 2,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 2
                },
                "tree": {
                    "path": {
                        "ident": "solana_program",
                        "position": {
                            "end_column": 31,
                            "end_line": 2,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 17,
                            "start_line": 2
                        },
                        "tree": {
                            "ident": "sysvar",
                            "position": {
                                "end_column": 39,
                                "end_line": 2,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                "start_column": 33,
                                "start_line": 2
                            }
                        }
                    }
                }
            },
            "access_path": "[1].use.tree.path",
            "metadata": {
                "position": {
                    "end_column": 15,
                    "end_line": 2,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                    "start_column": 4,
                    "start_line": 2
                }
            },
            "children": [
                {
                    "raw_node": {
                        "ident": "solana_program",
                        "position": {
                            "end_column": 31,
                            "end_line": 2,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 17,
                            "start_line": 2
                        },
                        "tree": {
                            "ident": "sysvar",
                            "position": {
                                "end_column": 39,
                                "end_line": 2,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                "start_column": 33,
                                "start_line": 2
                            }
                        }
                    },
                    "access_path": "[1].use.tree.path.tree.path",
                    "metadata": {
                        "position": {
                            "end_column": 31,
                            "end_line": 2,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 17,
                            "start_line": 2
                        }
                    },
                    "children": [
                        {
                            "raw_node": {
                                "ident": "sysvar",
                                "position": {
                                    "end_column": 39,
                                    "end_line": 2,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 33,
                                    "start_line": 2
                                }
                            },
                            "access_path": "[1].use.tree.path.tree.path.tree",
                            "metadata": {
                                "position": {
                                    "end_column": 39,
                                    "end_line": 2,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 33,
                                    "start_line": 2
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "sysvar"
                        }
                    ],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "solana_program"
                }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "anchor_lang"
        },
        {
            "raw_node": {
                "ident": "declare_id",
                "position": {
                    "end_column": 10,
                    "end_line": 3,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                    "start_column": 0,
                    "start_line": 3
                }
            },
            "access_path": "[2].macro.path.segments[0]",
            "metadata": {
                "position": {
                    "end_column": 10,
                    "end_line": 3,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                    "start_column": 0,
                    "start_line": 3
                }
            },
            "children": [],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "declare_id"
        },
        {
            "raw_node": {
                "attrs": [
                    {
                        "meta": {
                            "path": {
                                "segments": [
                                    {
                                        "ident": "program",
                                        "position": {
                                            "end_column": 9,
                                            "end_line": 5,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                            "start_column": 2,
                                            "start_line": 5
                                        }
                                    }
                                ]
                            }
                        },
                        "style": "outer"
                    }
                ],
                "content": [
                    {
                        "use": {
                            "tree": {
                                "path": {
                                    "ident": "super",
                                    "position": {
                                        "end_column": 13,
                                        "end_line": 7,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                        "start_column": 8,
                                        "start_line": 7
                                    },
                                    "tree": "*"
                                }
                            }
                        }
                    },
                    {
                        "fn": {
                            "ident": "check_sysvar_address",
                            "inputs": [
                                {
                                    "typed": {
                                        "pat": {
                                            "ident": {
                                                "ident": "ctx",
                                                "position": {
                                                    "end_column": 35,
                                                    "end_line": 9,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                    "start_column": 32,
                                                    "start_line": 9
                                                }
                                            }
                                        },
                                        "ty": {
                                            "path": {
                                                "segments": [
                                                    {
                                                        "arguments": {
                                                            "angle_bracketed": {
                                                                "args": [
                                                                    {
                                                                        "type": {
                                                                            "path": {
                                                                                "segments": [
                                                                                    {
                                                                                        "ident": "CheckSysvarAddress",
                                                                                        "position": {
                                                                                            "end_column": 29,
                                                                                            "end_line": 17,
                                                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                                                            "start_column": 11,
                                                                                            "start_line": 17
                                                                                        }
                                                                                    }
                                                                                ]
                                                                            }
                                                                        }
                                                                    }
                                                                ]
                                                            }
                                                        },
                                                        "ident": "Context",
                                                        "position": {
                                                            "end_column": 44,
                                                            "end_line": 9,
                                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                            "start_column": 37,
                                                            "start_line": 9
                                                        }
                                                    }
                                                ]
                                            }
                                        }
                                    }
                                }
                            ],
                            "output": {
                                "path": {
                                    "segments": [
                                        {
                                            "arguments": {
                                                "angle_bracketed": {
                                                    "args": [
                                                        {
                                                            "type": {
                                                                "tuple": {
                                                                    "elems": []
                                                                }
                                                            }
                                                        }
                                                    ]
                                                }
                                            },
                                            "ident": "Result",
                                            "position": {
                                                "end_column": 75,
                                                "end_line": 9,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                "start_column": 69,
                                                "start_line": 9
                                            }
                                        }
                                    ]
                                }
                            },
                            "position": {
                                "end_column": 31,
                                "end_line": 9,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                "start_column": 11,
                                "start_line": 9
                            },
                            "stmts": [
                                {
                                    "macro": {
                                        "delimiter": "paren",
                                        "path": {
                                            "segments": [
                                                {
                                                    "ident": "require_eq",
                                                    "position": {
                                                        "end_column": 18,
                                                        "end_line": 10,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                        "start_column": 8,
                                                        "start_line": 10
                                                    }
                                                }
                                            ]
                                        },
                                        "semi_token": "True",
                                        "tokens": [
                                            {
                                                "ident": "ctx",
                                                "position": {
                                                    "end_column": 35,
                                                    "end_line": 9,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                    "start_column": 32,
                                                    "start_line": 9
                                                }
                                            },
                                            {
                                                "punct": {
                                                    "op": ".",
                                                    "spacing": "alone"
                                                }
                                            },
                                            {
                                                "ident": "accounts"
                                            },
                                            {
                                                "punct": {
                                                    "op": ".",
                                                    "spacing": "alone"
                                                }
                                            },
                                            {
                                                "ident": "rent",
                                                "position": {
                                                    "end_column": 8,
                                                    "end_line": 18,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 18
                                                }
                                            },
                                            {
                                                "punct": {
                                                    "op": ".",
                                                    "spacing": "alone"
                                                }
                                            },
                                            {
                                                "ident": "key"
                                            },
                                            {
                                                "group": {
                                                    "delimiter": "parenthesis",
                                                    "stream": []
                                                }
                                            },
                                            {
                                                "punct": {
                                                    "op": ",",
                                                    "spacing": "alone"
                                                }
                                            },
                                            {
                                                "ident": "sysvar",
                                                "position": {
                                                    "end_column": 39,
                                                    "end_line": 2,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                    "start_column": 33,
                                                    "start_line": 2
                                                }
                                            },
                                            {
                                                "punct": {
                                                    "op": ":",
                                                    "spacing": "joint"
                                                }
                                            },
                                            {
                                                "punct": {
                                                    "op": ":",
                                                    "spacing": "alone"
                                                }
                                            },
                                            {
                                                "ident": "rent",
                                                "position": {
                                                    "end_column": 8,
                                                    "end_line": 18,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 18
                                                }
                                            },
                                            {
                                                "punct": {
                                                    "op": ":",
                                                    "spacing": "joint"
                                                }
                                            },
                                            {
                                                "punct": {
                                                    "op": ":",
                                                    "spacing": "alone"
                                                }
                                            },
                                            {
                                                "ident": "ID"
                                            }
                                        ]
                                    }
                                },
                                {
                                    "macro": {
                                        "delimiter": "paren",
                                        "path": {
                                            "segments": [
                                                {
                                                    "ident": "msg",
                                                    "position": {
                                                        "end_column": 11,
                                                        "end_line": 11,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                        "start_column": 8,
                                                        "start_line": 11
                                                    }
                                                }
                                            ]
                                        },
                                        "semi_token": "True",
                                        "tokens": [
                                            {
                                                "lit": "\"Rent Key -> {}\""
                                            },
                                            {
                                                "punct": {
                                                    "op": ",",
                                                    "spacing": "alone"
                                                }
                                            },
                                            {
                                                "ident": "ctx",
                                                "position": {
                                                    "end_column": 35,
                                                    "end_line": 9,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                    "start_column": 32,
                                                    "start_line": 9
                                                }
                                            },
                                            {
                                                "punct": {
                                                    "op": ".",
                                                    "spacing": "alone"
                                                }
                                            },
                                            {
                                                "ident": "accounts"
                                            },
                                            {
                                                "punct": {
                                                    "op": ".",
                                                    "spacing": "alone"
                                                }
                                            },
                                            {
                                                "ident": "rent",
                                                "position": {
                                                    "end_column": 8,
                                                    "end_line": 18,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                    "start_column": 4,
                                                    "start_line": 18
                                                }
                                            },
                                            {
                                                "punct": {
                                                    "op": ".",
                                                    "spacing": "alone"
                                                }
                                            },
                                            {
                                                "ident": "key"
                                            },
                                            {
                                                "group": {
                                                    "delimiter": "parenthesis",
                                                    "stream": []
                                                }
                                            },
                                            {
                                                "punct": {
                                                    "op": ".",
                                                    "spacing": "alone"
                                                }
                                            },
                                            {
                                                "ident": "to_string"
                                            },
                                            {
                                                "group": {
                                                    "delimiter": "parenthesis",
                                                    "stream": []
                                                }
                                            }
                                        ]
                                    }
                                },
                                {
                                    "expr": [
                                        {
                                            "call": {
                                                "args": [
                                                    {
                                                        "tuple": {
                                                            "elems": []
                                                        }
                                                    }
                                                ],
                                                "func": {
                                                    "path": {
                                                        "segments": [
                                                            {
                                                                "ident": "Ok",
                                                                "position": {
                                                                    "end_column": 10,
                                                                    "end_line": 12,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                                    "start_column": 8,
                                                                    "start_line": 12
                                                                }
                                                            }
                                                        ]
                                                    }
                                                }
                                            }
                                        }, "False"
                                    ]
                                }
                            ],
                            "vis": "pub"
                        }
                    }
                ],
                "ident": "secure",
                "position": {
                    "end_column": 14,
                    "end_line": 6,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                    "start_column": 8,
                    "start_line": 6
                },
                "vis": "pub"
            },
            "access_path": "[3].mod",
            "metadata": {
                "position": {
                    "end_column": 14,
                    "end_line": 6,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                    "start_column": 8,
                    "start_line": 6
                }
            },
            "children": [
                {
                    "raw_node": {
                        "ident": "program",
                        "position": {
                            "end_column": 9,
                            "end_line": 5,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 2,
                            "start_line": 5
                        }
                    },
                    "access_path": "[3].mod.attrs[0].meta.path.segments[0]",
                    "metadata": {
                        "position": {
                            "end_column": 9,
                            "end_line": 5,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 2,
                            "start_line": 5
                        }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "program"
                },
                {
                    "raw_node": {
                        "ident": "super",
                        "position": {
                            "end_column": 13,
                            "end_line": 7,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 8,
                            "start_line": 7
                        },
                        "tree": "*"
                    },
                    "access_path": "[3].mod.content[0].use.tree.path",
                    "metadata": {
                        "position": {
                            "end_column": 13,
                            "end_line": 7,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 8,
                            "start_line": 7
                        }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "super"
                },
                {
                    "raw_node": {
                        "ident": "check_sysvar_address",
                        "inputs": [
                            {
                                "typed": {
                                    "pat": {
                                        "ident": {
                                            "ident": "ctx",
                                            "position": {
                                                "end_column": 35,
                                                "end_line": 9,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                "start_column": 32,
                                                "start_line": 9
                                            }
                                        }
                                    },
                                    "ty": {
                                        "path": {
                                            "segments": [
                                                {
                                                    "arguments": {
                                                        "angle_bracketed": {
                                                            "args": [
                                                                {
                                                                    "type": {
                                                                        "path": {
                                                                            "segments": [
                                                                                {
                                                                                    "ident": "CheckSysvarAddress",
                                                                                    "position": {
                                                                                        "end_column": 29,
                                                                                        "end_line": 17,
                                                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                                                        "start_column": 11,
                                                                                        "start_line": 17
                                                                                    }
                                                                                }
                                                                            ]
                                                                        }
                                                                    }
                                                                }
                                                            ]
                                                        }
                                                    },
                                                    "ident": "Context",
                                                    "position": {
                                                        "end_column": 44,
                                                        "end_line": 9,
                                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                        "start_column": 37,
                                                        "start_line": 9
                                                    }
                                                }
                                            ]
                                        }
                                    }
                                }
                            }
                        ],
                        "output": {
                            "path": {
                                "segments": [
                                    {
                                        "arguments": {
                                            "angle_bracketed": {
                                                "args": [
                                                    {
                                                        "type": {
                                                            "tuple": {
                                                                "elems": []
                                                            }
                                                        }
                                                    }
                                                ]
                                            }
                                        },
                                        "ident": "Result",
                                        "position": {
                                            "end_column": 75,
                                            "end_line": 9,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                            "start_column": 69,
                                            "start_line": 9
                                        }
                                    }
                                ]
                            }
                        },
                        "position": {
                            "end_column": 31,
                            "end_line": 9,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 9
                        },
                        "stmts": [
                            {
                                "macro": {
                                    "delimiter": "paren",
                                    "path": {
                                        "segments": [
                                            {
                                                "ident": "require_eq",
                                                "position": {
                                                    "end_column": 18,
                                                    "end_line": 10,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                    "start_column": 8,
                                                    "start_line": 10
                                                }
                                            }
                                        ]
                                    },
                                    "semi_token": "True",
                                    "tokens": [
                                        {
                                            "ident": "ctx",
                                            "position": {
                                                "end_column": 35,
                                                "end_line": 9,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                "start_column": 32,
                                                "start_line": 9
                                            }
                                        },
                                        {
                                            "punct": {
                                                "op": ".",
                                                "spacing": "alone"
                                            }
                                        },
                                        {
                                            "ident": "accounts"
                                        },
                                        {
                                            "punct": {
                                                "op": ".",
                                                "spacing": "alone"
                                            }
                                        },
                                        {
                                            "ident": "rent",
                                            "position": {
                                                "end_column": 8,
                                                "end_line": 18,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                "start_column": 4,
                                                "start_line": 18
                                            }
                                        },
                                        {
                                            "punct": {
                                                "op": ".",
                                                "spacing": "alone"
                                            }
                                        },
                                        {
                                            "ident": "key"
                                        },
                                        {
                                            "group": {
                                                "delimiter": "parenthesis",
                                                "stream": []
                                            }
                                        },
                                        {
                                            "punct": {
                                                "op": ",",
                                                "spacing": "alone"
                                            }
                                        },
                                        {
                                            "ident": "sysvar",
                                            "position": {
                                                "end_column": 39,
                                                "end_line": 2,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                "start_column": 33,
                                                "start_line": 2
                                            }
                                        },
                                        {
                                            "punct": {
                                                "op": ":",
                                                "spacing": "joint"
                                            }
                                        },
                                        {
                                            "punct": {
                                                "op": ":",
                                                "spacing": "alone"
                                            }
                                        },
                                        {
                                            "ident": "rent",
                                            "position": {
                                                "end_column": 8,
                                                "end_line": 18,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                "start_column": 4,
                                                "start_line": 18
                                            }
                                        },
                                        {
                                            "punct": {
                                                "op": ":",
                                                "spacing": "joint"
                                            }
                                        },
                                        {
                                            "punct": {
                                                "op": ":",
                                                "spacing": "alone"
                                            }
                                        },
                                        {
                                            "ident": "ID"
                                        }
                                    ]
                                }
                            },
                            {
                                "macro": {
                                    "delimiter": "paren",
                                    "path": {
                                        "segments": [
                                            {
                                                "ident": "msg",
                                                "position": {
                                                    "end_column": 11,
                                                    "end_line": 11,
                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                    "start_column": 8,
                                                    "start_line": 11
                                                }
                                            }
                                        ]
                                    },
                                    "semi_token": "True",
                                    "tokens": [
                                        {
                                            "lit": "\"Rent Key -> {}\""
                                        },
                                        {
                                            "punct": {
                                                "op": ",",
                                                "spacing": "alone"
                                            }
                                        },
                                        {
                                            "ident": "ctx",
                                            "position": {
                                                "end_column": 35,
                                                "end_line": 9,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                "start_column": 32,
                                                "start_line": 9
                                            }
                                        },
                                        {
                                            "punct": {
                                                "op": ".",
                                                "spacing": "alone"
                                            }
                                        },
                                        {
                                            "ident": "accounts"
                                        },
                                        {
                                            "punct": {
                                                "op": ".",
                                                "spacing": "alone"
                                            }
                                        },
                                        {
                                            "ident": "rent",
                                            "position": {
                                                "end_column": 8,
                                                "end_line": 18,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                "start_column": 4,
                                                "start_line": 18
                                            }
                                        },
                                        {
                                            "punct": {
                                                "op": ".",
                                                "spacing": "alone"
                                            }
                                        },
                                        {
                                            "ident": "key"
                                        },
                                        {
                                            "group": {
                                                "delimiter": "parenthesis",
                                                "stream": []
                                            }
                                        },
                                        {
                                            "punct": {
                                                "op": ".",
                                                "spacing": "alone"
                                            }
                                        },
                                        {
                                            "ident": "to_string"
                                        },
                                        {
                                            "group": {
                                                "delimiter": "parenthesis",
                                                "stream": []
                                            }
                                        }
                                    ]
                                }
                            },
                            {
                                "expr": [
                                    {
                                        "call": {
                                            "args": [
                                                {
                                                    "tuple": {
                                                        "elems": []
                                                    }
                                                }
                                            ],
                                            "func": {
                                                "path": {
                                                    "segments": [
                                                        {
                                                            "ident": "Ok",
                                                            "position": {
                                                                "end_column": 10,
                                                                "end_line": 12,
                                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                                "start_column": 8,
                                                                "start_line": 12
                                                            }
                                                        }
                                                    ]
                                                }
                                            }
                                        }
                                    }, "False"
                                ]
                            }
                        ],
                        "vis": "pub"
                    },
                    "access_path": "[3].mod.content[1].fn",
                    "metadata": {
                        "position": {
                            "end_column": 31,
                            "end_line": 9,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 11,
                            "start_line": 9
                        }
                    },
                    "children": [
                        {
                            "raw_node": {
                                "ident": {
                                    "ident": "ctx",
                                    "position": {
                                        "end_column": 35,
                                        "end_line": 9,
                                        "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                        "start_column": 32,
                                        "start_line": 9
                                    }
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.inputs[0].typed.pat",
                            "metadata": {},
                            "children": [
                                {
                                    "raw_node": {
                                        "ident": "ctx",
                                        "position": {
                                            "end_column": 35,
                                            "end_line": 9,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                            "start_column": 32,
                                            "start_line": 9
                                        }
                                    },
                                    "access_path": "[3].mod.content[1].fn.inputs[0].typed.pat.ident",
                                    "metadata": {
                                        "position": {
                                            "end_column": 35,
                                            "end_line": 9,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                            "start_column": 32,
                                            "start_line": 9
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "ctx"
                                }
                            ],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "ctx"
                        },
                        {
                            "raw_node": {
                                "arguments": {
                                    "angle_bracketed": {
                                        "args": [
                                            {
                                                "type": {
                                                    "path": {
                                                        "segments": [
                                                            {
                                                                "ident": "CheckSysvarAddress",
                                                                "position": {
                                                                    "end_column": 29,
                                                                    "end_line": 17,
                                                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                                    "start_column": 11,
                                                                    "start_line": 17
                                                                }
                                                            }
                                                        ]
                                                    }
                                                }
                                            }
                                        ]
                                    }
                                },
                                "ident": "Context",
                                "position": {
                                    "end_column": 44,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 37,
                                    "start_line": 9
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.inputs[0].typed.ty.path.segments[0]",
                            "metadata": {
                                "position": {
                                    "end_column": 44,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 37,
                                    "start_line": 9
                                }
                            },
                            "children": [
                                {
                                    "raw_node": {
                                        "ident": "CheckSysvarAddress",
                                        "position": {
                                            "end_column": 29,
                                            "end_line": 17,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                            "start_column": 11,
                                            "start_line": 17
                                        }
                                    },
                                    "access_path": "[3].mod.content[1].fn.inputs[0].typed.ty.path.segments[0].arguments.angle_bracketed.args[0].type.path.segments[0]",
                                    "metadata": {
                                        "position": {
                                            "end_column": 29,
                                            "end_line": 17,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                            "start_column": 11,
                                            "start_line": 17
                                        }
                                    },
                                    "children": [],
                                    "parent": "{...}",
                                    "root": "False",
                                    "args": [],
                                    "ident": "CheckSysvarAddress"
                                }
                            ],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "Context"
                        },
                        {
                            "raw_node": {
                                "arguments": {
                                    "angle_bracketed": {
                                        "args": [
                                            {
                                                "type": {
                                                    "tuple": {
                                                        "elems": []
                                                    }
                                                }
                                            }
                                        ]
                                    }
                                },
                                "ident": "Result",
                                "position": {
                                    "end_column": 75,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 69,
                                    "start_line": 9
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.output.path.segments[0]",
                            "metadata": {
                                "position": {
                                    "end_column": 75,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 69,
                                    "start_line": 9
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "Result"
                        },
                        {
                            "raw_node": {
                                "ident": "require_eq",
                                "position": {
                                    "end_column": 18,
                                    "end_line": 10,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 8,
                                    "start_line": 10
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[0].macro.path.segments[0]",
                            "metadata": {
                                "position": {
                                    "end_column": 18,
                                    "end_line": 10,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 8,
                                    "start_line": 10
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "require_eq"
                        },
                        {
                            "raw_node": {
                                "ident": "ctx",
                                "position": {
                                    "end_column": 35,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 32,
                                    "start_line": 9
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[0].macro.tokens[0]",
                            "metadata": {
                                "position": {
                                    "end_column": 35,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 32,
                                    "start_line": 9
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "ctx"
                        },
                        {
                            "raw_node": {
                                "ident": "accounts"
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[0].macro.tokens[2]",
                            "metadata": {},
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "accounts"
                        },
                        {
                            "raw_node": {
                                "ident": "rent",
                                "position": {
                                    "end_column": 8,
                                    "end_line": 18,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 4,
                                    "start_line": 18
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[0].macro.tokens[4]",
                            "metadata": {
                                "position": {
                                    "end_column": 8,
                                    "end_line": 18,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 4,
                                    "start_line": 18
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "rent"
                        },
                        {
                            "raw_node": {
                                "ident": "key"
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[0].macro.tokens[6]",
                            "metadata": {},
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "key"
                        },
                        {
                            "raw_node": {
                                "ident": "sysvar",
                                "position": {
                                    "end_column": 39,
                                    "end_line": 2,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 33,
                                    "start_line": 2
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[0].macro.tokens[9]",
                            "metadata": {
                                "position": {
                                    "end_column": 39,
                                    "end_line": 2,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 33,
                                    "start_line": 2
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "sysvar"
                        },
                        {
                            "raw_node": {
                                "ident": "rent",
                                "position": {
                                    "end_column": 8,
                                    "end_line": 18,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 4,
                                    "start_line": 18
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[0].macro.tokens[12]",
                            "metadata": {
                                "position": {
                                    "end_column": 8,
                                    "end_line": 18,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 4,
                                    "start_line": 18
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "rent"
                        },
                        {
                            "raw_node": {
                                "ident": "ID"
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[0].macro.tokens[15]",
                            "metadata": {},
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "ID"
                        },
                        {
                            "raw_node": {
                                "ident": "msg",
                                "position": {
                                    "end_column": 11,
                                    "end_line": 11,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 8,
                                    "start_line": 11
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[1].macro.path.segments[0]",
                            "metadata": {
                                "position": {
                                    "end_column": 11,
                                    "end_line": 11,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 8,
                                    "start_line": 11
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "msg"
                        },
                        {
                            "raw_node": {
                                "ident": "ctx",
                                "position": {
                                    "end_column": 35,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 32,
                                    "start_line": 9
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[1].macro.tokens[2]",
                            "metadata": {
                                "position": {
                                    "end_column": 35,
                                    "end_line": 9,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 32,
                                    "start_line": 9
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "ctx"
                        },
                        {
                            "raw_node": {
                                "ident": "accounts"
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[1].macro.tokens[4]",
                            "metadata": {},
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "accounts"
                        },
                        {
                            "raw_node": {
                                "ident": "rent",
                                "position": {
                                    "end_column": 8,
                                    "end_line": 18,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 4,
                                    "start_line": 18
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[1].macro.tokens[6]",
                            "metadata": {
                                "position": {
                                    "end_column": 8,
                                    "end_line": 18,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 4,
                                    "start_line": 18
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "rent"
                        },
                        {
                            "raw_node": {
                                "ident": "key"
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[1].macro.tokens[8]",
                            "metadata": {},
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "key"
                        },
                        {
                            "raw_node": {
                                "ident": "to_string"
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[1].macro.tokens[11]",
                            "metadata": {},
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "to_string"
                        },
                        {
                            "raw_node": {
                                "ident": "Ok",
                                "position": {
                                    "end_column": 10,
                                    "end_line": 12,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 8,
                                    "start_line": 12
                                }
                            },
                            "access_path": "[3].mod.content[1].fn.stmts[2].expr[0].call.func.path.segments[0]",
                            "metadata": {
                                "position": {
                                    "end_column": 10,
                                    "end_line": 12,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 8,
                                    "start_line": 12
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "Ok"
                        }
                    ],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "check_sysvar_address"
                }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "secure"
        },
        {
            "raw_node": {
                "attrs": [
                    {
                        "meta": {
                            "list": {
                                "delimiter": "paren",
                                "path": {
                                    "segments": [
                                        {
                                            "ident": "derive",
                                            "position": {
                                                "end_column": 8,
                                                "end_line": 16,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                "start_column": 2,
                                                "start_line": 16
                                            }
                                        }
                                    ]
                                },
                                "tokens": [
                                    {
                                        "ident": "Accounts"
                                    }
                                ]
                            }
                        },
                        "style": "outer"
                    }
                ],
                "fields": {
                    "named": [
                        {
                            "colon_token": "True",
                            "ident": "rent",
                            "position": {
                                "end_column": 8,
                                "end_line": 18,
                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                "start_column": 4,
                                "start_line": 18
                            },
                            "ty": {
                                "path": {
                                    "segments": [
                                        {
                                            "arguments": {
                                                "angle_bracketed": {
                                                    "args": [
                                                        {
                                                            "lifetime": "info"
                                                        }
                                                    ]
                                                }
                                            },
                                            "ident": "AccountInfo",
                                            "position": {
                                                "end_column": 21,
                                                "end_line": 18,
                                                "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                                "start_column": 10,
                                                "start_line": 18
                                            }
                                        }
                                    ]
                                }
                            }
                        }
                    ]
                },
                "generics": {
                    "params": [
                        {
                            "lifetime": {
                                "bounds": [],
                                "lifetime": "info"
                            }
                        }
                    ]
                },
                "ident": "CheckSysvarAddress",
                "position": {
                    "end_column": 29,
                    "end_line": 17,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                    "start_column": 11,
                    "start_line": 17
                },
                "vis": "pub"
            },
            "access_path": "[4].struct",
            "metadata": {
                "position": {
                    "end_column": 29,
                    "end_line": 17,
                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                    "start_column": 11,
                    "start_line": 17
                }
            },
            "children": [
                {
                    "raw_node": {
                        "ident": "derive",
                        "position": {
                            "end_column": 8,
                            "end_line": 16,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 2,
                            "start_line": 16
                        }
                    },
                    "access_path": "[4].struct.attrs[0].meta.list.path.segments[0]",
                    "metadata": {
                        "position": {
                            "end_column": 8,
                            "end_line": 16,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 2,
                            "start_line": 16
                        }
                    },
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "derive"
                },
                {
                    "raw_node": {
                        "ident": "Accounts"
                    },
                    "access_path": "[4].struct.attrs[0].meta.list.tokens[0]",
                    "metadata": {},
                    "children": [],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "Accounts"
                },
                {
                    "raw_node": {
                        "colon_token": "True",
                        "ident": "rent",
                        "position": {
                            "end_column": 8,
                            "end_line": 18,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 4,
                            "start_line": 18
                        },
                        "ty": {
                            "path": {
                                "segments": [
                                    {
                                        "arguments": {
                                            "angle_bracketed": {
                                                "args": [
                                                    {
                                                        "lifetime": "info"
                                                    }
                                                ]
                                            }
                                        },
                                        "ident": "AccountInfo",
                                        "position": {
                                            "end_column": 21,
                                            "end_line": 18,
                                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                            "start_column": 10,
                                            "start_line": 18
                                        }
                                    }
                                ]
                            }
                        }
                    },
                    "access_path": "[4].struct.fields.named[0]",
                    "metadata": {
                        "position": {
                            "end_column": 8,
                            "end_line": 18,
                            "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                            "start_column": 4,
                            "start_line": 18
                        }
                    },
                    "children": [
                        {
                            "raw_node": {
                                "arguments": {
                                    "angle_bracketed": {
                                        "args": [
                                            {
                                                "lifetime": "info"
                                            }
                                        ]
                                    }
                                },
                                "ident": "AccountInfo",
                                "position": {
                                    "end_column": 21,
                                    "end_line": 18,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 10,
                                    "start_line": 18
                                }
                            },
                            "access_path": "[4].struct.fields.named[0].ty.path.segments[0]",
                            "metadata": {
                                "position": {
                                    "end_column": 21,
                                    "end_line": 18,
                                    "source_file": "../SolanaPlayground/sealevel-attacks//programs/10-sysvar-address-checking/secure/src/lib.rs",
                                    "start_column": 10,
                                    "start_line": 18
                                }
                            },
                            "children": [],
                            "parent": "{...}",
                            "root": "False",
                            "args": [],
                            "ident": "AccountInfo"
                        }
                    ],
                    "parent": "{...}",
                    "root": "False",
                    "args": [],
                    "ident": "rent"
                }
            ],
            "parent": "{...}",
            "root": "False",
            "args": [],
            "ident": "CheckSysvarAddress"
        }
    ],
    "parent": {
        "raw_node": {},
        "access_path": "EMPTY_ACCESS_PATH",
        "metadata": {},
        "children": [],
        "parent": {},
        "root": "False",
        "args": []
    },
    "root": "False",
    "args": [],
    "ident": "EMPTY_IDENT"
}
