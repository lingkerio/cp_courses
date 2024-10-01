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

            // 收集非终结符和终结符
            var nonTerminals = new HashSet<string>(grammar.Keys);
            var terminals = new HashSet<string>();
            foreach (var prods in grammar.Values)
            {
                foreach (var prod in prods)
                {
                    foreach (var sym in prod)
                    {
                        if (!nonTerminals.Contains(sym))
                        {
                            terminals.Add(sym);
                        }
                    }
                }
            }

            // 输入句子
            string sentence = "the dog saw a cat in the park";
            string[] tokens = sentence.Split(' ');

            // 解析句子
            var memo = new Dictionary<(string, int, int), List<Node>>();
            var trees = Parse(
                grammar,
                tokens,
                nonTerminals,
                terminals,
                memo,
                "S",
                0,
                tokens.Length
            );

            // 输出所有可能的解析树
            foreach (var (tree, idx) in trees.Enumerate())
            {
                Console.WriteLine($"解析树 {idx + 1}:");
                Console.Write(TreeToString("", tree));
            }
        }

        static string TreeToString(string indent, Node tree)
        {
            switch (tree)
            {
                case Leaf leaf:
                    return $"{indent}{leaf.Value}\n";
                case Internal internalNode:
                    string result = $"{indent}{internalNode.Label}\n";
                    foreach (var child in internalNode.Children)
                    {
                        result += TreeToString(indent + "  ", child);
                    }
                    return result;
                default:
                    throw new Exception("未知的节点类型");
            }
        }

        static List<List<int>> PossibleSplits(int i, int j, int n)
        {
            if (n == 1)
            {
                if (i < j)
                {
                    return
                    [
                        [],
                    ];
                }
                else
                {
                    return [];
                }
            }
            else
            {
                var positions = Enumerable.Range(i + 1, j - i - 1).ToList();
                return Combinations(positions, n - 1);
            }
        }

        static List<List<int>> Combinations(List<int> positions, int k)
        {
            return CombinationsHelper(positions, 0, k);
        }

        static List<List<int>> CombinationsHelper(List<int> positions, int start, int k)
        {
            var results = new List<List<int>>();
            if (k == 0)
            {
                results.Add([]);
            }
            else
            {
                for (int i = start; i < positions.Count; i++)
                {
                    int pos = positions[i];
                    var restCombinations = CombinationsHelper(positions, i + 1, k - 1);
                    foreach (var comb in restCombinations)
                    {
                        var newComb = new List<int> { pos };
                        newComb.AddRange(comb);
                        results.Add(newComb);
                    }
                }
            }
            return results;
        }

        static List<List<T>> CartesianProduct<T>(List<List<T>> sequences)
        {
            var result = new List<List<T>> { new() };
            foreach (var sequence in sequences)
            {
                var temp = new List<List<T>>();
                foreach (var acc in result)
                {
                    foreach (var item in sequence)
                    {
                        var newList = new List<T>(acc) { item };
                        temp.Add(newList);
                    }
                }
                result = temp;
            }
            return result;
        }

        static List<List<string>> LookupRules(
            Dictionary<string, List<List<string>>> grammar,
            string nt
        )
        {
            if (grammar.TryGetValue(nt, out var rules))
            {
                return rules;
            }
            else
            {
                return [];
            }
        }

        static List<Node> Parse(
            Dictionary<string, List<List<string>>> grammar,
            string[] tokens,
            HashSet<string> nonTerminals,
            HashSet<string> terminals,
            Dictionary<(string, int, int), List<Node>> memo,
            string nt,
            int i,
            int j
        )
        {
            var key = (nt, i, j);
            if (memo.TryGetValue(key, out var result))
            {
                return result;
            }

            var results = new List<Node>();

            if (i >= j)
            {
                // 无结果
            }
            else if (terminals.Contains(nt))
            {
                if (i + 1 == j && tokens[i] == nt)
                {
                    results.Add(new Leaf(nt));
                }
            }
            else if (nonTerminals.Contains(nt))
            {
                var rules = LookupRules(grammar, nt);
                foreach (var production in rules)
                {
                    int n = production.Count;
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
                            var subTrees = Parse(
                                grammar,
                                tokens,
                                nonTerminals,
                                terminals,
                                memo,
                                symbol,
                                i,
                                j
                            );
                            foreach (var subTree in subTrees)
                            {
                                results.Add(new Internal(nt, [subTree]));
                            }
                        }
                    }
                    else
                    {
                        var splitsList = PossibleSplits(i, j, n);
                        foreach (var splits in splitsList)
                        {
                            var positions = new List<int> { i };
                            positions.AddRange(splits);
                            positions.Add(j);

                            var children = new List<List<Node>>();
                            bool failed = false;
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
    }
}
