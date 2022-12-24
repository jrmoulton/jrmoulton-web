# My experience starting with open source

Ultimately, this article is the culprit.

It isn't guilty of forcing me to to start my open source work, or even to write about it... but
rather of making me spend a ton of time building a site generator to make it look pretty

And boy does it look pretty.

This is the article that made me want to actually put in the effort to create this blog (the
content I mean, not necessarily the the site generator) and this is the fateful story.

## How it happened

I recently decided to contribute to some open source projects and I took the advice I had been given
on youtube for getting started with open source and I made a contribution to... Drum Rooollll...
<br> Yep! the documentation. 

Kind of underwhelming, I know, but it was defnitely a good way to start. It didn't cause me stress
worrying if it was high enough quality and I leared how to actually open and work a pull request. 

I'm highly suspicious that in the future any new project that I contribute will probably first
receive a new conributor to the documentation.

The image below is the first PR that I ever opened. It wasn't perfect (later corrections were made)
and it wasn't any too special but it was honest work.
![Time First PR](/images/first_pr_top.png)
Basically when I was using the slint (then sixtyfps) framework I found a part of the documentation
that was confusing to me and I opened a PR that made some clarifications. 

### The result!
![First PR result](/images/first_pr_middle.png)

#### And some corrections...
![First PR Corrections](/images/first_pr_bottom.png)


The slint team members were super nice to work with

### Why docs??

For me, this has multiple benefits. One is that I can get a feel for how that particular project is
done. Maybe they have a particular merge strategy and they would prefer if all of the commits are
squashed. These are just nice things to just get out of the way. 

But the other reason is probably the bigger reason

I don't want to spend a chunk of my time working on a project when the maintainers are hostile.
Luckily this has never happened to me and it doesn't seem common but I have seen it. 

I defnitely don't want to have to deal with that.

## But why at all?

I decided to start contributing to open source projects because I
was going to be applying for summer internships and I was hoping that open source contributions
would look good on a resume and in interviews. And that was honestly the main motivation for the
first PR. Since then I have found that I genuinely enjoy the challenge of working on new projects
and unfamiliar codebases. I enjoy seeing other people use and improve on my changes and it is
definitely a great way to learn more. 

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

### Some edits

``` date
    12/17/2022
```
