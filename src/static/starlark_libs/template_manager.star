# template_matcher.star

# Templates dictionary using simplified 'path' notation
TEMPLATES = {}

# check if ctx.accounts.authority.key != &token.owner
TEMPLATES["CHECK_CTX_ACCOUNT_AUTHORITY_KEY_TOKEN_OWNER"] = {
    "pattern": { 
        "cond": {
            "binary": {
                "left": {"path": ["ctx", "accounts", "authority", "key"]},
                "op": "!=",
                "right": {"path": ["token", "owner"]},
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
                "left": {"path": ["spl_token", "ID"]},
                "op": "!=",
                "right": {"path": ["ctx", "accounts", "token_program", "key"]},
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
                "base": {"path": ["ctx", "accounts", "authority", "is_signer"]},
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
                "expr": {"path": ["ctx", "accounts", "authority", "is_signer"]},
                "op": "!"
            }
        }
    },
    "priority_rule": ["op", "expr"],
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
            keys = sorted(
                node.keys(),
                key=lambda k: (
                    priority_rule.index(k) if k in priority_rule else len(priority_rule)
                ),
            )
            for key in keys:
                value = node[key]
                if key == "path":
                    for ident in value:
                        pattern.append(("ident", ident))
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
    node: dict, pattern, priority_rule, result
):
    
    pattern_keys = get_keys_from_pattern(pattern)

    def _extract(n: dict):
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
            # atm we just check for pattern like {"cond": {"binary": ...}} or {"cond": {"unary": ...}} or {"cond": {"field": ...}}
            patt_found = False
            if len(template["pattern"].keys()) == 1:
                first_pattern_key = list(template["pattern"].keys())[0]
                current_patt = current.get(first_pattern_key, {})
                if current_patt != {} and len(current_patt.keys()) == 1 and current_patt.keys() == template["pattern"][first_pattern_key].keys():
                    second_pattern_key = list(current_patt.keys())[0]
                    current_stmt = current_patt.get(second_pattern_key, {})
                    if current_stmt != {} and len(current_patt.keys()) == 1 and current_stmt.keys() == template["pattern"][first_pattern_key][second_pattern_key].keys():
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

def match_sequence_in_ast(
    ast, pattern, priority_rule, template
) -> bool:
    tokens = extract_ast_to_sequence(ast, pattern, priority_rule, template)

    sym_pattern = generate_symmetric_template(template) # used to check for a symmetrical rules like == or != (since a != b is the same than b != a for example)
    len_pattern = len(pattern)
    len_tokens = len(tokens)

    if len_tokens < len_pattern:
        return False
    for idx_pattern in range(0, len_tokens - len_pattern + 1):
        _tokens = tokens[idx_pattern : idx_pattern + len_pattern]
        if pattern == _tokens or (sym_pattern != None and sym_pattern in _tokens):
            return True
    return False


def is_matching_template(ast, template_key):
    template = TEMPLATES.get(template_key)
    if not template:
        return False

    # TODO: Why raw_node
    current_node = ast["parent"]["raw_node"]

    return match_sequence_in_ast(
        current_node, template_to_linear_pattern(template), template["priority_rule"], template
    )

template_manager = struct(
    TEMPLATES=TEMPLATES,
    is_matching_template=is_matching_template,
)