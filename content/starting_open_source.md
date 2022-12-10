# My experience starting with open source

This first/second article is the one that made me want to actually put in the effort to create this blog. I recently decided to contribute to some open source projects and I took the advice I had been given on youtube for getting started with open source and I made a contribution to... Yep! the documentation. 

Kind of underwhelming, I know, but it was defnitely a good way to start because it didn't cause me stress worrying if it was high enough quality or not and I leared how to actually open and work a pull request. And in general any new project that I contribute to I will probably first contribute to the documentation first. 

For me this has multiple benefits. One is that I can get a feel for how that particular project is done. Maybe they have a particular merge strategy and they would prefer if all of the commits are squashed. This is a nice thing to just get out of the way. 

But the other reason is probably the bigger reason. I don't want to spend a chunk of my time working on a project when the maintainers are hostile. Luckily this has never happened to me and it doesn't seem common. But even still I don't want to have to deal with that.

## Motivation - why did I want to

## How/ when did I decide to start
I decided to start contributing to open source projects because I was going to be applying for summer internships and I was hoping that open source contributions would look good on a resume and in interviews. And that was honestly the main motivation for the first PR. Since then I have found that I genuinely enjoy the challenge of working on new projects and unfamiliar codebases. I enjoy seeing other people use and improve on my changes and it is definitely a great way to learn more. 
### This is an h3

# Now this is a title

## What were the challenges getting started

``` rust
fn parse_expression(lexer: &mut PeekLex, prec: Precedence) -> ParseResult<Expr> {
    use TokenKind::*;

    let mut left_exp = parse_left_expression(lexer)?;

    let mut peek_op_token = match lexer.peek().cloned() {
        Some(token) => token,
        None => return Ok(left_exp),
    };
    while prec < peek_op_token.kind.precedence() {
        left_exp = match peek_op_token.kind {
            // All binary tokens should be matched here. This includes LParen but LParen will match
            // to a different function because it is the start of a function call which needs
            // a few more checks than just regular binary expressions
            Plus | Minus | Slash | Asterisk | Eq | Ne | LT | GT | Assign | BitOr | Or | BitAnd
            | And => parse_binary_expression(lexer, left_exp)?,

            LParen => parse_call_expression(lexer, left_exp)?,

            LBracket => parse_array_index(lexer, left_exp)?,

            Dot => parse_method_expression(lexer, left_exp)?,

            _ => Err(Report::new(ParseError::UnexpectedToken(peek_op_token))
                .attach_printable("Expected a binary operator"))?,
        };
        peek_op_token = match lexer.peek().cloned() {
            Some(token) => token,
            None => return Ok(left_exp),
        };
    }

    // inner expressions should never be terminated so this check to match_semicolon
    // checks if the calling function wants the expression to have the option of
    // being terminated or not.
    Ok(left_exp)
}
```

``` c
int main() {
    int x = 3;
    return x;
}
````

``` java
public static void main(String[] args) {
    auto x = new ArrayList<Date>();
    return;
}
```

## What were the scary things

## What have been the results

## What did I learn

``` date
    12/04/2022
```
