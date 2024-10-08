#include <algorithm>
#include <functional>
#include <iostream>
#include <iterator>
#include <sstream>
#include <string>
#include <tuple>
#include <unordered_map>
#include <unordered_set>
#include <vector>

// Define overloaded for std::visit
template <class... Ts>
struct overloaded : Ts... {
    using Ts::operator()...;
};
template <class... Ts>
overloaded(Ts...) -> overloaded<Ts...>;

// Custom hash function for std::tuple
namespace std {
template <>
struct hash<std::tuple<std::string, size_t, size_t>> {
    size_t operator()(const std::tuple<std::string, size_t, size_t>& key) const
    {
        auto hash1 = std::hash<std::string> {}(std::get<0>(key));
        auto hash2 = std::hash<size_t> {}(std::get<1>(key));
        auto hash3 = std::hash<size_t> {}(std::get<2>(key));
        return hash1 ^ (hash2 << 1) ^ (hash3 << 2);
    }
};
}

using Grammar = std::unordered_map<std::string, std::vector<std::vector<std::string>>>;

enum class NodeType { Leaf,
    Internal };

struct Node {
    NodeType type;
    std::string label;
    std::vector<Node> children;

    Node(std::string label)
        : type(NodeType::Leaf)
        , label(std::move(label))
    {
    }
    Node(std::string label, std::vector<Node> children)
        : type(NodeType::Internal)
        , label(std::move(label))
        , children(std::move(children))
    {
    }
};

std::string tree_to_string(const std::string& indent, const Node& tree)
{
    std::ostringstream result;
    if (tree.type == NodeType::Leaf) {
        result << indent << tree.label << "\n";
    } else {
        result << indent << tree.label << "\n";
        for (const auto& child : tree.children) {
            result << tree_to_string(indent + "  ", child);
        }
    }
    return result.str();
}

std::string tree_to_typst(const std::string& indent, const Node& tree)
{
    std::ostringstream result;
    if (tree.type == NodeType::Leaf) {
        result << indent << "tree(\"" << tree.label << "\")";
    } else {
        result << indent << "tree(\"" << tree.label << "\",\n";
        for (size_t i = 0; i < tree.children.size(); ++i) {
            result << tree_to_typst(indent + "  ", tree.children[i]);
            if (i < tree.children.size() - 1) {
                result << ",\n";
            }
        }
        result << "\n"
               << indent << ")";
    }
    return result.str();
}

std::vector<std::vector<size_t>> possible_splits(size_t i, size_t j, size_t n)
{
    if (n == 1) {
        return i < j ? std::vector<std::vector<size_t>> { std::vector<size_t> {} } : std::vector<std::vector<size_t>> {};
    } else {
        std::vector<size_t> positions;
        for (size_t pos = i + 1; pos < j; ++pos) {
            positions.push_back(pos);
        }

        std::function<std::vector<std::vector<size_t>>(std::vector<size_t>, const std::vector<size_t>&, size_t)> combinations;
        combinations = [&](std::vector<size_t> acc, const std::vector<size_t>& remaining_positions, size_t k) {
            if (k == 0) {
                return std::vector<std::vector<size_t>> { acc };
            } else {
                std::vector<std::vector<size_t>> result;
                for (size_t idx = 0; idx < remaining_positions.size(); ++idx) {
                    auto pos = remaining_positions[idx];
                    std::vector<size_t> new_positions;
                    for (size_t i = 0; i < remaining_positions.size(); ++i) {
                        if (i > idx) {
                            new_positions.push_back(remaining_positions[i]);
                        }
                    }
                    auto new_acc = acc;
                    new_acc.push_back(pos);
                    auto sub_combinations = combinations(new_acc, new_positions, k - 1);
                    result.insert(result.end(), sub_combinations.begin(), sub_combinations.end());
                }
                return result;
            }
        };

        return combinations({}, positions, n - 1);
    }
}

std::vector<std::vector<std::string>> lookup_rules(const Grammar& grammar, const std::string& nt)
{
    auto it = grammar.find(nt);
    return it != grammar.end() ? it->second : std::vector<std::vector<std::string>> {};
}

std::pair<Grammar, std::vector<std::pair<std::string, std::vector<std::string>>>> remove_epsilons(const Grammar& grammar)
{
    std::function<std::vector<std::vector<std::string>>(const std::vector<std::string>&, const std::unordered_set<std::string>&)> remove_eps_from_rhs;
    remove_eps_from_rhs = [&](const std::vector<std::string>& rhs_list, const std::unordered_set<std::string>& eps_nonterms) {
        if (rhs_list.empty()) {
            return std::vector<std::vector<std::string>> { std::vector<std::string> {} };
        }
        std::vector<std::vector<std::string>> result;
        auto hd = rhs_list.front();
        auto tl = std::vector<std::string>(rhs_list.begin() + 1, rhs_list.end());
        auto rest = remove_eps_from_rhs(tl, eps_nonterms);
        if (eps_nonterms.find(hd) != eps_nonterms.end()) {
            result.insert(result.end(), rest.begin(), rest.end());
            for (auto& r : rest) {
                auto new_r = r;
                new_r.insert(new_r.begin(), hd);
                result.push_back(new_r);
            }
        } else {
            for (auto& r : rest) {
                auto new_r = r;
                new_r.insert(new_r.begin(), hd);
                result.push_back(new_r);
            }
        }
        return result;
    };

    auto find_eps_nonterms = [](const Grammar& grammar) {
        std::unordered_set<std::string> eps_nonterms;
        for (const auto& [lhs, rhs_list] : grammar) {
            for (const auto& rhs : rhs_list) {
                if (rhs == std::vector<std::string> { "ε" }) {
                    eps_nonterms.insert(lhs);
                }
            }
        }
        return eps_nonterms;
    };

    std::function<std::unordered_set<std::string>(std::unordered_set<std::string>, const Grammar&)> update_eps_nonterms;
    update_eps_nonterms = [&](std::unordered_set<std::string> eps_nonterms, const Grammar& grammar) {
        std::unordered_set<std::string> new_eps_nonterms = eps_nonterms;
        for (const auto& [lhs, rhs_list] : grammar) {
            for (const auto& rhs : rhs_list) {
                if (std::all_of(rhs.begin(), rhs.end(), [&](const std::string& sym) { return eps_nonterms.find(sym) != eps_nonterms.end(); })) {
                    new_eps_nonterms.insert(lhs);
                }
            }
        }
        if (new_eps_nonterms.size() > eps_nonterms.size()) {
            return update_eps_nonterms(new_eps_nonterms, grammar);
        } else {
            return new_eps_nonterms;
        }
    };

    auto eps_nonterms = update_eps_nonterms(find_eps_nonterms(grammar), grammar);

    Grammar new_grammar;
    for (const auto& [lhs, rhs_list] : grammar) {
        std::vector<std::vector<std::string>> new_rhs_list;
        for (const auto& rhs : rhs_list) {
            if (rhs == std::vector<std::string> { "ε" }) {
                continue;
            }
            auto new_rhs = remove_eps_from_rhs(rhs, eps_nonterms);
            new_rhs_list.insert(new_rhs_list.end(), new_rhs.begin(), new_rhs.end());
        }
        new_grammar[lhs] = new_rhs_list;
    }

    auto generate_new_productions = [&](const Grammar& grammar, const std::unordered_set<std::string>& eps_nonterms) {
        std::vector<std::pair<std::string, std::vector<std::string>>> new_productions;
        for (const auto& [lhs, rhs_list] : grammar) {
            for (const auto& rhs : rhs_list) {
                if (std::any_of(rhs.begin(), rhs.end(), [&](const std::string& sym) { return eps_nonterms.find(sym) != eps_nonterms.end(); })) {
                    std::vector<std::string> new_rhs;
                    std::copy_if(rhs.begin(), rhs.end(), std::back_inserter(new_rhs), [&](const std::string& sym) { return eps_nonterms.find(sym) == eps_nonterms.end(); });
                    if (!new_rhs.empty() && std::find(new_productions.begin(), new_productions.end(), std::make_pair(lhs, new_rhs)) == new_productions.end()) {
                        new_productions.emplace_back(lhs, new_rhs);
                    }
                }
            }
        }
        return new_productions;
    };

    auto new_productions = generate_new_productions(new_grammar, eps_nonterms);

    return { new_grammar, new_productions };
}

std::vector<Node> parse(
    const Grammar& grammar,
    const std::vector<std::string>& tokens,
    const std::unordered_set<std::string>& non_terminals,
    const std::unordered_set<std::string>& terminals,
    std::unordered_map<std::tuple<std::string, size_t, size_t>, std::vector<Node>>& memo,
    const std::string& nt,
    size_t i,
    size_t j)
{
    auto key = std::make_tuple(nt, i, j);
    if (memo.find(key) != memo.end()) {
        return memo[key];
    }

    std::vector<Node> results;
    if (i >= j) {
        // No results
    } else if (terminals.find(nt) != terminals.end()) {
        if (i + 1 == j && tokens[i] == nt) {
            results.emplace_back(nt);
        }
    } else if (non_terminals.find(nt) != non_terminals.end()) {
        auto rules = lookup_rules(grammar, nt);
        for (const auto& production : rules) {
            size_t n = production.size();
            if (n == 1) {
                const auto& symbol = production[0];
                if (terminals.find(symbol) != terminals.end()) {
                    if (i + 1 == j && tokens[i] == symbol) {
                        results.emplace_back(nt, std::vector<Node> { Node(symbol) });
                    }
                } else {
                    auto sub_trees = parse(grammar, tokens, non_terminals, terminals, memo, symbol, i, j);
                    for (auto& sub_tree : sub_trees) {
                        results.emplace_back(nt, std::vector<Node> { std::move(sub_tree) });
                    }
                }
            } else {
                for (const auto& splits : possible_splits(i, j, n)) {
                    std::vector<size_t> positions = { i };
                    positions.insert(positions.end(), splits.begin(), splits.end());
                    positions.push_back(j);

                    std::vector<std::vector<Node>> children;
                    bool failed = false;
                    for (size_t idx = 0; idx < production.size(); ++idx) {
                        const auto& ai = production[idx];
                        size_t start = positions[idx];
                        size_t end = positions[idx + 1];
                        auto sub_trees = parse(grammar, tokens, non_terminals, terminals, memo, ai, start, end);
                        if (sub_trees.empty()) {
                            failed = true;
                            break;
                        }
                        children.push_back(std::move(sub_trees));
                    }
                    if (!failed) {
                        std::vector<std::vector<Node>> combinations = { {} };
                        for (const auto& sub_trees : children) {
                            std::vector<std::vector<Node>> new_combinations;
                            for (const auto& acc_subtree : combinations) {
                                for (const auto& t : sub_trees) {
                                    auto new_acc = acc_subtree;
                                    new_acc.push_back(t);
                                    new_combinations.push_back(std::move(new_acc));
                                }
                            }
                            combinations = std::move(new_combinations);
                        }
                        for (auto& combination : combinations) {
                            results.emplace_back(nt, std::move(combination));
                        }
                    }
                }
            }
        }
    }

    memo[key] = results;
    return results;
}

int main()
{
    // Define the grammar
    Grammar grammar = {
        { "S", { { "NP", "VP" } } },
        { "NP", { { "Det", "N" }, { "NP", "PP" } } },
        { "VP", { { "V", "NP" }, { "VP", "PP" } } },
        { "PP", { { "P", "NP" } } },
        { "Det", { { "the" }, { "a" } } },
        { "N", { { "cat" }, { "dog" }, { "telescope" }, { "park" } } },
        { "V", { { "saw" }, { "walked" } } },
        { "P", { { "in" }, { "with" } } }
    };

    // Eliminate epsilon productions
    auto [new_grammar, new_productions] = remove_epsilons(grammar);

    // Collect non-terminals and terminals
    std::unordered_set<std::string> non_terminals;
    for (const auto& [key, _] : new_grammar) {
        non_terminals.insert(key);
    }

    std::unordered_set<std::string> terminals;
    for (const auto& [_, prods] : new_grammar) {
        for (const auto& prod : prods) {
            for (const auto& sym : prod) {
                if (non_terminals.find(sym) == non_terminals.end()) {
                    terminals.insert(sym);
                }
            }
        }
    }

    // Input sentence
    std::string sentence = "the dog saw a cat in the park";
    std::istringstream iss(sentence);
    std::vector<std::string> tokens { std::istream_iterator<std::string> { iss }, std::istream_iterator<std::string> {} };

    // Parse the sentence starting from 'S'
    std::unordered_map<std::tuple<std::string, size_t, size_t>, std::vector<Node>> memo;
    auto trees = parse(new_grammar, tokens, non_terminals, terminals, memo, "S", 0, tokens.size());

    // Print all possible parse trees
    for (size_t idx = 0; idx < trees.size(); ++idx) {
        std::cout << "Parse tree " << idx + 1 << ":\n";
        std::cout << tree_to_string("", trees[idx]);
        std::cout << "Typst tree code " << idx + 1 << ":\n";
        std::cout << "#" << tree_to_typst("", trees[idx]) << "\n";
    }

    return 0;
}