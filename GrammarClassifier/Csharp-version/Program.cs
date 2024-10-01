using System;
using System.Collections.Generic;
using System.Linq;

namespace GrammarClassifier
{
    enum SymbolType
    {
        Terminal,
        NonTerminal
    }

    class Symbol(SymbolType type, string value)
    {
        public SymbolType Type { get; } = type;
        public string Value { get; } = value;

        public override string ToString() => Value;
    }

    class Production(List<Symbol> lhs, List<Symbol> rhs)
    {
        public List<Symbol> Lhs { get; } = lhs;
        public List<Symbol> Rhs { get; } = rhs;
    }

    class Program
    {
        static Symbol ParseSymbol(string s, List<Symbol> nonTerminals, List<Symbol> terminals)
        {
            if (nonTerminals.Any(nt => nt.Value == s))
            {
                return new Symbol(SymbolType.NonTerminal, s);
            }
            else if (terminals.Any(t => t.Value == s))
            {
                return new Symbol(SymbolType.Terminal, s);
            }
            else
            {
                throw new Exception($"Unknown symbol: {s}");
            }
        }

        static Production ParseProduction((List<string> lhs, List<string> rhs) production, List<Symbol> nonTerminals, List<Symbol> terminals)
        {
            var lhsSymbols = production.lhs.Select(s => ParseSymbol(s, nonTerminals, terminals)).ToList();
            var rhsSymbols = production.rhs.Select(s => ParseSymbol(s, nonTerminals, terminals)).ToList();
            return new Production(lhsSymbols, rhsSymbols);
        }

        static List<Production> ParseProductions(List<(List<string> lhs, List<string> rhs)> productions, List<Symbol> nonTerminals, List<Symbol> terminals)
        {
            return productions.Select(p => ParseProduction(p, nonTerminals, terminals)).ToList();
        }

        static bool IsLeftLinear(List<Production> productions)
        {
            return productions.All(p => p.Lhs.Count == 1 && p.Lhs[0].Type == SymbolType.NonTerminal &&
                                        (p.Rhs.Count == 1 && p.Rhs[0].Type == SymbolType.Terminal ||
                                         p.Rhs.Count >= 2 && p.Rhs.Last().Type == SymbolType.NonTerminal && p.Rhs.Take(p.Rhs.Count - 1).All(s => s.Type == SymbolType.Terminal)));
        }

        static bool IsRightLinear(List<Production> productions)
        {
            return productions.All(p => p.Lhs.Count == 1 && p.Lhs[0].Type == SymbolType.NonTerminal &&
                                        (p.Rhs.Count == 1 && p.Rhs[0].Type == SymbolType.Terminal ||
                                         p.Rhs.Count >= 2 && p.Rhs.First().Type == SymbolType.Terminal && p.Rhs.Skip(1).All(s => s.Type == SymbolType.NonTerminal)));
        }

        static bool IsRegularGrammar(List<Production> productions)
        {
            return productions.All(p => p.Lhs.Count == 1 && p.Lhs[0].Type == SymbolType.NonTerminal &&
                                        (p.Rhs.Count == 1 && p.Rhs[0].Type == SymbolType.Terminal ||
                                         p.Rhs.Count == 2 && p.Rhs[0].Type == SymbolType.Terminal && p.Rhs[1].Type == SymbolType.NonTerminal));
        }

        static bool IsType2(List<Production> productions)
        {
            return productions.All(p => p.Lhs.Count == 1 && p.Lhs[0].Type == SymbolType.NonTerminal);
        }

        static bool IsType1(List<Production> productions)
        {
            return productions.All(p => p.Lhs.Count >= 1 && p.Rhs.Count >= 1 && p.Lhs.Count <= p.Rhs.Count);
        }

        static string ClassifyGrammar(List<Symbol> nonTerminals, List<Symbol> terminals, List<(List<string> lhs, List<string> rhs)> productions)
        {
            var parsedProductions = ParseProductions(productions, nonTerminals, terminals);
            if (IsRegularGrammar(parsedProductions))
            {
                return "正则文法";
            }
            else if (IsRightLinear(parsedProductions))
            {
                return "右线性文法";
            }
            else if (IsLeftLinear(parsedProductions))
            {
                return "左线性文法";
            }
            else if (IsType1(parsedProductions))
            {
                return "上下文有关文法";
            }
            else if (IsType2(parsedProductions))
            {
                return "上下文无关文法";
            }
            else
            {
                return "未知文法";
            }
        }

        static List<Symbol> ReadSymbols(string prompt)
        {
            Console.Write($"{prompt} (用空格分隔): ");
            var line = Console.ReadLine();
            if (string.IsNullOrEmpty(line))
            {
                throw new Exception("输入不能为空");
            }
            return line.Split(' ')
                       .Select(s => new Symbol(char.IsUpper(s[0]) ? SymbolType.NonTerminal : SymbolType.Terminal, s))
                       .ToList();
        }

        static List<(List<string> lhs, List<string> rhs)> ReadProductions()
        {
            var productions = new List<(List<string> lhs, List<string> rhs)>();
            while (true)
            {
                Console.Write("输入产生式 (格式: LHS -> RHS，用空格分隔，输入空行结束): ");
                var line = Console.ReadLine();
                if (string.IsNullOrEmpty(line))
                {
                    break;
                }
                line = line.Trim();
                var parts = line.Split("->");
                if (parts.Length != 2)
                {
                    Console.WriteLine("格式错误，请重新输入。");
                    continue;
                }
                var lhs = parts[0].Trim().Split(' ').ToList();
                var rhs = parts[1].Trim().Split(' ').ToList();
                productions.Add((lhs, rhs));
            }
            return productions;
        }

        static void Main()
        {
            var nonTerminals = ReadSymbols("输入非终结符");
            var terminals = ReadSymbols("输入终结符");
            var productions = ReadProductions();
            Console.Write("输入开始符号: ");
            var startSymbolInput = Console.ReadLine()?.Trim();
            if (string.IsNullOrEmpty(startSymbolInput))
            {
                throw new Exception("输入不能为空");
            }
            var startSymbol = nonTerminals.FirstOrDefault(nt => nt.Value == startSymbolInput) ?? throw new Exception($"Unknown start symbol: {startSymbolInput}");
            var grammarType = ClassifyGrammar(nonTerminals, terminals, productions);
            Console.WriteLine($"文法类型: {grammarType}");
        }
    }
}
