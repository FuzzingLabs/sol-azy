# template_matcher.star

# Templates dictionary using simplified 'idents' notation
TEMPLATES = {}

# check if ctx.accounts.authority.key != &token.owner
TEMPLATES["CHECK_CTX_ACCOUNT_AUTHORITY_KEY_TOKEN_OWNER"] = {
    "pattern": { 
        "cond": {
            "binary": {
                "left": {"idents": ["ctx", "accounts", "authority", "key"]},
                "op": "!=",
                "right": {"idents": ["token", "owner"]},
            }
        }
    },
    "priority_rule": ["left", "op", "right"],
}

# check if &spl_token::ID != ctx.accounts.token_program.key
TEMPLATES["CHECK_SPLTOKEN_ID_CTX_ACCOUNT_AUTHORITY_KEY"] = {
    "pattern": { 
        "cond": {
            "binary": {
                "left": {"idents": ["spl_token", "ID"]},
                "op": "!=",
                "right": {"idents": ["ctx", "accounts", "token_program", "key"]},
            }
        }
    },
    "priority_rule": ["left", "op", "right"],
}

# check if ctx.accounts.authority.is_signer
TEMPLATES["CHECK_CTX_ACCOUNTS_AUTHORITY_ISSIGNER"] = {
    "pattern": { 
        "cond": {
            "field": {
                "base": {"idents": ["ctx", "accounts", "authority", "is_signer"]},
            }
        }
    },
    "priority_rule": ["base"],
}

# check if !ctx.accounts.authority.is_signer
TEMPLATES["CHECK_NOT_CTX_ACCOUNTS_AUTHORITY_ISSIGNER"] = {
    "pattern": { 
        "cond": {
            "unary": {
                "expr": {"idents": ["ctx", "accounts", "authority", "is_signer"]},
                "op": "!"
            }
        }
    },
    "priority_rule": ["op", "expr"],
}

# usecase example if ctx.accounts.user_a.key() == ctx.accounts.user_b.key() 
TEMPLATES["CHECK_CTX_ACCOUNTS_WILDCARD_KEY_EQ"] = {
    "pattern": {
        "cond": {
            "binary": {
                "left": {"idents": ["ctx", "accounts", "*"], "method": "key"},
                "op": "==",
                "right": {"idents": ["ctx", "accounts", "*"], "method": "key"},
            }
        }
    },
    "priority_rule": ["left", "op", "right"],
}

# check require_eq!(ctx.accounts.rent.key(), sysvar::rent::ID);
TEMPLATES["REQUIRE_CTX_ACCOUNTS_RENT_KEY_SYSVAR_RENT_ID"] = {
    "pattern": {
        "macro": {
                "path": {"idents": ["require_eq"]},
                "tokens": {"idents": ["ctx", "accounts", "rent", "key", "sysvar", "rent", "ID"]},
                "delimiter": "", # we don't use it for now
                "semi_token": "", # we don't use it for now
            }
    },
    "priority_rule": ["path", "tokens", "delimiter", "semi_token"],
}

# check if solana_program::program::invoke is called
TEMPLATES["CALL_FN_SOLANAPROGRAM_PROGRAM_INVOKE"] = {
    "pattern": {
        "call": {
                "args": "", # we don't use it for now
                "func": {"idents": ["solana_program", "program", "invoke"]},
        }
    },
    "priority_rule": ["func", "args"],
}

def generate_call_fn_template(*idents):
    return { 
        "pattern": {
        "call": {
                "args": "", # we don't use it for now
                "func": {"idents": idents},
            }
        },
        "priority_rule": ["func", "args"],
    }

def generate_symmetric_template(template):
    template = template.get("pattern", {})
    if template == {}:
        return None
    cond = template.get("cond", {})
    if cond == {}:
        return None
    binary = cond.get("binary", {})
    if binary == {}:
        return None
    left = binary.get("left")
    right = binary.get("right")
    op = binary.get("op")

    # Only generate symmetry for == and !=
    if op not in ("==", "!="):
        return None

    symmetric_template = {
        "pattern": { 
            "cond": {
                "binary": {
                    "left": right,
                    "op": op,
                    "right": left,
                }
            }
        },
        "priority_rule": ["left", "op", "right"],
    }

    return symmetric_template


def template_to_linear_pattern(template):
    pattern = []
    priority_rule = template["priority_rule"]

    def walk(node):
        if isinstance(node, dict):
            keys = list(node.keys())
            # Apply priority rule order explicitly for method/idents
            if "idents" in keys and "method" in keys:
                # in AST method node is declared BEFORE idents
                keys = ["method", "idents"] + [k for k in keys if k not in ("method", "idents")]
            else:
                keys = sorted(
                            list(keys),
                            key=lambda k: (
                                priority_rule.index(k) if k in priority_rule else len(priority_rule)
                            ),
                        )

            for key in keys:
                value = node[key]
                if key == "idents":
                    for ident in value:
                        if ident == "*":
                            pattern.append(("wildcard", "*"))
                        else:
                            pattern.append(("ident", ident))
                elif key == "method":
                    pattern.append(("method", value))
                elif key == "op":
                    pattern.append(("op", value))
                elif key == "ident":
                    pattern.append(("ident", value))
                else:
                    walk(value)
        elif isinstance(node, list):
            for item in node:
                walk(item)

    walk(template)
    return pattern

def all_templates_to_patterns(templates):
    return {key: template_to_linear_pattern(value) for key, value in templates.items()}

def get_keys_from_pattern(pattern):
    keys = set([duo[0] for duo in pattern])
    return list(keys)

def extract_info(
    node, pattern, priority_rule, result
):
    pattern_keys = get_keys_from_pattern(pattern)

    def _extract(n):
        if isinstance(n, list):
            for item in n:
                _extract(item)
        elif isinstance(n, dict):
            # go into subnodes recursively to grab the infos
            for k, v in n.items():
                if k in pattern_keys:
                    result.append((k, v))
                if k not in priority_rule and k not in pattern_keys:
                    if isinstance(v, dict):
                        _extract(v)
                    elif isinstance(v, list):
                        for item in v:
                            _extract(item)
        else: # probably a str or int or bool but we don't need it at this step, otherwise the "if k in pattern_keys:" should have captured it
            return

    for key in priority_rule:
        if key in node:
            if key in pattern_keys:
                # that's for node like "op" which directly have it's data in its value
                _extract(node)
            else:
                _extract(node[key])

    return result

def extract_ast_to_sequence(node, pattern, priority_rule, template):
    result = []
    stack = [node]
    seen = set()
    _str_node = str(node)
    approx_nb_element = _str_node.count(",") + 1 + _str_node.count("[") + _str_node.count("{")

    for _ in range(approx_nb_element):
        if not stack:
            break

        current = stack.pop()
        current_step = repr(current)

        # TODO: verify why this is problem to not check whether the node is seen (in starlark engine)
        # starlark memory management magic problem bypass, without this it will do infinite cycle probably due to pointer reference things
        if current_step in seen:
            continue
        seen.add(current_step)

        if isinstance(current, dict):
            # TODO: maybe a more generic pattern matching than just 3 if case could be better but atm that is sufficient 
            # atm we just check for pattern like {"cond": {"binary": ...}} or {"cond": {"unary": ...}} or {"cond": {"field": ...}}
            patt_found = False
            if len(template["pattern"].keys()) == 1:
                first_pattern_key = list(template["pattern"].keys())[0]
                current_patt = current.get(first_pattern_key, {})
                if current_patt != {} and current_patt.keys() == template["pattern"][first_pattern_key].keys():
                    keys = sorted(
                            list(current_patt.keys()),
                            key=lambda k: (
                                priority_rule.index(k) if k in priority_rule else len(priority_rule)
                            ),
                        )
                    if keys == priority_rule:
                        extract_info(current_patt, pattern, priority_rule, result)
                        patt_found = True
                    else:
                        second_pattern_key = list(current_patt.keys())[0]
                        current_stmt = current_patt.get(second_pattern_key, {})
                    
                    if not patt_found and current_stmt != {} and len(current_patt.keys()) == 1 and current_stmt.keys() == template["pattern"][first_pattern_key][second_pattern_key].keys():
                        keys = sorted(
                            list(current_stmt.keys()),
                            key=lambda k: (
                                priority_rule.index(k) if k in priority_rule else len(priority_rule)
                            ),
                        )
                        if keys == priority_rule:
                            extract_info(current_stmt, pattern, priority_rule, result)
                            patt_found = True
            if not patt_found:
                for value in current.values():
                    stack.append(value)

        elif isinstance(current, list):
            for item in current:
                stack.append(item)
    return result

def match_tokens(pattern, tokens):
    len_pattern = len(pattern)
    len_tokens = len(tokens)

    if len_pattern > len_tokens:
        return False

    for i in range(len_tokens - len_pattern + 1):
        window = tokens[i:i+len_pattern]
        match = True
        for p, t in zip(pattern, window):
            if p[0] == "wildcard":
                if t[0] != "ident":  # wildcard only matches identifiers
                    match = False
                    break
            elif p != t:
                match = False
                break
        if match:
            return True
    return False


def match_sequence_in_ast(
    ast, pattern, priority_rule, template
) -> bool:
    tokens = extract_ast_to_sequence(ast, pattern, priority_rule, template)

    sym_pattern = generate_symmetric_template(template) # used to check for a symmetrical rules like == or != (since a != b is the same than b != a for example)
    len_pattern = len(pattern)
    len_tokens = len(tokens)

    for idx_pattern in range(0, len_tokens - len_pattern + 1):
        _tokens = tokens[idx_pattern : idx_pattern + len_pattern]
        if match_tokens(pattern, _tokens) or (sym_pattern != None and match_tokens(sym_pattern, _tokens)):
            return True
    return False


def is_matching_template_by_key(ast, template_key):
    template = TEMPLATES.get(template_key)
    if not template:
        return False

    return match_sequence_in_ast(
        ast, template_to_linear_pattern(template), template["priority_rule"], template
    )

def is_matching_template(ast, template):
    if isinstance(template, str):
        print("Should use is_matching_template_by_key instead of is_matching_template")
    return match_sequence_in_ast(
        ast, template_to_linear_pattern(template), template["priority_rule"], template
    )

template_manager = struct(
    TEMPLATES=TEMPLATES,
    is_matching_template=is_matching_template,
    is_matching_template_by_key=is_matching_template_by_key,
)