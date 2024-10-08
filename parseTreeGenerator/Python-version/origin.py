import itertools

# Define the grammar
grammar = {
    'S': [['NP', 'VP']],
    'NP': [['Det', 'N'], ['NP', 'PP']],
    'VP': [['V', 'NP'], ['VP', 'PP']],
    'PP': [['P', 'NP']],
    'Det': [['the'], ['a']],
    'N': [['cat'], ['dog'], ['telescope'], ['park']],
    'V': [['saw'], ['walked']],
    'P': [['in'], ['with']]
}

# Function to eliminate epsilon productions
def remove_epsilons(grammar):
    def remove_eps_from_rhs(rhs_list, eps_nonterms):
        if not rhs_list:
            return [[]]
        hd, *tl = rhs_list
        rest = remove_eps_from_rhs(tl, eps_nonterms)
        if hd in eps_nonterms:
            new_rest = rest + [[hd] + r for r in rest]
            return new_rest
        else:
            return [[hd] + r for r in rest]

    def find_eps_nonterms(grammar):
        return {lhs for lhs, rhs_list in grammar.items() if any(rhs == ['ε'] for rhs in rhs_list)}

    def update_eps_nonterms(eps_nonterms, grammar):
        new_eps_nonterms = eps_nonterms.copy()
        for lhs, rhs_list in grammar.items():
            if any(all(sym in eps_nonterms for sym in rhs) for rhs in rhs_list):
                new_eps_nonterms.add(lhs)
        if len(new_eps_nonterms) > len(eps_nonterms):
            return update_eps_nonterms(new_eps_nonterms, grammar)
        else:
            return new_eps_nonterms

    eps_nonterms = update_eps_nonterms(find_eps_nonterms(grammar), grammar)

    new_grammar = {}
    for lhs, rhs_list in grammar.items():
        new_rhs_list = []
        for rhs in rhs_list:
            if rhs == ['ε']:
                continue
            new_rhs_list.extend(remove_eps_from_rhs(rhs, eps_nonterms))
        new_grammar[lhs] = new_rhs_list

    def generate_new_productions(grammar, eps_nonterms):
        new_productions = []
        for lhs, rhs_list in grammar.items():
            for rhs in rhs_list:
                if any(sym in eps_nonterms for sym in rhs):
                    new_rhs = [sym for sym in rhs if sym not in eps_nonterms]
                    if new_rhs and (lhs, new_rhs) not in new_productions:
                        new_productions.append((lhs, new_rhs))
        return new_productions

    new_productions = generate_new_productions(new_grammar, eps_nonterms)

    return new_grammar, new_productions

# Eliminate epsilon productions
grammar, new_productions = remove_epsilons(grammar)

# Collect non-terminals and terminals
non_terminals = set(grammar.keys())
rhs_symbols = set()
for prods in grammar.values():
    for prod in prods:
        for symbol in prod:
            rhs_symbols.add(symbol)
terminals = rhs_symbols - non_terminals

# Input sentence
sentence = "the dog saw a cat in the park"
tokens = sentence.split()

# Memoization cache
memo = {}

# ParseTree class to represent the parse trees
class ParseTree:
    def __init__(self, label, children):
        self.label = label
        self.children = children  # List of ParseTrees or terminal strings

    def to_string(self, indent=''):
        if not self.children:
            return indent + self.label + '\n'
        result = indent + self.label + '\n'
        for child in self.children:
            if isinstance(child, ParseTree):
                result += child.to_string(indent + '  ')
            else:
                result += indent + '  ' + str(child) + '\n'
        return result

    def to_typst(self, indent=''):
        if not self.children:
            return f'{indent}tree("{self.label}")'
        new_indent = indent + '  '
        children_typst = [child.to_typst(new_indent) for child in self.children]
        children_str = ',\n'.join(children_typst)
        return f'{indent}tree("{self.label}",\n{children_str}\n{indent})'

# Parsing function
def parse(N, i, j):
    key = (N, i, j)
    if key in memo:
        return memo[key]

    results = []

    if i >= j:
        return []

    if N in terminals:
        if i + 1 == j and tokens[i] == N:
            tree = ParseTree(N, [])
            results.append(tree)
    elif N in non_terminals:
        for production in grammar[N]:
            n = len(production)
            if n == 1:
                symbol = production[0]
                if symbol in terminals:
                    if i + 1 == j and tokens[i] == symbol:
                        child = ParseTree(symbol, [])
                        tree = ParseTree(N, [child])
                        results.append(tree)
                else:
                    sub_trees = parse(symbol, i, j)
                    for sub_tree in sub_trees:
                        tree = ParseTree(N, [sub_tree])
                        results.append(tree)
            else:
                for splits in possible_splits(i, j, n):
                    positions = [i] + list(splits) + [j]
                    failed = False
                    children_options = []
                    for idx in range(n):
                        Ai = production[idx]
                        start = positions[idx]
                        end = positions[idx+1]
                        sub_trees = parse(Ai, start, end)
                        if not sub_trees:
                            failed = True
                            break
                        else:
                            children_options.append(sub_trees)
                    if not failed:
                        for children in itertools.product(*children_options):
                            tree = ParseTree(N, list(children))
                            results.append(tree)
    else:
        pass

    memo[key] = results
    return results

# Function to generate possible splits
def possible_splits(i, j, n):
    if n == 1:
        if i < j:
            return [[]]
        else:
            return []
    else:
        positions = range(i+1, j)
        splits = list(itertools.combinations(positions, n-1))
        return splits

# Parse the sentence starting from 'S'
trees = parse('S', 0, len(tokens))

# Print all possible parse trees
for idx, tree in enumerate(trees):
    print(f"Parse tree {idx+1}:")
    print(tree.to_string())
    print(f"Typst tree code {idx+1}:")
    print(f"#{tree.to_typst()}")