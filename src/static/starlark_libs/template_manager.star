# template_matcher.star

# Templates dictionary using simplified 'path' notation
TEMPLATES = {}

# check if ctx.accounts.authority.key != &token.owner
TEMPLATES["CHECK_CTX_ACCOUNT_AUTHORITY_KEY_TOKEN_OWNER"] = {
    "binary": {
        "left": {"path": ["ctx", "accounts", "authority", "key"]},
        "op": "!=",
        "right": {"path": ["token", "owner"]},
    },
    "priority_rule": ["left", "op", "right"],
}

# check if &spl_token::ID != ctx.accounts.token_program.key
TEMPLATES["CHECK_SPLTOKEN_ID_CTX_ACCOUNT_AUTHORITY_KEY"] = {
    "binary": {
        "left": {"path": ["spl_token", "ID"]},
        "op": "!=",
        "right": {"path": ["ctx", "accounts", "token_program", "key"]},
    },
    "priority_rule": ["left", "op", "right"],
}

def generate_symmetric_template(template):
    binary = template.get("binary", {})
    left = binary.get("left")
    right = binary.get("right")
    op = binary.get("op")

    # Only generate symmetry for == and !=
    if op not in ("==", "!="):
        return None

    symmetric_template = {
        "binary": {
            "left": right,
            "op": op,
            "right": left,
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

def extract_ast_to_sequence(node, pattern, priority_rule):
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
            keys = sorted(
                current.keys(),
                key=lambda k: (
                    priority_rule.index(k) if k in priority_rule else len(priority_rule)
                ),
            )
            if keys == priority_rule:
                extract_info(current, pattern, priority_rule, result)
            else:
                for value in current.values():
                    stack.append(value)

        elif isinstance(current, list):
            for item in current:
                stack.append(item)
    return result




def match_sequence_in_ast(
    ast, pattern, priority_rule, template
) -> bool:
    tokens = extract_ast_to_sequence(ast, pattern, priority_rule)

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

    current_node = ast["parent"]["raw_node"]

    return match_sequence_in_ast(
        current_node, template_to_linear_pattern(template), template["priority_rule"], template
    )

template_manager = struct(
    TEMPLATES=TEMPLATES,
    is_matching_template=is_matching_template,
)