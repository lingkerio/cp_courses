using System;
using System.Collections.Generic;
using System.Linq;

namespace parseTreeGenerator
{
    abstract class Node { }

    class Leaf(string value) : Node
    {
        public string Value { get; } = value;
    }

    class Internal(string label, List<Node> children) : Node
    {
        public string Label { get; } = label;
        public List<Node> Children { get; } = children;
    }

    static class EnumerableExtensions
    {
        public static IEnumerable<(T item, int index)> Enumerate<T>(this IEnumerable<T> source)
        {
            return source.Select((item, index) => (item, index));
        }
    }

    class Program
    {
        static void Main()
        {
            // 定义文法
            var grammar = new Dictionary<string, List<List<string>>>
            {
                {
                    "S",

                    [
                        ["NP", "VP"],
                    ]
                },
                {
                    "NP",

                    [
                        ["Det", "N"],
                        ["NP", "PP"],
                    ]
                },
                {
                    "VP",

                    [
                        ["V", "NP"],
                        ["VP", "PP"],
                    ]
                },
                {
                    "PP",

                    [
                        ["P", "NP"],
                    ]
                },
                {
                    "Det",

                    [
                        ["the"],
                        ["a"],
                    ]
                },
                {
                    "N",

                    [
                        ["cat"],
                        ["dog"],
                        ["telescope"],
                        ["park"],
                    ]
                },
                {
                    "V",

                    [
                        ["saw"],
                        ["walked"],
                    ]
                },
                {
                    "P",

                    [
                        ["in"],
                        ["with"],
                    ]
                },
            };

            // 消除epsilon产生式
            var (newGrammar, newProductions) = RemoveEpsilons(grammar);

            // 收集非终结符和终结符
            var nonTerminals = new HashSet<string>(newGrammar.Keys);
            var terminals = new HashSet<string>(newGrammar.Values.SelectMany(prods => prods.SelectMany(prod => prod)).Where(sym => !nonTerminals.Contains(sym)));

            // 输入句子
            var sentence = "the dog saw a cat in the park";
            var tokens = sentence.Split(' ');

            // 解析句子从 'S' 开始
            var memo = new Dictionary<(string, int, int), List<Node>>();
            var trees = Parse(newGrammar, tokens, nonTerminals, terminals, memo, "S", 0, tokens.Length);

            // 打印所有可能的解析树
            for (int idx = 0; idx < trees.Count; idx++)
            {
                Console.WriteLine($"Parse tree {idx + 1}:");
                Console.WriteLine(TreeToString("", trees[idx]));
                Console.WriteLine($"Typst tree code {idx + 1}:");
                Console.WriteLine($"#{TreeToTypst("", trees[idx])}");
            }
        }

        static (Dictionary<string, List<List<string>>>, List<(string, List<string>)>) RemoveEpsilons(Dictionary<string, List<List<string>>> grammar)
        {
            var epsNonterms = FindEpsNonterms(grammar);
            var newGrammar = new Dictionary<string, List<List<string>>>();

            foreach (var (lhs, rhsList) in grammar)
            {
                var newRhsList = new List<List<string>>();
                foreach (var rhs in rhsList)
                {
                    if (rhs.SequenceEqual(["ε"]))
                        continue;

                    var newRhs = RemoveEpsFromRhs(rhs, epsNonterms);
                    newRhsList.AddRange(newRhs);
                }
                newGrammar[lhs] = newRhsList;
            }

            var newProductions = GenerateNewProductions(newGrammar, epsNonterms);
            return (newGrammar, newProductions);
        }

        static HashSet<string> FindEpsNonterms(Dictionary<string, List<List<string>>> grammar)
        {
            var epsNonterms = new HashSet<string>();
            foreach (var (lhs, rhsList) in grammar)
            {
                if (rhsList.Any(rhs => rhs.SequenceEqual(["ε"])))
                {
                    epsNonterms.Add(lhs);
                }
            }
            return epsNonterms;
        }

        static List<List<string>> RemoveEpsFromRhs(List<string> rhsList, HashSet<string> epsNonterms)
        {
            if (rhsList.Count == 0)
                return [[]];

            var hd = rhsList.First();
            var tl = rhsList.Skip(1).ToList();
            var rest = RemoveEpsFromRhs(tl, epsNonterms);

            var result = new List<List<string>>();
            if (epsNonterms.Contains(hd))
            {
                result.AddRange(rest);
                result.AddRange(rest.Select(r => new List<string> { hd }.Concat(r).ToList()));
            }
            else
            {
                result.AddRange(rest.Select(r => new List<string> { hd }.Concat(r).ToList()));
            }
            return result;
        }

        static List<(string, List<string>)> GenerateNewProductions(Dictionary<string, List<List<string>>> grammar, HashSet<string> epsNonterms)
        {
            var newProductions = new List<(string, List<string>)>();
            foreach (var (lhs, rhsList) in grammar)
            {
                foreach (var rhs in rhsList)
                {
                    if (rhs.Any(sym => epsNonterms.Contains(sym)))
                    {
                        var newRhs = rhs.Where(sym => !epsNonterms.Contains(sym)).ToList();
                        if (newRhs.Count != 0 && !newProductions.Contains((lhs, newRhs)))
                        {
                            newProductions.Add((lhs, newRhs));
                        }
                    }
                }
            }
            return newProductions;
        }

        static List<Node> Parse(
            Dictionary<string, List<List<string>>> grammar,
            string[] tokens,
            HashSet<string> nonTerminals,
            HashSet<string> terminals,
            Dictionary<(string, int, int), List<Node>> memo,
            string nt,
            int i,
            int j)
        {
            var key = (nt, i, j);
            if (memo.TryGetValue(key, out List<Node>? value))
                return value;

            var results = new List<Node>();
            if (i >= j)
                return results;

            if (terminals.Contains(nt))
            {
                if (i + 1 == j && tokens[i] == nt)
                {
                    results.Add(new Leaf(nt));
                }
            }
            else if (nonTerminals.Contains(nt))
            {
                var rules = grammar[nt];
                foreach (var production in rules)
                {
                    var n = production.Count;
                    if (n == 1)
                    {
                        var symbol = production[0];
                        if (terminals.Contains(symbol))
                        {
                            if (i + 1 == j && tokens[i] == symbol)
                            {
                                results.Add(new Internal(nt, [new Leaf(symbol)]));
                            }
                        }
                        else
                        {
                            var subTrees = Parse(grammar, tokens, nonTerminals, terminals, memo, symbol, i, j);
                            results.AddRange(subTrees.Select(subTree => new Internal(nt, [subTree])));
                        }
                    }
                    else
                    {
                        foreach (var splits in PossibleSplits(i, j, n))
                        {
                            var positions = new List<int> { i }.Concat(splits).Concat([j]).ToList();
                            var children = new List<List<Node>>();
                            var failed = false;

                            for (int idx = 0; idx < n; idx++)
                            {
                                var ai = production[idx];
                                int start = positions[idx];
                                int end = positions[idx + 1];
                                var subTrees = Parse(
                                    grammar,
                                    tokens,
                                    nonTerminals,
                                    terminals,
                                    memo,
                                    ai,
                                    start,
                                    end
                                );
                                if (subTrees.Count == 0)
                                {
                                    failed = true;
                                    break;
                                }
                                children.Add(subTrees);
                            }
                            if (!failed)
                            {
                                var combinations = CartesianProduct(children);
                                foreach (var combination in combinations)
                                {
                                    results.Add(new Internal(nt, combination));
                                }
                            }
                        }
                    }
                }
            }

            memo[key] = results;
            return results;
        }

        static List<List<int>> PossibleSplits(int i, int j, int n)
        {
            if (n == 1)
            {
                return i < j ? [new()] : [];
            }
            else
            {
                var positions = Enumerable.Range(i + 1, j - i - 1).ToList();
                return Combinations([], positions, n - 1);
            }
        }

        static List<List<int>> Combinations(List<int> acc, List<int> remainingPositions, int k)
        {
            if (k == 0)
            {
                return [acc];
            }
            else
            {
                var result = new List<List<int>>();
                for (int idx = 0; idx < remainingPositions.Count; idx++)
                {
                    var pos = remainingPositions[idx];
                    var newPositions = remainingPositions.Skip(idx + 1).ToList();
                    var newAcc = new List<int>(acc) { pos };
                    var subCombinations = Combinations(newAcc, newPositions, k - 1);
                    result.AddRange(subCombinations);
                }
                return result;
            }
        }

        static List<List<Node>> CartesianProduct(List<List<Node>> lists)
        {
            IEnumerable<IEnumerable<Node>> result = [[]];
            foreach (var list in lists)
            {
                result = from seq in result
                         from item in list
                         select seq.Concat([item]);
            }
            return result.Select(seq => seq.ToList()).ToList();
        }

        static string TreeToString(string indent, Node tree)
        {
            switch (tree)
            {
                case Leaf leaf:
                    return $"{indent}{leaf.Value}\n";
                case Internal internalNode:
                    var result = $"{indent}{internalNode.Label}\n";
                    foreach (var child in internalNode.Children)
                    {
                        result += TreeToString(indent + "  ", child);
                    }
                    return result;
                default:
                    throw new InvalidOperationException("Unknown node type");
            }
        }

        static string TreeToTypst(string indent, Node tree)
        {
            switch (tree)
            {
                case Leaf leaf:
                    return $"{indent}tree(\"{leaf.Value}\")";
                case Internal internalNode:
                    var newIndent = indent + "  ";
                    var childrenTypst = internalNode.Children.Select(child => TreeToTypst(newIndent, child)).ToList();
                    var childrenStr = string.Join(",\n", childrenTypst);
                    return $"{indent}tree(\"{internalNode.Label}\",\n{childrenStr}\n{indent})";
                default:
                    throw new InvalidOperationException("Unknown node type");
            }
        }
    }
}