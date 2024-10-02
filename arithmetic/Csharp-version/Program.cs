using System;
using System.Collections.Generic;
using System.Text.RegularExpressions;

enum TokenType
{
    Number,
    Operator,
    LParen,
    RParen
}

class Token(TokenType type, string value)
{
    public TokenType Type { get; } = type;
    public string Value { get; } = value;

    public override string ToString() => $"{Type}: {Value}";
}

class Lexer(string input)
{
    private int _position = 0;

    public List<Token> Tokenize()
    {
        var tokens = new List<Token>();
        while (_position < input.Length)
        {
            var currentChar = input[_position];

            if (char.IsWhiteSpace(currentChar))
            {
                _position++;
                continue;
            }

            if (char.IsDigit(currentChar))
            {
                int start = _position;
                while (_position < input.Length && char.IsDigit(input[_position]))
                {
                    _position++;
                }
                var number = input[start.._position];
                tokens.Add(new Token(TokenType.Number, number));
                continue;
            }

            if ("+-*/()".Contains(currentChar))
            {
                tokens.Add(new Token(currentChar == '(' ? TokenType.LParen : currentChar == ')' ? TokenType.RParen : TokenType.Operator, currentChar.ToString()));
                _position++;
                continue;
            }

            throw new Exception($"Unexpected character: {currentChar}");
        }

        return tokens;
    }
}

abstract class Expr
{
    public class Number(int value) : Expr
    {
        public int Value { get; } = value;
    }

    public class Binary(Token op, Expr left, Expr right) : Expr
    {
        public Token Operator { get; } = op;
        public Expr Left { get; } = left;
        public Expr Right { get; } = right;
    }
}

class Parser(List<Token> tokens)
{
    private int _position = 0;

    private Token? CurrentToken => _position < tokens.Count ? tokens[_position] : null;


    private Token Consume(TokenType expectedType)
    {
        var token = CurrentToken;
        if (token == null || token.Type != expectedType)
            throw new Exception($"Expected token of type {expectedType}, but got {token?.Type}");

        _position++;
        return token;
    }

    public Expr Parse()
    {
        return Expression();
    }

    private Expr Expression()
    {
        var node = Term();
        while (CurrentToken != null && (CurrentToken.Value == "+" || CurrentToken.Value == "-"))
        {
            var op = Consume(TokenType.Operator);
            var rightNode = Term();
            node = new Expr.Binary(op, node, rightNode);
        }
        return node;
    }

    private Expr Term()
    {
        var node = Factor();
        while (CurrentToken != null && (CurrentToken.Value == "*" || CurrentToken.Value == "/"))
        {
            var op = Consume(TokenType.Operator);
            var rightNode = Factor();
            node = new Expr.Binary(op, node, rightNode);
        }
        return node;
    }

    private Expr Factor()
    {
        var token = CurrentToken ?? throw new Exception("Unexpected end of input");
        if (token.Type == TokenType.Number)
        {
            Consume(TokenType.Number);
            return new Expr.Number(int.Parse(token.Value));
        }

        if (token.Type == TokenType.LParen)
        {
            Consume(TokenType.LParen);
            var node = Expression();
            Consume(TokenType.RParen);
            return node;
        }

        throw new Exception("Invalid syntax");
    }
}

class Program
{
    static int Evaluate(Expr node)
    {
        switch (node)
        {
            case Expr.Number number:
                return number.Value;
            case Expr.Binary binary:
                var leftVal = Evaluate(binary.Left);
                var rightVal = Evaluate(binary.Right);
                return binary.Operator.Value switch
                {
                    "+" => leftVal + rightVal,
                    "-" => leftVal - rightVal,
                    "*" => leftVal * rightVal,
                    "/" => leftVal / rightVal,
                    _ => throw new Exception("Unexpected operator")
                };
            default:
                throw new Exception("Unexpected expression");
        }
    }

    static int Calculate(string input)
    {
        var lexer = new Lexer(input);
        var tokens = lexer.Tokenize();
        var parser = new Parser(tokens);
        var ast = parser.Parse();
        return Evaluate(ast);
    }

    static void Main()
    {
        var result = Calculate("3 + 5 * (2 - 8)");
        Console.WriteLine(result); // 输出计算结果
    }
}