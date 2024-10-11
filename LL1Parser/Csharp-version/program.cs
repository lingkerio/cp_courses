using System;
using System.Collections.Generic;

class Program
{
    // 定义文法规则
    private delegate Dictionary<string, List<List<string>>> Grammar();

    static void Main()
    {
        var grammar = new Dictionary<string, List<List<string>>>
        {
            {
                "E",

                [
                    ["T", "E'"],
                ]
            },
            {
                "E'",

                [
                    ["ADD_SUB", "T", "E'"],
                    [""],
                ]
            },
            {
                "T",

                [
                    ["F", "T'"],
                ]
            },
            {
                "T'",

                [
                    ["MUL_DIV", "F", "T'"],
                    [""],
                ]
            },
            {
                "F",

                [
                    ["number"],
                    ["(", "E", ")"],
                ]
            },
            {
                "ADD_SUB",

                [
                    ["+"],
                    ["-"],
                ]
            },
            {
                "MUL_DIV",

                [
                    ["*"],
                    ["div"],
                    ["mod"],
                ]
            },
        };

        var startSymbol = "E";

        var first = ComputeFirst(grammar);
        var follow = ComputeFollow(grammar, startSymbol);

        // 打印 First 和 Follow 集合
        Console.WriteLine("First 集合: ");
        foreach (var item in first)
        {
            Console.WriteLine($"{item.Key}: {string.Join(", ", item.Value)}");
        }

        Console.WriteLine("Follow 集合: ");
        foreach (var item in follow)
        {
            Console.WriteLine($"{item.Key}: {string.Join(", ", item.Value)}");
        }

        var parsingTable = BuildParsingTable(grammar, first, follow);

        // 打印预测分析表
        Console.WriteLine("预测分析表: ");
        foreach (var entry in parsingTable)
        {
            Console.WriteLine(
                $"({entry.Key.Item1}, {entry.Key.Item2}) -> {string.Join(" ", entry.Value)}"
            );
        }

        var input = "number mod number div number * number".Split(' ').ToList();
        var success = Parse(grammar, parsingTable, startSymbol, input);

        Console.WriteLine($"解析结果: {(success ? "成功" : "失败")}");
    }

    // 计算First集合
    static Dictionary<string, HashSet<string>> ComputeFirst(
        Dictionary<string, List<List<string>>> grammar
    )
    {
        var first = new Dictionary<string, HashSet<string>>();
        foreach (var nonTerminal in grammar.Keys)
        {
            first[nonTerminal] = [];
        }

        bool changed = true;
        while (changed)
        {
            changed = false;
            foreach (var nonTerminal in grammar.Keys)
            {
                var productions = grammar[nonTerminal];
                foreach (var production in productions)
                {
                    if (UpdateFirstSet(grammar, first, nonTerminal, production))
                    {
                        changed = true;
                    }
                }
            }
        }

        return first;
    }

    static bool UpdateFirstSet(
        Dictionary<string, List<List<string>>> grammar,
        Dictionary<string, HashSet<string>> first,
        string nonTerminal,
        List<string> production
    )
    {
        bool changed = false;
        foreach (var symbol in production)
        {
            if (grammar.ContainsKey(symbol))
            {
                var firstSet = new HashSet<string>(first[symbol]);
                int lenBefore = first[nonTerminal].Count;
                first[nonTerminal].UnionWith(firstSet);
                int lenAfter = first[nonTerminal].Count;
                if (lenBefore != lenAfter)
                    changed = true;
                if (!firstSet.Contains(""))
                    break;
            }
            else
            {
                int lenBefore = first[nonTerminal].Count;
                first[nonTerminal].Add(symbol);
                int lenAfter = first[nonTerminal].Count;
                if (lenBefore != lenAfter)
                    changed = true;
                break;
            }
        }
        return changed;
    }

    // 计算Follow集合
    static Dictionary<string, HashSet<string>> ComputeFollow(
        Dictionary<string, List<List<string>>> grammar,
        string startSymbol
    )
    {
        var follow = new Dictionary<string, HashSet<string>>();
        foreach (var nonTerminal in grammar.Keys)
        {
            follow[nonTerminal] = [];
        }

        follow[startSymbol].Add("$");

        bool changed = true;
        while (changed)
        {
            changed = false;
            foreach (var nonTerminal in grammar.Keys)
            {
                var productions = grammar[nonTerminal];
                foreach (var production in productions)
                {
                    if (UpdateFollowSet(grammar, follow, nonTerminal, production))
                    {
                        changed = true;
                    }
                }
            }
        }

        return follow;
    }

    static bool UpdateFollowSet(
        Dictionary<string, List<List<string>>> grammar,
        Dictionary<string, HashSet<string>> follow,
        string nonTerminal,
        List<string> production
    )
    {
        bool changed = false;
        for (int i = 0; i < production.Count; i++)
        {
            var symbol = production[i];
            if (grammar.ContainsKey(symbol))
            {
                var followSet = new HashSet<string>(follow[symbol]);
                if (i + 1 < production.Count)
                {
                    var nextSymbol = production[i + 1];
                    if (grammar.ContainsKey(nextSymbol))
                    {
                        var firstSet = ComputeFirst(grammar)[nextSymbol];
                        followSet.UnionWith(firstSet);
                        followSet.Remove("");
                        if (firstSet.Contains(""))
                        {
                            followSet.UnionWith(follow[nonTerminal]);
                        }
                    }
                    else
                    {
                        followSet.Add(nextSymbol);
                    }
                }
                else
                {
                    followSet.UnionWith(follow[nonTerminal]);
                }

                int lenBefore = follow[symbol].Count;
                follow[symbol].UnionWith(followSet);
                int lenAfter = follow[symbol].Count;
                if (lenBefore != lenAfter)
                    changed = true;
            }
        }
        return changed;
    }

    // 构建预测分析表
    static Dictionary<(string, string), List<string>> BuildParsingTable(
        Dictionary<string, List<List<string>>> grammar,
        Dictionary<string, HashSet<string>> first,
        Dictionary<string, HashSet<string>> follow
    )
    {
        var table = new Dictionary<(string, string), List<string>>();

        foreach (var nonTerminal in grammar.Keys)
        {
            var productions = grammar[nonTerminal];
            foreach (var production in productions)
            {
                var firstSet = ComputeFirstForProduction(production, first);
                foreach (var terminal in firstSet)
                {
                    if (terminal != "")
                    {
                        table[(nonTerminal, terminal)] = production;
                    }
                }
                if (firstSet.Contains(""))
                {
                    foreach (var terminal in follow[nonTerminal])
                    {
                        table[(nonTerminal, terminal)] = production;
                        Console.WriteLine($"{nonTerminal} --- {terminal} {production}");
                    }
                    if (follow[nonTerminal].Contains("$"))
                    {
                        table[(nonTerminal, "$")] = production;
                    }
                }
            }
        }

        return table;
    }

    static HashSet<string> ComputeFirstForProduction(
        List<string> production,
        Dictionary<string, HashSet<string>> first
    )
    {
        var result = new HashSet<string>();
        foreach (var symbol in production)
        {
            if (first.TryGetValue(symbol, out HashSet<string>? value))
            {
                result.UnionWith(value);
                if (!value.Contains(""))
                    break;
            }
            else
            {
                result.Add(symbol);
                break;
            }
        }
        return result;
    }

    // LL(1) 解析器
    static bool Parse(
        Dictionary<string, List<List<string>>> grammar,
        Dictionary<(string, string), List<string>> parsingTable,
        string startSymbol,
        List<string> input
    )
    {
        var stack = new Stack<string>();
        stack.Push("$");
        stack.Push(startSymbol);
        var inputIter = input.GetEnumerator();
        inputIter.MoveNext();
        var lookahead = inputIter.Current;

        Console.WriteLine("初始栈: " + string.Join(", ", stack));
        Console.WriteLine("初始输入: " + string.Join(", ", input));

        while (stack.Count > 0)
        {
            var top = stack.Pop();
            Console.WriteLine($"栈顶: {top}");

            if (!grammar.ContainsKey(top))
            {
                if (top == lookahead)
                {
                    Console.WriteLine($"匹配终结符: {lookahead}");
                    if (inputIter.MoveNext())
                    {
                        lookahead = inputIter.Current;
                    }
                    else
                    {
                        lookahead = "$";
                    }
                }
                else
                {
                    Console.WriteLine($"错误：栈顶与输入不匹配 ({top} != {lookahead})");
                    return false;
                }
            }
            else
            {
                if (parsingTable.TryGetValue((top, lookahead), out var production))
                {
                    Console.WriteLine($"使用产生式: {top} -> {string.Join(" ", production)}");
                    for (int i = production.Count - 1; i >= 0; i--)
                    {
                        if (production[i] != "")
                        {
                            stack.Push(production[i]);
                        }
                    }
                }
                else
                {
                    Console.WriteLine($"错误：没有匹配的产生式 ({top}, {lookahead})");
                    return false;
                }
            }

            Console.WriteLine("当前栈: " + string.Join(", ", stack));
        }

        return lookahead == "$";
    }
}
